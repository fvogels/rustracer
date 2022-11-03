use std::collections::HashSet;

use crate::data::graph::{Graph, VertexId};

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

type RegexGraph = Graph<VertexLabel, EdgeLabel>;

fn add_to_graph(
    graph: &mut RegexGraph,
    regex: &Regex,
    start_node: VertexId,
    node_label: VertexLabel,
) {
    fn add(graph: &mut RegexGraph, regex: &Regex, start_node: VertexId) -> VertexId {
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

struct GraphWalker<'a> {
    graph: &'a RegexGraph,
    current_position: HashSet<VertexId>,
}

impl<'a> GraphWalker<'a> {
    pub fn new(graph: &'a RegexGraph, start_node: VertexId) -> GraphWalker<'a> {
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
            true
        }
    }

    fn follow_epsilons(&mut self) {
        let mut todo: Vec<_> = self.current_position.iter().map(|&x| x).collect();
        let mut new_position = HashSet::new();

        while let Some(node) = todo.pop() {
            let reachable_by_epsilon = self
                .graph
                .reachable_through(node, &EdgeLabel::Epsilon)
                .expect("Bug");

            for n in reachable_by_epsilon {
                new_position.insert(n);
                todo.push(n);
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
    fn graphwalker_follow() {
        fn ps(walker: &GraphWalker) -> Vec<VertexId> {
            walker.current_position.iter().copied().collect()
        }

        let mut graph: RegexGraph = Graph::new();

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
}
