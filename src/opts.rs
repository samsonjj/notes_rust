use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rust_notes",
    about = r#"A simple cli for taking daily notes.
type `notes --tutorial` for a brief introducition"#
)]
pub struct Opts {
    #[structopt()]
    pub note_name: Option<String>,

    #[structopt(short = "e", long = "editor", env = "EDITOR")]
    pub editor: Option<String>,

    #[structopt(
        long = "repo-path",
        env = "NOTES_REPO",
        help = "filesystem path for storing notes"
    )]
    pub repo_path: Option<String>,

    #[structopt(
        short = "o",
        long = "offset",
        allow_hyphen_values = true,
        help = "-1 for yesterday, +1 for tomorrow, etc."
    )]
    pub date_offset: Option<i64>,

    #[structopt(short = "t", long = "tutorial", help = "show the tutorial")]
    pub tutorial: bool,
}

impl Opts {
    pub fn load() -> Opts {
        <Opts as StructOpt>::from_args()
    }
}
