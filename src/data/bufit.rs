pub struct BufferedIterator<I: Iterator>
where
    I::Item: Copy + Clone,
{
    iterator: I,
    last: Option<I::Item>,
}

impl<I: Iterator> BufferedIterator<I>
where
    I::Item: Copy + Clone,
{
    pub fn new(mut iterator: I) -> Self {
        let last = iterator.next();

        BufferedIterator { iterator, last }
    }

    pub fn current(&self) -> Option<I::Item> {
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
