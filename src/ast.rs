#[derive(Debug)]
pub enum Node {
    Number { value: u64 },
    Add { lhs: Box<Node>, rhs: Box<Node> },
    //Sub { lhs: Box<Node>, rhs: Box<Node> },
    Mul { lhs: Box<Node>, rhs: Box<Node> },
    //Div { lhs: Box<Node>, rhs: Box<Node> },
    If { cond: Box<Node>, then_branch: Box<Node>, else_branch: Box<Node> },
    Lt { lhs: Box<Node>, rhs: Box<Node> },
    Gt { lhs: Box<Node>, rhs: Box<Node> },
    
}
