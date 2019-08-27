mod calc;
mod ui;

fn main() {
    let mut u = ui::UI::new();
    loop {
        u.run();
    }
}
