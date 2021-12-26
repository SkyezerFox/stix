use styxc_ast::{
    func::{ExternFunc, FuncDecl, ParenArgument},
    Block, Declaration, Expr, Literal, Node, Stmt,
};
use styxc_types::Type;

/// An enum of linkage types.
#[derive(Debug)]
pub enum Linkage {
    /// The function is declared locally, and is not exported.
    Local,
    /// The function is declared in the scope of the module being compiled.
    Module,
    /// The function is declared externally, and has been imported.
    External,
}

/// Represents a callable function.
#[derive(Debug)]
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The arguments of the function.
    pub args: Vec<ParenArgument>,
    /// The linkage type of this function.
    pub linkage: Linkage,
    /// The return type of this function.
    pub ret_type: Type,
}

impl From<Function> for Type {
    fn from(func: Function) -> Self {
        Type::Func(
            func.args
                .iter()
                .map(|arg| arg.type_expr.value.clone().into())
                .collect(),
            func.ret_type.into(),
        )
    }
}

impl From<&Function> for Type {
    fn from(func: &Function) -> Self {
        Type::Func(
            func.args
                .iter()
                .map(|arg| arg.type_expr.value.clone().into())
                .collect(),
            func.ret_type.clone().into(),
        )
    }
}

impl Function {
    pub fn from_declaration(decl: &FuncDecl, linkage: Linkage) -> Self {
        Function {
            name: decl.ident.value.clone(),
            args: decl
                .args
                .clone()
                .into_iter()
                .map(|node| node.value)
                .collect(),
            linkage,
            ret_type: decl
                .ret_type_expr
                .clone()
                .map_or(Type::Unit, |type_expr| type_expr.value.into()),
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    /// The name of this variable.
    pub name: String,
    /// The mutability of this variable.
    pub mutable: bool,
    /// The type of this variable.
    pub ty: Type,
}

/// Represents a type variable declared via the `type x = ` expression.
pub struct TypeVariable {
    /// The name of this type variable.
    name: String,
    /// The type held by this type variable.
    ty: Type,
}

/// Represents a stack.
#[derive(Debug)]
pub struct Stack<T> {
    /// The contents of the stack.
    contents: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new, empty stack.
    pub fn new() -> Stack<T> {
        Stack {
            contents: Vec::new(),
        }
    }

    /// Return the size of the stack.
    pub fn size(&self) -> usize {
        self.contents.len()
    }

    /// Get an object from the stack.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.contents.get(index)
    }

    /// Get an object from the stack without checking if the stack is empty.
    pub fn get_unchecked(&self, index: usize) -> &T {
        self.contents.get(index).unwrap()
    }

    /// Push an item onto the stack.
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
    }

    /// Pop an item off of the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }

    /// Find an item in the stack using the given predicate.
    pub fn find<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<&T> {
        for item in self.contents.iter().rev() {
            if predicate(item) {
                return Some(item);
            }
        }
        None
    }

    /// Find an item in the stack using the given predicate, returning a mutable reference to the item.
    pub fn find_mut<F: Fn(&T) -> bool>(&mut self, predicate: F) -> Option<&mut T> {
        for item in self.contents.iter_mut().rev() {
            if predicate(item) {
                return Some(item);
            }
        }
        None
    }
}

/// A utility for walking the AST.
pub struct Walker {
    current_function: Option<Function>,
    variables: Stack<Variable>,
    functions: Stack<Function>,
}

impl Walker {
    /// Create a new walker.
    pub fn new() -> Self {
        Walker {
            current_function: None,
            variables: Stack::new(),
            functions: Stack::new(),
        }
    }

    /// Return the current function. This clones the stored function.
    pub fn current_function(&self) -> Option<&Function> {
        match &self.current_function {
            Some(s) => Some(&s),
            None => None,
        }
    }

    /// Enters the current block, declaring all classes and functions in it.
    pub fn enter_block(&mut self, block: &Block) {
        self.declare_all_in_stmts(&block.stmts);
    }

    /// Declares all functions and classes in the given statements.
    pub fn declare_all_in_stmts(&mut self, stmts: &Vec<Node<Stmt>>) {
        for stmt in stmts {
            match &stmt.value {
                Stmt::FuncDecl(func) => self.declare_function(&func.value),
                Stmt::ExternFunc(func) => self.declare_external_function(&func.value),
                _ => (),
            }
        }
    }

    /// Declare a function.
    pub fn declare_function(&mut self, func: &FuncDecl) {
        self.functions.push(Function {
            name: func.ident.value.clone(),
            args: func.args.iter().map(|arg| arg.value.clone()).collect(),
            linkage: Linkage::Local,
            ret_type: func
                .ret_type_expr
                .clone()
                .map_or(Type::Unit, |type_expr| type_expr.value.into()),
        })
    }

    /// Declare an external function.
    pub fn declare_external_function(&mut self, extern_func: &ExternFunc) {
        self.functions.push(Function {
            name: extern_func.ident.value.clone(),
            args: extern_func
                .args
                .iter()
                .map(|arg| arg.value.clone())
                .collect(),
            linkage: Linkage::External,
            ret_type: extern_func
                .ret_type_expr
                .clone()
                .map_or(Type::Unit, |type_expr| type_expr.value.into()),
        })
    }

    /// Declare a variable.
    pub fn declare_variable(&mut self, decl: &Declaration) {
        todo!()
    }

    /// Lookup a variable available in the current scope.
    pub fn lookup_variable<S: AsRef<str>>(&self, name: S) -> Option<&Variable> {
        self.variables.find(|v| v.name == name.as_ref())
    }

    /// Lookup a variable available in the current scope, returning a mutable reference to the variable.
    pub fn lookup_variable_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut Variable> {
        self.variables.find_mut(|v| v.name == name.as_ref())
    }

    /// Lookup a funciton available in the current scope.
    pub fn lookup_function(&self, name: &str) -> Option<&Function> {
        self.functions.find(|f| f.name == name.as_ref())
    }

    /// Lookup a funciton available in the current scope.
    pub fn lookup_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.find_mut(|f| f.name == name.as_ref())
    }

    /// Get the type of an expression in the current scope.
    pub fn get_expr_type(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(literal) => match literal.value {
                Literal::Bool(_) => Type::Bool,
                Literal::Int(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::String(_) => Type::String,
                Literal::Char(_) => Type::Char,
                Literal::Array(_) => Type::Array(Box::new(Type::Infer)),
            },
            Expr::Ident(ident) => match self.lookup_variable(&ident.value) {
                Some(var) => var.ty.clone(),
                None => Type::Infer,
            },
            Expr::BinaryExpr(bin_op) => {
                let lhs = self.get_expr_type(&bin_op.value.lhs.value);
                let rhs = self.get_expr_type(&bin_op.value.lhs.value);
                lhs.intersect(rhs)
            }
            Expr::Block(_) => todo!(),
            Expr::FuncCall(_) => todo!(),
            Expr::Conditional(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::While(_) => todo!(),
        }
    }

    /// Proceed to the next statement, declaring any variables and functions.
    pub fn next_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Declaration(decls) => {
                todo!()
            }
            Stmt::FuncDecl(func) => self.declare_function(&func.value),
            Stmt::ExternFunc(extern_func) => self.declare_external_function(&extern_func.value),
            _ => (),
        }
    }
}
