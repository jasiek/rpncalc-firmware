use crate::calc::Calculator;
use ncurses as n;
use std::convert::TryInto;

pub struct UI {
    stack_win: n::WINDOW,
    help_win: n::WINDOW,
    calc: Calculator<f32>,
}

const WINDOW_HEIGHT: i32 = 20;
const WINDOW_LENGTH: i32 = 40;

const KEY_0: i32 = 48;
const KEY_9: i32 = 58;
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
r - 1/x
s - square root
^ - power
d - drop
c - clear

q - quit";

impl UI {
    pub fn new() -> Self {
        n::initscr();
        n::cbreak();
        n::noecho();
        let ui = Self {
            stack_win: n::newwin(WINDOW_HEIGHT, WINDOW_LENGTH, 0, 0),
            help_win: n::newwin(WINDOW_HEIGHT, WINDOW_LENGTH, 0, WINDOW_LENGTH+1),
            calc: Calculator::new(),
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
        let miny: i32 = WINDOW_HEIGHT - len32 - 1;

        let mut stack_idx = 0;
        n::mvwprintw(self.stack_win, 0, 1, "Stack");
        for y in miny..(WINDOW_HEIGHT - 1) {
            // TODO: right-justify formatting here
            n::mvwprintw(self.stack_win, y, 1, &stack[stack_idx].to_string());
            stack_idx += 1;
        }

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

    pub fn run(&mut self) {
        loop {
            let c = n::getch();
            match c {
                KEY_0..=KEY_9 => {
                    let d: u8 = (c - 48).try_into().unwrap();
                    self.calc.digit(d);
                }
                KEY_C => {
                    self.calc.clear();
                }
                KEY_D => {
                    self.calc.drop();
                }
                KEY_E => {
                    self.calc.roll();
                }
                KEY_Q => {
                    return;
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
            self.draw();
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        n::endwin();
    }
}
