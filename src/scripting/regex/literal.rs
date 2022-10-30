use super::{Regex, Consumer};

pub struct Literal {
    expected: char,
}

pub struct LiteralConsumer {
    expected: Option<char>,
}

impl Literal {
    pub fn new(expected: char) -> Literal {
        Literal { expected }
    }
}

impl Regex for Literal {
    fn create_consumer(&self) -> Box<dyn Consumer> {
        let consumer = LiteralConsumer { expected: Some(self.expected) };
        Box::new(consumer)
    }
}

impl Consumer for LiteralConsumer {
    fn feed(&mut self, ch: char) -> bool {
        match self.expected {
            None => false,
            Some(expected) => {
                if expected == ch {
                    self.expected = None;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn done(&self) -> bool {
        self.expected.is_none()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn literal_match() {
        let regex = Literal::new('a');
        let mut consumer = regex.create_consumer();

        assert!(consumer.feed('a'));
        assert!(consumer.done());
    }

    #[rstest]
    fn literal_no_match() {
        let regex = Literal::new('a');
        let mut consumer = regex.create_consumer();

        assert!(!consumer.feed('b'));
        assert!(!consumer.done());
    }
}
