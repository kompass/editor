use std::str::Chars;
use super::text::is_line_ending;

pub struct Rope {
	data: RopeData,
	line_ending_count: usize,
	length: usize,
}

pub enum RopeData {
	Leaf(String),
	Branch(Box<Rope>, Box<Rope>),
}

impl Rope {
	pub fn new() -> Rope {
		Rope {
			data: RopeData::Leaf(String::new()),
			line_ending_count: 0,
			length: 0,
		}
	}

	pub fn line_count(&self) -> usize {
		self.line_ending_count + 1
	}

	pub fn char_index_to_line_index(&self, index: usize) -> usize {
		match self.data {
			RopeData::Leaf(ref text) => {
				let mut line_count = 0;
				for (i, c) in text.chars().enumerate() {
					if i == index {
						break;
					}

					if is_line_ending(c) {
						line_count += 1;
					}
				}

				return line_count;
			},

			RopeData::Branch(ref left, ref right) => {
				if index < left.length {
					return left.char_index_to_line_index(index);
				}
				else {
					return left.line_ending_count + right.char_index_to_line_index(index);
				}
			},
		}
	}

	pub fn line(&self, line_index: usize) -> RopeCharIter {
		let a = self.begining_of_line(line_index);
		let b = self.ending_of_line(line_index);

		let (mut chunk_iter, i) = RopeChunkIter::from_char_index(self, a);

		let mut cur_chunk = chunk_iter.next().unwrap().chars();

		for _ in 1..(i-1) {
			cur_chunk.next();
		}

		return RopeCharIter {
			chunk_iter: chunk_iter,
			cur_chunk: cur_chunk,
			length: Some(b - a),
		}
	}

	pub fn begining_of_line(&self, line_index: usize) -> usize {
		assert!(line_index <= self.line_ending_count, "Rope::begining_of_line(): line index must be less than the number of lines.");

		match self.data {
			RopeData::Leaf(ref text) => {
				let mut line_count = 0;

				for (i, c) in text.chars().enumerate() {
					if line_count == line_index {
						return i;
					}

					if is_line_ending(c) {
						line_count += 1;
					}
				}

				panic!("Rope::begining_of_line: the number of line breaks in a chunk is not the same as his line_break_count.");
			},

			RopeData::Branch(ref left, ref right) => {
				if line_index <= left.line_ending_count {
					left.begining_of_line(line_index)
				}
				else {
					left.length + right.begining_of_line(line_index - left.line_ending_count)
				}
			}
		}
	}

	pub fn ending_of_line(&self, line_index: usize) -> usize {
		assert!(line_index <= self.line_ending_count, "Rope::endin_of_line(): line index must be less than the number of lines.");

		if line_index == self.line_ending_count {
			self.length - 1
		}
		else {
			self.begining_of_line(line_index + 1) - 1
		}
	}
}

pub struct RopeCharIter<'a> {
	chunk_iter: RopeChunkIter<'a>,
	cur_chunk: Chars<'a>,
	length: Option<usize>,
}

impl<'a> Iterator for RopeCharIter<'a> {
	type Item = char;

	fn next(&mut self) -> Option<char> {
		if let Some(l) = self.length {
			if l == 0 {
				return None;
			}
		}

		loop {
            if let Some(c) = self.cur_chunk.next() {
                if let Some(ref mut l) = self.length {
                    *l -= 1;
                }
                return Some(c);
            }
            else {   
                if let Some(s) = self.chunk_iter.next() {
                    self.cur_chunk = s.chars();
                    continue;
                }
                else {
                    return None;
                }
            }
        }
	}
}

pub struct RopeChunkIter<'a> {
	node_stack: Vec<&'a Rope>,
}

impl<'a> RopeChunkIter<'a> {
	fn from_char_index(rope: &'a Rope, index: usize) -> (RopeChunkIter<'a>, usize) {
		RopeChunkIter::from_char_index_intern(rope, index, vec![])
	}

	fn from_char_index_intern(rope: &'a Rope, index: usize, mut stack: Vec<&'a Rope>) -> (RopeChunkIter<'a>, usize) {
		assert!(index < rope.length, "RopeChunkIter::from_to(): index must be less than the length of the text.");

		match rope.data {
			RopeData::Leaf(_) => (RopeChunkIter { node_stack: vec![rope] }, index),
			RopeData::Branch(ref left, ref right) => {
				if left.length < index {
					stack.push(right);
					RopeChunkIter::from_char_index_intern(left, index, stack)
				}
				else {
					RopeChunkIter::from_char_index_intern(right, index - left.length, stack)
				}
			}
		}
	}
}

impl<'a> Iterator for RopeChunkIter<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<&'a str> {
		if let Some(node) = self.node_stack.pop() {
			match node.data {
				RopeData::Leaf(ref text) => Some(&text[..]),
				RopeData::Branch(ref left, ref right) => {
					self.node_stack.push(right);
					self.node_stack.push(left);
					self.next()
				}
			}
		}
		else {
			None
		}
	}
}