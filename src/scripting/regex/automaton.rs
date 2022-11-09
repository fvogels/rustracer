use crate::data::graph::{Graph, VertexId};

use super::{nfa::NFABuilder, defs::{NFA, DFA, VertexLabel}, Regex, dfa::{nfa_to_dfa, DFAWalker}};

pub struct AutomatonBuilder<T: Copy + Clone> {
    nfa_builder: NFABuilder<T, char, NFA>,
}

impl<T: Copy + Clone> AutomatonBuilder<T> {
    pub fn new() -> Self {
        AutomatonBuilder {
            nfa_builder: NFABuilder::new()
        }
    }

    pub fn add(&mut self, regex: Regex, terminal: T) {
        self.nfa_builder.add(regex.regex.as_ref(), terminal)
    }

    pub fn eject(&mut self) -> Automaton<T> {
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
            VertexLabel::NonTerminal => None,
            VertexLabel::Terminal(t) => Some(t),
        }
    }

    pub fn reset(&mut self) {
        self.walker.set_active_position(self.start)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::scripting::regex::literal_seq;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn simple() {
        let mut builder = AutomatonBuilder::new();
        builder.add(literal_seq("abc".chars()), 1);
        builder.add(literal_seq("123".chars()), 2);
        let mut automaton = builder.eject();

        assert!(automaton.feed('a'));
        assert!(automaton.feed('b'));
        assert!(automaton.feed('c'));
        assert!(!automaton.feed('d'));
        assert_eq!(Some(&1), automaton.finish());
        automaton.reset();

        assert!(automaton.feed('a'));
        assert!(automaton.feed('b'));
        assert!(automaton.feed('c'));
        assert!(!automaton.feed('d'));
        assert_eq!(Some(&1), automaton.finish());
        automaton.reset();

        assert!(automaton.feed('1'));
        assert!(automaton.feed('2'));
        assert!(automaton.feed('3'));
        assert!(!automaton.feed('4'));
        assert_eq!(Some(&2), automaton.finish());
        automaton.reset();
    }
}