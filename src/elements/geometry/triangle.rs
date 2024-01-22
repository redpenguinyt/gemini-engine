use crate::elements::view::{utils, ColChar, Pixel, Vec2D, ViewElement};

use super::Line;

/// The `Triangle` takes three [`Vec2D`]s and returns a triangle with those vertices when blit to a [`View`](super::super::View)
pub struct Triangle {
    /// The 3 cornes of the triangle
    pub corners: [Vec2D; 3],
    /// The [`ColChar`] used to fill the triange
    pub fill_char: ColChar,
}

impl Triangle {
    /// Create a new triangle from three separate positions and a `ColChar`
    #[must_use]
    pub const fn new(pos0: Vec2D, pos1: Vec2D, pos2: Vec2D, fill_char: ColChar) -> Self {
        Self::with_array([pos0, pos1, pos2], fill_char)
    }

    /// Create a new triangle from an array of `Vec2D`s and a `ColChar`
    #[must_use]
    pub const fn with_array(corners: [Vec2D; 3], fill_char: ColChar) -> Self {
        Self { corners, fill_char }
    }

    /// Return the triangle's points as an array
    #[deprecated = "Triangle has been restructured, just use `Triangle.corners` now"]
    #[must_use]
    pub const fn corners(&self) -> [Vec2D; 3] {
        self.corners
    }

    /// Draw a pseudo-line between the independent and dependent positions. Returns rounded values as `isize`s. If you don't want the values rounded, use [`Triangle::interpolate_floating()`]
    #[must_use]
    pub fn interpolate(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<isize> {
        Self::interpolate_floating(i0, d0, i1, d1)
            .iter()
            .map(|n| n.round() as isize)
            .collect()
    }

    /// Draw a pseudo-line between the independent and dependent positions
    #[must_use]
    pub fn interpolate_floating(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<f64> {
        if i0 == i1 {
            return vec![d0];
        }
        let mut values = vec![];

        let a = (d1 - d0) / (i1 - i0) as f64;
        let mut d = d0;
        for _i in i0..=i1 {
            values.push(d);
            d += a;
        }
        values
    }

    /// Takes three corner [`Vec2D`]s and returns the points you should plot to the screen to make a triangle
    #[must_use]
    pub fn draw(corners: [Vec2D; 3]) -> Vec<Vec2D> {
        let mut points = vec![];
        let mut corners = corners;
        corners.sort_unstable_by_key(|k| k.y);
        let (x0, y0) = corners[0].as_tuple();
        let (x1, y1) = corners[1].as_tuple();
        let (x2, y2) = corners[2].as_tuple();

        let mut x01 = Self::interpolate(y0, x0 as f64, y1, x1 as f64);
        let x12 = Self::interpolate(y1, x1 as f64, y2, x2 as f64);
        let x02 = Self::interpolate(y0, x0 as f64, y2, x2 as f64);

        x01.pop();
        let x012 = [x01, x12].concat();

        let m = (x012.len() as f64 / 2.0).floor() as usize;
        let (x_left, x_right) = if x02[m] < x012[m] {
            (x02, x012)
        } else {
            (x012, x02)
        };

        for (i, y) in (y0..y2).enumerate() {
            for x in x_left[i]..x_right[i] {
                points.push(Vec2D::new(x, y));
            }
        }

        // Outline (will probably remove later)
        points.append(&mut Line::draw(corners[0], corners[1]));
        points.append(&mut Line::draw(corners[1], corners[2]));
        points.append(&mut Line::draw(corners[2], corners[0]));

        points
    }
}

impl ViewElement for Triangle {
    fn active_pixels(&self) -> Vec<Pixel> {
        utils::points_to_pixels(self.active_points(), self.fill_char)
    }

    fn active_points(&self) -> Vec<Vec2D> {
        Self::draw(self.corners)
    }
}
