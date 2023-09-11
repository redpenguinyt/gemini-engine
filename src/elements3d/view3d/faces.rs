use crate::elements::{view::colchar::ColChar, Polygon};

/// A Face contains indices to a mesh's collection of vertices and a fill_char to fill the face. Indices should be arranged in a clockwise order, as if they appear counter-clockwise when rendering they will not be rendered at all (this is how gemini-engine) handles backface culling and maximises performance
#[derive(Debug, Clone)]
pub struct IndexFace {
    pub v_indexes: Vec<usize>,
    pub fill_char: ColChar,
}

impl IndexFace {
    pub fn new(v_indexes: Vec<usize>, fill_char: ColChar) -> Self {
        Self {
            v_indexes,
            fill_char,
        }
    }

    /// Return the face as a vec of `IndexTriangle`s
    pub fn triangulate(self) -> Vec<IndexTriangle> {
        let triangles = Polygon::triangulate(self.v_indexes);

        triangles
            .iter()
            .map(|indexes| IndexTriangle::new(*indexes, self.fill_char))
            .collect::<Vec<IndexTriangle>>()
    }
}

/// A Face contains indices to a mesh's collection of vertices and a fill_char to fill the face. Indices should be arranged in a clockwise order, as if they appear counter-clockwise when rendering they will not be rendered at all (this is how gemini-engine) handles backface culling and maximises performance
#[derive(Debug, Clone)]
pub struct IndexTriangle {
    pub v_indexes: [usize; 3],
    pub fill_char: ColChar,
}

impl IndexTriangle {
    pub fn new(v_indexes: [usize; 3], fill_char: ColChar) -> Self {
        Self {
            v_indexes,
            fill_char,
        }
    }
}
