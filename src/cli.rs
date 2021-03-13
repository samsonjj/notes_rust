use structopt::StructOpt;

pub trait Cli {
    fn load() -> Self;
}

#[derive(StructOpt)]
#[structopt(
    name = "rust_notes",
    about = "A simple cli for taking daily notes.",
)]
pub struct CliImpl {
    #[structopt()]
    pub note_name: Option<String>,

    #[structopt(short = "e", long = "editor", env = "EDITOR")]
    pub editor: Option<String>,
}

impl Cli for CliImpl {
    fn load() -> CliImpl {
        <CliImpl as StructOpt>::from_args()
    }
}
