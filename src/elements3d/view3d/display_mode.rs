use crate::elements::view::ColChar;

/// `DisplayMode` determines how the [`Viewport`](super::Viewport) renders our 3D objects. This is the Gemini equivalent of Blender's Viewport Shading options
pub enum DisplayMode {
    /// `DisplayMode::Debug` does the same thing, but shows the vertices as the indices that represent them (this is useful when you are constructing a mesh)
    Debug,
    /// `DisplayMode::Points` only renders the object's vertices as single pixels with the [`ColChar`] chosen with the [`fill_char`](DisplayMode::Points::fill_char) enum parameter
    Points {
        /// The desired appearance of the points
        fill_char: ColChar,
    },
    /// `DisplayMode::Wireframe` renders the edges of the meshes, without filling in the shapes. You can choose whether you want to render with backface culling using the [`backface_culling`](DisplayMode::Wireframe::backface_culling) enum parameter
    Wireframe { backface_culling: bool },
    /// `DisplayMode::Solid` renders the full faces of all the meshes. This is normally the final render
    Solid,
}
