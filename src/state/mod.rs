mod rope;
mod text;

pub use self::rope::Rope;

pub trait Editable {
    fn step(&mut self, mov: Movement);
    fn move_to(&mut self, pos: usize);
    fn move_at(&mut self, line: usize, col: usize);
    fn insert(&mut self, c: char);
    fn insert_forward(&mut self, c: char);
    fn delete(&mut self) -> Option<char>;
    fn delete_forward(&mut self) -> Option<char>;
    fn pos(&self) -> usize;
    fn line(&self) -> usize;
    fn col(&self) -> usize;
    fn line_count(&self) -> usize;
    fn len(&self) -> usize;
    fn iter(&self) -> CharIter;
    fn iter_line(&self, line: usize) -> CharIter;
    fn line_index_to_char_index(&self, line: usize) -> usize;
}

#[derive(Clone)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    LineStart,
    LineEnd,
    PageUp(usize),
    PageDown(usize),
}

pub struct CharIter<'a> {
	rope: &'a Rope,
}