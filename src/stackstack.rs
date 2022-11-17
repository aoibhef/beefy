pub struct StackStack {
    stacks: Vec<Vec<i64>>,
}

impl StackStack {
    pub fn new() -> StackStack {
        StackStack { stacks: vec![vec![]] }
    }

    pub fn push(&mut self, n: i64) {
        match self.stacks.last_mut() {
            Some(s) => s.push(n),
            None => unreachable!()
        }
    }

    pub fn pop(&mut self) -> i64 {
        match self.stacks.last_mut() {
            Some(s) => s.pop().unwrap_or(0),
            None => unreachable!()
        }
    }
}