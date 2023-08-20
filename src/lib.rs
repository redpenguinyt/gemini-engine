pub mod elements;
pub mod gameloop;

#[cfg(test)]
mod vec2d_tests {
    use super::*;
	use elements::Vec2D;

    #[test]
    fn add_vec2d() {
        assert_eq!(Vec2D::new(15, -3), Vec2D::new(13, 4) + Vec2D::new(2, -7));
    }

	#[test]
    fn subtract_vec2d() {
        assert_eq!(Vec2D::new(2, -10), Vec2D::new(17, 4) - Vec2D::new(15, 14));
    }

	#[test]
	fn rem_vec2d_over() {
		assert_eq!(Vec2D::new(4, 1), Vec2D::new(9, 11) % Vec2D::new(5, 10))
	}

	#[test]
	fn rem_vec2d_under() {
		assert_eq!(Vec2D::new(4, 1), Vec2D::new(-1, -109) % Vec2D::new(5, 10))
	}
}