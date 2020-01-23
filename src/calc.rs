use num::Float;
use std::str::FromStr;
use std::string::ToString;

pub struct Calculator<T: Float + ToString + FromStr> {
    stack: Vec<T>,
}

impl<T: Float + ToString + FromStr> Calculator<T> {
    pub fn new() -> Self {
        let v: Vec<T> = vec![T::zero()];
        Self { stack: v }
    }

    pub fn stack(&self) -> &Vec<T> {
        &self.stack
    }

    pub fn enter(&mut self) {
        let zero = T::zero();
        if zero != *self.stack.last().unwrap() {
            self.stack.push(zero);
        }
    }

    pub fn replace(&mut self, number: T) {
        self.stack.pop();
        self.stack.push(number);
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.enter();
    }

    pub fn drop(&mut self) {
        self.stack.pop();
    }

    pub fn swap(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(a);
            self.stack.push(b);
        }
    }

    pub fn roll(&mut self) {
        self.stack.rotate_right(1);
    }

    // 2-argument functions

    pub fn add(&mut self) {
        self.apply2(|a, b| a + b);
    }

    pub fn sub(&mut self) {
        self.apply2(|a, b| b - a);
    }

    pub fn mul(&mut self) {
        self.apply2(|a, b| a * b);
    }

    pub fn div(&mut self) {
        self.apply2(|a, b| b / a);
    }

    // 1-argument functions

    pub fn sqrt(&mut self) {
        self.apply1(|a| a.sqrt());
    }

    pub fn reciprocal(&mut self) {
        self.apply1(|a| a.recip());
    }

    pub fn pow(&mut self) {
        self.apply2(|a, b| b.powf(a));
    }

    fn apply1(&mut self, f: fn(T) -> T) {
        if self.stack.len() >= 1 {
            let a = self.stack.pop().unwrap();
            self.stack.push(f(a));
        }
    }

    fn apply2(&mut self, f: fn(T, T) -> T) {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(f(a, b));
        }
    }
}
