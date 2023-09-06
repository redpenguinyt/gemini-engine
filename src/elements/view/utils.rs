use super::{ColChar, Vec2D, Point};

/// Combine a vector of [`Vec2D`]s and a single `fill_char` into a vector of `(Vec2D, char)` tuples, ready to return for `ViewElement::active_pixels`. Useful if your [`ViewElement`](super::ViewElement) only has one fill character across all of it
pub fn points_to_pixels(points: Vec<Vec2D>, fill_char: ColChar) -> Vec<Point> {
    points.iter().map(|e| Point::new(e.clone(), fill_char)).collect()
}

pub fn interpolate(i0: isize, d0: f64, i1: isize, d1: f64) -> Vec<isize> {
    if i0 == i1 {
        return vec![d0.round() as isize];
    }
    let mut values = vec![];

    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut d = d0;
    for _i in i0..(i1 + 1) {
        values.push(d.clone().round() as isize);
        d += a;
    }
    values
}

/// Returns true if the points in the vector are arranged in a clockwise orientation
pub fn is_clockwise(points: &Vec<Vec2D>) -> bool {
    let mut m = vec![];
    for i in 0..points.len() {
        let (p1, p2) = (points[i], points[(i + 1) % points.len()]);
        m.push((p1.x - p2.x) * (p1.y + p2.y));
    }

    m.iter().sum::<isize>() < 0
}

/// Wrapping is used to determine how you want to handle out-of-bounds pixels during plotting pixels to the screen. Here's how each possible value functions:
///
/// [`Wrapping::Wrap`] wraps any out of bounds pixels around to the other side. This is useful if you have an object that travels the entirety of the screen and appears on the other side when it reaches the end.
///
/// [`Wrapping::Ignore`] simply skips all out-of-bounds pixels. This is useful if you might have an object clipping through the edge of the screen.
///
/// [`Wrapping::Panic`] will `panic!` if any pixels are out of bounds. You should use this if you have your own wrapping system implemented
#[derive(Copy)]
pub enum Wrapping {
    Wrap,
    Ignore,
    Panic,
}

impl Clone for Wrapping {
    fn clone(&self) -> Self {
        match self {
            Wrapping::Wrap => Wrapping::Wrap,
            Wrapping::Ignore => Wrapping::Ignore,
            Wrapping::Panic => Wrapping::Panic,
        }
    }
}

/// `BlitCache` is used if there is chance that you might have to render the same thing multiple times without moving or changing it.
#[derive(Debug)]
pub struct BlitCache<T> {
    independent: Vec<T>,
    dependent: Vec<Vec2D>,
}

impl<T> BlitCache<T>
where
    T: PartialEq<T>,
{
    pub const DEFAULT: BlitCache<T> = BlitCache {
        independent: vec![],
        dependent: vec![],
    };

    pub fn new(independent: Vec<T>, dependent: Vec<Vec2D>) -> Self {
        Self {
            independent,
            dependent,
        }
    }

    pub fn is_default(&self) -> bool {
        self == &BlitCache::DEFAULT
    }

    /// Returns the stored dependent value. Returns None if the cache is set to its default
    pub fn dependent(&self) -> Option<Vec<Vec2D>> {
        match self.is_default() {
            false => Some(self.dependent.clone()),
            true => None,
        }
    }

    pub fn is_cache_valid(&self, other_independent: &Vec<T>) -> bool {
        if self.independent.len() != other_independent.len() {
            return false;
        }
        self.independent
            .iter()
            .zip(other_independent)
            .filter(|&(a, b)| a == b)
            .count()
            == self.independent.len()
    }
}

impl<T> Clone for BlitCache<T>
where
    T: PartialEq<T>,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            independent: self.independent.clone(),
            dependent: self.dependent.clone(),
        }
    }
}

impl<T> PartialEq for BlitCache<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.independent == other.independent && self.dependent == other.dependent
    }
}
