//! houses the Repo struct
use std::path::PathBuf;
use crate::editor::Editor;
use crate::NoteError;

pub struct Repo<'a> {
    editor: &'a Editor<'a>,
}

impl<'a> Repo<'a> {
    pub fn new(editor: &'a Editor) -> Repo<'a> {
        Repo{
            editor,
        }
    }

    pub fn init(self) -> Result<Repo<'a>, NoteError>{
        let path = self.get_path();
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(self)
    }

    fn get_path(&self) -> PathBuf {
        let home = std::env::var("HOME")
            .expect("HOME environment variable is not set");
        let path = PathBuf::from(home)
            .join(".notes");
        path
    }

    pub fn create_nested_file(&self, path: &PathBuf) {
        let mut path = path.clone();
        let file_path = path.clone();

        std::env::set_current_dir(self.get_path()).unwrap();

        // create any directories, if incoming path is nested
        path.pop();
        std::fs::create_dir_all(path).unwrap();

        // create file, only if it does not already exist
        // 🌵 otherwise File::create will truncate it
        if !file_path.exists() {
            std::fs::File::create(file_path).unwrap();
        }
    }

    pub fn open_in_editor(&self, path: &PathBuf) {
        self.create_nested_file(&path);
        self.editor.open_file(&path, &self.get_path()).unwrap();
    }
}
