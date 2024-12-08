use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

use clap::Parser;
use lang::{eval_str, parser::print_ast};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn eval_file(file_name: String) {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            eval(&content);
        }
        Err(e) => eprintln!("Não é possivel avaliar expressão, Erro: {}", e),
    }
}

fn eval_file_ast(file_name: String) {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            print_ast(&content);
        }
        Err(e) => eprintln!("Não é possivel avaliar expressão, Erro: {}", e),
    }
}

fn repl() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit()" {
                    break;
                }
                if input.trim().is_empty() {
                    continue;
                }
                eval(&input);
            }
            _ => {}
        }
    }
}

fn eval(input: &String) {
    match eval_str(input) {
        Ok(Some(result)) => {
            println!("{}", result);
        }
        Ok(None) => {}
        Err(e) => eprintln!("Não é possivel avaliar expressão, Erro: {}", e),
    }
}

fn main() {
    println!("Interpretador LANG");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1].ends_with(".lg") {
            if args[2] == "a" {
                eval_file_ast(args[1].clone());
            }
            eval_file(args[1].clone())
        } else {
            eval(&args[1])
        }
    } else {
        repl()
    }
}

#[test]
fn test_comments() {
    assert_eq!(
        eval_str(&"// 2+2\n 1+1".to_string()).unwrap(),
        Some(2),
        "expected 1+1=2"
    );
    assert_eq!(
        eval_str(&"// 2+2".to_string()).unwrap(),
        None,
        "expected 1+1=2"
    );
}

#[cfg(test)]
mod main_tests {
    use super::*;
    #[test]
    fn math_expressions() {
        assert_eq!(
            eval_str(&"0+1*1*1".to_string()).unwrap(),
            Some(1),
            "expected 0+1*1*1"
        );
        assert_eq!(
            eval_str(&"1+1".to_string()).unwrap(),
            Some(2),
            "expected 1+1=2"
        );
        assert_eq!(
            eval_str(&"1*(1+2)".to_string()).unwrap(),
            Some(3),
            "expected 1*(1+2)=3"
        );
    }
}

#[cfg(test)]
mod var_tests {
    use super::*;
    #[test]
    fn vars_declare_match() {
        assert_eq!(
            eval_str(&"let x = 1; let y = 2; y + x;".to_string()).unwrap(),
            Some(3)
        );
    }
    #[test]
    fn vars_reassign_math() {
        assert_eq!(
            eval_str(&"let x = 1; let y = 2; x = 3; x + y;".to_string()).unwrap(),
            Some(5)
        );
    }

    #[test]
    fn vars_undeclared_variable() {
        assert_eq!(
            eval_str(&"a + 1;".to_string()),
            Err("Variable 'a' not found".to_string())
        );
    }
}
