#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// カーソルの位置
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
// 文字の扱い
pub struct Character {
    pub element: char,
    pub start: usize,  // 文字の起点となるx座標
    pub length: usize, // element の長さ (unicode)
}
