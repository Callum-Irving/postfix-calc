#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}
// TODO: Make expression a Vec of ExprTokens
#[derive(Debug, PartialEq, Clone)]
pub enum ExprToken {
    Number(f32),
    Operator(Operator),
    Argument(usize), // Variable index
}

// TODO: When using a function inside another function definition, expand it in place (implement in
// another file)

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub tokens: Vec<ExprToken>,
    pub num_arguments: usize,
}

// TODO: cleanup

impl Expr {
    pub fn get_tokens(&self) -> Vec<ExprToken> {
        self.tokens.clone()
    }

    pub fn new(tokens: Vec<ExprToken>) -> Expr {
        let num_arguments = tokens
            .iter()
            .filter(|token| matches!(token, ExprToken::Argument(_)))
            .count();

        Expr {
            tokens,
            num_arguments,
        }
    }

    // pub fn parse_expr(
    //     tokens: Vec<TokenType>,
    //     arguments: Option<Vec<&str>>,
    // ) -> Result<Self, String> {
    //     let mut nodes = vec![];
    //     let mut num_arguments = 0;
    //     use Operator::*;
    //     for token in tokens {
    //         nodes.push(match token {
    //             TokenType::Str(token) => match token {
    //                 "+" => ExprToken::Operator(Plus),
    //                 "-" => ExprToken::Operator(Minus),
    //                 "*" => ExprToken::Operator(Multiply),
    //                 "/" => ExprToken::Operator(Divide),
    //                 _ => {
    //                     if let Ok(num) = token.parse() {
    //                         ExprToken::Number(num)
    //                     } else if arguments.is_some() {
    //                         if let Ok(arg_index) = arguments.as_ref().unwrap().binary_search(&token)
    //                         {
    //                             num_arguments += 1;
    //                             ExprToken::Argument(arg_index)
    //                         } else {
    //                             return Err(format!("could not parse token: {}", token));
    //                         }
    //                     } else {
    //                         return Err(format!("could not parse token: {}", token));
    //                     }
    //                 }
    //             }
    //         });
    //     }
    //     Ok(Expr {
    //         tokens: nodes,
    //         num_arguments,
    //     })
    // }

    pub fn eval(&self, arguments: Option<Vec<f32>>) -> Result<Vec<f32>, String> {
        let mut args = vec![];
        if arguments.is_some() {
            args = arguments.unwrap();
            if args.len() != self.num_arguments {
                return Err("incorrect number of arguments provided".to_owned());
            }
        }

        let mut stack: Vec<f32> = vec![];
        for node in self.tokens.iter() {
            match node {
                ExprToken::Operator(op) => {
                    let right = stack
                        .pop()
                        .ok_or("not enough values provided in expression")?;
                    let left = stack
                        .pop()
                        .ok_or("not enough values provided in expression")?;
                    stack.push(match op {
                        Operator::Plus => left + right,
                        Operator::Minus => left - right,
                        Operator::Multiply => left * right,
                        Operator::Divide => left / right,
                    });
                }
                ExprToken::Number(num) => stack.push(*num),
                ExprToken::Argument(index) => stack.push(args[*index]),
            }
        }

        Ok(stack)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_expr() {
//         let tokens = vec![
//             TokenType::Str("1"),
//             TokenType::Str("2"),
//             TokenType::Str("3"),
//             TokenType::Str("+"),
//             TokenType::Str("+"),
//         ];
//         let expr = Expr::parse_expr(tokens, None).expect("could not parse expression");
//         assert_eq!(
//             expr.tokens,
//             vec![
//                 ExprToken::Number(1_f32),
//                 ExprToken::Number(2_f32),
//                 ExprToken::Number(3_f32),
//                 ExprToken::Operator(Operator::Plus),
//                 ExprToken::Operator(Operator::Plus)
//             ]
//         );
//         let result = expr.eval(None).expect("could not evaluate expression");
//         assert_eq!(result, vec![6_f32]);
//
//         let tokens = vec![
//             TokenType::Str("1"),
//             TokenType::Str("a"),
//             TokenType::Str("+"),
//             TokenType::Str("3"),
//             TokenType::Str("+"),
//             TokenType::Str("6"),
//             TokenType::Str("*"),
//         ];
//         let expr = Expr::parse_expr(tokens, Some(vec!["a"])).expect("could not parse expression");
//         let result = expr
//             .eval(Some(vec![3_f32]))
//             .expect("could not evaluate expression");
//         assert_eq!(result, vec![42_f32]);
//
//         let tokens = vec![
//             TokenType::Str("1"),
//             TokenType::Str("a"),
//             TokenType::Str("+"),
//             TokenType::Str("3"),
//             TokenType::Str("+"),
//             TokenType::Str("6"),
//             TokenType::Str("*"),
//             TokenType::Str("b"),
//             TokenType::Str("+"),
//         ];
//         let expr =
//             Expr::parse_expr(tokens, Some(vec!["a", "b"])).expect("could not parse expression");
//         let result = expr
//             .eval(Some(vec![3_f32, 3_f32]))
//             .expect("could not evaluate expression");
//         assert_eq!(result, vec![45_f32]);
//
//         let tokens = vec![
//             TokenType::Str("3"),
//             TokenType::Str("3"),
//             TokenType::Expr(&expr),
//         ];
//         let expr2 = Expr::parse_expr(tokens, None).expect("could not parse expression");
//         let result = expr2.eval(None).expect("could not evaluate expression");
//         assert_eq!(result, vec![45_f32]);
//     }
// }
