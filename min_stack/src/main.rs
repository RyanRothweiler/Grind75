fn main() {
    println!("Hello, world!");
}

struct StackEntryValue {
    val: i32,
    current_min: i32,
}

struct MinStack {
    stack: Vec<StackEntryValue>,
}

impl MinStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        let curr_min: i32 = match self.stack.last() {
            Some(v) => v.current_min,
            None => i32::MAX,
        };

        let new_entry = StackEntryValue {
            val,
            current_min: i32::min(curr_min, val),
        };

        self.stack.push(new_entry);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().current_min
    }
}
