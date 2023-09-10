use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Vec2D,
};
use gemini_engine::elements3d::{DisplayMode, Mesh3D, Transform3D, Vec3D, Viewport};
use gemini_engine::gameloop;

const FPS: u32 = 20;
const FOV: f64 = 95.0;

fn main() {
    let mut frame_skip = false;
    let mut view = View::new(350, 90, ColChar::BACKGROUND);

    let mut viewport = Viewport::new(
        Transform3D::new_tr(Vec3D::new(0.0, 0.0, 5.0), Vec3D::new(-0.5, 0.3, 0.0)),
        FOV,
        Vec2D::new((view.width / 2) as isize, (view.height / 2) as isize),
    );

    let cube = Mesh3D::default_cube();

    loop {
        let now = gameloop::Instant::now();
        view.clear();

        viewport.transform.rotation.y -= 0.05;

        match frame_skip {
            true => frame_skip = false,
            false => {
                view.blit(
                    &viewport.render(vec![&cube], DisplayMode::Solid),
                    Wrapping::Ignore,
                );
                view.display_render().unwrap();
            }
        }

        let elapsed = now.elapsed();
        println!(
            "Elapsed: {:.2?}µs | Frame skip: {}",
            elapsed.as_micros(),
            frame_skip
        );

        frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
    }
}
