use crate::elements::{view::ViewElement, Pixel, Vec2D};

/// Contains references to all added objects. Meant to be used specifically for collision calculations
#[derive(Clone)]
pub struct CollisionContainer<'a> {
    /// The elements used to define the collision hitbox. This can be anything that implements [`ViewElement`]
    pub elements: Vec<&'a dyn ViewElement>,
}

impl<'a> Default for CollisionContainer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CollisionContainer<'a> {
    /// Create a new `CollisionContainer`
    #[must_use]
    pub const fn new() -> Self {
        Self { elements: vec![] }
    }

    /// Add an element to the container
    pub fn push(&mut self, element: &'a impl ViewElement) {
        self.elements.push(element);
    }

    /// Return a list of all the positions at which the collision box is active
    #[deprecated = "This is now just a proxy for active_points, use `CollisionContainer::active_points` instead"]
    #[must_use]
    pub fn generate_collision_points(&self) -> Vec<Vec2D> {
        self.active_points()
    }

    /// Returns true if there is an element from the `CollisionContainer` at the given coordinates
    #[must_use]
    pub fn contains(&self, pos: Vec2D) -> bool {
        let collision_points = self.active_points();

        collision_points.contains(&pos)
    }

    /// Returns true if the given [`ViewElement`] is overlapping the `CollisionContainer`
    pub fn overlaps_element(&self, element: &impl ViewElement) -> bool {
        self.will_overlap_element(element, Vec2D::ZERO)
    }

    /// Returns true if the element will be overlapping the `CollisionContainer` when the offset is applied
    pub fn will_overlap_element(&self, element: &impl ViewElement, offset: Vec2D) -> bool {
        let collision_points = self.active_points();

        for element_point in element.active_points() {
            if collision_points.contains(&(element_point + offset)) {
                return true;
            }
        }

        false
    }
}

impl<'a> From<Vec<&'a dyn ViewElement>> for CollisionContainer<'a> {
    fn from(elements: Vec<&'a dyn ViewElement>) -> Self {
        Self { elements }
    }
}

impl<'a> ViewElement for CollisionContainer<'a> {
    fn active_pixels(&self) -> Vec<Pixel> {
        self.elements
            .iter()
            .flat_map(|e| e.active_pixels())
            .collect()
    }

    fn active_points(&self) -> Vec<Vec2D> {
        self.elements
            .iter()
            .flat_map(|e| e.active_points())
            .collect()
    }
}
