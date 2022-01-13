mod constants;
mod expression;
mod interpreter;

use interpreter::{CalcError, Context, FnSymbol, Symbol};
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
                Ok(Some(results)) => display_nums(results),
                Ok(None) => println!(),
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

fn handle_line(context: &mut Context, line: String) -> Result<Option<Vec<Float>>, CalcError> {
    let input = line.trim_end();
    if input == "exit" {
        exit(0);
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let ret = if tokens.get(0) == Some(&"let") {
        let (name, symbol) = parse_symbol_def(context, &tokens)?;
        context.add_symbol(name, symbol)?;
        None
    } else {
        let expr = context.parse_expr(tokens)?;
        let results = context.eval_expr(&expr)?;
        Some(results)
    };

    Ok(ret)
}

fn parse_symbol_def(context: &Context, tokens: &Vec<&str>) -> Result<(String, Symbol), CalcError> {
    let mut iter = tokens.iter().cloned().skip(1);
    let name = iter.next().ok_or(CalcError::ParseError)?.to_owned();
    // TODO: Make sure variables do not call functions of the same name
    match iter.next() {
        Some("of") => {
            // TODO: Unnessecary allocation of memory
            let mut args = vec![];
            while let Some(arg) = iter.next() {
                if arg == "=" {
                    break;
                }
                args.push(arg.to_owned());
            }
            let func_args = args
                .iter()
                .map(|name| (name.clone(), Symbol::Variable(Float::with_val(1, 0))));
            let mut temp_ctx = context.clone();
            temp_ctx.symbols.extend(func_args);

            let tokens: Vec<&str> = iter.collect();
            if tokens.contains(&name.as_str()) {
                return Err(CalcError::RecursiveFunction);
            }
            let expr = temp_ctx.parse_expr(tokens)?;

            Ok((name, Symbol::Function(FnSymbol { args, expr })))
        }
        Some("=") => {
            // TODO: Assert that  only one value is returned (no multivariable defs yet)
            let tokens: Vec<&str> = iter.collect();
            if tokens.contains(&name.as_str())
                && matches!(context.symbols.get(&name), Some(Symbol::Function(_)))
            {
                return Err(CalcError::VariableFunctionCall);
            }
            let expr = context.parse_expr(tokens)?;
            let result = context.eval_expr(&expr)?[0].clone();
            Ok((name, Symbol::Variable(result)))
        }
        _ => Err(CalcError::ParseError),
    }
}
