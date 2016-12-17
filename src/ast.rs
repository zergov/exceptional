use num::rational::{BigRational};

#[derive(Clone, Eq, Debug, Hash, Ord, PartialEq, PartialOrd)]
pub enum Literal {
    // Nil,
    Number(BigRational),
    CharString(String),
    Boolean(bool),
    // Vec(Vec<Value>),
    Map(Vec<(Expression, Expression)>),
    Fn(Box<(Vec<(String)>, Vec<Statement>)>),
}

#[derive(Clone, Eq, Debug, Hash, Ord, PartialEq, PartialOrd)]
pub enum Statement {
    Assign(bool, String, Box<Expression>),
    Call(String, Box<Vec<Expression>>),
}

#[derive(Clone, Eq, Debug, Hash, Ord, PartialEq, PartialOrd)]
pub enum Expression {
    BinOp(String, Box<Expression>, Box<Expression>),
    Literal(Literal),
}
