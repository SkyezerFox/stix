//! Defines data structures for various operation types within the language.

use std::{error::Error, str::FromStr};

use crate::{Expr, Ident, Node};

/// Enum representing operator associativity.
///
/// Some operators are evaluated from left-to-right, while others are evaluated from right-to-left.
/// This property is known as an operator's associativity. In order for the compiler to correctly
/// generate machine code that performs as expected, the associativity of each operator must be defined
/// in the language specification.
///
/// This enum contains two values:
/// - `Associativity::Left`: The left-to-right associativity.
/// - `Associativity::Right`: The right-to-left associativity.
///
/// Each operator is then matched to either one of these options, and compiled as such.
#[derive(Debug, PartialEq)]
pub enum Associativity {
    /// Left-to-right associativity.
    Ltr,
    /// Right-to-left associativity.
    Rtl,
}

/// Enum representing unary operator types.
///
/// Unary operators are operators that act on a single argument, such as `x++`, or `!x`.
#[derive(Debug, PartialEq)]
pub enum UnaryExprKind {
    /// The suffix increment operator, `++`.
    Increment,
    /// The suffix decrement operator, `--`.
    Decrement,
    /// The prefix increment operator, `++`.
    /// The index operator, `[n]`
    Index(u64),
    /// The address-of operator, `&`.
    AddressOf,
    /// The bitwise not operator, `~`.
    BitwiseNot,
    /// The logical not operator, `!`.
    LogicalNot,
    /// The de-reference operator, `*`.
    Dereference,
    /// The negation operator, `-`.
    Negation,
    /// The call operator, `()`.
    Call(Vec<Expr>),
}

impl FromStr for UnaryExprKind {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnaryExprKind::*;

        // match index operator
        if s.starts_with("[") && s.ends_with("]") {
            let mut chars = s.chars();
            chars.next();
            chars.next_back();
            let inner: String = chars.collect();
            let index: u64 = inner.parse::<u64>().unwrap_or(0);
            return Ok(Index(index));
        }

        match s {
            "++" => Ok(Increment),
            "--" => Ok(Decrement),
            "&" => Ok(AddressOf),
            "~" => Ok(BitwiseNot),
            "!" => Ok(LogicalNot),
            "*" => Ok(Dereference),
            _ => Err("invalid unary operator".into()),
        }
    }
}

impl UnaryExprKind {
    /// Fetch the precedence of this unary operator.
    pub const fn precedence(&self) -> usize {
        use UnaryExprKind::*;
        match self {
            Increment | Decrement | Index(_) => 1,
            _ => 2,
        }
    }

    /// Fetch the associativity of this unary operator.

