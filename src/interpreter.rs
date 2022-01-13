use crate::constants::get_fn;
use crate::constants::{BuiltinFn, LITERALS};
use crate::expression::{Expr, Token};

use rug::Float;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct Context {
    symbols: HashMap<String, Symbol>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            symbols: HashMap::new(),
        }
    }

    pub fn parse_expr(&self, tokens: Vec<&str>) -> Result<Expr, CalcError> {
        let mut expr = Expr { tokens: vec![] };
        for token in tokens {
            if let Ok(num) = Float::parse(token) {
                expr.tokens.push(Token::Number(Float::with_val(63, num)));
            } else if LITERALS.get(token).is_some()
                || get_fn(token).is_some()
                || self.symbols.get(token).is_some()
            {
                expr.tokens.push(Token::Symbol(token.to_owned()));
            } else {
                return Err(CalcError::ParseError);
            }
        }
        Ok(expr)
    }

    pub fn eval_expr(&self, expr: &Expr) -> Result<Vec<Float>, CalcError> {
        let mut stack: Vec<Float> = vec![];
        for token in &expr.tokens {
            match token {
                Token::Number(num) => stack.push(num.clone()),
                Token::Symbol(name) => {
                    // Check constants
                    if let Some(num) = LITERALS.get(name.as_str()) {
                        stack.push(num.clone());
                    } else if let Some(func) = get_fn(name.as_str()) {
                        match func {
                            BuiltinFn::Unary(func) => {
                                let x = stack.pop();
                                match x {
                                    Some(num) => stack.push(func(num)),
                                    None => return Err(CalcError::NotEnoughStack),
                                }
                            }
                            BuiltinFn::Binary(func) => {
                                let y = stack.pop();
                                let x = stack.pop();
                                match (x, y) {
                                    (Some(x), Some(y)) => stack.push(func(x, y)),
                                    _ => return Err(CalcError::NotEnoughStack),
                                }
                            }
                        }
                    } else if let Some(symbol) = self.symbols.get(name) {
                        match symbol {
                            Symbol::Function(func) => {
                                if func.args.len() > stack.len() {
                                    return Err(CalcError::NotEnoughStack);
                                }
                                let args = stack.split_off(stack.len() - func.args.len());

                                // Create temporary context
                                let mut temp_ctx = self.clone();
                                let mut tuples = func
                                    .args
                                    .iter()
                                    .cloned()
                                    .zip(args.into_iter().map(|num| Symbol::Variable(num)));
                                temp_ctx.symbols.extend(&mut tuples);

                                let mut result = temp_ctx.eval_expr(&func.expr).unwrap();
                                stack.append(&mut result);
                            }
                            Symbol::Variable(num) => {
                                stack.push(num.clone());
                            }
                        }
                        todo!();
                    } else {
                        return Err(CalcError::UnexpectedToken(name.into()));
                    }
                }
            }
        }
        Ok(stack)
    }
}

#[derive(Clone)]
pub enum Symbol {
    Function(FnSymbol),
    Variable(Float),
}

#[derive(Clone)]
pub struct FnSymbol {
    args: Vec<String>, // Arg names
    expr: Expr,
}

#[derive(Debug)]
pub enum CalcError {
    ParseError,
    UnexpectedToken(String),
    NotEnoughStack,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::ParseError => write!(f, "could not parse expression"),
            CalcError::UnexpectedToken(token) => write!(f, "unexpected token: {}", token),
            &CalcError::NotEnoughStack => write!(f, "not enough values on stack"),
        }
    }
}
