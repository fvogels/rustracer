use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{
    data::{
        graph::{Graph, VertexId},
        graphwalker::GraphWalker,
    },
    util::tag::Tag,
};

use super::{
    defs::{EdgeLabel, VertexLabel, DFA, NFA},
    nfa::NFAWalker,
};

struct DFABuilder<V, E: Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> {
    walker: NFAWalker<V, E, NFA>,
    dfa: Graph<Option<V>, E, DFA>,
    mapping: HashMap<Vec<VertexId<NFA>>, VertexId<DFA>>,
    start: VertexId<DFA>,
}

impl<V: Copy + Clone, E: Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> DFABuilder<V, E, NFA, DFA> {
    fn new(nfa: Graph<VertexLabel<V>, EdgeLabel<E>, NFA>, start_vertex: VertexId<NFA>) -> Self {
        let walker = NFAWalker::new(nfa, start_vertex);
        let mut dfa = Graph::new();
        let nfa_start = walker.active_positions();
        let dfa_start = dfa.create_vertex(None);
        let mut mapping = HashMap::new();
        mapping.insert(Self::canonical_vertices(nfa_start), dfa_start);

        DFABuilder {
            walker,
            dfa,
            mapping,
            start: dfa_start,
        }
    }

    fn canonical_vertices(nfa_vertices: &HashSet<VertexId<NFA>>) -> Vec<VertexId<NFA>> {
        let mut sorted_vertices: Vec<_> = nfa_vertices.iter().copied().collect();
        sorted_vertices.sort();

        sorted_vertices
    }

    fn map_to_dfa_vertex(
        &mut self,
        nfa_vertices: &HashSet<VertexId<NFA>>,
    ) -> (VertexId<DFA>, bool) {
        let mut sorted_vertices: Vec<_> = Self::canonical_vertices(nfa_vertices);

        if let Some(dfa_vertex) = self.mapping.get(&sorted_vertices) {
            (*dfa_vertex, false)
        } else {
            let dfa_vertex = self.dfa.create_vertex(None);
            self.mapping.insert(sorted_vertices, dfa_vertex);
            (dfa_vertex, true)
        }
    }

    fn walk(&mut self, ch: E) {
        self.walker.walk(ch);
    }

    fn convert(&mut self) {
        let mut queue = vec![self.walker.active_positions().clone()];

        while let Some(nfa_departure_vertices) = queue.pop() {
            let (dfa_departure_vertex, _) = self.map_to_dfa_vertex(&nfa_departure_vertices);

            self.walker.set_active_positions(&nfa_departure_vertices);

            match self.walker.priority_terminal_label() {
                Some(label) => {
                    *self
                        .dfa
                        .vertex_label_mut(dfa_departure_vertex)
                        .expect("Bug") = Some(label.clone());
                }
                None => ()
            }

            for edge_label in self.walker.departing_arcs() {
                match edge_label {
                    EdgeLabel::Char(ch) => {
                        self.walk(ch);

                        let nfa_arrival_vertices = self.walker.active_positions().clone();
                        let (dfa_arrival_vertex, is_new) =
                            self.map_to_dfa_vertex(&nfa_arrival_vertices);
                        self.dfa
                            .create_edge(dfa_departure_vertex, dfa_arrival_vertex, ch)
                            .expect("Bug");

                        if is_new {
                            queue.push(nfa_arrival_vertices);
                        }

                        self.walker.set_active_positions(&nfa_departure_vertices);
                    }
                    EdgeLabel::Epsilon => {}
                }
            }
        }
    }

    fn eject(self) -> (Graph<Option<V>, E, DFA>, VertexId<DFA>) {
        (self.dfa, self.start)
    }
}

pub fn nfa_to_dfa<V: Copy + Clone, E: Hash + Eq + Copy + Clone>(
    nfa: Graph<VertexLabel<V>, EdgeLabel<E>, NFA>,
    start_vertex: VertexId<NFA>,
) -> (Graph<Option<V>, E, DFA>, VertexId<DFA>) {
    let mut converter = DFABuilder::new(nfa, start_vertex);
    converter.convert();
    converter.eject()
}

pub struct DFAWalker<V, E: Hash + Eq + Copy + Clone, T: Tag = ()> {
    walker: GraphWalker<Option<V>, E, T>,
}

impl<V, E: Hash + Eq + Copy + Clone, T: Tag> DFAWalker<V, E, T> {
    pub fn new(graph: Graph<Option<V>, E, T>, start_vertex: VertexId<T>) -> Self {
        DFAWalker {
            walker: GraphWalker::new(graph, start_vertex),
        }
    }

    pub fn walk(&mut self, ch: E) -> bool {
        if self.walker.walk(&|lbl| *lbl == ch) {
            debug_assert_eq!(1, self.walker.active_positions.len());
            true
        } else {
            false
        }
    }

