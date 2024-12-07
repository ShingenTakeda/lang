use crate::ast::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Mul,
    Push { value: u64 },
    Lt,
    Gt,
    Jz(usize),
    Jump(usize),
}

pub fn ast_to_bytecode(node: Node, ops: &mut Vec<Op>) {
    match node {
        Node::Number { value } => ops.push(Op::Push { value }),
        Node::Add { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Add {});
        }
        Node::Mul { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Mul {});
        }
        Node::Lt { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Lt);
        }
        Node::Gt { lhs, rhs } => {
            ast_to_bytecode(*lhs, ops);
            ast_to_bytecode(*rhs, ops);
            ops.push(Op::Gt);
        }
        Node::If { cond, then_branch, else_branch } => {
            ast_to_bytecode(*cond, ops);
            let jz_index = ops.len();
            ops.push(Op::Jz(0));
            ast_to_bytecode(*then_branch, ops);
            let jump_index = ops.len();
            ops.push(Op::Jump(0));
            let else_index = ops.len();
            ast_to_bytecode(*else_branch, ops);
            let end_index = ops.len();
            if let Op::Jz(ref mut target) = ops[jz_index] {
                *target = else_index;
            }
            if let Op::Jump(ref mut target) = ops[jump_index] {
                *target = end_index;
            }
        }
        
        
    }
}

pub fn eval_bytecode(ast: Vec<Node>) -> Option<u64> {
    let mut ops = vec![];
    for a in ast {
        ast_to_bytecode(a, &mut ops);
    }

    let mut stack: Vec<u64> = vec![];
    let mut pc = 0;

    while pc < ops.len() {
        match ops[pc] {
            Op::Push { value } => {
                stack.push(value);
            }
            Op::Add => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            Op::Mul => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            Op::Lt => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push((lhs < rhs) as u64);
            }
            Op::Gt => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push((lhs > rhs) as u64);
            }
            Op::Jz(target) => {
                let val = stack.pop().unwrap();
                if val == 0 {
                    pc = target;
                    continue;
                }
            }
            Op::Jump(target) => {
                pc = target;
                continue;
            }
        }
        pc += 1;
    }

    stack.pop()
}

