use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("lang.l");
lrpar_mod!("lang.y");

pub fn print_ast(input: &String) {
    let lexer_def = lang_l::lexerdef(); // Lex the input.
    let lexer = lexer_def.lexer(&input);
    let (res, errs) = lang_y::parse(&lexer); // Parse the input.
                                             // Check for errors
    for e in errs {
        println!("{}", e.pp(&lexer, &lang_y::token_epp));
    }

    match res {
        Some(Ok(r)) => println!("{:?}", r),
        _ => eprintln!("Não foi possivel avaliar a expressão"),
    }
}
