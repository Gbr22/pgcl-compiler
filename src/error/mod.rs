use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Error {
    pub text: String,
    pub start_pos: Position,
    pub end_pos: Position,
}
