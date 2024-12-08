use lang::{err::InterpError, instruction::EvalResult, scope::Scope, LANG};
use std::{
    env, fs,
    io::{self, stdout, BufRead, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let scope = Scope::new();
    let lang = &mut LANG::new();
    if args.len() > 1 {
        let result;
        if args[1].ends_with(".lg") {
            result = run_from_file(&args[1], lang, scope.clone())
        } else {
            result = eval_statement(&args[1], lang, scope.clone());
        }
        if let Err(e) = result {
            print_err(e);
        }
    } else {
        repl(lang, scope.clone());
    }
}

fn print_err(err: InterpError) {
    eprintln!("Evaluation error: {}", err)
}

pub fn run_from_file(
    file_name: &str,
    lang: &mut LANG,
    scope: Scope,
) -> Result<Option<EvalResult>, InterpError> {
    let file_path = file_name;
    match fs::read_to_string(file_name) {
        Ok(content) => eval_statement(content.as_str(), lang, scope),
        Err(_) => Err(InterpError::ProgramFileNotFound(file_path.to_string())),
    }
}

fn repl(lang: &mut LANG, scope: Scope) {
    let stdin = io::stdin();
    loop {
        print!("👉 ");
        stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                match eval_statement(l, lang, scope.clone()) {
                    Ok(Some(EvalResult::Value(value))) => {
                        println!("{}", value);
                    }
                    Err(err) => print_err(err),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn eval_statement(
    input: &str,
    lang: &mut LANG,
    scope: Scope,
) -> Result<Option<EvalResult>, InterpError> {
    let ast = lang.from_str(input);
    match ast {
        Ok(ast_node) => {
            let bytecode = LANG::ast_to_bytecode(ast_node);

            match lang.eval(&bytecode, scope) {
                Ok(eval_result) => return Ok(eval_result),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}
