use std::{
    borrow::{Borrow, BorrowMut},
    cell::Cell,
    collections::HashSet,
    hash::Hash,
    ops::Deref,
};

use super::defs::{EdgeLabel, RegularExpression, VertexLabel};
use crate::{
    data::{
        graph::{Graph, VertexId},
        graphwalker::GraphWalker,
    },
    util::tag::Tag,
};

pub struct NFABuilder<V, E: Copy + Clone, T: Tag> {
    graph: Graph<VertexLabel<V>, EdgeLabel<E>, T>,
    start: VertexId<T>,
    next_priority: i32,
}

impl<V, E: Copy + Clone, T: Tag> NFABuilder<V, E, T> {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let start = graph.create_vertex(VertexLabel::NonTerminal);

        NFABuilder { graph, start, next_priority: 0 }
    }

    pub fn add(&mut self, regex: &RegularExpression<E>, terminal_vertex_label: V) {
        let terminal_vertex = self.add_helper(regex, self.start);
        let label = VertexLabel::Terminal(terminal_vertex_label, self.next_priority);

        *self.graph.vertex_label_mut(terminal_vertex).expect("Bug") = label;
        self.next_priority += 1;
    }

    fn add_helper(
        &mut self,
        regex: &RegularExpression<E>,
        start_vertex: VertexId<T>,
    ) -> VertexId<T> {
        match regex {
            RegularExpression::Epsilon => {
                let vertex = self.graph.create_vertex(VertexLabel::NonTerminal);
                self.graph
                    .create_edge(start_vertex, vertex, EdgeLabel::Epsilon)
                    .expect("Bug");
                vertex
            }
            RegularExpression::Literal(c) => {
                let vertex = self.graph.create_vertex(VertexLabel::NonTerminal);
                self.graph
                    .create_edge(start_vertex, vertex, EdgeLabel::Char(*c))
                    .expect("Bug");
                vertex
            }
            RegularExpression::Sequence(ref children) => {
                let mut finish = start_vertex;

                for child in children {
                    finish = self.add_helper(child, finish);
                }

                finish
            }
            RegularExpression::Alternatives(ref children) => {
                let finish = self.graph.create_vertex(VertexLabel::NonTerminal);

                for child in children {
                    let vertex = self.add_helper(child, start_vertex);
                    self.graph
                        .create_edge(vertex, finish, EdgeLabel::Epsilon)
                        .expect("Bug");
                }

                finish
            }
            RegularExpression::Kleene(ref child) => {
                let finish = self.graph.create_vertex(VertexLabel::NonTerminal);
                let exit = self.add_helper(child, start_vertex);

                self.graph
                    .create_edge(start_vertex, finish, EdgeLabel::Epsilon)
                    .expect("Bug");
                self.graph
                    .create_edge(exit, finish, EdgeLabel::Epsilon)
                    .expect("Bug");
                self.graph
                    .create_edge(exit, start_vertex, EdgeLabel::Epsilon)
                    .expect("Bug");

                finish
            }
        }
    }

    pub fn eject(self) -> (Graph<VertexLabel<V>, EdgeLabel<E>, T>, VertexId<T>) {
        (self.graph, self.start)
    }
}

pub struct NFAWalker<V, E: Hash + Eq + Copy + Clone, T: Tag = ()> {
    walker: GraphWalker<VertexLabel<V>, EdgeLabel<E>, T>,
}

impl<V, E: Hash + Eq + Copy + Clone, T: Tag> NFAWalker<V, E, T> {
    pub fn new(graph: Graph<VertexLabel<V>, EdgeLabel<E>, T>, start_vertex: VertexId<T>) -> Self {
        let mut result = NFAWalker {
            walker: GraphWalker::new(graph, start_vertex),
        };
        result.walk_epsilon();
        result
    }

    pub fn walk(&mut self, ch: E) -> bool {
        if self.walk_char(ch) {
            self.walk_epsilon();
            true
        } else {
            false
        }
    }

    fn walk_char(&mut self, ch: E) -> bool {
        fn is_char<E: Eq>(lbl: &EdgeLabel<E>, ch: E) -> bool {
            match lbl {
                EdgeLabel::Epsilon => false,
                EdgeLabel::Char(c) => *c == ch,
            }
        }

        self.walker.walk(&|lbl| is_char(lbl, ch))
    }

    fn walk_epsilon(&mut self) {
        fn is_epsilon<E>(lbl: &EdgeLabel<E>) -> bool {
            match lbl {
                EdgeLabel::Epsilon => true,
                EdgeLabel::Char(c) => false,
            }
        }

        self.walker.walk_transitively(&is_epsilon)
    }

    pub fn active_vertex_labels(&self) -> Vec<&VertexLabel<V>> {
        self.walker.active_vertex_labels()
    }

    pub fn priority_terminal_label(&self) -> Option<&V> {
        fn vertex_label_priority<V>(lbl: &VertexLabel<V>) -> Option<i32> {
            match lbl {
                VertexLabel::NonTerminal => None,
                VertexLabel::Terminal(_, priority) => Some(*priority)
            }
        }

        let active_vertex_labels = self.active_vertex_labels();
        active_vertex_labels.iter().filter_map(|&lbl| {
            match lbl  {
                VertexLabel::NonTerminal => None,
                VertexLabel::Terminal(v, p) => Some((v, *p))
            }
        }).min_by_key(|(_v, p)| *p).map(|(v, _p)| v)
    }

