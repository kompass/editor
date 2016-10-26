extern crate termion;

mod state;

use termion::raw::IntoRawMode;
use termion::{clear, cursor, color};
use std::io::{Write, stdout, stdin};
use termion::input::TermRead;
use termion::event::Key;

fn main() {
	// Enter raw mode.
	let mut stdout = stdout().into_raw_mode().unwrap();
	let stdin = stdin();

	writeln!(stdout, "{clear}{goto_print}Hey there.{goto_input}", clear = clear::All, goto_print = cursor::Goto(4, 2), goto_input = cursor::Goto(4, 3)).unwrap();

	for c in stdin.keys() {
		match c.unwrap() {
			Key::Char(c) => write!(stdout, "{}", c).unwrap(),
			Key::Ctrl('c') => break,
			_ => continue,
		};

		stdout.flush().unwrap();
	}

	// Here the destructor is automatically called, and the terminal state is restored.
}