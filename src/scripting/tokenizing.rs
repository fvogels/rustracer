use std::collections::HashSet;

use crate::{data::graph::{Graph, VertexId}, util::tag::{Tag, define_tag}};

define_tag!(NFA);
define_tag!(DFA);

pub struct Tokenizer {}

enum Regex {
    Epsilon,
    Literal(char),
    // Sequence(Vec<Box<Regex>>),
    // Alternatives(Vec<Box<Regex>>),
    // Kleene(Box<Regex>),
}

#[derive(Debug, PartialEq, Eq)]
enum VertexLabel {
    NonTerminal,
}

#[derive(Debug, PartialEq, Eq)]
enum EdgeLabel {
    Epsilon,
    Char(char),
}

type RegexGraph<Tag> = Graph<VertexLabel, EdgeLabel, Tag>;

type NFAGraph = RegexGraph<NFA>;
type DFAGraph = RegexGraph<DFA>;

fn add_to_graph(
    graph: &mut NFAGraph,
    regex: &Regex,
    start_node: VertexId<NFA>,
    node_label: VertexLabel,
) {
    fn add(graph: &mut NFAGraph, regex: &Regex, start_node: VertexId<NFA>) -> VertexId<NFA> {
        match regex {
            Regex::Epsilon => {
                let node = graph.create_vertex(VertexLabel::NonTerminal);
                graph
                    .create_edge(start_node, node, EdgeLabel::Epsilon)
                    .expect("Bug");
                node
            }
            Regex::Literal(c) => {
                let node = graph.create_vertex(VertexLabel::NonTerminal);
                graph
                    .create_edge(start_node, node, EdgeLabel::Char(*c))
                    .expect("Bug");
                node
            }
        }
    }

    let exit_node = add(graph, regex, start_node);
    *graph.vertex_label_mut(exit_node).expect("Bug") = node_label;
}

struct GraphWalker<'a, T: Tag> {
    graph: &'a RegexGraph<T>,
    current_position: HashSet<VertexId<T>>,
}

impl<'a, T: Tag> GraphWalker<'a, T> {
    pub fn new(graph: &'a RegexGraph<T>, start_node: VertexId<T>) -> GraphWalker<'a, T> {
        let mut result = GraphWalker {
            graph,
            current_position: HashSet::from([start_node]),
        };

        result.follow_epsilons();
        result
    }

    pub fn follow(&mut self, ch: char) -> bool {
        let mut new_position = HashSet::new();

        for node in self.current_position.iter() {
            for n in self
                .graph
                .reachable_through(*node, &EdgeLabel::Char(ch))
                .expect("Bug")
            {
                new_position.insert(n);
            }
        }

        if new_position.is_empty() {
            false
        } else {
            self.current_position = new_position;
            self.follow_epsilons();
            true
        }
    }

    fn follow_epsilons(&mut self) {
        let mut todo: Vec<_> = self.current_position.iter().map(|&x| x).collect();

        while let Some(node) = todo.pop() {
            let reachable_by_epsilon = self
                .graph
                .reachable_through(node, &EdgeLabel::Epsilon)
                .expect("Bug");

            for n in reachable_by_epsilon {
                if self.current_position.insert(n) {
                    todo.push(n);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::assert_same_elements;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn graphwalker_follow_no_epsilons() {
        fn ps(walker: &GraphWalker<NFA>) -> Vec<VertexId<NFA>> {
            walker.current_position.iter().copied().collect()
        }

        let mut graph: NFAGraph = Graph::new();

        let v1 = graph.create_vertex(VertexLabel::NonTerminal);
        let v2 = graph.create_vertex(VertexLabel::NonTerminal);
        let v3 = graph.create_vertex(VertexLabel::NonTerminal);
        let v4 = graph.create_vertex(VertexLabel::NonTerminal);

        for (s, e, c) in vec![
            (v1, v2, 'a'),
            (v1, v2, 'b'),
            (v1, v3, 'b'),
            (v2, v1, 'c'),
            (v2, v2, 'd'),
            (v2, v3, 'a'),
            (v2, v4, 'b'),
            (v3, v1, 'c'),
            (v3, v3, 'b'),
            (v3, v4, 'b'),
            (v4, v1, 'c'),
        ] {
            graph.create_edge(s, e, EdgeLabel::Char(c));
        }

        let mut walker = GraphWalker::new(&graph, v1);
        assert_same_elements!(vec![v1], ps(&walker));

        assert!(walker.follow('a'));
        assert_same_elements!(vec![v2], ps(&walker));

        assert!(walker.follow('d'));
        assert_same_elements!(vec![v2], ps(&walker));

        assert!(walker.follow('a'));
        assert_same_elements!(vec![v3], ps(&walker));

        assert!(walker.follow('b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(walker.follow('b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(!walker.follow('x'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(walker.follow('c'));
        assert_same_elements!(vec![v1], ps(&walker));

        assert!(walker.follow('b'));
        assert_same_elements!(vec![v2, v3], ps(&walker));

        assert!(walker.follow('b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));
    }

    #[rstest]
    fn graphwalker_follow_with_epsilons() {
        fn ps(walker: &GraphWalker<NFA>) -> Vec<VertexId<NFA>> {
            walker.current_position.iter().copied().collect()
        }

        let mut graph: NFAGraph = Graph::new();

        let v1 = graph.create_vertex(VertexLabel::NonTerminal);
        let v2 = graph.create_vertex(VertexLabel::NonTerminal);
        let v3 = graph.create_vertex(VertexLabel::NonTerminal);
        let v4 = graph.create_vertex(VertexLabel::NonTerminal);
        let v5 = graph.create_vertex(VertexLabel::NonTerminal);

        for (s, e, c) in vec![
            (v1, v2, EdgeLabel::Epsilon),
            (v1, v5, EdgeLabel::Char('a')),
            (v1, v3, EdgeLabel::Char('a')),
            (v1, v4, EdgeLabel::Char('a')),
            (v4, v1, EdgeLabel::Char('a')),
            (v4, v5, EdgeLabel::Epsilon),
        ] {
            graph.create_edge(s, e, c);
        }

        let mut walker = GraphWalker::new(&graph, v1);
        assert_same_elements!(vec![v1, v2], ps(&walker));

        assert!(walker.follow('a'));
        assert_same_elements!(vec![v3, v4, v5], ps(&walker));

        assert!(walker.follow('a'));
        assert_same_elements!(vec![v1, v2], ps(&walker));

        assert!(!walker.follow('b'));
        assert_same_elements!(vec![v1, v2], ps(&walker));
    }
}
