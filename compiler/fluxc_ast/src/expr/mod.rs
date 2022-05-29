//! Contains the expression AST data structures.

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;
pub(crate) mod types;

pub use block_expr::*;
pub use control::*;
pub use literal::*;
pub use operation::*;
pub use types::*;

use crate::{FuncCall, Ident, Node};

/// The enumeration of possible expression types.
///
/// This enum holds the different kinds of expressions that can occur in Flux
/// source code. Every expression wraps a `Node` which holds the actual data
/// that represents the expression, as well as the span of the source code that
/// the expression represents.
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// A literal expression.
    Literal(
        /// The inner literal type.
        Node<Literal>,
    ),
    /// An identifier expression.
    Ident(
        /// The inner identifier type.
        Node<Ident>,
    ),
    /// A binary operation expression.
    BinaryExpr(
        /// The inner binary expression type.
        Node<BinaryExpr>,
    ),
    UnaryExpr(Node<UnaryExpr>),
    /// A block of code (e.g. `{ /* ... */ }`).
    Block(
        /// The inner block type.
        Node<Block>,
    ),
    /// A function call expression.
    FuncCall(Node<FuncCall>),
    /// A conditional expression.
    Conditional(Node<Conditional>),
    /// An Unconditional loop expression.
    Loop(Node<Loop>),
    /// A conditional loop expression.
    While(Node<While>),
    /// A match expression.
    Match(Node<Match>),
}

impl Typed for Expr {
    fn as_type(&self) -> TypeExpr {
        match self {
            Expr::Literal(literal) => literal.as_type(),
            Expr::Ident(_) => todo!(),
            Expr::BinaryExpr(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::FuncCall(_) => todo!(),
            Expr::Conditional(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::UnaryExpr(_) => todo!(),
            Expr::Match(_) => todo!(),
        }
    }
}
