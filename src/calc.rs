use num::PrimInt;
use std::string::ToString;
use std::str::FromStr;
    
pub struct Calculator<T: PrimInt + ToString + FromStr> {
    stack: Vec<T>,
}

impl<T: PrimInt + ToString + FromStr> Calculator<T> {
    pub fn new() -> Self {
        let v: Vec<T> = vec![T::zero()];
        Self { stack: v }
    }

    pub fn stack(&self) -> &Vec<T> {
        &self.stack
    }

    pub fn digit(&mut self, d: u8) {
        // TODO: this needs cleanup
        if let Some(top_n) = self.stack.pop() {
            let mut top_s = top_n.to_string();
            let arg = d.to_string();
            top_s.push_str(&arg);
            if let Ok(top_n) = top_s.parse() {
                self.stack.push(top_n);
            }
        }
    }

    pub fn enter(&mut self) {
        let zero = T::zero();
        if  zero != *self.stack.last().unwrap() {
            self.stack.push(zero);
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

    fn apply(&mut self, f: fn(T, T) -> T) {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(f(a, b));
        }
    }
}
