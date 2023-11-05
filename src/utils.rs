use crate::elements::Vec2D;
use std::{fmt, io, sync::OnceLock};

#[macro_use]
pub mod macros;

static TERMINAL_PREPARED: OnceLock<bool> = OnceLock::new();

pub fn get_termsize_as_vec2d() -> Option<Vec2D> {
    let size = termsize::get()?;
    Some(Vec2D::new(size.cols as isize, size.rows as isize + 1))
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
