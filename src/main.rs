// use clap::{App, Arg};
// use std::cmp::{max, min};
// use std::ffi::OsStr;
use std::fs;
// use std::io::{self, stdin, stdout, Error, Write};
use std::path;
// use termion::clear;
// use termion::cursor;
// use termion::event::{Event, Key};
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
// use termion::screen::AlternateScreen;
// use unicode_width::UnicodeWidthChar;

pub mod structs;

use structs::{Cursor, Document};

impl Default for Editor {
    fn default() -> Self {
        Self {
            cursor_position: Cursor { row: 0, column: 0 },
            row_offset: 0,
            path: None,
            document: Document {
                rows: vec![Vec::new()],
            },

            edited: false,
        }
    }
}
// エディタの内部状態
struct Editor {
    cursor_position: Cursor,
    path: Option<path::PathBuf>,
    // 画面の一番上のy座標
    row_offset: usize,
    document: Document,
    edited: bool,
    // should_quit: bool,
    // terminal: Terminal,
    // status_message: StatusMessage,
    // quit_times: u8,
}

impl Editor {
    fn open(&mut self, path: &path::Path) {
        let contents = fs::read_to_string(path).ok();
        match contents {
            Some(contents) => println!("{}", contents),
            None => print!("Something went wrong :D"),
        }
    }
}
fn main() {
    let mut state = Editor::default();
    let file_path = "assets/ example.txt";
    state.open(path::Path::new(file_path))
}
