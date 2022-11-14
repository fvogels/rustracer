pub struct BufferedIterator<T: Copy + Clone, I: Iterator<Item=T>> {
    iterator: I,
    last: Option<T>
}

impl<T: Copy + Clone, I: Iterator<Item=T>> BufferedIterator<T, I> {
    pub fn new(mut iterator: I) -> Self {
        let last = iterator.next();

        BufferedIterator { iterator, last }
    }

    pub fn current(&self) -> Option<T> {
        self.last
    }

    pub fn next(&mut self) {
        self.last = self.iterator.next();
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::BufferedIterator;

    #[cfg(test)]
    fn buffered_iterator() {
        let string = "abcd";
        let mut buffered = BufferedIterator::new(string.chars());

        assert_eq!(Some('a'), buffered.current());
        buffered.next();
        assert_eq!(Some('b'), buffered.current());
        buffered.next();
        assert_eq!(Some('c'), buffered.current());
        buffered.next();
        assert_eq!(Some('d'), buffered.current());
        buffered.next();
        assert_eq!(None, buffered.current());
    }
}