use std::convert::TryInto;
use ncurses as n;
use crate::calc::Calculator;

pub struct UI {
    stack_win: n::WINDOW,
    calc: Calculator
}

const WINDOW_HEIGHT: i32 = 20;
const WINDOW_LENGTH: i32 = 20;

impl UI {
    pub fn new() -> Self {
        n::initscr();
        n::raw();
        n::noecho();
        Self {
            stack_win: n::newwin(WINDOW_HEIGHT, WINDOW_LENGTH, 0, 0),
            calc: Calculator::new()
        }
    }

    fn draw(&self) {
        let stack = self.calc.stack();
        n::box_(self.stack_win, 0, 0);
        let len32: i32 = stack.len().try_into().unwrap();
        let miny: i32 = WINDOW_HEIGHT - len32 - 1;
        let x = 1;

        let mut stack_idx = 0;
        for y in miny..(WINDOW_HEIGHT - 1) {
            // TODO: right-justify formatting here
            n::mvwprintw(self.stack_win, y, x, &stack[stack_idx].to_string());
            stack_idx += 1;
        }
        
        n::refresh();
        n::wrefresh(self.stack_win);
    }
    
    pub fn run(&mut self) {
        self.draw();
        let c = n::getch();
        match c {
            48..=58 => {
                let d: u8 = (c - 48).try_into().unwrap();
                self.calc.digit(d);
            }
            _ => {
            }
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        n::endwin();
    }
}
