use rug::Float;

#[derive(Clone)]
pub struct Expr {
    pub tokens: Vec<Token>,
}

#[derive(Clone)]
pub enum Token {
    Number(Float),
    Symbol(String),
}
