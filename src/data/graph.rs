use std::{collections::HashMap};


pub struct Graph<N, A> {
    nodes: Vec<Node<N, A>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidNode,
    NoArcs,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId {
    id: usize,
}

struct Node<N, A> {
    label: N,
    arcs: HashMap<usize, Vec<A>>,
}

impl<N, A> Graph<N, A> {
    pub fn new() -> Graph<N, A> {
        Graph {
            nodes: Vec::new(),
        }
    }

    pub fn create_node(&mut self, label: N) -> NodeId {
        let id = self.nodes.len();
        let node = Node {
            arcs: HashMap::new(),
            label
        };

        self.nodes.push(node);
        NodeId { id }
    }

    pub fn link(&mut self, from: NodeId, to: NodeId, label: A) -> Result<(), Error> {
        let mut node = self.nodes.get_mut(from.id).ok_or(Error::InvalidNode)?;
        let mut arcs = &mut node.arcs;
        let mut labels = arcs.entry(to.id).or_insert_with(|| Vec::new());
        labels.push(label);
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn reachable_from(&self, from: NodeId) -> Result<Vec<NodeId>, Error> {
        self.get_node(from).map(|n| n.arcs.keys().map(|id| NodeId { id: *id }).collect())
    }

    pub fn arcs_between(&self, from: NodeId, to: NodeId) -> Result<&Vec<A>, Error> {
        self.get_node(from).and_then(|n| n.arcs.get(&to.id).ok_or(Error::NoArcs))
    }

    fn get_node(&self, id: NodeId) -> Result<&Node<N, A>, Error> {
        self.nodes.get(id.id).ok_or(Error::InvalidNode)
    }

    fn get_node_mut(&mut self, id: NodeId) -> Result<&mut Node<N, A>, Error> {
        self.nodes.get_mut(id.id).ok_or(Error::InvalidNode)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::assert_same_elements;

    #[cfg(test)]
    use super::*;

    #[test]
    fn node_count() {
        let mut graph: Graph<(), ()> = Graph::new();

        let _node = graph.create_node(());
        assert_eq!(1, graph.node_count());

        let _node = graph.create_node(());
        assert_eq!(2, graph.node_count());

        let _node = graph.create_node(());
        assert_eq!(3, graph.node_count());
    }

    #[test]
    fn reachable_from() {
        let mut graph: Graph<(), ()> = Graph::new();

        let n1 = graph.create_node(());
        let n2 = graph.create_node(());
        let n3 = graph.create_node(());

        assert_same_elements!(vec![], graph.reachable_from(n1).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n2).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n3).unwrap());

        graph.link(n1, n2, ()).unwrap();
        assert_same_elements!(vec![n2], graph.reachable_from(n1).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n2).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n3).unwrap());

        graph.link(n1, n3, ()).unwrap();
        assert_same_elements!(vec![n2, n3], graph.reachable_from(n1).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n2).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n3).unwrap());

        graph.link(n2, n3, ()).unwrap();
        assert_same_elements!(vec![n2, n3], graph.reachable_from(n1).unwrap());
        assert_same_elements!(vec![n3], graph.reachable_from(n2).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(n3).unwrap());

        graph.link(n3, n1, ()).unwrap();
        assert_same_elements!(vec![n2, n3], graph.reachable_from(n1).unwrap());
        assert_same_elements!(vec![n3], graph.reachable_from(n2).unwrap());
        assert_same_elements!(vec![n1], graph.reachable_from(n3).unwrap());
    }

    #[test]
    fn arcs_between() {
        let mut graph: Graph<(), i32> = Graph::new();

        let n1 = graph.create_node(());
        let n2 = graph.create_node(());
        let n3 = graph.create_node(());

        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n1, n2));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n1, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n2));

        graph.link(n1, n2, 1).unwrap();
        assert_same_elements!(vec![1], graph.arcs_between(n1, n2).unwrap());
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n1, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n2));

        graph.link(n1, n3, 2).unwrap();
        assert_same_elements!(vec![1], graph.arcs_between(n1, n2).unwrap());
        assert_same_elements!(vec![2], graph.arcs_between(n1, n3).unwrap());
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n2));

        graph.link(n1, n3, 3).unwrap();
        assert_same_elements!(vec![1], graph.arcs_between(n1, n2).unwrap());
        assert_same_elements!(vec![2, 3], graph.arcs_between(n1, n3).unwrap());
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n1));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n2));

        graph.link(n3, n1, 4).unwrap();
        assert_same_elements!(vec![1], graph.arcs_between(n1, n2).unwrap());
        assert_same_elements!(vec![2, 3], graph.arcs_between(n1, n3).unwrap());
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n3));
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n2, n1));
        assert_same_elements!(vec![4], graph.arcs_between(n3, n1).unwrap());
        assert_eq!(Err(Error::NoArcs), graph.arcs_between(n3, n2));
    }
}
