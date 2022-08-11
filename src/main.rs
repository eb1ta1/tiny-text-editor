use clap::{App, Arg};
use std::cmp::{max, min};
use std::ffi::OsStr;
use std::fmt::write;
use std::fs;
use std::io::{self, stdin, stdout, Error, Write};
use std::path;
use structs::Character;
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use unicode_width::UnicodeWidthStr;

pub mod structs;

use structs::Cursor;

impl Default for Editor {
    fn default() -> Self {
        Self {
            characters: vec![Vec::new()],
            cursor: Cursor { x: 1, y: 1 },
            row_offset: 0,
            path: None,
            edited: false,
        }
    }
}
// エディタの内部状態
struct Editor {
    characters: Vec<Vec<Character>>,
    cursor: Cursor,
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
            self.characters.push(line_vec)
        }
        // println!("{:?}", characters_vec)
        // println!("open() is running");
    }
    fn terminal_size() -> (usize, usize) {
        let (terminal_height, terminal_width) = termion::terminal_size().unwrap();
        (terminal_height as usize, terminal_width as usize)
    }
    fn draw<T: Write>(&self, out: &mut T) -> Result<(), io::Error> {
        let (terminal_height, terminal_width) = Self::terminal_size();
        write!(out, "{}", clear::All)?;
        write!(out, "{}", cursor::Goto(1, 1))?;

        // 画面上の行、列
        let mut x = 0;
        let mut y = 0;

        'outer: for i in self.row_offset..self.characters.len() {
            for j in 0..self.characters[i].len() {
                let c = &self.characters[i][j];
                let width = c.length;
                if y + width >= terminal_width {
                    x += 1;
                    y = 0;
                    if x >= terminal_height {
                        break 'outer;
                    } else {
                        write!(out, "\r\n")?;
                    }
                }
                write!(out, "{}", c.element)?;
                y += width;
            }
            x += 1;
            y = 0;
            if x < terminal_height && x > 1 {
                write!(out, "\r\n")?;
            }
        }
        write!(
            out,
            "{}",
            cursor::Goto(self.cursor.x as u16, self.cursor.y as u16)
        )?;
        out.flush()?;
        Ok(())
    }

    fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.row_offset = min(self.row_offset, self.cursor.x);
        if self.cursor.x + 1 >= rows {
            self.row_offset = max(self.row_offset, self.cursor.x + 1 - rows);
        }
    }
    fn cursor_up(&mut self) {
        if self.cursor.y > 1 {
            self.cursor.y -= 1;
            if !self.characters[self.cursor.y].is_empty() {
                self.cursor.x = min(self.characters[self.cursor.y].len(), self.cursor.x);
            } else {
                self.cursor.x = 1;
            }
        } else {
            self.cursor.x = 1;
        }
    }
    fn cursor_down(&mut self) {
        if self.cursor.y + 1 < self.characters.len() {
            self.cursor.y += 1;
            self.cursor.x = min(self.characters[self.cursor.y].len(), self.cursor.x);
        } else {
            self.cursor.x = 1;
        }
    }
    fn cursor_left(&mut self) {
        if self.cursor.x >= 1 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 1 {
            self.cursor.y -= 1;
            self.cursor.x = self.characters[self.cursor.y].len();
        }
    }
    fn cursor_right(&mut self) {
        if self.cursor.x < self.characters[self.cursor.y].len() {
            self.cursor.x += 1;
        } else if self.cursor.y + 1 < self.characters.len() {
            self.cursor.y += 1;
            self.cursor.x = 1;
        }
    }
}
fn main() {
    let mut state = Editor::default();
    let file_path = "assets/example.txt";
    state.open(path::Path::new(file_path));
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    state.draw(&mut stdout).unwrap();
    for evt in stdin.events() {
        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            }
            Event::Key(Key::Up) => {
                state.cursor_up();
            }
            Event::Key(Key::Down) => {
                state.cursor_down();
            }
            Event::Key(Key::Left) => {
                state.cursor_left();
            }
            Event::Key(Key::Right) => {
                state.cursor_right();
            }
            _ => {}
        }
        state.draw(&mut stdout).unwrap();
    }
}
