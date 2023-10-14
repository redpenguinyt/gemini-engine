//! This module contains basic geometry structs that implement [`ViewElement`](super::view::ViewElement), such as [`Line`] or [`Triangle`]

mod line;
pub use line::Line;

mod triangle;
pub use triangle::Triangle;

mod polygon;
pub use polygon::Polygon;

mod rect;
pub use rect::Rect;
