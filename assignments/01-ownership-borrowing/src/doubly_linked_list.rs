// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {}

type Link = Option<Box<Node>>;

pub struct LinkedStack {}

impl LinkedStack {
    fn new() -> Self {
        todo!();
    }

    fn push(&mut self, val: i32) {
        todo!()
    }

    fn pop(&mut self) -> Option<i32> {
        todo!();
    }
}
