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
}