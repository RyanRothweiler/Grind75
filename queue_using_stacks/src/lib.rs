struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            input: vec![],
            output: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.flip();
        return self.output.pop().unwrap();
    }

    fn peek(&mut self) -> i32 {
        self.flip();
        return *self.output.last().unwrap();
    }

    fn empty(&self) -> bool {
        return self.input.len() == 0 && self.output.len() == 0;
    }

    fn flip(&mut self) {
        if self.output.len() == 0 {
            loop {
                match self.input.pop() {
                    Some(v) => self.output.push(v),
                    None => break,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(q.empty(), false);

        assert_eq!(q.pop(), 1);
        assert_eq!(q.pop(), 2);

        q.push(4);

        assert_eq!(q.pop(), 3);
        assert_eq!(q.pop(), 4);
    }
}
