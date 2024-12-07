pub mod ast;
pub mod bytecode;
pub mod parser;

use bytecode::eval_bytecode;
use parser::parse_str;

pub fn eval_str(input: &String) -> Result<Option<u64>, String> {
    match parse_str(input) {
        Some(Ok(ast)) => Ok(eval_bytecode(ast)),
        Some(Err(_)) => Err("Não pode avaliar input".to_string()),
        _ => Ok(None),
    }
}
