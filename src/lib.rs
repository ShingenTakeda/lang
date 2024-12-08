use bytecode::block_to_bytecode;
use instruction::{BinaryOp, EvalResult, Instruction, StackValue};
use lrlex::{lrlex_mod, DefaultLexerTypes};
use lrpar::{lrpar_mod, LexParseError, NonStreamingLexer};
use scope::Scope;

lrlex_mod!("lang.l");
lrpar_mod!("lang.y");

pub mod ast;
pub mod bytecode;
pub mod err;
pub mod instruction;
pub mod scope;

use ast::AstNode;
use err::InterpError;

use crate::{
    instruction::JumpInstruction,
    scope::{Function, Object},
};

pub struct LANG {
    stack: Vec<StackValue>,
}

impl LANG {
    pub fn new() -> Self {
        LANG { stack: vec![] }
    }

    pub fn stack_pop(&mut self) -> Result<StackValue, InterpError> {
        return Ok(self.stack.pop().ok_or(InterpError::EmptyStack)?);
    }

    pub fn stack_push(&mut self, val: StackValue) {
        self.stack.push(val);
    }

    pub fn from_str(&self, input: &str) -> Result<Vec<AstNode>, InterpError> {
        let lexer_def = lang_l::lexerdef();
        let lexer = lexer_def.lexer(input);
        let (ast_exp, errs) = lang_y::parse(&lexer);

        let err_msg = self.get_parse_err(&lexer, errs);
        if err_msg.is_empty() == false {
            return Err(InterpError::ParseError(err_msg));
        }

        match ast_exp {
            Some(res) => match res {
                Ok(exp) => Ok(exp),
                Err(_) => Err(InterpError::ParseError(err_msg)),
            },
            None => Err(InterpError::ParseError(err_msg)),
        }
    }

    fn get_parse_err(
        &self,
        lexer: &dyn NonStreamingLexer<DefaultLexerTypes>,
        errs: Vec<LexParseError<u32, DefaultLexerTypes>>,
    ) -> String {
        let msgs = errs
            .iter()
            .map(|e| e.pp(lexer, &lang_y::token_epp))
            .collect::<Vec<String>>();
        return msgs.join("\n");
    }

    pub fn ast_to_bytecode(ast: Vec<AstNode>) -> Vec<Instruction> {
        return block_to_bytecode(ast);
    }

