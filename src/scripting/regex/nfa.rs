use super::Regex;
use crate::{data::{graph::{Graph, VertexId}}, util::tag::Tag};

pub trait VertexLabel: Copy + Clone {
    fn nonterminal() -> Self;
}

pub trait EdgeLabel {
    fn epsilon() -> Self;

    fn from_char(ch: char) -> Self;
}

pub struct NFABuilder<V: VertexLabel, E: EdgeLabel, T: Tag> {
    graph: Graph<V, E, T>,
    start: VertexId<T>,
}

impl<V: VertexLabel, E: EdgeLabel, T: Tag> NFABuilder<V, E, T> {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let start = graph.create_vertex(V::nonterminal());

        NFABuilder { graph, start }
    }

    pub fn add(&mut self, regex: &Regex, terminal_vertex_label: V) {
        let terminal_vertex = self.add_helper(regex, self.start);
        *self.graph.vertex_label_mut(terminal_vertex).expect("Bug") = terminal_vertex_label;
    }

    fn add_helper(&mut self, regex: &Regex, start_vertex: VertexId<T>) -> VertexId<T> {
        match regex {
            Regex::Epsilon => {
                let vertex = self.graph.create_vertex(V::nonterminal());
                self.graph
                    .create_edge(start_vertex, vertex, E::epsilon())
                    .expect("Bug");
                vertex
            }
            Regex::Literal(c) => {
                let vertex = self.graph.create_vertex(V::nonterminal());
                self.graph
                    .create_edge(start_vertex, vertex, E::from_char(*c))
                    .expect("Bug");
                vertex
            },
            Regex::Sequence(ref children) => {
                let mut finish = start_vertex;

                for child in children {
                    finish = self.add_helper(child, finish);
                }

                finish
            }
        }
    }

    fn nfa<'a>(&'a self) -> &'a Graph<V, E, T> {
        &self.graph
    }

    fn start_vertex(&self) -> VertexId<T> {
        self.start
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use crate::{assert_same_elements, data::graphwalker::GraphWalker};

    #[cfg(test)]
    use super::*;

    #[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
    enum Label {
        Epsilon,
        Char(char)
    }

    impl EdgeLabel for Label {
        fn epsilon() -> Self {
            Label::Epsilon
        }

        fn from_char(ch: char) -> Self {
            Label::Char(ch)
        }
    }

    impl VertexLabel for Option<i32> {
        fn nonterminal() -> Self {
            None
        }
    }

    #[rstest]
    fn nfa_literal() {
        type V = Option<i32>;
        type E = Label;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        builder.add(&Regex::Literal('a'), Some(1));
        builder.add(&Regex::Literal('b'), Some(2));

        let mut walker = GraphWalker::new(builder.nfa(), builder.start_vertex());
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        walker.follow(|lbl| *lbl == Label::Char('a'));
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        assert_same_elements!(vec![&Some(1)], walker.active_vertex_labels());

        walker.set_active_positions(&HashSet::from([builder.start_vertex()]));
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        walker.follow(|lbl| *lbl == Label::Char('b'));
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        assert_same_elements!(vec![&Some(2)], walker.active_vertex_labels());
    }

    #[rstest]
    fn nfa_sequence() {
        type V = Option<i32>;
        type E = Label;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = Regex::Sequence(vec![Box::new(Regex::Literal('a')), Box::new(Regex::Literal('b')), Box::new(Regex::Literal('c'))]);
        builder.add(&regex, Some(1));

        let mut walker = GraphWalker::new(builder.nfa(), builder.start_vertex());
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        walker.follow(|lbl| *lbl == Label::Char('a'));
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        walker.follow(|lbl| *lbl == Label::Char('b'));
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        walker.follow(|lbl| *lbl == Label::Char('c'));
        walker.follow_transitively(|lbl| *lbl == Label::Epsilon);
        assert_same_elements!(vec![&Some(1)], walker.active_vertex_labels());
    }
}
