use std::collections::HashSet;

use crate::{data::graph::{Graph, VertexId, Vertex}, util::tag::{Tag, define_tag}};

define_tag!(NFA);
define_tag!(DFA);

pub struct Tokenizer {}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    Identifier(String),
    Integer(i64),
    FloatingPointNumber(f64),
}

enum Regex {
    Epsilon,
    Literal(char),
    Sequence(Vec<Box<Regex>>),
    // Alternatives(Vec<Box<Regex>>),
    // Kleene(Box<Regex>),
}

#[derive(Debug, PartialEq)]
enum VertexLabel {
    NonTerminal,
    Terminal(TokenType)
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
            },
            Regex::Sequence(ref children) => {
                let mut finish = start_node;

                for child in children {
                    finish = add(graph, child, finish);
                }

                finish
            }
        }
    }

    let exit_node = add(graph, regex, start_node);
    *graph.vertex_label_mut(exit_node).expect("Bug") = node_label;
}

struct GraphWalker<'a, VertexLabel, EdgeLabel, T: Tag = ()> {
    graph: &'a Graph<VertexLabel, EdgeLabel, T>,
    start_position: VertexId<T>,
    current_position: HashSet<VertexId<T>>,
}

impl<'a, VertexLabel, EdgeLabel, T: Tag> GraphWalker<'a, VertexLabel, EdgeLabel, T> {
    pub fn new(graph: &'a Graph<VertexLabel, EdgeLabel, T>, start_node: VertexId<T>) -> GraphWalker<'a, VertexLabel, EdgeLabel, T> {
        let mut result = GraphWalker {
            graph,
            start_position: start_node,
            current_position: HashSet::from([start_node]),
        };

        result
    }

    pub fn reset(&mut self) {
        self.current_position = HashSet::from([self.start_position]);
    }

    pub fn follow<P: Fn(&EdgeLabel) -> bool>(&mut self, predicate: P) -> bool {
        let mut new_position = HashSet::new();

        for node in self.current_position.iter() {
            for n in self
                .graph
                .reachable_through(*node, |lbl| predicate(lbl))
                .expect("Bug")
            {
                new_position.insert(n);
            }
        }

        if new_position.is_empty() {
            false
        } else {
            self.current_position = new_position;
            true
        }
    }

    fn follow_transitively<P: Fn(&EdgeLabel) -> bool>(&mut self, predicate: P) {
        let mut todo: Vec<_> = self.current_position.iter().map(|&x| x).collect();

        while let Some(node) = todo.pop() {
            let reachable_by_epsilon = self
                .graph
                .reachable_through(node, |lbl| predicate(lbl))
                .expect("Bug");

            for n in reachable_by_epsilon {
                if self.current_position.insert(n) {
                    todo.push(n);
                }
            }
        }
    }

    // pub fn terminals(&self) -> Vec<TokenType> {
    //     fn is_terminal<VertexLabel, EdgeLabel, T: Tag>(walker: &GraphWalker<VertexLabel, EdgeLabel, T>, vertex: VertexId<T>) -> Option<TokenType> {
    //         match walker.graph.vertex_label(vertex).expect("Bug") {
    //             VertexLabel::NonTerminal => None,
    //             VertexLabel::Terminal(token_type) => Some(token_type.clone())
    //         }
    //     }

    //     self.current_position.iter().copied().filter_map(|v| is_terminal(self, v)).collect()
    // }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::assert_same_elements;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn graphwalker_follow() {
        fn ps(walker: &GraphWalker<(), char>) -> Vec<VertexId> {
            walker.current_position.iter().copied().collect()
        }

        let mut graph = Graph::new();

        let v1 = graph.create_vertex(());
        let v2 = graph.create_vertex(());
        let v3 = graph.create_vertex(());
        let v4 = graph.create_vertex(());

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
            graph.create_edge(s, e, c);
        }

        let mut walker = GraphWalker::new(&graph, v1);
        assert_same_elements!(vec![v1], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'a'));
        assert_same_elements!(vec![v2], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'd'));
        assert_same_elements!(vec![v2], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'a'));
        assert_same_elements!(vec![v3], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(!walker.follow(|lbl| *lbl == 'x'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'c'));
        assert_same_elements!(vec![v1], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v2, v3], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));
    }

    #[rstest]
    fn graphwalker_follow_transitively() {
        fn ps(walker: &GraphWalker<(), char>) -> Vec<VertexId> {
            walker.current_position.iter().copied().collect()
        }

        let mut graph = Graph::new();

        let v1 = graph.create_vertex(());
        let v2 = graph.create_vertex(());
        let v3 = graph.create_vertex(());
        let v4 = graph.create_vertex(());
        let v5 = graph.create_vertex(());

        for (s, e, c) in vec![
            (v1, v1, 'a'),
            (v1, v2, 'a'),
            (v2, v3, 'a'),
            (v2, v5, 'a'),
            (v3, v3, 'c'),
            (v3, v4, 'a'),
            (v5, v1, 'b'),
        ] {
            graph.create_edge(s, e, c).unwrap();
        }

        let mut walker = GraphWalker::new(&graph, v1);
        assert_same_elements!(vec![v1], ps(&walker));

        walker.follow_transitively(|lbl| *lbl == 'a');
        assert_same_elements!(vec![v1, v2, v3, v4, v5], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v1], ps(&walker));

        walker.follow_transitively(|lbl| *lbl == 'c');
        assert_same_elements!(vec![v1], ps(&walker));

        walker.follow_transitively(|lbl| *lbl == 'a');
        assert_same_elements!(vec![v1, v2, v3, v4, v5], ps(&walker));

        assert!(walker.follow(|lbl| *lbl == 'c'));
        assert_same_elements!(vec![v3], ps(&walker));

        walker.follow_transitively(|lbl| *lbl == 'a');
        assert_same_elements!(vec![v3, v4], ps(&walker));
    }
}
