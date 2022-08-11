// use clap::{App, Arg};
// use std::cmp::{max, min};
// use st&d::ffi::OsStr;
use std::fs;
// use std::io::{self, stdin, stdout, Error, Write};
use std::path;
use structs::Character;
// use termion::clear;
// use termion::cursor;
// use termion::event::{Event, Key};
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
// use termion::screen::AlternateScreen;
use unicode_width::UnicodeWidthStr;

pub mod structs;

use structs::Cursor;

#[allow(dead_code)]
impl Default for Editor {
    fn default() -> Self {
        Self {
            cursor_position: Cursor { row: 0, column: 0 },
            row_offset: 0,
            path: None,
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
    edited: bool,
    // should_quit: bool,
    // terminal: Terminal,
    // status_message: StatusMessage,
    // quit_times: u8,
}

impl Editor {
    fn open(&mut self, path: &path::Path) {
        let string: String;
        let contents = fs::read_to_string(path).ok();
        match contents {
            Some(contents) => string = contents,
            None => string = "".to_string(),
        };
        let separated_string: Vec<&str> = string.split("\n").collect();
        let mut characters_vec = Vec::new();
        for string in separated_string {
            let mut line_vec = Vec::new();
            let mut start: usize = 0;
            for character in string.to_string().chars() {
                let length = character.to_string().width();
                start += length;
                line_vec.push(Character {
                    element: character,
                    start: start - length,
                    length,
                })
            }
            characters_vec.push(line_vec)
        }
        // println!("{:?}", characters_vec)
    }
    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }
}
fn main() {
    let mut state = Editor::default();
    let file_path = "assets/example.txt";
    state.open(path::Path::new(file_path));
}
