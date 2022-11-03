use std::collections::HashMap;

pub struct Graph<VertexLabel, EdgeLabel> {
    vertices: Vec<Vertex<VertexLabel, EdgeLabel>>,
}

pub struct Vertex<VertexLabel, EdgeLabel> {
    label: VertexLabel,
    departing_edges: HashMap<VertexId, Vec<EdgeLabel>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VertexId {
    index: usize,
}

#[derive(Debug)]
pub enum Error {
    InvalidVertexId(VertexId),
    NoArcsTo(VertexId),
}

impl<VertexLabel, EdgeLabel> Graph<VertexLabel, EdgeLabel> {
    pub fn new() -> Graph<VertexLabel, EdgeLabel> {
        Graph {
            vertices: Vec::new(),
        }
    }

    pub fn create_vertex(&mut self, label: VertexLabel) -> VertexId {
        let index = self.vertices.len();
        let vertex = Vertex {
            label,
            departing_edges: HashMap::new(),
        };
        self.vertices.push(vertex);

        VertexId { index }
    }

    pub fn vertex_label(&self, vertex: VertexId) -> Result<&VertexLabel, Error> {
        self.get_vertex(vertex).map(|v| &v.label)
    }

    pub fn vertex_label_mut(&mut self, vertex: VertexId) -> Result<&mut VertexLabel, Error> {
        self.get_vertex_mut(vertex).map(|v| &mut v.label)
    }

    pub fn create_edge(
        &mut self,
        start: VertexId,
        end: VertexId,
        label: EdgeLabel,
    ) -> Result<(), Error> {
        let start = self.get_vertex_mut(start)?;
        start.add_edge_to(end, label)
    }

    pub fn get_vertex(&self, id: VertexId) -> Result<&Vertex<VertexLabel, EdgeLabel>, Error> {
        self.vertices
            .get(id.index)
            .ok_or(Error::InvalidVertexId(id))
    }

    pub fn get_vertex_mut(
        &mut self,
        id: VertexId,
    ) -> Result<&mut Vertex<VertexLabel, EdgeLabel>, Error> {
        self.vertices
            .get_mut(id.index)
            .ok_or(Error::InvalidVertexId(id))
    }

    pub fn reachable_from(&self, id: VertexId) -> Result<Vec<VertexId>, Error> {
        self.get_vertex(id).map(|v| v.reachable_from())
    }

    pub fn arcs_between(&self, from: VertexId, to: VertexId) -> Result<&Vec<EdgeLabel>, Error> {
        self.get_vertex(from).and_then(|v| v.arcs_to(to))
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

impl<VertexLabel, EdgeLabel: PartialEq> Graph<VertexLabel, EdgeLabel> {
    pub fn reachable_through(
        &self,
        id: VertexId,
        label: &EdgeLabel,
    ) -> Result<Vec<VertexId>, Error> {
        self.get_vertex(id).map(|v| v.reachable_through(label))
    }
}

impl<VertexLabel, EdgeLabel> Vertex<VertexLabel, EdgeLabel> {
    fn add_edge_to(&mut self, end: VertexId, label: EdgeLabel) -> Result<(), Error> {
        let vector = self
            .departing_edges
            .entry(end)
            .or_insert_with(|| Vec::new());
        vector.push(label);
        Ok(())
    }

    fn reachable_from(&self) -> Vec<VertexId> {
        self.departing_edges.keys().copied().collect()
    }

    fn arcs_to(&self, to: VertexId) -> Result<&Vec<EdgeLabel>, Error> {
        self.departing_edges.get(&to).ok_or(Error::NoArcsTo(to))
    }
}

impl<VertexLabel, EdgeLabel: PartialEq> Vertex<VertexLabel, EdgeLabel> {
    fn reachable_through(&self, label: &EdgeLabel) -> Vec<VertexId> {
        self.departing_edges
            .iter()
            .filter_map(|(&id, edges)| {
                if edges.contains(&label) {
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

        graph.create_edge(v1, v2, ());
        graph.create_edge(v1, v3, ());
        graph.create_edge(v1, v4, ());
        graph.create_edge(v3, v3, ());
        graph.create_edge(v4, v1, ());
        graph.create_edge(v4, v2, ());

        assert_same_elements!(vec![v2, v3, v4], graph.reachable_from(v1).unwrap());
        assert_same_elements!(vec![], graph.reachable_from(v2).unwrap());
        assert_same_elements!(vec![v3], graph.reachable_from(v3).unwrap());
        assert_same_elements!(vec![v1, v2], graph.reachable_from(v4).unwrap());
    }

    #[rstest]
    fn reachable_by() {
        let mut graph: Graph<i32, char> = Graph::new();

        let v1 = graph.create_vertex(1);
        let v2 = graph.create_vertex(2);
        let v3 = graph.create_vertex(3);
        let v4 = graph.create_vertex(4);

        graph.create_edge(v1, v1, 'a');
        graph.create_edge(v1, v2, 'a');
        graph.create_edge(v1, v2, 'b');
        graph.create_edge(v1, v3, 'b');
        graph.create_edge(v1, v2, 'c');
        graph.create_edge(v1, v3, 'c');
        graph.create_edge(v1, v4, 'c');

        assert_same_elements!(vec![v1, v2], graph.reachable_through(v1, &'a').unwrap());
        assert_same_elements!(vec![v2, v3], graph.reachable_through(v1, &'b').unwrap());
        assert_same_elements!(vec![v2, v3], graph.reachable_through(v1, &'b').unwrap());
        assert_same_elements!(vec![v2, v3, v4], graph.reachable_through(v1, &'c').unwrap());
    }
}
