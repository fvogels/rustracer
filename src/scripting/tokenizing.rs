use std::{collections::HashSet};

use crate::data::graph::{Graph, NodeId};

pub struct Tokenizer {

}

enum Regex {
    Epsilon,
    Literal(char),
    // Sequence(Vec<Box<Regex>>),
    // Alternatives(Vec<Box<Regex>>),
    // Kleene(Box<Regex>),
}

#[derive(Debug, PartialEq, Eq)]
enum NodeLabel {
    NonTerminal,
}

#[derive(Debug, PartialEq, Eq)]
enum ArcLabel {
    Epsilon,
    Char(char)
}

fn add_to_graph(graph: &mut Graph<NodeLabel, ArcLabel>, regex: &Regex, start_node: NodeId, node_label: NodeLabel) {
    fn add(graph: &mut Graph<NodeLabel, ArcLabel>, regex: &Regex, start_node: NodeId) -> NodeId {
        match regex {
            Regex::Epsilon => {
                let node = graph.create_node(NodeLabel::NonTerminal);
                graph.link(start_node, node, ArcLabel::Epsilon).expect("Bug");
                node
            },
            Regex::Literal(c) => {
                let node = graph.create_node(NodeLabel::NonTerminal);
                graph.link(start_node, node, ArcLabel::Char(*c)).expect("Bug");
                node
            },
        }
    }

    let exit_node = add(graph, regex, start_node);
    *graph.node_label_mut(exit_node).expect("Bug") = node_label;
}

struct GraphWalker<'a> {
    graph: &'a Graph<NodeLabel, ArcLabel>,
    current_position: HashSet<NodeId>,
}

impl<'a> GraphWalker<'a> {
    pub fn new(graph: &'a Graph<NodeLabel, ArcLabel>, start_node: NodeId) -> GraphWalker<'a> {
        let mut result = GraphWalker {
            graph,
            current_position: HashSet::from([start_node])
        };

        result.follow_epsilons();
        result
    }

    pub fn follow(&mut self, ch: char) -> bool {
        let mut new_position = HashSet::new();

        for node in self.current_position.iter() {
            for n in self.graph.reachable_by(*node, &ArcLabel::Char(ch)).expect("Bug") {
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
            let reachable_by_epsilon = self.graph.reachable_by(node, &ArcLabel::Epsilon).expect("Bug");

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


}