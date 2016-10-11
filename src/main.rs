#![feature(plugin)]
#![plugin(clippy)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate ncurses as nc;
extern crate regex;

mod errors;

use regex::Regex;
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
  let matcher = Regex::new(r"/.*$").unwrap();
  let left = path.to_str().unwrap();
  nc::printw(" ");
  nc::printw(left);

  let right = "[ 1 ]";
  let rl = right.len() as i32;
  nc::mv(0, max_x - rl - 1);
  nc::printw(right);
  nc::printw(" ");
}