    pub const fn associativity(&self) -> Associativity {
        use UnaryExprKind::*;
        match self {
            Increment | Decrement | Index(_) => Associativity::Ltr,
            _ => Associativity::Rtl,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BinaryOp {
    /// The addition operator, `+`.
    Plus,
    /// The subtraction operator, `-`.
    Minus,
    /// The multiplication operator, `*`.
    Mul,
    /// The division operator, `/`.
    Div,
    /// The modulo operator, `%`.
    Mod,
    /// The bitwise AND operator, `&`.
    BitwiseAnd,
    /// The bitwise OR operator, `|`.
    BitwiseOr,
    /// The bitwise XOR operator, `^`.
    BitwiseXor,
    /// The logical AND operator, `&&`.
    LogicalAnd,
    /// The logical OR operator, `||`.
    LogicalOr,
    /// The bitwise left shift operator, `<<`.
    Shl,
    /// The bitwise right shift operator, `>>`.
    Shr,
    /// The equality operator, `==`.
    Eq,
    /// The inequality operator, `!=`.
    Ne,
    /// The less-than operator, `<`.
    Lt,
    /// The greater-than operator, `>`.
    Gt,
    /// The less-than-or-equal operator, `<=`.
    Le,
    /// The greater-than-or-equal operator, `>=`.
    Ge,
    /// The assignment operator, `=`.
    Assign,
    /// The assignment operator, `+=`.
    PlusEq,
    /// The assignment operator, `-=`.
    MinusEq,
    /// The assignment operator, `*=`.
    MulEq,
    /// The assignment operator, `/=`.
    DivEq,
    /// The assignment operator, `%=`.
    ModEq,
    /// The assignment operator, `&=`.
    BitwiseAndEq,
    /// The assignment operator, `|=`.
    BitwiseOrEq,
    /// The assignment operator, `^=`.
    BitwiseXorEq,
    /// The assignment operator, `<<=`.
    ShlEq,
    /// The assignment operator, `>>=`.
    ShrEq,
}

impl FromStr for BinaryOp {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<BinaryOp, Self::Err> {
        use BinaryOp::*;
        match s {
            "+" => Ok(Plus),
            "-" => Ok(Minus),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Mod),
            "&" => Ok(BitwiseAnd),
            "|" => Ok(BitwiseOr),
            "^" => Ok(BitwiseXor),
            "<<" => Ok(Shl),
            ">>" => Ok(Shr),
            "==" => Ok(Eq),
            "!=" => Ok(Ne),
            "<" => Ok(Lt),
            ">" => Ok(Gt),
            "<=" => Ok(Le),
            ">=" => Ok(Ge),
            "=" => Ok(Assign),
            "+=" => Ok(PlusEq),
            "-=" => Ok(MinusEq),
            "*=" => Ok(MulEq),
            "/=" => Ok(DivEq),
            "%=" => Ok(ModEq),
            "&=" => Ok(BitwiseAndEq),
            "|=" => Ok(BitwiseOrEq),
            "^=" => Ok(BitwiseXorEq),
            "<<=" => Ok(ShlEq),
            ">>=" => Ok(ShrEq),
            _ => Err("invalid binary operator".into()),
        }
    }
}

impl BinaryOp {
    /// Fetch the precedence of this binary operator.
    pub const fn precedence(&self) -> usize {
        match self {
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 5,
            BinaryOp::Plus | BinaryOp::Minus => 6,
            BinaryOp::Shl | BinaryOp::Shr => 7,
            BinaryOp::BitwiseAnd => 8,
            BinaryOp::BitwiseXor => 9,
            BinaryOp::BitwiseOr => 10,
            BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge => 11,
            BinaryOp::Eq | BinaryOp::Ne => 12,
            BinaryOp::LogicalAnd => 13,
            BinaryOp::LogicalOr => 14,
            _ => 15,
        }
    }

    /// Fetch the associativity of this binary operator.
    pub const fn associativity(&self) -> Associativity {
        match self {
            _ => Associativity::Ltr,
        }
    }
}

/// A binary expression.
#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    /// The left hand side of the binary expression.
    pub lhs: Box<Node<Expr>>,
    /// The right hand side of the binary expression.
    pub rhs: Box<Node<Expr>>,
    /// The kind of binary expression.
    pub kind: BinaryOp,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentKind {
    /// The assignment operator, `=`.
    Assign,
    /// The bitwise left-shift assignment operator, `<<=`.
    ShlAssign,
    /// The bitwise right-shift assignment operator, `>>=`.
    ShrAssign,
    /// The bitwise AND assignment operator, `&=`.
    AndAssign,
    /// The bitwise OR assignment operator, `|=`.
    OrAssign,
    /// The bitwise XOR assignment operator, `^=`.
    XorAssign,
    /// The assignment by sum operator, `+=`.
    AddAssign,
    /// The assignment by difference operator, `-=`.
    SubAssign,
    /// The assignment by product operator, `*=`.
    MulAssign,
    /// The assignment by division operator, `/=`.
    DivAssign,
    /// The assignment by modulo operator, `%=`.
    ModAssign,
}

///  A variable assignment.
#[derive(Debug, PartialEq)]

pub struct Assignment {
    /// The identifier being assigned to.
    pub ident: Ident,
    /// The declared value.
    pub value: Expr,
    /// The kind of assignment.
    pub kind: AssignmentKind,
}
