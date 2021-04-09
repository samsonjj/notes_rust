use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rust_notes",
    about = "A simple cli for taking daily notes."
)]
pub struct Opts {
    #[structopt()]
    pub note_name: Option<String>,

    #[structopt(short = "e", long = "editor", env = "EDITOR")]
    pub editor: Option<String>,

    #[structopt(long = "repo", env = "NOTES_REPO")]
    pub repo_path: Option<String>,

    #[structopt(short = "o", long = "offset", allow_hyphen_values = true)]
    pub date_offset: Option<i64>,
}

impl Opts {
    pub fn load() -> Opts {
        <Opts as StructOpt>::from_args()
    }
}
