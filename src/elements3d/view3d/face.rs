use crate::elements::view::ColChar;

/// A Face contains indices to a mesh's collection of vertices and a fill_char to fill the face. Indices should be arranged in a clockwise order, as if they appear counter-clockwise when rendering they will not be rendered at all (this is how gemini-engine handles backface culling and maximises performance)
#[derive(Debug, Clone)]
pub struct Face {
    pub v_indices: Vec<usize>,
    pub fill_char: ColChar,
}

impl Face {
    pub const fn new(v_indices: Vec<usize>, fill_char: ColChar) -> Self {
        Self {
            v_indices,
            fill_char,
        }
    }

    /// Returns a vector with the elements found at the vertex indices of the given slice
    pub fn index_into<T: Copy>(&self, vertices: &[T]) -> Vec<T> {
        self.v_indices.iter().map(|vi| vertices[*vi]).collect()
    }
}
