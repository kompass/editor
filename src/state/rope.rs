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

pub struct RopeCharIter<'a> {
	chunk_iter: RopeChunkIter<'a>,
	cur_chunk: Chars<'a>,
	length: Option<usize>,
}

impl<'a> Iterator for RopeCharIter<'a> {
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

impl Rope {
	pub fn new() -> Rope {
		Rope {
			data: RopeData::Leaf(String::new()),
			line_ending_count: 0,
			length: 0,
		}
	}

	pub fn char_index_to_line_index(&self, index: usize) -> usize {
		match self.data {
			RopeData::Leaf(ref text) => {
				let mut line_count = 0;
				for (i, c) in text.chars.enumerate() {
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
		/*match self.data {
			RopeData::Leaf(ref text) => {
				return 
			}
		}*/

		return RopeCharIter {

		}
	}
}

pub struct RopeChunkIter<'a> {
	node_stack: Vec<&'a Rope>,
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