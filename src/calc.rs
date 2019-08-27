pub struct Calculator {
    stack: Vec<i32>,
}

impl Calculator {
    pub fn new() -> Self {
        let v: Vec<i32> = vec![0];
        Self { stack: v }
    }

    pub fn stack(&self) -> Vec<i32> {
        self.stack.clone()
    }

    pub fn digit(&mut self, d: u8) {}

    pub fn add(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(a + b);
        }
    }
}
