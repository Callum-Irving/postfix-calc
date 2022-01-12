use std::{collections::HashMap, io::stdin};

use expression::{Expr, ExprToken, Operator};

mod expression;

// TODO: HOW TO CALL FUNCTIONS???
// TODO: Implement Expr from other module
// TODO: New expressions
// if tokens[0] == "let", function expression
// if tokens[0] == "var", var expression
// multiple vars can be assigned at once, in order of last var is first on stack
// functions can return multiple values

enum Item {
    Function(expression::Expr),
    Variable(f32),
}

enum ItemOrCalc {
    Item(Item),
    Calc(Vec<f32>),
}

struct State {
    items: HashMap<String, Item>,
}

fn handle_line(state: &State, input: &String) -> Result<ItemOrCalc, String> {
    let mut raw_tokens: Vec<&str> = input.split_whitespace().collect();
    // "let" => function declaration
    // "var" => variable declaration
    // _ => calculation
    match raw_tokens[0] {
        "let" => {
            // Splitting at first equals sign
            let equals = match raw_tokens.iter().position(|t| *t == "=") {
                Some(index) => index,
                None => {
                    return Err("no equals sign in function declaration".into());
                }
            };
            let second_half = &raw_tokens.split_off(equals)[1..];

            // Parse arguments from first half
            raw_tokens.remove(0); // Slow and not really needed!
            if raw_tokens.len() < 2 {
                return Err("function with no args should be var declaration".into());
            }

            // First word is function name
            // make sure this word starts with a letter
            let func_name = raw_tokens[0];
            if !func_name.chars().next().unwrap().is_alphabetic() {
                return Err(format!(
                    "function name must start with letter: {}",
                    func_name
                ));
            }
            // all remaining words are arguments
            // make sure all words start with letters
            let arguments = raw_tokens[1..].to_vec();
            for arg in arguments.iter() {
                if !arg.chars().next().unwrap().is_alphabetic() {
                    return Err(format!("argument name must start with letter: {}", arg));
                }
            }

            // Parse second half as expression
            // when a word is found, search for it in arguments
            // if it is in arguments, use ExprToken::Argument(index)
            // if not in arguments, search for item in state
            // if in state.items match function variable
            // if not in state.items, return Err("unknown token: '{}'", token)

            let mut expr_tokens = vec![];
            for token in second_half {
                match *token {
                    "+" => expr_tokens.push(ExprToken::Operator(Operator::Plus)),
                    "-" => expr_tokens.push(ExprToken::Operator(Operator::Minus)),
                    "*" => expr_tokens.push(ExprToken::Operator(Operator::Multiply)),
                    "/" => expr_tokens.push(ExprToken::Operator(Operator::Divide)),
                    word => {
                        if let Ok(num) = word.parse() {
                            expr_tokens.push(ExprToken::Number(num));
                        } else if let Some(arg_index) = arguments.iter().position(|a| *a == word) {
                            expr_tokens.push(ExprToken::Argument(arg_index));
                        } else if let Some(state_item) = state.items.get(word) {
                            // TODO: Expand function in place
                            match state_item {
                                Item::Function(expr) => {
                                    todo!();
                                    // TODO: Eval expression so far to get args
                                    // let temp_expr = Expr::new(expr_tokens.clone());
                                    // TODO: Check that num args is on stack
                                    let args = expr_tokens
                                        .split_off(expr_tokens.len() - expr.num_arguments);
                                    for token in &expr.tokens {
                                        match token {
                                            ExprToken::Argument(arg_num) => {
                                                expr_tokens.push(args[*arg_num].clone());
                                            }
                                            other => expr_tokens.push(other.clone()),
                                        }
                                    }
                                }
                                Item::Variable(value) => {
                                    expr_tokens.push(ExprToken::Number(*value))
                                }
                            }
                        } else {
                            return Err(format!("unknown token: '{}'", word));
                        }
                    }
                }
            }

            let expression = Expr::new(expr_tokens);

            return Ok(ItemOrCalc::Item(Item::Function(expression)));
        }
        "var" => {
            todo!()
        }
        _ => {
            // Normal expr
            let mut tokens: Vec<ExprToken> = vec![];

            for token in raw_tokens {
                match token {
                    "+" => tokens.push(ExprToken::Operator(Operator::Plus)),
                    "-" => tokens.push(ExprToken::Operator(Operator::Minus)),
                    "*" => tokens.push(ExprToken::Operator(Operator::Multiply)),
                    "/" => tokens.push(ExprToken::Operator(Operator::Divide)),
                    word => {
                        // Check float literal
                        // Check function call
                        // Check variable
                        if let Ok(num) = word.parse() {
                            tokens.push(ExprToken::Number(num))
                        } else if let Some(state_item) = state.items.get(word) {
                            match state_item {
                                Item::Function(expr) => {
                                    // Pop num args
                                    let temp_expr = Expr::new(tokens.clone());
                                    // TODO: Error check
                                    let args = temp_expr.eval(None).unwrap();
                                    // Iterate backwards for each arg
                                    // If operator add 1 to iter count
                                    // if value, subtract one form iter count
                                    // take all of these from tokens into temp expr
                                    // eval temp expr
                                    // Replace expression args with values from temp expr
                                    // Push resulting values
                                    let args = 
                                    todo!();
                                },
                                Item::Variable(value) => tokens.push(ExprToken::Number(*value))
                            }
                        }
                    },
                }
            }

            let expr = Expr::new(tokens);
            let result = expr.eval(None);
            match result {
                Ok(values) => return Ok(ItemOrCalc::Calc(values)),
                Err(err) => return Err(err),
            }
        }
    };
}

