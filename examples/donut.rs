//! A visual recreation of donut-c in gemini
use gemini_engine::{
    elements::{
        view::{ColChar, Wrapping},
        View,
    },
    elements3d::{view3d::Light, DisplayMode, Mesh3D, Transform3D, Vec3D, Viewport},
    fps_gameloop,
};

const FPS: f32 = 60.0;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(82, 32, ColChar::EMPTY);
    let viewport = Viewport::new(
        Transform3D::new_tr(Vec3D::new(0.0, 0.0, 20.0), Vec3D::ZERO),
        FOV,
        view.center(),
    );

    let lights = vec![
        Light::new_ambient(0.3),
        Light::new_directional(0.7, Vec3D::new(1.0, -1.0, -1.0)),
    ];

    let mut donut = Mesh3D::torus(1.8, 1.0, 32, 16);

    fps_gameloop!(
        {
            donut.transform.rotation.x += 0.05;
            donut.transform.rotation.z += 0.05;
        },
        {
            view.clear();
            view.blit(
                &viewport.render(
                    vec![&donut],
                    DisplayMode::Illuminated {
                        lights: lights.clone(),
                    },
                ),
                Wrapping::Ignore,
            );
            let _ = view.display_render();
        },
        FPS
    );
}
