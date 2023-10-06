use std::{thread::sleep, time::Duration};

/// Sleep for a single frame at the declared FPS, while also subtracting the Duration taken to process the frame. Returns a bool value depending on whether or not the frame took longer to render than the intended fps
/// ## Example
/// ```rust,no_run
/// use gemini_engine::gameloop;
///
/// let mut frame_skip = false;
/// const FPS: f32 = 60.0;
/// loop {
///     let now = gameloop::Instant::now();
///
///     // all code here will run at 60 FPS
///
///     if frame_skip {
///         frame_skip = false;
///     } else {
///         // calculations and rendering
///     }
///
///     frame_skip = gameloop::sleep_fps(FPS, Some(now.elapsed()));
/// }
pub fn sleep_fps(fps: f32, elapsed: Option<Duration>) -> bool {
    let elapsed = elapsed.unwrap_or(Duration::ZERO);
    let frame_length = Duration::from_secs_f32(1.0 / fps);
    if frame_length > elapsed {
        sleep(frame_length - elapsed);
        false
    } else {
        true
    }
}
