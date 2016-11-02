use super::Rope;
use std::cmp;

struct Text {
	rope: Rope,
	pos: usize,
}

impl Text {
	pub fn new() -> Text {
		Text {
			rope: Rope::new(),
			pos: 0,
		}
	}
}

impl Editable for Text {
	fn step(&mut self, mov: Movement) {
		match mov {
			Movement::Up => {
				if self.line > 0 {
                    
				}
			}
		}
	}

    fn move_at(&mut self, line: usize, col: usize) {
    	let line_index = cmp::min(line, self.rope.line_count());
    	let line = self.line(line_index);
        let col_index = cmp::min(col, line.count());
        let line_begin = begining_of_line(line_index);

        self.pos = line_begin + col_index;
    }

    fn insert(&mut self, c: char);
    fn insert_forward(&mut self, c: char);
    fn delete(&mut self) -> Option<char>;
    fn delete_forward(&mut self) -> Option<char>;
    fn pos(&self) -> usize;
    fn line(&self) -> usize {
    	self.rope.line_from_index(pos)
    }

    fn col(&self) -> usize;
    fn line_count(&self) -> usize;
    fn len(&self) -> usize;
    fn iter(&self) -> CharIter;
    fn iter_line(&self, line: usize) -> CharIter;
    fn line_index_to_char_index(&self, line: usize) -> usize;
}

pub fn is_line_ending(c: char) -> bool {
    match c {
    	'\u{000A}' => true,
        _ => false
    }
}