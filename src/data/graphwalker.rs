use std::{collections::HashSet, hash::Hash};
use crate::{data::graph::{Graph, VertexId}, util::tag::Tag};

pub struct GraphWalker<V, E: Hash + Eq, T: Tag = ()> {
    graph: Graph<V, E, T>,
    pub active_positions: HashSet<VertexId<T>>,
}

impl<V, E: Hash + Eq + Copy + Clone, T: Tag> GraphWalker<V, E, T> {
    pub fn new(graph: Graph<V, E, T>, start_vertex: VertexId<T>) -> Self {
        GraphWalker {
            graph,
            active_positions: HashSet::from([start_vertex]),
        }
    }

    pub fn walk<P: Fn(&E) -> bool>(&mut self, predicate: &P) -> bool {
        let mut new_position = HashSet::new();

        for vertex in self.active_positions.iter() {
            for n in self
                .graph
                .reachable_through(*vertex, predicate)
                .expect("Bug")
            {
                new_position.insert(n);
            }
        }

        if new_position.is_empty() {
            false
        } else {
            self.active_positions = new_position;
            true
        }
    }

    pub fn walk_transitively<P: Fn(&E) -> bool>(&mut self, predicate: &P) {
        let mut todo: Vec<_> = self.active_positions.iter().map(|&x| x).collect();

        while let Some(vertex) = todo.pop() {
            let reachable_by_epsilon = self
                .graph
                .reachable_through(vertex, predicate)
                .expect("Bug");

            for n in reachable_by_epsilon {
                if self.active_positions.insert(n) {
                    todo.push(n);
                }
            }
        }
    }

    pub fn active_vertex_labels(&self) -> Vec<&V> {
        self.active_positions.iter().copied().map(|v| self.graph.vertex_label(v).expect("Bug")).collect()
    }

    pub fn departing_arcs(&self) -> HashSet<E> {
        let mut result = HashSet::new();

        for active_position in self.active_positions.iter() {
            for edge_label in self.graph.arcs_departing_from(*active_position).expect("Bug") {
                result.insert(*edge_label);
            }
        }

        result
    }

    pub fn set_active_positions(&mut self, positions: &HashSet<VertexId<T>>) {
        self.active_positions.clone_from(positions);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::assert_same_elements;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn graphwalker_walk_without_epsilons() {
        fn ps(walker: &GraphWalker<(), char>) -> Vec<VertexId> {
            walker.active_positions.iter().copied().collect()
        }

        let mut graph: Graph<(), char, ()> = Graph::new();

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
            graph.create_edge(s, e, c).expect("Bug");
        }

        let mut walker = GraphWalker::new(graph, v1);
        assert_same_elements!(vec![v1], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'a'));
        assert_same_elements!(vec![v2], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'd'));
        assert_same_elements!(vec![v2], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'a'));
        assert_same_elements!(vec![v3], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(!walker.walk(&|lbl| *lbl == 'x'));
        assert_same_elements!(vec![v3, v4], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'c'));
        assert_same_elements!(vec![v1], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v2, v3], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == 'b'));
        assert_same_elements!(vec![v3, v4], ps(&walker));
    }

    #[rstest]
    fn graphwalker_walk_with_epsilons() {
        type V = ();
        type E = Option<char>;

        fn ps(walker: &GraphWalker<V, E>) -> Vec<VertexId> {
            walker.active_positions.iter().copied().collect()
        }

        let mut graph: Graph<V, E, ()> = Graph::new();

        let v1 = graph.create_vertex(());
        let v2 = graph.create_vertex(());
        let v3 = graph.create_vertex(());
        let v4 = graph.create_vertex(());
        let v5 = graph.create_vertex(());

        for (s, e, c) in vec![
            (v1, v2, Some('a')),
            (v2, v3, None),
            (v2, v5, None),
            (v3, v3, Some('c')),
            (v3, v4, None),
            (v5, v1, Some('b')),
        ] {
            graph.create_edge(s, e, c).unwrap();
        }

        let mut walker = GraphWalker::new(graph, v1);
        assert_same_elements!(vec![v1], ps(&walker));

        walker.walk(&|lbl| *lbl == Some('a'));
        walker.walk_transitively(&|lbl| *lbl == None);
        assert_same_elements!(vec![v2, v3, v4, v5], ps(&walker));

        assert!(walker.walk(&|lbl| *lbl == Some('b')));
        walker.walk_transitively(&|lbl| *lbl == None);
        assert_same_elements!(vec![v1], ps(&walker));

        walker.walk(&|lbl| *lbl == Some('a'));
        walker.walk_transitively(&|lbl| *lbl == None);
        assert_same_elements!(vec![v2, v3, v4, v5], ps(&walker));

        walker.walk(&|lbl| *lbl == Some('c'));
        walker.walk_transitively(&|lbl| *lbl == None);
        assert_same_elements!(vec![v3, v4], ps(&walker));
    }
}
