pub mod structs;
// use clap::{App, Arg};
// use std::ffi::OsStr;
use std::cmp::{max, min};
use std::io::{self, stdin, stdout, Write};
use std::{fs, vec};
use std::{path, usize};
use structs::{Cursor, Editor};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use unicode_width::UnicodeWidthStr;

impl Default for Editor {
    fn default() -> Self {
        Self {
            buffer: vec![Vec::new()],
            start_positions: vec![],
            cursor: Cursor { y: 0, x: 0 },
            row_offset: 0,
            path: None,
        }
    }
}

impl Editor {
    fn open(&mut self, path: &path::Path) {
        self.buffer = fs::read_to_string(path)
            .ok()
            .map(|s| {
                let buffer: Vec<Vec<char>> = s
                    .lines()
                    .map(|line| line.trim_end().chars().collect())
                    .collect();
                if buffer.is_empty() {
                    vec![Vec::new()]
                } else {
                    buffer
                }
            })
            .unwrap_or_else(|| vec![Vec::new()]);
        for vec in self.buffer.clone() {
            let start_position = self.calc_start_positions(vec);
            self.start_positions.push(start_position);
        }
        self.path = Some(path.into());
        self.cursor = Cursor { y: 0, x: 0 };
        self.row_offset = 0;
    }
    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }
    fn calc_start_positions(&mut self, vec: Vec<char>) -> Vec<usize> {
        let mut start_position = Vec::new();
        let mut cnt: usize = 0;
        for character in &vec {
            let width: usize = character.to_string().width();

            cnt += width;
            start_position.push(cnt - width);
        }
        start_position
    }
    fn draw<T: Write>(&mut self, out: &mut T) -> Result<(), io::Error> {
        let (rows, _cols) = Self::terminal_size();

        write!(out, "{}", clear::All)?;
        write!(out, "{}", cursor::Goto(1, 1))?;

        let mut row = 0;
        let mut _col = 0;

        for i in self.row_offset..self.buffer.len() {
            for j in 0..=self.buffer[i].len() {
                if let Some(c) = self.buffer[i].get(j) {
                    write!(out, "{}", c)?;
                }
            }
            row += 1;
            _col = 0;
            if row >= rows {
                break;
            } else {
                write!(out, "\r\n")?;
            }
        }

        if !(self.start_positions[self.cursor.y].is_empty()) {
            if self.cursor.x == self.start_positions[self.cursor.y].len() {
                let last_char_width = self.buffer[self.cursor.y][self.cursor.x - 1]
                    .to_string()
                    .width();
                write!(
                    out,
                    "{}",
                    cursor::Goto(
                        (self.start_positions[self.cursor.y][self.cursor.x - 1] + last_char_width)
                            as u16
                            + 1,
                        self.cursor.y as u16 + 1
                    )
                )?;
            } else if let Some((x, y)) = Some((
                self.start_positions[self.cursor.y][self.cursor.x],
                self.cursor.y,
            )) {
                write!(out, "{}", cursor::Goto(x as u16 + 1, y as u16 + 1))?;
            }
        } else if let Some((x, y)) = Some((0, self.cursor.y)) {
            write!(out, "{}", cursor::Goto(x as u16 + 1, y as u16 + 1))?;
        }

        out.flush()?;
        Ok(())
    }
    fn _draw_status_bar(&mut self, _debug_mode: bool) {}
    fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.row_offset = min(self.row_offset, self.cursor.y);
        if self.cursor.y + 1 >= rows {
            self.row_offset = max(self.row_offset, self.cursor.y + 1 - rows);
        }
    }
    fn cursor_up(&mut self) {
        // if self.cursor.y > 0 {
        //     if self.buffer[self.cursor.y - 1].is_empty() || self.buffer[self.cursor.y].is_empty() {
        //         self.cursor.x = 0;
        //     } else {
        //         let mut prev_x_position_data = Vec::new();
        //         if self.start_positions[self.cursor.y].len() <= self.cursor.x {
        //             prev_x_position_data
        //                 .push(self.start_positions[self.cursor.y][self.cursor.x - 1]);
        //         } else {
        //             prev_x_position_data.push(self.start_positions[self.cursor.y][self.cursor.x]);
        //         }
        //         let mut flag = false;
        //         let prev_x_position: usize = prev_x_position_data[0];
        //         for i in 0..self.start_positions[self.cursor.y - 1].len() - 1 {
        //             if (prev_x_position <= self.start_positions[self.cursor.y - 1][i])
        //                 && (prev_x_position < self.start_positions[self.cursor.y - 1][i + 1])
        //             {
        //                 self.cursor.x = min(self.buffer[self.cursor.y - 1].len(), i);
        //                 flag = true;
        //                 break;
        //             }
        //         }
        //         if !flag {
        //             self.cursor.x = self.buffer[self.cursor.y - 1].len();
        //         }
        //     }
        //     self.cursor.y -= 1;
        // } else {
        //     self.cursor.x = 0;
        // }
        if self.cursor.y > 0 {
            if self.buffer[self.cursor.y - 1].is_empty() || self.buffer[self.cursor.y].is_empty() {
                self.cursor.x = 0;
            } else {
                let mut prev_x_position_data = Vec::new();
                if self.start_positions[self.cursor.y].len() <= self.cursor.x {
                    prev_x_position_data
                        .push(self.start_positions[self.cursor.y][self.cursor.x - 1]);
                } else {
                    prev_x_position_data.push(self.start_positions[self.cursor.y][self.cursor.x]);
                }
                let prev_x_position = prev_x_position_data[0];
                let mut flag = false;
                for i in 0..self.start_positions[self.cursor.y - 1].len() - 1 {
                    if (self.start_positions[self.cursor.y - 1][i] <= prev_x_position)
                        && (prev_x_position < self.start_positions[self.cursor.y - 1][i + 1])
                    {
                        self.cursor.x = min(self.buffer[self.cursor.y - 1].len(), i);
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    self.cursor.x = self.buffer[self.cursor.y - 1].len();
                }
            }
            self.cursor.y -= 1;
        } else {
            self.cursor.x = 0;
        }
        self.scroll();
    }
    fn cursor_down(&mut self) {
        if self.cursor.y + 1 < self.buffer.len() {
            if self.buffer[self.cursor.y + 1].is_empty() || self.buffer[self.cursor.y].is_empty() {
                self.cursor.x = 0;
            } else {
                let mut prev_x_position_data = Vec::new();
                if self.start_positions[self.cursor.y].len() <= self.cursor.x {
                    prev_x_position_data
                        .push(self.start_positions[self.cursor.y][self.cursor.x - 1]);
                } else {
                    prev_x_position_data.push(self.start_positions[self.cursor.y][self.cursor.x]);
                }
                let prev_x_position = prev_x_position_data[0];
                let mut flag = false;
                for i in 0..self.start_positions[self.cursor.y + 1].len() - 1 {
                    if (self.start_positions[self.cursor.y + 1][i] <= prev_x_position)
                        && (prev_x_position < self.start_positions[self.cursor.y + 1][i + 1])
                    {
                        self.cursor.x = min(self.buffer[self.cursor.y + 1].len(), i);
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    self.cursor.x = self.buffer[self.cursor.y + 1].len();
                }
            }
            self.cursor.y += 1;
        } else {
            self.cursor.x = self.buffer[self.cursor.y].len();
        }
        self.scroll();
    }
    fn cursor_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
            if !(self.buffer[self.cursor.y].is_empty()) {
                self.cursor.x = self.buffer[self.cursor.y].len();
            } else {
                self.cursor.x = 0;
            }
        }
        self.scroll();
    }
    fn cursor_right(&mut self) {
        if self.cursor.x < self.buffer[self.cursor.y].len() {
            self.cursor.x += 1;
        } else if self.cursor.y + 1 < self.buffer.len() {
            self.cursor.y += 1;
            self.cursor.x = 0;
        } else {
            self.cursor.x = self.buffer[self.cursor.y].len();
        }
        self.scroll();
    }
    fn insert(&mut self, c: char) {
        if c == '\n' {
            let rest: Vec<char> = self.buffer[self.cursor.y].drain(self.cursor.x..).collect();
            self.buffer.insert(self.cursor.y + 1, rest);
            self.start_positions.insert(self.cursor.y, vec![]);
            self.cursor.y += 1;
            self.cursor.x = 0;
            self.scroll();
        } else if !c.is_control() {
            self.buffer[self.cursor.y].insert(self.cursor.x, c);
            self.start_positions[self.cursor.y] =
                self.calc_start_positions(self.buffer[self.cursor.y].clone());
            self.cursor_right();
        }
    }
    fn back_space(&mut self) {
        if self.cursor == (Cursor { y: 0, x: 0 }) {
            return;
        }

        if self.cursor.x == 0 {
            let line = self.buffer.remove(self.cursor.y);
            self.cursor.y -= 1;
            self.cursor.x = self.buffer[self.cursor.y].len();
            self.buffer[self.cursor.y].extend(line.into_iter());
            self.start_positions.remove(self.cursor.y);
        } else {
            self.cursor_left();
            self.buffer[self.cursor.y].remove(self.cursor.x);

            self.start_positions[self.cursor.y] =
                self.calc_start_positions(self.buffer[self.cursor.y].clone());
        }
    }
    fn delete(&mut self) {
        if self.cursor.y == self.buffer.len() - 1
            && self.cursor.x == self.buffer[self.cursor.y].len()
        {
            return;
        }

        if self.cursor.x == self.buffer[self.cursor.y].len() {
            let line = self.buffer.remove(self.cursor.y + 1);
            self.buffer[self.cursor.y].extend(line.into_iter());
            self.start_positions.remove(self.cursor.y);
        } else {
            self.buffer[self.cursor.y].remove(self.cursor.x);
            self.start_positions[self.cursor.y] =
                self.calc_start_positions(self.buffer[self.cursor.y].clone());
        }
    }
    fn save(&self) {
        if let Some(path) = self.path.as_ref() {
            if let Ok(mut file) = fs::File::create(path) {
                for line in &self.buffer {
                    for &c in line {
                        write!(file, "{}", c).unwrap();
                    }
                    writeln!(file).unwrap();
                }
            }
        }
    }
}

fn main() {
    // let matches = App::new("Editor")
    //     .about("A text editor")
    //     .bin_name("Editor")
    //     .arg(Arg::with_name("file").required(true))
    //     .get_matches();

    // let file_path: &OsStr = matches.value_of_os("file").unwrap();

    let file_path = "assets/example.txt";
    let mut state = Editor::default();

    state.open(path::Path::new(file_path));

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    state.draw(&mut stdout).unwrap();

    for evt in stdin.events() {
        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            }
            Event::Key(Key::Ctrl('s')) => {
                state.save();
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
            Event::Key(Key::Char(c)) => {
                state.insert(c);
            }
            Event::Key(Key::Backspace) => {
                state.back_space();
            }
            Event::Key(Key::Delete) => {
                state.delete();
            }
            _ => {}
        }
        state.draw(&mut stdout).unwrap();
    }
}
