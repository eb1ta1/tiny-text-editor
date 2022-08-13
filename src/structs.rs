use std::path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// カーソルの位置　0-indexed
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    pub buffer: Vec<Vec<char>>,
    pub start_positions: Vec<Vec<usize>>,
    pub cursor: Cursor,
    pub row_offset: usize,
    pub path: Option<path::PathBuf>,
    pub widths: Vec<usize>,
}
