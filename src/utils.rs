use std::{sync::OnceLock, io, fmt};

static TERMINAL_PREPARED: OnceLock<bool> = OnceLock::new();

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
            .rows;
        let rows_us = usize::try_from(rows).expect("u16 couldnt convert to usize");
        writeln!(
            f,
            "{}",
            vec!['\n'; rows_us].iter().cloned().collect::<String>()
        )
        .unwrap();
        println!("terminal prepared");
        TERMINAL_PREPARED.get_or_init(|| true);
    }

    Ok(())
}