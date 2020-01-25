use crate::calc::Calculator;
use ncurses as n;
use num::Float;
use std::char;
use std::convert::TryInto;
use std::str::FromStr;
use std::string::ToString;

pub struct UI<'a, T: Float + ToString + FromStr> {
    stack_win: n::WINDOW,
    help_win: n::WINDOW,
    calc: &'a mut Calculator<T>,
    current_number: String,
}

const WINDOW_HEIGHT: i32 = 20;
const WINDOW_LENGTH: i32 = 40;

const KEY_DOT: i32 = 46;
const KEY_0: i32 = 48;
const KEY_1: i32 = 49;
const KEY_2: i32 = 50;
const KEY_3: i32 = 51;
const KEY_4: i32 = 52;
const KEY_5: i32 = 53;
const KEY_6: i32 = 54;
const KEY_7: i32 = 55;
const KEY_8: i32 = 56;
const KEY_9: i32 = 57;
const KEY_EQ: i32 = 61;
const KEY_C: i32 = 99;
const KEY_D: i32 = 100;
const KEY_E: i32 = 101;
const KEY_Q: i32 = 113;
const KEY_R: i32 = 114;
const KEY_S: i32 = 115;
const KEY_W: i32 = 119;
const KEY_PLUS: i32 = 43;
const KEY_MINUS: i32 = 45;
const KEY_MULTIPLY: i32 = 42;
const KEY_DIVIDE: i32 = 47;
const KEY_ENTER: i32 = 10;
const KEY_CARET: i32 = 94;

const HELP: &str = "0-9 - digits
= - toggle +/-
r - 1/x
s - square root
^ - power
d - drop
c - clear
e - roll
w - swap

q - quit";

impl<'a, T: Float + ToString + FromStr> UI<'a, T> {
    pub fn new(calc: &'a mut Calculator<T>) -> Self {
        n::initscr();
        n::cbreak();
        n::noecho();
        let ui = Self {
            stack_win: n::newwin(WINDOW_HEIGHT, WINDOW_LENGTH, 0, 0),
            help_win: n::newwin(WINDOW_HEIGHT, WINDOW_LENGTH, 0, WINDOW_LENGTH + 1),
            calc: calc,
            current_number: String::new(),
        };
        ui.draw();
        ui
    }

    fn draw(&self) {
        let stack = self.calc.stack();
        n::refresh();
        n::wclear(self.stack_win);
        n::wclear(self.help_win);
        n::box_(self.stack_win, 0, 0);
        n::box_(self.help_win, 0, 0);

        let len32: i32 = stack.len().try_into().unwrap();
        let miny: i32 = WINDOW_HEIGHT - len32 - 2;

        let mut stack_idx = 0;
        n::mvwprintw(self.stack_win, 0, 1, "Stack");
        for y in miny..(WINDOW_HEIGHT - 2) {
            // TODO: right-justify formatting here
            n::mvwprintw(self.stack_win, y, 1, &stack[stack_idx].to_string());
            stack_idx += 1;
        }
        n::mvwprintw(self.stack_win, WINDOW_HEIGHT - 2, 1, &self.current_number);

        // draw help
        let mut help_idx = 1;
        n::mvwprintw(self.help_win, 0, 1, "Help");
        for l in HELP.lines() {
            n::mvwprintw(self.help_win, help_idx, 1, l);
            help_idx += 1;
        }

        n::wrefresh(self.stack_win);
        n::wrefresh(self.help_win);
    }

    pub fn compose(&mut self, c: i32) {
        match c {
            KEY_0 => {
                if self.current_number.len() > 0 {
                    self.current_number.push('0');
                }
            }
            KEY_1..=KEY_9 => {
                let d = (c - 48).try_into().unwrap();
                self.current_number.push(char::from_digit(d, 10).unwrap());
            }
            KEY_DOT => {
                if !self.current_number.contains(".") {
                    self.current_number.push('.');
                }
            }
            KEY_EQ => {
                if self.current_number.contains("-") {
                    self.current_number.remove(0);
                } else {
                    self.current_number.insert(0, '-');
                }
            }
            _ => {
                panic!("unmatched");
            }
        }
        if let Ok(number) = self.current_number.parse() {
            self.calc.replace(number);
        }
    }

    pub fn run(&mut self) {
        loop {
            let c = n::getch();
            match c {
                KEY_Q => {
                    break;
                }
                _ => {
                    self.feed(c);
                }
            }
        }
    }

    pub fn feed_test_helper(&mut self, chars: Vec<i32>) {
        for c in chars {
            self.feed(c);
        }
    }

    pub fn feed(&mut self, c: i32) {
        match c {
            KEY_Q => {
                // TODO: does this belong here?
                return;
            }
            KEY_EQ | KEY_DOT | KEY_0..=KEY_9 => {
                self.compose(c);
            }

            // Operations
            KEY_C => {
                self.calc.clear();
            }
            KEY_D => {
                self.calc.drop();
            }
            KEY_E => {
                self.calc.roll();
            }
            KEY_R => {
                self.calc.reciprocal();
            }
            KEY_S => {
                self.calc.sqrt();
            }
            KEY_W => {
                self.calc.swap();
            }
            KEY_PLUS => {
                self.calc.add();
            }
            KEY_MINUS => {
                self.calc.sub();
            }
            KEY_MULTIPLY => {
                self.calc.mul();
            }
            KEY_DIVIDE => {
                self.calc.div();
            }
            KEY_ENTER => {
                self.calc.enter();
            }
            KEY_CARET => {
                self.calc.pow();
            }
            _ => {}
        }

        // Cleanup after an operation the stack
        match c {
            KEY_EQ | KEY_DOT | KEY_0..=KEY_9 => {}
            _ => {
                self.current_number.clear();
            }
        }
        self.draw();
    }
}

impl<T: Float + ToString + FromStr> Drop for UI<'_, T> {
    fn drop(&mut self) {
        n::endwin();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_number() {
        let mut c = Calculator::<f64>::new();
        let mut u = UI::new(&mut c);
        u.feed_test_helper(
            [
                KEY_9, KEY_8, KEY_7, KEY_6, KEY_DOT, KEY_5, KEY_4, KEY_3, KEY_2, KEY_0, KEY_1,
            ]
            .to_vec(),
        );
        assert_eq!(u.current_number, "9876.543201");
        assert_eq!(u.calc.peek(), 9876.543201);
        u.feed(KEY_EQ);
        assert_eq!(u.current_number, "-9876.543201");
        assert_eq!(u.calc.peek(), -9876.543201);
        u.feed(KEY_ENTER);
        assert_eq!(u.current_number, "");
        assert_eq!(u.calc.peek(), 0.0);
    }
}
