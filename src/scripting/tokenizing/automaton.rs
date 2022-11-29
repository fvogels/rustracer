use crate::regex::Regex;

pub struct Automaton<T: Copy + Clone> {
    rules: Vec<(Regex, T)>,
    state: Vec<(Regex, T)>
}

pub struct AutomatonBuilder<T: Copy + Clone> {
    rules: Vec<(Regex, T)>,
}

impl<T: Copy + Clone> Automaton<T> {
    pub fn reset(&mut self) {
        self.state = self.rules.clone();
    }

    pub fn is_terminal(&self) -> bool {
        self.state.iter().any(|(regex, _)| regex.is_terminal())
    }

    pub fn feed(&mut self, ch: char) -> bool {
        let mut next_generation: Vec<(Regex, T)> = self.state.iter().filter_map(|(regex, t)| {
            regex.try_feed(ch).map(|r| (r, *t))
        }).collect();

        if next_generation.is_empty() {
            false
        } else {
            self.state = next_generation;
            true
        }
    }

    pub fn current(&self) -> Option<T> {
        self.state.iter().find(|(regex, _t)| regex.is_terminal()).map(|(_regex, t)| t.clone())
    }
}

impl<T: Copy + Clone> AutomatonBuilder<T> {
    pub fn new() -> Self {
        AutomatonBuilder { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, regex: Regex, t: T) {
        self.rules.push((regex, t));
    }

    pub fn eject(self) -> Automaton<T> {
        Automaton {
            rules: self.rules.clone(),
            state: self.rules,
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn test() {
        let mut builder = AutomatonBuilder::new();

        builder.add_rule(Regex::literal('('), 0);
        builder.add_rule(Regex::literal(')'), 1);
        builder.add_rule(Regex::integer(10), 2);

        let mut automaton = builder.eject();

        assert!(!automaton.is_terminal());
        assert!(automaton.feed('('));
        assert!(automaton.is_terminal());
        assert_eq!(Some(0), automaton.current());
        assert!(!automaton.feed('('));

        automaton.reset();

        assert!(!automaton.is_terminal());
        assert!(automaton.feed(')'));
        assert!(automaton.is_terminal());
        assert_eq!(Some(1), automaton.current());
        assert!(!automaton.feed(')'));

        automaton.reset();

        assert!(!automaton.is_terminal());
        assert!(automaton.feed('1'));
        assert!(automaton.is_terminal());
        assert_eq!(Some(2), automaton.current());
        assert!(automaton.feed('1'));
        assert!(automaton.is_terminal());
        assert_eq!(Some(2), automaton.current());
    }
}