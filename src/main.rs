#![feature(plugin)]
#![plugin(clippy)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate ncurses as nc;

mod errors;

use errors::*;
use std::env;

fn main () {
  // setup ncurses
  nc::initscr();
  nc::raw();

  // allow for extended keyboard
  nc::keypad(nc::stdscr(), true);
  nc::noecho();

  // get screen sizes
  let mut max_x = 0;
  let mut max_y = 0;
  nc::getmaxyx(nc::stdscr(), &mut max_y, &mut max_x);
  header(max_x, max_y);

  nc::getch();

  // terminate
  nc::endwin();
}

// render the header bar
fn header (max_x: i32, max_y: i32) {
  let path = env::current_dir().unwrap();
  let left = path.to_str().unwrap();
  nc::printw(left);

  let right = "[ 1 ] 2 3 4 5 6";
  let rl = right.len() as i32;
  nc::mv(0, max_x - rl);
  nc::printw(right);
}
