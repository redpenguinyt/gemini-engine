## Gemini

[![Crates.io](https://img.shields.io/crates/v/gemini-engine)](https://crates.io/crates/gemini-engine) ![Stars](https://img.shields.io/github/stars/redpenguinyt/gemini-rust?color=yellow) ![Last commit](https://img.shields.io/github/last-commit/redpenguinyt/gemini-rust) ![Code size](https://img.shields.io/github/languages/code-size/redpenguinyt/gemini-rust) ![Crates.io](https://img.shields.io/crates/d/gemini-engine) [![Issues](https://img.shields.io/github/issues/redpenguinyt/gemini-rust)](https://github.com/redpenguinyt/gemini-rust/issues)

Gemini is a monospaced ASCII rendering engine, capable of rendering 2D and 3D graphics in a terminal or console. This is a loose port of [GeminiEngine](https://github.com/redpenguinyt/GeminiEngine) which was made in Python but was scrapped due to performance limitations.

**IMPORTANT**: You HAVE to use a monospace font in the terminal for the engine to render the view properly

- [Changelog](https://github.com/redpenguinyt/gemini-rust/commits)
- [Crates.io](https://crates.io/crates/gemini-engine)
- [Documentation](https://docs.rs/gemini-engine)

## Quick start
Let's get started with a simple program to demonstrate how Gemini works:
```rust
use gemini::elements::{Point, Vec2D, view::{View, ColChar, Wrapping}};
use gemini::gameloop;

const FPS: u32 = 30;

fn main() {
    let mut view = View::new(40, 8, ColChar::BACKGROUND);
    let mut point = Point::new(Vec2D::new(10,5), ColChar::SOLID);

    loop {
        view.clear();

        point.pos.x += 1;

        view.blit(&point, Wrapping::Wrap);
        view.display_render().unwrap();

        gameloop::sleep_fps(FPS, None);
    }
}
```
Ok, let's go over this and see what's going on. We start by creating a `View` and `Point`. the `View` takes two numbers for the width and height, as well as a `ColChar`. The `Point` takes a `Vec2D` and a `ColChar`.

We use `ColChar` to say exactly what each pixel should look like and what colour it should be. Here we used the built in `ColChar::BACKGROUND` and `ColChar::SOLID` to keep the code simple. You can read more in the `ColChar` documentation.

At its heart, `Vec2D` is just a pair of `isize` integers for defining things such as position, size and movement. We used it here to define the `Point`'s starting position, before the game loop.

Now that we've got initialisation out of the way, let's get on to the juicy part: the main loop. In Gemini the main loop always goes as follows:
1. Clear the `View`
2. Work through any logic you might have (moving things around, taking inputs etc.)
3. Blit all the `ViewElements` to the screen
4. `View.display_render`
5. Sleep

In our case, we want to move our `Point` one unit to the right every frame, so we increase its value by one here. Next we blit the `Point` to the `View` (adding it to the `View`'s internal canvas) and render. Rendering will display the view in the terminal (make sure your terminal is large enough to fit the whole image!). The last line of our code sleeps for `1/FPS` seconds. We pass None in place of what would normally be a Some(Duration) type, displaying the amount of time it took to blit and render everything so that `gameloop::sleep_fps` can accomodate for the time taken to render. Since this example program is quite simple, we've just passed None. You can see how best to write a gameloop in the `gameloop` documentation.

There you have it! You've written your first program with Gemini! As of me writing this now it's still very much a work in progress, so any feedback or issue requests would be appreciated :)