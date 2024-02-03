use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Position {
    pub row: u32,
    pub col: u32,
}

#[wasm_bindgen]
pub fn get_position(text: &str, index: usize) -> Position {
    let mut row = 0;
    let mut col = 0;
    let mut i = 0;
    for char in text.chars() {
        if i >= index {
            break;
        }
        if char == '\n' {
            col = 0;
            row += 1;
        } else {
            col += 1;
        }
        i += 1;
    }

    Position { row, col }
}
