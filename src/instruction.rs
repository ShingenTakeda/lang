use std::{
    fmt::{Display, Error, Formatter},
    mem::discriminant,
};

use crate::{err::InterpError, scope::Object};

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    Integer(u64),
    Function(String, Box<Object>),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JumpInstruction {
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
    Value(StackValue),
    Jump(JumpInstruction),
}

impl StackValue {
    pub fn as_int(&self) -> Result<u64, InterpError> {
        match self {
            StackValue::Integer(v) => Ok(*v),
            a => Err(InterpError::EvalError(
                format!("Expected StackValue Integer stack, got {}!", a).to_string(),
            )),
        }
    }
    pub fn as_bool(&self) -> Result<bool, InterpError> {
        match self {
            StackValue::Boolean(v) => Ok(*v),
            a => Err(InterpError::EvalError(
                format!("Expected StackValue Boolean, got {}!", a).to_string(),
            )),
        }
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Display for StackValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let a = match self {
            StackValue::Integer(val) => f.write_str(format!("{}", val).as_str()),
            StackValue::Boolean(val) => f.write_str(format!("{}", val).as_str()),
            StackValue::Function(id, ..) => f.write_str(format!("function {}", id).as_str()),
        };
        return a;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    LessThan,
    GreaterThan,
    Add,
    Mul,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    Assign { name: String },
    Declare { name: String },
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            BinaryOp::LessThan => f.write_str("LessThan"),
            BinaryOp::GreaterThan => f.write_str("GreaterThan"),
            BinaryOp::Add => f.write_str("Add"),
            BinaryOp::Mul => f.write_str("Mul"),
            BinaryOp::Assign { .. } => f.write_str("Assign"),
            BinaryOp::Equal => f.write_str("Equal"),
            BinaryOp::NotEqual => f.write_str("NotEqual"),
            BinaryOp::LogicalAnd => f.write_str("LogicalAnd"),
            BinaryOp::LogicalOr => f.write_str("LogicalOr"),
            BinaryOp::Declare { .. } => f.write_str("Declare"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    BinaryOp {
        op: BinaryOp,
    },
    Push {
        value: StackValue,
    },
    PrintLn,
    Load {
        id: String,
    },
    Return {
        block: Vec<Instruction>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        block: Vec<Instruction>,
    },
    FunctionCall {
        id: String,
        args: Vec<Vec<Instruction>>,
    },
    Conditional {
        condition: Vec<Instruction>,
        block: Vec<Instruction>,
        alternative: Option<Vec<Instruction>>,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Instruction::Conditional { .. } => f.write_str("Conditional"),
            Instruction::Push { .. } => f.write_str("Push"),
            Instruction::PrintLn => f.write_str("PrintLn"),
            Instruction::Load { .. } => f.write_str("Load"),
            Instruction::Return { .. } => f.write_str("Return"),
            Instruction::FunctionDeclaration { .. } => f.write_str("FunctionDeclaration"),
            Instruction::FunctionCall { .. } => f.write_str("FunctionCall"),
            Instruction::BinaryOp { op } => f.write_str(format!("BinaryOp({})", op).as_str()),
        }
    }
}
