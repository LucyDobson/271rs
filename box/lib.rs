#[derive(Debug)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Stack {
    vals: Option<Node>,
}

pub fn init() -> Stack {
    return Stack {
        vals: None,
    };
}

pub fn push(val: String, mut s: Stack) -> Stack {
    todo!();
}

pub fn pop(mut s: Stack) -> (Option<String>, Stack) {
    todo!();
}
