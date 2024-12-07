use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("lang.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("lang.y");

mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = &args[1]; // Create a lexer
        let lexer_def = lang_l::lexerdef(); // Lex the input.
        let lexer = lexer_def.lexer(&input);
        let (res, errs) = lang_y::parse(&lexer); // Parse the input.
                                                 // Check for errors
        for e in errs {
            println!("{}", e.pp(&lexer, &lang_y::token_epp));
        }
        // Print results
        match res {
            Some(Ok(r)) => println!("{:?}", r),
            _ => eprintln!("Unable to evaluate expression."),
        }
    } else {
        println!("Please provide at least one cli argument!")
    }
}
