extern crate ncurses;

use ncurses::*;

use std::fmt;

fn main() {
    // set locale to terminals
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "zh_CN.UTF-8");

    initscr();

    addstr("Hello World!");

    refresh();

    getch();

    endwin();

    // println!("Here is a gay list. {}", HomosexualList(vec![1., 2., 3.]));
}

struct HomosexualList(Vec<f64>);

impl fmt::Display for HomosexualList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (idx, val) in vec.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }

        write!(f, "]")
    }
}