    fn eval_function_args(
        &mut self,
        args: &Vec<Vec<Instruction>>,
        scope: Scope,
    ) -> Result<Vec<EvalResult>, InterpError> {
        let mut result = vec![];
        for arg_set in args {
            match self.eval(arg_set, scope.clone()) {
                Ok(Some(x)) => result.push(x),
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }
        return Ok(result);
    }

    fn construct_function_scope(
        &self,
        id: String,
        outer_scope: Scope,
        args: &Vec<EvalResult>,
        params: &Vec<String>,
    ) -> Result<Scope, InterpError> {
        if params.len() != args.len() {
            return Err(InterpError::FunctionArgumentsMissmatch(
                id,
                params.len(),
                args.len(),
            ));
        }
        let func_scope = Scope::from_scope(id.to_string(), outer_scope.clone());
        // bind args and params to funciton scope
        for (i, arg) in args.iter().enumerate() {
            if let EvalResult::Value(val) = arg {
                func_scope.clone().dec_var(params[i].clone(), val.clone());
            }
        }
        Ok(func_scope)
    }

    fn eval_function_call(
        &mut self,
        func_name: &String,
        args: &Vec<EvalResult>,
        scope: &Scope,
    ) -> Result<Option<EvalResult>, InterpError> {
        match scope.get_var(func_name.clone()) {
            Some(Object::Value {
                value: StackValue::Function(id, f_dec),
            }) => match *f_dec {
                Object::Function {
                    scope,
                    func: Function { params, block, .. },
                } => {
                    let func_scope =
                        self.construct_function_scope(id.clone(), *scope.clone(), args, &params)?;
                    return self.eval(&block.clone(), func_scope);
                }
                _ => Err(InterpError::UndefinedFunction(id.to_string())),
            },
            Some(Object::Function {
                func: Function { params, block, .. },
                scope,
            }) => {
                let func_scope = self.construct_function_scope(
                    func_name.clone(),
                    *scope.clone(),
                    args,
                    &params,
                )?;
                return self.eval(&block.clone(), func_scope);
            }
            _ => Err(InterpError::UndefinedFunction(func_name.to_string())),
        }
    }

    fn eval_binary_op(&mut self, op: &BinaryOp, scope: Scope) -> Result<StackValue, InterpError> {
        match op {
            BinaryOp::LessThan => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                Ok(StackValue::Boolean(op2 < op1))
            }
            BinaryOp::GreaterThan => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                Ok(StackValue::Boolean(op1 < op2))
            }
            BinaryOp::Add => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                Ok(StackValue::Integer(
                    op1.checked_add(op2)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?,
                ))
            }
            BinaryOp::Mul => {
                let op1 = self.stack_pop()?.as_int()?;
                let op2 = self.stack_pop()?.as_int()?;
                Ok(StackValue::Integer(
                    op1.checked_mul(op2)
                        .ok_or(InterpError::Numeric("overflowed".to_string()))?,
                ))
            }
            BinaryOp::Assign { name, .. } => {
                let val = self.stack_pop()?;
                match scope.set_var(name.to_string(), val) {
                    Some(val) => Ok(val),
                    None => return Err(InterpError::UndeclaredVariable(name.to_string())),
                }
            }
            BinaryOp::Declare { name, .. } => {
                let val = self.stack_pop()?;
                scope.dec_var(name.to_string(), val.clone());
                Ok(val)
            }
            BinaryOp::Equal => Ok(self.eval_eq()?),
            BinaryOp::NotEqual => Ok(StackValue::Boolean(!self.eval_eq()?.as_bool()?)),
            BinaryOp::LogicalAnd => {
                let op1 = self.stack_pop()?;
                let op2 = self.stack_pop()?;
                if op1.is_same_type(&op2) {
                    Ok(StackValue::Boolean(op1.as_bool()? && op2.as_bool()?))
                } else {
                    return Err(InterpError::EvalError(
                        format!(
                            "Operand {} and Operand {} cannot be applied to logical LogicalAnd operation",
                            op1, op2
                        )
                        .to_string(),
                    ));
                }
            }
            BinaryOp::LogicalOr => {
                let op1 = self.stack_pop()?;
                let op2 = self.stack_pop()?;
                if op1.is_same_type(&op2) {
                    Ok(StackValue::Boolean(op1.as_bool()? || op2.as_bool()?))
                } else {
                    return Err(InterpError::EvalError(
                        format!(
                            "Operand {} and Operand {} cannot be applied to logical LogicalOr operation",
                            op1, op2
                        )
                        .to_string(),
                    ));
                }
            }
        }
    }

    fn eval_eq(&mut self) -> Result<StackValue, InterpError> {
        let op1 = self.stack_pop()?;
        let op2 = self.stack_pop()?;
        let stack_value;
        if op1.is_same_type(&op2) {
            stack_value = StackValue::Boolean(op1 == op2);
            self.stack_push(stack_value.clone())
        } else {
            return Err(InterpError::EvalError(
                format!(
                    "Operand {} and Operand {} cannot be applied to logical LogicalOr operation",
                    op1, op2
                )
                .to_string(),
            ));
        }
        Ok(stack_value)
    }

    pub fn eval_input(input: String) -> Result<Option<EvalResult>, InterpError> {
        let scope = Scope::new();
        let lang = &mut LANG::new();
        let ast = lang.from_str(input.as_str()).unwrap();
        let bytecode = LANG::ast_to_bytecode(ast);
        lang.eval(&bytecode, scope)
    }

    pub fn eval(
        &mut self,
        instructions: &Vec<Instruction>,
        scope: Scope,
    ) -> Result<Option<EvalResult>, InterpError> {
        for instruction in instructions {
            match instruction {
                Instruction::Return { block } => {
                    if let Some(EvalResult::Value(v)) = self.eval(block, scope.clone())? {
                        self.stack_push(v);
                    }
                    return Ok(Some(EvalResult::Jump(JumpInstruction::Return {})));
                }
                Instruction::FunctionDeclaration {
                    block,
                    name,
                    params,
                } => match scope.get_var(name.clone()) {
                    Some(..) => return Err(InterpError::FunctionDuplicate(name.to_string())),
                    None => {
                        scope.dec_func(name.clone(), params.to_vec(), block.to_vec());
                    }
                },
                Instruction::FunctionCall { id, args } => {
                    let args = self.eval_function_args(&args, scope.clone())?;
                    if let Some(EvalResult::Value(x)) =
                        self.eval_function_call(id, &args, &scope.clone())?
                    {
                        self.stack_push(x);
                    }
                }
                Instruction::Push { value } => self.stack_push(value.clone()),
                Instruction::PrintLn => {
                    println!("{}", self.stack_pop()?);
                }
                Instruction::Load { id } => match scope.get_var(id.to_string()) {
                    Some(obj) => match obj.clone() {
                        Object::Value { value } => self.stack_push(value),
                        Object::Function { .. } => {
                            self.stack_push(StackValue::Function(id.to_string(), Box::new(obj)))
                        }
                    },
                    _ => return Err(InterpError::UndefinedReference(id.to_string())),
                },
                Instruction::BinaryOp { op } => {
                    let val = self.eval_binary_op(op, scope.clone())?;
                    self.stack_push(val);
                }
                Instruction::Conditional {
                    condition,
                    block,
                    alternative,
                } => {
                    if let Ok(Some(EvalResult::Value(StackValue::Boolean(val)))) =
                        self.eval(condition, scope.clone())
                    {
                        let mut block_result = None;
                        if val {
                            block_result = self.eval(block, scope.clone())?;
                        } else if let Some(alt) = alternative {
                            block_result = self.eval(alt, scope.clone())?;
                        }
                        if let Some(EvalResult::Jump(JumpInstruction::Return)) = block_result {
                            break;
                        }
                    }
                }
            }
        }
        let result;
        if self.stack.is_empty() {
            result = Ok(None);
        } else {
            let val = self.stack_pop()?;
            result = Ok(Some(EvalResult::Value(val)));
        }
        return result;
    }
}