fn main() {
    let mut input = String::new();
    let mut state = State {
        items: HashMap::new(),
    };
    while stdin().read_line(&mut input).is_ok() && input.len() > 0 {
        input.truncate(input.trim_end().len());
        if input == "exit" {
            break;
        }

        let result = handle_line(&state, &input);
        // TODO: Make result a vec and then have it be the entire remaining stack

        if result.is_ok() {
            match result.unwrap() {
                ItemOrCalc::Item(item) => {
                    state.items.insert("name".into(), item);
                }
                ItemOrCalc::Calc(result) => println!("{:?}", result),
            }
        } else {
            println!("ERROR: {}", result.err().unwrap());
        }

        input.clear();
    }
}

fn old_main() {
    let mut input = String::new();
    //  read_line still returns Ok when input is empty so we have to check input len
    while stdin().read_line(&mut input).is_ok() && input.len() > 0 {
        input.truncate(input.trim_end().len());
        if input == "exit" {
            break;
        }

        let mut stack: Vec<f32> = vec![];
        let tokens = input.split_whitespace();
        let ops = ["+", "-", "*", "/"];

        // let mut tokens = tokens.peekable();
        // let first_token = match tokens.peek() {
        //     Some(token) => token,
        //     None => {
        //         println!("ERROR: no expression provided");
        //         input.clear();
        //         continue;
        //     }
        // };

        // match *first_token {
        //     "let" => {
        //         // Grammar:
        //         // "let" "(" params ")" name "=" expr
        //     }
        //     "var" => {
        //         // Grammar:
        //         // "var" names "=" expr
        //     }
        //     _ => {}
        // }

        let mut success = true;
        for token in tokens {
            if ops.contains(&token) {
                let right = match stack.pop() {
                    Some(value) => value,
                    None => {
                        println!("ERROR: missing 2 arguments for operator '{}'", token);
                        success = false;
                        break;
                    }
                };
                let left = match stack.pop() {
                    Some(value) => value,
                    None => {
                        println!("ERROR: missing 1 argument for operator '{}'", token);
                        success = false;
                        break;
                    }
                };
                let res = match token {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("bad token"),
                };
                stack.push(res);
            } else {
                let value = match token.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("ERROR: could not parse '{}' as an integer", token);
                        success = false;
                        break;
                    }
                };
                stack.push(value);
            }
        }

        if !success {
            input.clear();
            continue;
        }

        let result = match stack.pop() {
            Some(value) => value,
            None => {
                println!("ERROR: no expression provided");
                input.clear();
                continue;
            }
        };

        if stack.len() > 0 {
            println!("ERROR: multiple values remaining after calculations");
            input.clear();
            continue;
        }

        println!("{}", result);

        input.clear();
    }
}
