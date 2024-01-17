use crate::elements::Vec2D;
use std::{fmt, io, sync::OnceLock};

#[macro_use]
pub mod macros;

static TERMINAL_PREPARED: OnceLock<bool> = OnceLock::new();

/// Returns the size of the terminal as a Vec2D, using the termsize crate's [get function](https://docs.rs/termsize/latest/termsize/fn.get.html)
pub fn get_termsize_as_vec2d() -> Option<Vec2D> {
    let size = termsize::get()?;
    Some(Vec2D::new(size.cols as isize, size.rows as isize + 1))
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
/// Returns an error if [`termsize::get`] returns `None`
pub(crate) fn prepare_terminal(f: &mut fmt::Formatter<'_>) -> io::Result<()> {
    let cell = TERMINAL_PREPARED.get();
    if cell.is_none() {
        let rows = termsize::get()
            .ok_or(io::Error::new(
                std::io::ErrorKind::NotFound,
                "Couldnt get termsize",
            ))?
            .rows as usize;
        write!(f, "{}", vec!['\n'; rows].iter().collect::<String>()).unwrap();
        TERMINAL_PREPARED.get_or_init(|| true);
    }

    Ok(())
}
