use std::io::stdin;

fn main() {
    let mut input = String::new();
    //  read_line still returns Ok when input is empty so we have to check input len
    while stdin().read_line(&mut input).is_ok() && input.len() > 0 {
        input.truncate(input.trim_end().len());
        if input == "exit" {
            break;
        }

        let mut stack: Vec<i32> = vec![];
        let tokens = input.split_whitespace();
        let ops = ["+", "-", "*", "/"];

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
            continue;
        }

        let result = match stack.pop() {
            Some(value) => value,
            None => {
                println!("ERROR: no expression provided");
                continue;
            }
        };

        if stack.len() > 0 {
            println!("ERROR: multiple values remaining after calculations");
            continue;
        }

        println!("{}", result);

        input.clear();
    }
}
