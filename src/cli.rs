use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rust_notes",
    about = "A simple cli for taking daily notes.",
)]
pub struct Opts {
    #[structopt()]
    pub note_name: Option<String>,

    #[structopt(short = "e", long = "editor", env = "EDITOR")]
    pub editor: Option<String>,
}

impl Opts {
    pub fn load() -> Opts {
        <Opts as StructOpt>::from_args()
    }
}
