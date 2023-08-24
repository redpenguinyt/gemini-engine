use crate::elements::view::colchar::ColChar;

#[derive(Debug)]
pub struct Face {
    pub v_indexes: Vec<usize>,
    pub fill_char: ColChar,
}

impl Face {
    pub fn new(v_indexes: Vec<usize>, fill_char: ColChar) -> Self {
        Self {
            v_indexes,
            fill_char,
        }
    }
}

impl Clone for Face {
    fn clone(&self) -> Self {
        Self {
            v_indexes: self.v_indexes.clone(),
            fill_char: self.fill_char,
        }
    }
}
