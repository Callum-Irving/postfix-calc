mod constants;
mod expression;
mod interpreter;

use interpreter::{CalcError, Context};
use rug::Float;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main() {
    let mut context = Context::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => match handle_line(&mut context, input) {
                Ok(results) => display_nums(results),
                Err(err) => eprintln!("ERROR: {}", err),
            },
            Err(_) => error_exit("ERROR: could not read line from stdin", 1),
        }
    }
}

fn display_nums(nums: Vec<Float>) {
    println!(
        "{}",
        nums.iter()
            .map(|num| format_float(num))
            .collect::<Vec<String>>()
            .join(", ")
    );
}

fn format_float(num: &Float) -> String {
    let output = format!("{:.1$}", num.to_f64(), 10);
    if output.contains(".") && !output.contains("e") {
        output
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        output
    }
}

fn error_exit(msg: &str, code: i32) {
    eprintln!("{}", msg);
    exit(code);
}

fn handle_line(context: &mut Context, line: String) -> Result<Vec<Float>, CalcError> {
    let input = line.trim_end();
    if input == "exit" {
        exit(0);
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let expr = context.parse_expr(tokens)?;
    context.eval_expr(&expr)
}
