use std::io;
use std::io::Read;

struct KeyboardInput {
}

#[derive(Debug)]
enum Key {
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
            return match buf[0] {
                0x2b => Some(Key::Add),
                0x2d => Some(Key::Subtract),
                0x2a => Some(Key::Multiply),
                0x2f => Some(Key::Divide),
                0x10 => Some(Key::Enter),
                99 => Some(Key::Clear),
                x @ 48..=59 => Some(Key::Digit(x - 48)),
                _ => None
            }
        }
        None
    }
}

fn main() {
    let k = KeyboardInput{};
    for b in k {
        println!("{:?}", b)
    }
}
