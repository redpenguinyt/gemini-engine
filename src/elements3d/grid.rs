use super::{Face, Transform3D, Vec3D, ViewElement3D};
use crate::elements::view::ColChar;

/// A flat grid to display where the ground is
pub struct Grid3D {
    pub transform: Transform3D,
    /// The length of each cell's width and depth
    pub cell_size: f64,
    /// The number of cells alon each side. The total number of cells will be `cell_count^2`
    pub cell_count: usize,
    generated_vertices: Vec<Vec3D>,
    generated_faces: Vec<Face>,
    pub fill_char: ColChar,
}

impl Grid3D {
    /// Create a new grid with the specified cell size and count
    pub fn new(cell_size: f64, cell_count: usize, fill_char: ColChar) -> Grid3D {
        let mut tmp = Grid3D {
            transform: Transform3D::DEFAULT,
            cell_size,
            cell_count,
            generated_vertices: vec![],
            generated_faces: vec![],
            fill_char,
        };
        tmp.reload();
        tmp
    }

    /// Regenerate the grid. Call this if you at any point change the `cell_size` or `cell_count` fields
    pub fn reload(&mut self) {
        let cell_count = self.cell_count as isize;
        let cell_size = self.cell_size;

        self.generated_vertices = [-1, 1]
            .iter()
            .flat_map(move |p| {
                (-cell_count / 2..=cell_count / 2).flat_map(move |b| {
                    let side = (cell_count / 2 * p) as f64 * cell_size;
                    let point_on_side = b as f64 * cell_size;
                    [
                        Vec3D::new(side, 0.0, point_on_side),
                        Vec3D::new(point_on_side, 0.0, side),
                    ]
                })
            })
            .collect();

        self.generated_faces = (0..self.generated_vertices.len() / 2)
            .map(|i| {
                Face::new(
                    vec![i, i + self.generated_vertices.len() / 2],
                    self.fill_char,
                )
            })
            .collect();
    }
}

impl ViewElement3D for Grid3D {
    fn get_transform(&self) -> Transform3D {
        self.transform
    }

    fn get_vertices(&self) -> &[Vec3D] {
        &self.generated_vertices
    }
    fn get_faces(&self) -> &[Face] {
        &self.generated_faces
    }
}
