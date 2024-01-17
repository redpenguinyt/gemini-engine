use crate::elements3d::Vec3D;

/// Characters for brightness. The first character is the darkest and the rightmost character is the brightest
pub const BRIGHTNESS_CHARS: &str = ".,-~:;=!*(%#$@";

/// The
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    /// Lights the entire scene equally. It's good to have at least one of these
    Ambient,
    // Point { position: Vec3D }, // TODO: ADD THIS ALREADY JFC
    /// Lights the scene from a specific direction. A surface facing the specified direction will be lit with the most intensity and a surface facing away from the direction will be lit with the least intensity or no intensity at all
    Directional {
        /// The direction the light is pointing
        direction: Vec3D,
    },
}

/// A light object used to define a scene's lighting. Used by [DisplayMode::Illuminated](super::DisplayMode::Illuminated)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    /// The type of light and the way it affects
    pub light_type: LightType,
    /// The intensity of the light
    pub intensity: f64,
}

impl Light {
    /// Create a new ambient light
    pub const fn new_ambient(intensity: f64) -> Self {
        Self {
            light_type: LightType::Ambient,
            intensity,
        }
    }

    // /// Create a new point light
    // pub const fn new_point(intensity: f64, position: Vec3D) -> Self {
    //     Self {
    //         light_type: LightType::Point { position },
    //         intensity,
    //     }
    // }

    /// Create a new directional light
    pub const fn new_directional(intensity: f64, direction: Vec3D) -> Self {
        Self {
            light_type: LightType::Directional { direction },
            intensity,
        }
    }

    /// Calculate the intensity of the light as it affects a surface with the given normal
    pub fn calculate_intensity(&self, normal: Vec3D) -> f64 {
        match self.light_type {
            LightType::Ambient => self.intensity,
            LightType::Directional { direction } => {
                let n_dot_l = normal.dot(direction);
                if n_dot_l > 0.0 {
                    self.intensity * n_dot_l / (normal.magnitude() * direction.magnitude())
                } else {
                    0.0
                }
            }
        }
    }
}