    pub fn active_vertex_label(&self) -> Option<&V> {
        let labels = self.walker.active_vertex_labels();
        debug_assert_eq!(1, labels.len());
        labels[0].as_ref()
    }

    pub fn set_active_position(&mut self, position: VertexId<T>) {
        self.walker.set_active_positions(&HashSet::from([position]));
    }

    pub fn active_position(&self) -> VertexId<T> {
        *self.walker.active_positions.iter().next().expect("Bug")
    }

    pub fn departing_arcs(&self) -> HashSet<E> {
        self.walker.departing_arcs()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rstest::rstest;

    use crate::{
        assert_same_elements,
        scripting::regex::{defs::RegularExpression, nfa::NFABuilder},
    };

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn literal() {
        fn create_dfa() -> (Graph<Option<i32>, char, DFA>, VertexId<DFA>) {
            let mut nfa_builder = NFABuilder::new();
            nfa_builder.add(&RegularExpression::Literal('a'), 1);
            nfa_builder.add(&RegularExpression::Literal('b'), 2);
            let (mut nfa, start_vertex) = nfa_builder.eject();
            nfa_to_dfa(nfa, start_vertex)
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);
            assert!(walker.walk('a'));
            assert_eq!(Some(&1), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);
            assert!(walker.walk('b'));
            assert_eq!(Some(&2), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);
            assert!(!walker.walk('c'));
        }
    }

    #[rstest]
    fn sequence() {
        fn create_dfa() -> (Graph<Option<i32>, char, DFA>, VertexId<DFA>) {
            let mut nfa_builder = NFABuilder::new();
            let regex = RegularExpression::Sequence(vec![
                Rc::new(RegularExpression::Literal('a')),
                Rc::new(RegularExpression::Literal('b')),
            ]);
            nfa_builder.add(&regex, 1);
            let regex = RegularExpression::Sequence(vec![
                Rc::new(RegularExpression::Literal('a')),
                Rc::new(RegularExpression::Literal('c')),
            ]);
            nfa_builder.add(&regex, 2);
            let regex = RegularExpression::Sequence(vec![
                Rc::new(RegularExpression::Literal('x')),
                Rc::new(RegularExpression::Literal('y')),
            ]);
            nfa_builder.add(&regex, 3);
            let (mut nfa, start_vertex) = nfa_builder.eject();
            nfa_to_dfa(nfa, start_vertex)
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('a'));
            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('b'));
            assert_eq!(Some(&1), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('a'));
            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('c'));
            assert_eq!(Some(&2), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('x'));
            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('y'));
            assert_eq!(Some(&3), walker.active_vertex_label());
        }
    }

    #[rstest]
    fn alternatives() {
        fn create_dfa() -> (Graph<Option<i32>, char, DFA>, VertexId<DFA>) {
            let mut nfa_builder = NFABuilder::new();
            let regex = RegularExpression::Alternatives(vec![
                Rc::new(RegularExpression::Literal('a')),
                Rc::new(RegularExpression::Literal('b')),
            ]);
            nfa_builder.add(&regex, 1);
            let regex = RegularExpression::Alternatives(vec![
                Rc::new(RegularExpression::Literal('x')),
                Rc::new(RegularExpression::Literal('y')),
            ]);
            nfa_builder.add(&regex, 2);
            let (mut nfa, start_vertex) = nfa_builder.eject();
            nfa_to_dfa(nfa, start_vertex)
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('a'));
            assert_eq!(Some(&1), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('b'));
            assert_eq!(Some(&1), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('x'));
            assert_eq!(Some(&2), walker.active_vertex_label());
        }

        {
            let (dfa, start) = create_dfa();
            let mut walker = DFAWalker::new(dfa, start);

            assert_eq!(None, walker.active_vertex_label());
            assert!(walker.walk('y'));
            assert_eq!(Some(&2), walker.active_vertex_label());
        }
    }

    #[rstest]
    fn kleene() {
        let mut nfa_builder = NFABuilder::new();
        let regex = RegularExpression::Kleene(Rc::new(RegularExpression::Literal('a')));
        nfa_builder.add(&regex, 1);
        let (mut nfa, start_vertex) = nfa_builder.eject();
        let (dfa, start) = nfa_to_dfa(nfa, start_vertex);

        let mut walker = DFAWalker::new(dfa, start);

        assert_eq!(Some(&1), walker.active_vertex_label());
        assert!(walker.walk('a'));
        assert_eq!(Some(&1), walker.active_vertex_label());
        assert!(walker.walk('a'));
        assert_eq!(Some(&1), walker.active_vertex_label());
        assert!(walker.walk('a'));
        assert_eq!(Some(&1), walker.active_vertex_label());
        assert!(walker.walk('a'));
    }
}
