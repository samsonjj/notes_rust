#![recursion_limit = "1024"]

type Result<T> = std::result::Result<T, NoteError>;

use notes_rust;
use notes_rust::NoteError;

fn main() -> Result<()> {
    if let Err(e) = notes_rust::run() {
        eprintln!("{}", e);
        return Err(e);
    }

    Ok(())
}
