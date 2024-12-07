use crate::ast::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Mul,
    Push { value: u64 },
}

pub fn ast_to_bytecode(node: Node, ops: &mut Vec<Op>) {
    match node {
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
        Node::Number { value } => ops.push(Op::Push { value }),
    }
}

pub fn eval_bytecode(ast: Vec<Node>) -> Option<u64> {
    let ops = &mut vec![];
    for a in ast {
        ast_to_bytecode(a, ops);
    }

    let mut stack: Vec<u64> = vec![]; // The VM stack

    for instruction in ops {
        match instruction {
            Op::Push { value } => stack.push(*value),
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
        }
    }

    stack.pop()
}
