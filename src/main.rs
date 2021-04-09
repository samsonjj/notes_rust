#![recursion_limit = "1024"]

type Result<T> = std::result::Result<T, NoteError>;

use notes_rust;
use notes_rust::NoteError;

fn main() -> Result<()> {
    // Error handling methodology:
    //  user error: return a NoteError, which gets passed up the call stack.
    //  system error: panic
    //  never error: just unwrap
    //

    // catch any error
    if let Err(e) = notes_rust::run() {
        eprintln!("{}", e);
        return Err(e);
    }

    Ok(())
}
