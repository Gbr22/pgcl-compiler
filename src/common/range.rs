use std::ops::Add;

use serde_derive::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct Range {
    pub start_index: usize,
    pub end_index: usize,
}

impl Range {
    pub fn new(start_index: usize, end_index: usize) -> Range {
        Range {
            start_index,
            end_index,
        }
    }
    pub fn null() -> Range {
        Range {
            start_index: 0,
            end_index: 0,
        }
    }
    pub fn between(a: Range, b: Range) -> Range {
        let start_index = a.end_index;
        let end_index = b.start_index;
        Range::new(start_index, end_index)
    }
    pub fn set_length(&mut self, len: usize) {
        self.end_index = self.start_index + len;
    }
    pub fn with_length(self, len: usize) -> Self {
        let mut s = self;
        s.set_length(len);
        s
    }
    pub fn with_start(self, start: usize) -> Self {
        let mut s = self;
        s.start_index = start;
        if s.start_index > s.end_index {
            s.end_index = s.start_index
        }
        s
    }
}

pub struct Len(pub usize);

impl Add<Len> for Range {
    type Output = Range;

    fn add(self, rhs: Len) -> Self::Output {
        Range::new(self.start_index, self.end_index + rhs.0)
    }
}

impl Add<Range> for Range {
    type Output = Range;

    fn add(self, rhs: Range) -> Self::Output {
        let min_start = usize::min(self.start_index, rhs.start_index);
        let max_end = usize::max(self.end_index, rhs.end_index);
        Range::new(min_start, max_end)
    }
}
