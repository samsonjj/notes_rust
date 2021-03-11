use std::path::PathBuf;
use structopt::StructOpt;

pub trait Cli {
    fn load() -> Self;
}

#[derive(StructOpt)]
#[structopt(
    name = "rust_notes",
    about = "A simple cli for taking daily notes."
)]
pub struct CliImpl {
    #[structopt(parse(from_os_str))]
    pub path: Option<std::path::PathBuf>,

    #[structopt(short = "e", long = "editor", env = "EDITOR")]
    pub editor: Option<String>,

    #[structopt(short = "b", long = "basepath", env = "NOTES_BASE_PATH")]
    pub base_path: Option<PathBuf>,
}

impl Cli for CliImpl {
    fn load() -> CliImpl {
        <CliImpl as StructOpt>::from_args()
    }
}