    pub fn set_active_positions(&mut self, positions: &HashSet<VertexId<T>>) {
        self.walker.set_active_positions(positions)
    }

    pub fn active_positions(&self) -> &HashSet<VertexId<T>> {
        &self.walker.active_positions
    }

    pub fn departing_arcs(&self) -> HashSet<EdgeLabel<E>> {
        self.walker.departing_arcs()
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, rc::Rc};

    use rstest::rstest;

    use crate::{data::graphwalker::GraphWalker};

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn literal() {
        let mut builder: NFABuilder<i32, char, ()> = NFABuilder::new();
        builder.add(&RegularExpression::Literal('a'), 1);
        builder.add(&RegularExpression::Literal('b'), 2);
        let (mut nfa, start) = builder.eject();

        let mut walker = NFAWalker::new(nfa, start);
        walker.walk('a');
        assert_eq!(
            Some(&1),
            walker.priority_terminal_label()
        );

        walker.set_active_positions(&HashSet::from([start]));
        walker.walk('b');
        assert_eq!(
            Some(&2),
            walker.priority_terminal_label()
        );
    }

    #[rstest]
    fn sequence() {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = RegularExpression::Sequence(vec![
            Rc::new(RegularExpression::Literal('a')),
            Rc::new(RegularExpression::Literal('b')),
            Rc::new(RegularExpression::Literal('c')),
        ]);
        builder.add(&regex, 1);
        let (mut nfa, start) = builder.eject();

        let mut walker = NFAWalker::new(nfa, start);
        walker.walk('a');
        walker.walk('b');
        walker.walk('c');

        assert_eq!(Some(&1), walker.priority_terminal_label());
    }

    #[rstest]
    fn alternatives1(#[values('a', 'b', 'c')] ch: char) {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = RegularExpression::Alternatives(vec![
            Rc::new(RegularExpression::Literal('a')),
            Rc::new(RegularExpression::Literal('b')),
            Rc::new(RegularExpression::Literal('c')),
        ]);
        builder.add(&regex, 1);
        let (mut nfa, start) = builder.eject();

        let mut walker = NFAWalker::new(nfa, start);

        assert_eq!(None, walker.priority_terminal_label());
        walker.walk(ch);
        assert_eq!(Some(&1), walker.priority_terminal_label());
    }

    #[rstest]
    fn alternatives2(#[values('a', 'b', 'c')] first: char, #[values('x', 'y', 'z')] second: char) {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = RegularExpression::Sequence(vec![
            Rc::new(RegularExpression::Alternatives(vec![
                Rc::new(RegularExpression::Literal('a')),
                Rc::new(RegularExpression::Literal('b')),
                Rc::new(RegularExpression::Literal('c')),
            ])),
            Rc::new(RegularExpression::Alternatives(vec![
                Rc::new(RegularExpression::Literal('x')),
                Rc::new(RegularExpression::Literal('y')),
                Rc::new(RegularExpression::Literal('z')),
            ])),
        ]);
        builder.add(&regex, 1);
        let (mut nfa, start) = builder.eject();

        let mut walker = NFAWalker::new(nfa, start);

        assert_eq!(None, walker.priority_terminal_label());
        walker.walk(first);
        assert_eq!(None, walker.priority_terminal_label());
        walker.walk(second);
        assert_eq!(Some(&1), walker.priority_terminal_label());
    }

    #[rstest]
    fn kleene() {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = RegularExpression::Kleene(Rc::new(RegularExpression::Literal('a')));
        builder.add(&regex, 1);
        let (mut nfa, start) = builder.eject();

        let mut walker = NFAWalker::new(nfa, start);

        assert_eq!(Some(&1), walker.priority_terminal_label());
        walker.walk('a');
        assert_eq!(Some(&1), walker.priority_terminal_label());
        walker.walk('a');
        assert_eq!(Some(&1), walker.priority_terminal_label());
        walker.walk('a');
    }

    #[rstest]
    fn kleene_of_sequence() {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = RegularExpression::Kleene(Rc::new(RegularExpression::Sequence(vec![
            Rc::new(RegularExpression::Literal('a')),
            Rc::new(RegularExpression::Literal('b')),
        ])));
        builder.add(&regex, 1);
        let (mut nfa, start) = builder.eject();

        let mut walker = NFAWalker::new(nfa, start);

        assert_eq!(Some(&1), walker.priority_terminal_label());
        walker.walk('a');
        assert_eq!(None, walker.priority_terminal_label());
        walker.walk('b');
        assert_eq!(Some(&1), walker.priority_terminal_label());
        walker.walk('a');
        assert_eq!(None, walker.priority_terminal_label());
        walker.walk('b');
        assert_eq!(Some(&1), walker.priority_terminal_label());
    }

    #[rstest]
    fn priority() {
        type V = i32;
        type E = char;
        type T = ();

        let mut builder: NFABuilder<V, E, T> = NFABuilder::new();
        let regex = RegularExpression::Sequence(vec![
            Rc::new(RegularExpression::Literal('a')),
            Rc::new(RegularExpression::Literal('b')),
            Rc::new(RegularExpression::Literal('c')),
        ]);
        builder.add(&regex, 1);
        builder.add(&regex, 2);

        let (mut nfa, start) = builder.eject();
        let mut walker = NFAWalker::new(nfa, start);

        walker.walk('a');
        walker.walk('b');
        walker.walk('c');

        assert_eq!(Some(&1), walker.priority_terminal_label());
    }
}
