use std::ops::Add;

use serde_derive::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct Range {
    pub start_index: usize,
    pub end_index: usize
}

impl Range {
    pub fn new(start_index: usize, end_index: usize) -> Range {
        Range {
            start_index,
            end_index
        }
    }
    pub fn null() -> Range {
        Range {
            start_index: 0,
            end_index: 0
        }
    }
    pub fn between(a: Range, b: Range) -> Range {
        let start_index = a.end_index;
        let end_index = b.start_index;
        Range::new(start_index, end_index)
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
        let min_start = usize::min(self.start_index,rhs.start_index);
        let max_end = usize::max(self.end_index,rhs.end_index);
        Range::new(min_start, max_end)
    }
}