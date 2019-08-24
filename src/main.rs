use std::io;
use std::io::Read;
use std::convert::TryInto;

struct KeyboardInput {
}

#[derive(Debug)]
enum Key {
    NoOp,
    Digit(u8),
    Add,
    Subtract,
    Multiply,
    Divide,
    Enter,
    Clear
}

impl Iterator for KeyboardInput {
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        let mut buf: [u8; 1] = [0];
        if let Ok(()) = io::stdin().read_exact(&mut buf) {
            let c: char = char::from(buf[0]);
            return match c {
                '+' => Some(Key::Add),
                '-' => Some(Key::Subtract),
                '*' => Some(Key::Multiply),
                '/' => Some(Key::Divide),
                '\n' => Some(Key::Enter),
                'c' => Some(Key::Clear),
                '0'..='9' => {
                    let digit: u8 = c.to_digit(10).unwrap().try_into().unwrap();
                    Some(Key::Digit(digit))
                },
                _ => {
                    Some(Key::NoOp)
                }
            }
        }
        // ^D ends up here
        None
    }
}

#[derive(Debug)]
struct Calculator {
    stack: Vec<i32>,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {
            stack: vec![0],
        }
    }
    
    fn append_digit(&mut self, d: u8) {
        let mut digits;
        match self.stack.pop() {
            Some(n) => {
                digits = n.to_string(); 
            },
            None => {
                digits = "0".to_string();
            }
        }
        digits.push_str(&d.to_string());
        let number = digits.parse::<i32>().unwrap();
        self.stack.push(number);
    }

    fn enter(&mut self) {
        if let Some(i) = self.stack.last() {
            if i.ne(&0) {
                self.stack.push(0);
            }
        }
    }

    fn add(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            self.stack.push(a+b);
        }
    }
}

fn main() {
    let k = KeyboardInput{};
    let mut c = Calculator::new();
    for b in k {
        println!("{:?}", c);
        match b {
            Key::Digit(d) => {
                c.append_digit(d);
            }
            Key::Enter => {
                c.enter();
            }
            Key::Add => {
                c.add();
            }
            _ => {}
        }
    }
}
