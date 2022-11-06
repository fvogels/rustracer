use std::{collections::HashSet, hash::Hash};
use crate::{data::graph::{Graph, VertexId}, util::tag::Tag};

pub struct GraphWalker<'a, V, E: Hash + Eq, T: Tag = ()> {
    graph: &'a Graph<V, E, T>,
    active_positions: HashSet<VertexId<T>>,
}

impl<'a, V, E: Hash + Eq + Copy + Clone, T: Tag> GraphWalker<'a, V, E, T> {
    pub fn new(graph: &'a Graph<V, E, T>, start_vertex: VertexId<T>) -> Self {
        let mut result = GraphWalker {
            graph,
            active_positions: HashSet::from([start_vertex]),
        };

        result
    }

    pub fn follow<P: Fn(&E) -> bool>(&mut self, predicate: P) -> bool {
        let mut new_position = HashSet::new();

        for vertex in self.active_positions.iter() {
            for n in self
                .graph
                .reachable_through(*vertex, |lbl| predicate(lbl))
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

    pub fn follow_transitively<P: Fn(&E) -> bool>(&mut self, predicate: P) {
        let mut todo: Vec<_> = self.active_positions.iter().map(|&x| x).collect();

        while let Some(vertex) = todo.pop() {
            let reachable_by_epsilon = self
                .graph
                .reachable_through(vertex, |lbl| predicate(lbl))
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
    fn graphwalker_follow() {
        fn ps(walker: &GraphWalker<(), char>) -> Vec<VertexId> {
            walker.active_positions.iter().copied().collect()
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
            walker.active_positions.iter().copied().collect()
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
