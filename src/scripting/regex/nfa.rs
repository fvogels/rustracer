use super::{Regex, VertexLabel, EdgeLabel};
use crate::{data::{graph::{Graph, VertexId}}, util::tag::Tag};


pub struct NFABuilder<V, E: Copy + Clone, T: Tag> {
    graph: Graph<VertexLabel<V>, EdgeLabel<E>, T>,
    start: VertexId<T>,
}

impl<V, E: Copy + Clone, T: Tag> NFABuilder<V, E, T> {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let start = graph.create_vertex(VertexLabel::NonTerminal);

        NFABuilder { graph, start }
    }

    pub fn add(&mut self, regex: &Regex<E>, terminal_vertex_label: V) {
        let terminal_vertex = self.add_helper(regex, self.start);
        *self.graph.vertex_label_mut(terminal_vertex).expect("Bug") = VertexLabel::Terminal(terminal_vertex_label);
    }

    fn add_helper(&mut self, regex: &Regex<E>, start_vertex: VertexId<T>) -> VertexId<T> {
        match regex {
            Regex::Epsilon => {
                let vertex = self.graph.create_vertex(VertexLabel::NonTerminal);
                self.graph
                    .create_edge(start_vertex, vertex, EdgeLabel::Epsilon)
                    .expect("Bug");
                vertex
            }
            Regex::Literal(c) => {
                let vertex = self.graph.create_vertex(VertexLabel::NonTerminal);
                self.graph
                    .create_edge(start_vertex, vertex, EdgeLabel::Char(*c))
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

    fn nfa<'a>(&'a self) -> &'a Graph<VertexLabel<V>, EdgeLabel<E>, T> {
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

    #[rstest]
    fn nfa_literal() {
        let mut builder: NFABuilder<i32, char, ()> = NFABuilder::new();
        builder.add(&Regex::Literal('a'), 1);
        builder.add(&Regex::Literal('b'), 2);

        let mut walker = GraphWalker::new(builder.nfa(), builder.start_vertex());
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        walker.follow(|lbl| *lbl == EdgeLabel::Char('a'));
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        assert_same_elements!(vec![&VertexLabel::Terminal(1)], walker.active_vertex_labels());

        walker.set_active_positions(&HashSet::from([builder.start_vertex()]));
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        walker.follow(|lbl| *lbl == EdgeLabel::Char('b'));
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        assert_same_elements!(vec![&VertexLabel::Terminal(2)], walker.active_vertex_labels());
    }

    #[rstest]
    fn nfa_sequence() {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = Regex::Sequence(vec![Box::new(Regex::Literal('a')), Box::new(Regex::Literal('b')), Box::new(Regex::Literal('c'))]);
        builder.add(&regex, 1);

        let mut walker = GraphWalker::new(builder.nfa(), builder.start_vertex());
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        walker.follow(|lbl| *lbl == EdgeLabel::Char('a'));
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        walker.follow(|lbl| *lbl == EdgeLabel::Char('b'));
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        walker.follow(|lbl| *lbl == EdgeLabel::Char('c'));
        walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);

        let labels =  walker.active_vertex_labels();
        assert_same_elements!(vec![&VertexLabel::Terminal(1)], walker.active_vertex_labels());
    }
}
