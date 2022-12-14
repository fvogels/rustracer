use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::util::tag::Tag;

pub struct Graph<VertexLabel, EdgeLabel, T: Tag = ()> {
    vertices: Vec<Vertex<VertexLabel, EdgeLabel, T>>,
    tag: PhantomData<T>,
}

struct Vertex<VertexLabel, EdgeLabel, T: Tag> {
    label: VertexLabel,
    departing_edges: HashMap<VertexId<T>, Vec<EdgeLabel>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VertexId<T: Tag = ()> {
    index: usize,
    tag: PhantomData<T>,
}

#[derive(Debug)]
pub enum Error<T: Tag> {
    InvalidVertexId(VertexId<T>),
    NoArcsTo(VertexId<T>),
}

impl<T: Tag> PartialOrd for VertexId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T: Tag> Ord for VertexId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<V, E, T: Tag> Graph<V, E, T> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            tag: PhantomData,
        }
    }

    pub fn create_vertex(&mut self, label: V) -> VertexId<T> {
        let index = self.vertices.len();
        let vertex = Vertex {
            label,
            departing_edges: HashMap::new(),
        };
        self.vertices.push(vertex);

        VertexId {
            index,
            tag: PhantomData,
        }
    }

    pub fn vertex_label(&self, vertex: VertexId<T>) -> Result<&V, Error<T>> {
        self.get_vertex(vertex).map(|v| &v.label)
    }

    pub fn vertex_label_mut(&mut self, vertex: VertexId<T>) -> Result<&mut V, Error<T>> {
        self.get_vertex_mut(vertex).map(|v| &mut v.label)
    }

    pub fn create_edge(
        &mut self,
        start: VertexId<T>,
        end: VertexId<T>,
        label: E,
    ) -> Result<(), Error<T>> {
        let start = self.get_vertex_mut(start)?;
        start.add_edge_to(end, label)
    }

    pub fn get_vertex(&self, id: VertexId<T>) -> Result<&Vertex<V, E, T>, Error<T>> {
        self.vertices
            .get(id.index)
            .ok_or(Error::InvalidVertexId(id))
    }

    pub fn get_vertex_mut(&mut self, id: VertexId<T>) -> Result<&mut Vertex<V, E, T>, Error<T>> {
        self.vertices
            .get_mut(id.index)
            .ok_or(Error::InvalidVertexId(id))
    }

    pub fn reachable_from(&self, id: VertexId<T>) -> Result<Vec<VertexId<T>>, Error<T>> {
        self.get_vertex(id).map(|v| v.reachable_from())
    }

    pub fn arcs_departing_from(&self, from: VertexId<T>) -> Result<Vec<&E>, Error<T>> {
        let vertex = self.get_vertex(from)?;

        Ok(vertex
            .departing_edges
            .values()
            .flatten()
            .collect::<Vec<&E>>())
    }

    pub fn arcs_between(&self, from: VertexId<T>, to: VertexId<T>) -> Result<&Vec<E>, Error<T>> {
        self.get_vertex(from).and_then(|v| v.arcs_to(to))
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn reachable_through<P: Fn(&E) -> bool>(
        &self,
        id: VertexId<T>,
        predicate: P,
    ) -> Result<Vec<VertexId<T>>, Error<T>> {
        self.get_vertex(id).map(|v| v.reachable_through(&predicate))
    }
}

impl<V, E, T: Tag> Vertex<V, E, T> {
    fn add_edge_to(&mut self, end: VertexId<T>, label: E) -> Result<(), Error<T>> {
        let vector = self
            .departing_edges
            .entry(end)
            .or_insert_with(|| Vec::new());
        vector.push(label);
        Ok(())
    }

    fn reachable_from(&self) -> Vec<VertexId<T>> {
        self.departing_edges.keys().copied().collect()
    }

    fn arcs_to(&self, to: VertexId<T>) -> Result<&Vec<E>, Error<T>> {
        self.departing_edges.get(&to).ok_or(Error::NoArcsTo(to))
    }

    fn reachable_through<P: Fn(&E) -> bool>(&self, predicate: &P) -> Vec<VertexId<T>> {
        self.departing_edges
            .iter()
            .filter_map(|(&id, edges)| {
                if edges.iter().any(|lbl| predicate(lbl)) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::assert_same_elements;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn vertex_count() {
        let mut graph: Graph<i32, ()> = Graph::new();
        assert_eq!(0, graph.vertex_count());

        graph.create_vertex(1);
        assert_eq!(1, graph.vertex_count());

        graph.create_vertex(21);
        assert_eq!(2, graph.vertex_count());

        graph.create_vertex(1);
        assert_eq!(3, graph.vertex_count());
    }

    #[rstest]
    fn vertex_label() {
        let mut graph: Graph<i32, ()> = Graph::new();

        let v1 = graph.create_vertex(1);
        let v2 = graph.create_vertex(2);
        let v3 = graph.create_vertex(3);

        assert_eq!(1, *graph.vertex_label(v1).unwrap());
        assert_eq!(2, *graph.vertex_label(v2).unwrap());
        assert_eq!(3, *graph.vertex_label(v3).unwrap());
    }

    #[rstest]
    fn reachable_from() {
        let mut graph: Graph<i32, ()> = Graph::new();

        let v1 = graph.create_vertex(1);
        let v2 = graph.create_vertex(2);
        let v3 = graph.create_vertex(3);
        let v4 = graph.create_vertex(4);

        graph.create_edge(v1, v2, ()).unwrap();
        graph.create_edge(v1, v3, ()).unwrap();
        graph.create_edge(v1, v4, ()).unwrap();
        graph.create_edge(v3, v3, ()).unwrap();
        graph.create_edge(v4, v1, ()).unwrap();
        graph.create_edge(v4, v2, ()).unwrap();

        assert_same_elements!(vec![v2, v3, v4], graph.reachable_from(v1).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(v2).unwrap());
        assert_same_elements!(vec![v3], graph.reachable_from(v3).unwrap());
        assert_same_elements!(vec![v1, v2], graph.reachable_from(v4).unwrap());
    }

    #[rstest]
    fn reachable_through() {
        let mut graph: Graph<i32, char> = Graph::new();

        let v1 = graph.create_vertex(1);
        let v2 = graph.create_vertex(2);
        let v3 = graph.create_vertex(3);
        let v4 = graph.create_vertex(4);

        graph.create_edge(v1, v1, 'a').expect("Bug");
        graph.create_edge(v1, v2, 'a').expect("Bug");
        graph.create_edge(v1, v2, 'b').expect("Bug");
        graph.create_edge(v1, v3, 'b').expect("Bug");
        graph.create_edge(v1, v2, 'c').expect("Bug");
        graph.create_edge(v1, v3, 'c').expect("Bug");
        graph.create_edge(v1, v4, 'c').expect("Bug");

        assert_same_elements!(
            vec![v1, v2],
            graph
                .reachable_through(v1, |lbl: &char| *lbl == 'a')
                .unwrap()
        );
        assert_same_elements!(
            vec![v2, v3],
            graph
                .reachable_through(v1, |lbl: &char| *lbl == 'b')
                .unwrap()
        );
        assert_same_elements!(
            vec![v2, v3],
            graph
                .reachable_through(v1, |lbl: &char| *lbl == 'b')
                .unwrap()
        );
        assert_same_elements!(
            vec![v2, v3, v4],
            graph
                .reachable_through(v1, |lbl: &char| *lbl == 'c')
                .unwrap()
        );
    }
}
