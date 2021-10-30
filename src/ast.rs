#![allow(dead_code)]

/// imagine interning or something here
pub type Symbol = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Block);

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Stmt>);

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Declaration(Declaration),
    Assignment(Assignment),
    FnDecl(FnDecl),
    If(IfStmt),
    Loop(Block),
    While(WhileStmt),
    Break,
    Return(Option<Expr>),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    name: Symbol,
    init: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub lhs: Symbol,
    pub rhs: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub body: Block,
    pub else_part: Box<ElsePart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElsePart {
    Else(Block),
    ElseIf(IfStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    UnaryOp,
    BinaryOp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Array(Vec<Expr>),
    Object,
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOp {
    pub expr: Expr,
    pub kind: UnaryOpKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    pub lhs: Expr,
    pub rhs: Expr,
    pub kind: BinaryOpKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    And,
    Or,
    Equal,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    NotEqual,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Call {
    Function(Expr, Vec<Expr>),
    Field(Expr, Vec<Expr>),
}
