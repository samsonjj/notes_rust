use std::path::PathBuf;

pub struct Repo {
    path: Option<PathBuf>,
}

impl Repo {
    pub fn new() -> Repo {
        Repo{ path: None }
    }

    pub fn get_path(&mut self) -> PathBuf {
        if let Some(path) = self.path.to_owned() {
            return path
        }

        let home = std::env::var("HOME")
            .expect("HOME environment variable is not set");

        let path = PathBuf::from(home)
            .join(".notes");

        std::fs::create_dir_all(&path)
            .expect(&*format!("failure to create .notes directory at {:?}", path));

        self.path = Some(path.clone());

        self.path.to_owned().unwrap()
    }
}

