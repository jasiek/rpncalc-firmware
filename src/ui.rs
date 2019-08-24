use std::convert::TryInto;
use ncurses as n;

pub struct UI<'a> {
    stack_win: n::WINDOW,
    stack: &'a Vec<i32>
}

const WindowHeight: i32 = 20;
const WindowLength: i32 = 20;

impl<'a> UI<'a> {
    pub fn new(stack: &'a Vec<i32>) -> Self {
        n::initscr();
        n::raw();
        n::noecho();
        Self {
            stack_win: n::newwin(WindowHeight, WindowLength, 0, 0),
            stack: stack
        }
    }

    fn draw(&self) {
        n::box_(self.stack_win, 0, 0);
        let len32: i32 = self.stack.len().try_into().unwrap();
        let miny: i32 = WindowHeight - len32 - 1;
        let x = 1;

        let mut stack_idx = 0;
        for y in miny..(WindowHeight - 1) {
            // TODO: right-justify formatting here
            n::mvwprintw(self.stack_win, y, x, &self.stack[stack_idx].to_string());
            stack_idx += 1;
        }
        
        n::refresh();
        n::wrefresh(self.stack_win);
    }
    
    pub fn run(&self) {
        self.draw();
        n::getch();
    }
}

impl<'a> Drop for UI<'a> {
    fn drop(&mut self) {
        n::endwin();
    }
}

struct EventIterator {
    digits: String
}

enum Event {
    StoreNumber,
    Add,
    Subtract,
    Multiply,
    Divide,
    Clear
}

impl EventIterator {
    fn new() -> Self {
        Self {
            digits: String::new()
        }
    }
}

// impl Iterator for EventIterator {
//     type Item = Event;

//     fn next(&mut self) -> Option<Event> {
//         let c = n::getch();
        
//     }
// }
