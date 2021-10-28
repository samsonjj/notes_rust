const TEXT: &str = include_str!("../docs/tutorial.txt");

pub trait Tutorial {
    fn display();
}

pub struct TutorialImpl {}

impl Tutorial for TutorialImpl {
    fn display() {
        println!("{}", TEXT);
    }
}
