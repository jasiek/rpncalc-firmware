use crate::calc::Calculator;
use ncurses as n;
use std::convert::TryInto;

pub struct UI {
    stack_win: n::WINDOW,
    calc: Calculator<i32>,
}

const WINDOW_HEIGHT: i32 = 20;
const WINDOW_LENGTH: i32 = 20;

const KEY_0: i32 = 48;
const KEY_9: i32 = 58;
const KEY_PLUS: i32 = 43;
const KEY_MINUS: i32 = 45;
const KEY_MULTIPLY: i32 = 42;
const KEY_DIVIDE: i32 = 47;
const KEY_ENTER: i32 = 10;

impl UI {
    pub fn new() -> Self {
        n::initscr();
        n::cbreak();
        n::noecho();
        let ui = Self {
            stack_win: n::newwin(WINDOW_HEIGHT, WINDOW_LENGTH, 0, 0),
            calc: Calculator::new(),
        };
        ui.draw();
        ui
    }

    fn draw(&self) {
        let stack = self.calc.stack();
        n::refresh();
        n::wclear(self.stack_win);
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

        n::wrefresh(self.stack_win);
    }

    pub fn run(&mut self) {
        let c = n::getch();
        match c {
            KEY_0..=KEY_9 => {
                let d: u8 = (c - 48).try_into().unwrap();
                self.calc.digit(d);
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
            _ => {}
        }
        self.draw();
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        n::endwin();
    }
}
