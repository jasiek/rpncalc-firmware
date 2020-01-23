extern crate num;
use crate::calc::Calculator;

mod calc;
mod ui;

fn main() {
    let mut calc = Calculator::<f64>::new();
    let mut u = ui::UI::new(&mut calc);
    u.run();
}
