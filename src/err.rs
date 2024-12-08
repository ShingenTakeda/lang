use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum InterpError {
    ParseError(String),
    EmptyStack,
    Numeric(String),
    UndefinedReference(String),
    FunctionDuplicate(String),
    FunctionArgumentsMissmatch(String, usize, usize),
    ProgramFileNotFound(String),
    UndefinedFunction(String),
    UndeclaredVariable(String),
    EvalError(String),
}

impl Display for InterpError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InterpError::EvalError(msg) => {
                f.write_str(format!("Evaluation error: {}!", msg).as_str())
            }
            InterpError::EmptyStack => f.write_str("Cannot pop from empty stack!"),
            InterpError::UndefinedFunction(id) => {
                f.write_str(format!("Cannot find function with id '{}'!", id).as_str())
            }
            InterpError::ParseError(line) => {
                f.write_str(format!("Parse error: {}!", line).as_str())
            }
            InterpError::Numeric(msg) => f.write_str(format!("Numeric error: {}!", msg).as_str()),
            InterpError::UndefinedReference(id) => {
                f.write_str(format!("Undefined reference '{}'!", id).as_str())
            }
            InterpError::ProgramFileNotFound(file_name) => {
                f.write_str(format!("Program file: '{}' cannot be found!", file_name).as_str())
            }
            InterpError::UndeclaredVariable(id) => {
                f.write_str(format!("Undefined variable '{}'!", id).as_str())
            }
            InterpError::FunctionDuplicate(id) => {
                f.write_str(format!("Function with the id: '{}' already defined", id).as_str())
            }
            InterpError::FunctionArgumentsMissmatch(id, expected, got) => f.write_str(
                format!(
                    "Unexpected number of function arguments. Function '{}' expected {} but got {} arguments",
                    id, expected, got
                )
                .as_str(),
            ),
        }
    }
}

impl Error for InterpError {
    fn description(&self) -> &str {
        match self {
            InterpError::EvalError(..) => "EvalError",
            InterpError::UndefinedFunction(..) => "UndefinedFunction",
            InterpError::EmptyStack => "EmptyStack",
            InterpError::ParseError(..) => "ParseError",
            InterpError::Numeric(..) => "Numeric",
            InterpError::UndefinedReference(..) => "VariableNotFound",
            InterpError::ProgramFileNotFound(..) => "ProgramFileNotFound",
            InterpError::UndeclaredVariable(..) => "UndeclaredVariable",
            InterpError::FunctionDuplicate(..) => "FunctionDuplicate",
            InterpError::FunctionArgumentsMissmatch(..) => "FunctionArgumentsMissmatch",
        }
    }
}
