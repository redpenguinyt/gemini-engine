use gemini_engine::elements::{
    view::{ColChar, Modifier, Vec2D, Wrapping},
    Line, Pixel, Rect, Sprite, View,
};
use gemini_engine::fps_gameloop;
use gemini_engine::gameloop::Duration;

const FPS: f32 = 20.0;
const FILL_CHAR: ColChar = ColChar::SOLID;
const BACKGROUND_CHAR: ColChar = ColChar::EMPTY;

fn main() {
    let mut view = View::new(60, 10, BACKGROUND_CHAR);
    view.coord_numbers_in_render = true;

    let mut pixel = Pixel::new(Vec2D::from((5u8, 9u8)), FILL_CHAR);

    let mut line = Line::new(Vec2D::new(2, 8), Vec2D::new(28, 7), FILL_CHAR);
    let mut line1_direction = -1;

    let rect = Rect::new(
        Vec2D { x: 11, y: 1 },
        Vec2D { x: 9, y: 3 },
        ColChar::SOLID.with_rgb(200, 30, 0),
    );

    let test_image = r"
  ______
 /|_||_\`.__
(   _    _ _\
=`-(_)--(_)-'   ";
    let mut sprite = Sprite::new(
        Vec2D::new(30, 1),
        test_image,
        Modifier::from_rgb(20, 200, 0),
    );

    let mut blit_elapsed = Duration::default();
    let mut render_elapsed = Duration::default();
    fps_gameloop!(
        {
            pixel.pos.x += 2;
            // loop the position back to the other side. This can be done with `Wrapping::Wrap` but it won't change the element's actual position, so the pixel position being printed would continue to increase without looping
            pixel.pos %= view.size();

            line.pos1.y += line1_direction;
            line.pos0.y = 10 - line.pos1.y;
            if line.pos1.y > 7 {
                line1_direction = -1;
            } else if line.pos1.y < 3 {
                line1_direction = 1;
            }

            sprite.pos.x += 1;
        },
        {
            view.clear();

            let now = Instant::now();
            view.blit(&pixel, Wrapping::Panic);
            view.blit(&line, Wrapping::Panic);
            view.blit(&rect, Wrapping::Panic);
            view.blit(&sprite, Wrapping::Wrap);
            blit_elapsed = now.elapsed();

            let now = Instant::now();
            view.display_render().unwrap();
            render_elapsed = now.elapsed();
        },
        FPS,
        |total_elapsed: Duration, _frame_skip| {
            println!(
                "Blitting: {:.2?} microseconds | Rendering: {:.2?} microseconds| Total: {:.2?}",
                blit_elapsed.as_micros(),
                render_elapsed.as_micros(),
                total_elapsed.as_micros()
            );
            println!("Pixel position: {}", pixel.pos);
        }
    );
}
