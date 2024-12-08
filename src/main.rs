use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

mod ast;
mod bytecode;

use lang::eval_str;

//pub fn eval_expr(exp: ast::Node) -> Result<u64, String> {
//    match exp {
//        ast::Node::Add { lhs, rhs } => eval_expr(*lhs)?
//            .checked_add(eval_expr(*rhs)?)
//            .ok_or("overflowed".to_string()),
//        ast::Node::Mul { lhs, rhs } => eval_expr(*lhs)?
//            .checked_mul(eval_expr(*rhs)?)
//            .ok_or("overflowed".to_string()),
//        ast::Node::Number { value } => Ok(value),
//    }
//}
//
//pub fn eval(ast: Vec<ast::Node>) -> Result<u64, String> {
//    for node in ast {
//        return eval_expr(node);
//    }
//    Err(String::from("Não pode ser avaliado!"))
//}

//fn from_str(input: &String) -> Result<Option<u64>, String> {
//    let lexer_def = lang_l::lexerdef(); // Lex the input.
//    let lexer = lexer_def.lexer(&input);
//    let (res, errs) = lang_y::parse(&lexer); // Parse the input.
//                                             // Check for errors
//    for e in errs {
//        println!("{}", e.pp(&lexer, &lang_y::token_epp));
//    }
//    // Print results
//    match res {
//        Some(Ok(r)) => Ok(eval_bytecode(r)),
//        _ => Err("Unable to evaluate expression.".to_string()),
//    }
//}

fn eval(input: &String) {
    match eval_str(input) {
        Ok(Some(result)) => {
            println!("{}", result);
        }
        _ => eprintln!("Não foi possivel avaliar a expressão"),
    }
}

fn repl() {
    loop {
        print!(">");
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

fn eval_file(file_name: String) {
    match fs::read_to_string(file_name) {
        Ok(content) => {
            eval(&content);
        }
        _ => eprintln!("Não foi possivel avaliar expressão no arquivo"),
    }
}

fn main() {
    println!("Interpretador LANG");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1].ends_with(".lg") {
            eval_file(args[1].clone())
        } else {
            eval(&args[1])
        }
    } else {
        repl()
    }
}

//Tests
#[test]
fn eval_comments() {
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

#[test]
fn eval_expressions() {
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
//Tests

//fn print_ast() {
//    let args: Vec<String> = env::args().collect();
//    if args.len() > 1 {
//        let input = &args[1]; // Create a lexer
//        let lexer_def = lang_l::lexerdef(); // Lex the input.
//        let lexer = lexer_def.lexer(&input);
//        let (res, errs) = lang_y::parse(&lexer); // Parse the input.
//                                                 // Check for errors
//        for e in errs {
//            println!("{}", e.pp(&lexer, &lang_y::token_epp));
//        }
//        // Print results
//        match res {
//            Some(Ok(r)) => println!("{:?}", r),
//            _ => eprintln!("Unable to evaluate expression."),
//        }
//    } else {
//        println!("Please provide at least one cli argument!")
//    }
//}
