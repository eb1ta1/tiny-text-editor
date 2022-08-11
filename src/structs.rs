#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// カーソルの位置
pub struct Cursor {
    pub row: usize,
    pub column: usize,
}

// 文字の扱い
pub struct Character {
    pub element: char,
    pub start: usize,  // 文字の起点となるx座標
    pub length: usize, // element の長さ。
}
// 編集するファイルの内容と状態
pub struct Document {
    pub rows: Vec<Vec<Character>>,
}
