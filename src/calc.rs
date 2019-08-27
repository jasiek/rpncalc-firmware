pub struct Calculator {
    stack: Vec<i32>,
}

impl Calculator {
    pub fn new() -> Self {
        let v: Vec<i32> = vec![0];
        Self { stack: v }
    }

    pub fn stack(&self) -> &Vec<i32> {
        &self.stack
    }

    pub fn digit(&mut self, d: u8) {
        let top_n = self.stack.pop().unwrap();
        let mut top_s = top_n.to_string();
        let arg = d.to_string();
        top_s.push_str(&arg);
        let top_n = top_s.parse().unwrap();
        self.stack.push(top_n);
    }

    pub fn enter(&mut self) {
        if 0i32 != *self.stack.last().unwrap() {
            self.stack.push(0);
        }
    }

    pub fn add(&mut self) {
        self.apply(|a, b| a + b);
    }

    pub fn sub(&mut self) {
        self.apply(|a, b| b - a);
    }

    pub fn mul(&mut self) {
        self.apply(|a, b| a * b);
    }

    pub fn div(&mut self) {
        self.apply(|a, b| b / a);
    }

    fn apply(&mut self, f: fn(i32, i32) -> i32) {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(f(a, b));
        }
    }
}
