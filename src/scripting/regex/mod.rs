pub trait Regex {
    fn create_consumer(&self) -> Box<dyn Consumer>;
}

pub trait Consumer {
    fn feed(&mut self, ch: char) -> bool;

    fn done(&self) -> bool;
}

pub mod literal;
