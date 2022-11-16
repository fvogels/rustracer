use crate::data::graph::{Graph, VertexId};

use super::{
    defs::{VertexLabel, DFA, NFA},
    dfa::{nfa_to_dfa, DFAWalker},
    nfa::NFABuilder,
    Regex,
};

pub struct AutomatonBuilder<T: Copy + Clone> {
    nfa_builder: NFABuilder<T, char, NFA>,
}

impl<T: Copy + Clone> AutomatonBuilder<T> {
    pub fn new() -> Self {
        AutomatonBuilder {
            nfa_builder: NFABuilder::new(),
        }
    }

    pub fn add(&mut self, regex: Regex, terminal: T) {
        self.nfa_builder.add(regex.regex.as_ref(), terminal)
    }

    pub fn eject(self) -> Automaton<T> {
        let (nfa, nfa_start) = self.nfa_builder.eject();
        let (dfa, dfa_start) = nfa_to_dfa(nfa, nfa_start);
        let walker = DFAWalker::new(dfa, dfa_start);
        let start = walker.active_position();

        Automaton::new(walker)
    }
}

pub struct Automaton<T> {
    walker: DFAWalker<T, char, DFA>,
    start: VertexId<DFA>,
}

impl<T> Automaton<T> {
    fn new(walker: DFAWalker<T, char, DFA>) -> Self {
        let start = walker.active_position();

        Automaton { walker, start }
    }

    pub fn feed(&mut self, ch: char) -> bool {
        self.walker.walk(ch)
    }

    pub fn finish(&mut self) -> Option<&T> {
        match self.walker.active_vertex_label() {
            None => None,
            Some(t) => Some(t),
        }
    }

    pub fn reset(&mut self) {
        self.walker.set_active_position(self.start)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::scripting::regex::{
        digit, integer, literal, literal_seq, one_or_more, optional, positive_integer, sequence,
    };

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn simple() {
        let mut builder = AutomatonBuilder::new();
        builder.add(literal_seq("abc".chars()), 1);
        builder.add(literal_seq("123".chars()), 2);
        let mut automaton = builder.eject();

        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('a'));
        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('b'));
        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('c'));
        assert_eq!(Some(&1), automaton.finish());
        assert!(!automaton.feed('d'));
        assert_eq!(Some(&1), automaton.finish());
        automaton.reset();

        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('a'));
        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('b'));
        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('c'));
        assert_eq!(Some(&1), automaton.finish());
        assert!(!automaton.feed('d'));
        assert_eq!(Some(&1), automaton.finish());
        automaton.reset();

        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('1'));
        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('2'));
        assert_eq!(None, automaton.finish());
        assert!(automaton.feed('3'));
        assert_eq!(Some(&2), automaton.finish());
        assert!(!automaton.feed('4'));
        assert_eq!(Some(&2), automaton.finish());
        automaton.reset();
    }

    #[rstest]
    fn match_positive_integer(#[values("1", "2", "123", "9876543210")] input: &str) {
        let mut builder = AutomatonBuilder::new();
        let regex = positive_integer();
        builder.add(regex, 1);
        let mut automaton = builder.eject();

        for ch in input.chars() {
            assert!(automaton.feed(ch));
        }

        assert_eq!(Some(&1), automaton.finish());
    }

    #[rstest]
    fn match_optional() {
        let mut builder = AutomatonBuilder::new();
        let regex = optional(literal('x'));
        builder.add(regex, 1);
        let mut automaton = builder.eject();

        assert_eq!(Some(&1), automaton.finish());
        automaton.feed('x');
        assert_eq!(Some(&1), automaton.finish());
    }

    #[rstest]
    fn match_two_optionals(#[values("", "x", "y", "xy")] input: &str) {
        let mut builder = AutomatonBuilder::new();
        let regex = sequence([optional(literal('x')), optional(literal('y'))].into_iter());
        builder.add(regex, 1);
        let mut automaton = builder.eject();

        assert_eq!(Some(&1), automaton.finish());

        for ch in input.chars() {
            automaton.feed(ch);
        }

        assert_eq!(Some(&1), automaton.finish());
    }

    #[rstest]
    fn match_integer(
        #[values("1", "2", "123", "9876543210", "-1", "-2", "-123", "-9876543210")] input: &str,
    ) {
        let mut builder = AutomatonBuilder::new();
        let regex = integer();
        builder.add(regex, 1);
        let mut automaton = builder.eject();

        for ch in input.chars() {
            assert!(automaton.feed(ch));
        }

        assert_eq!(Some(&1), automaton.finish());
    }
}
