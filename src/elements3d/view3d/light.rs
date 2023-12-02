use super::Vec3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    Ambient,
    // Point { position: Vec3D },
    Directional { direction: Vec3D },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub light_type: LightType,
    pub intensity: f64,
}

impl Light {
    pub const fn new_ambient(intensity: f64) -> Self {
        Self {
            light_type: LightType::Ambient,
            intensity,
        }
    }

    // pub const fn new_point(intensity: f64, position: Vec3D) -> Self {
    //     Self {
    //         light_type: LightType::Point { position },
    //         intensity,
    //     }
    // }

    pub const fn new_directional(intensity: f64, direction: Vec3D) -> Self {
        Self {
            light_type: LightType::Directional { direction },
            intensity,
        }
    }

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
