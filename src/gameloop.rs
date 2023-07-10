use std::{thread::sleep, time::Duration};

/// sleep for a single frame depending on the declared FPS, while also subtracting the Duration taken to process the frame. Returns a bool value depending on whether or not the frame took longer to render than the intended fps
/// ## Example
/// ```
/// use std::time::Instant;
///
/// let mut frame_skip = false;
/// let FPS = 60;
/// loop {
///     let now = Instant::now();
///     if frame_skip {
///         frame_skip = false;
///     } else {
///         // calculations and rendering
///     }
///
///     frame_skip = sleep_fps(FPS, now.elapsed());
/// }
pub fn sleep_fps(fps: u32, elapsed: Option<Duration>) -> bool {
    let elapsed = elapsed.unwrap_or(Duration::ZERO);
    let frame_length = Duration::from_secs_f32(1.0 / fps as f32);
    if frame_length > elapsed {
        sleep(frame_length - elapsed);
        return false;
    } else {
        return true;
    }
}
