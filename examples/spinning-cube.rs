//! An example of a spinning cube with `elements3d`
use gemini_engine::elements::view::{ColChar, View, Wrapping};
use gemini_engine::elements3d::{DisplayMode, Mesh3D, Transform3D, Vec3D, Viewport};
use gemini_engine::fps_gameloop;
use gemini_engine::gameloop;

const FPS: f32 = 30.0;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(350, 90, ColChar::BACKGROUND);

    let mut viewport = Viewport::new(
        Transform3D::new_tr(Vec3D::new(0.0, 0.0, 5.0), Vec3D::new(-0.5, 0.3, 0.0)),
        FOV,
        view.center(),
    );

    let cube = Mesh3D::default_cube();

    fps_gameloop!(
        {
            view.clear();
            viewport.transform.rotation.y -= 0.05;
        },
        {
            view.blit(
                &viewport.render(vec![&cube], DisplayMode::Solid),
                Wrapping::Ignore,
            );
            let _ = view.display_render();
        },
        FPS,
        |elapsed: gameloop::Duration, frame_skip| {
            println!(
                "Elapsed: {:.2?}µs | Frame skip: {}",
                elapsed.as_micros(),
                frame_skip
            );
        }
    );
}
