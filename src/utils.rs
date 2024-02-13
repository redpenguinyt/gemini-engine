use crate::elements::Vec2D;
use std::{fmt, sync::OnceLock};

#[macro_use]
pub mod macros;

static TERMINAL_PREPARED: OnceLock<bool> = OnceLock::new();

/// Returns the size of the terminal as a `Vec2D`, using the termsize crate's [get function](https://docs.rs/termsize/latest/termsize/fn.get.html)
///
/// # Panics
/// This function will panic if your target cannot safely convert an i16 to an isize
#[must_use]
pub fn get_termsize_as_vec2d() -> Option<Vec2D> {
    let size = termsize::get()?;
    Some(Vec2D::new(
        isize::try_from(size.cols).expect("isize cannot fit i16"),
        isize::try_from(size.rows).expect("isize cannot fit i16"),
    ))
}

/// Block the process until the console window is resized to
pub fn block_until_resized(view_size: Vec2D) {
    if let Some(size) = get_termsize_as_vec2d() {
        if size < view_size {
            println!("Please resize your console window to fit the render\r");
            loop {
                if get_termsize_as_vec2d().unwrap_or_else(|| unreachable!()) > view_size {
                    break;
                }
            }
        }
    }
}

/// Prepare the console by printing lines to move previous console lines out of the way. Can only be called once in a program run
///
/// Returns an error if [`termsize::get`] returns `None`, or if it fails to write to the formatter
pub fn prepare_terminal(f: &mut fmt::Formatter<'_>) -> Result<(), String> {
    // If the console hasn't been prepared before
    if TERMINAL_PREPARED.get().is_none() {
        // Prevent the console from being prepared again
        TERMINAL_PREPARED.get_or_init(|| true);

        let Some(size) = termsize::get() else {
            return Err(String::from("Couldnt get termsize"));
        };

        write!(f, "{}", "\n".repeat(size.rows as usize)).map_err(|e| e.to_string())?;
    }

    Ok(())
}
