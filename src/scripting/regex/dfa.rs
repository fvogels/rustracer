use std::{collections::{HashSet, HashMap}, hash::Hash};

use crate::{data::{graph::{Graph, VertexId}, graphwalker::GraphWalker}, util::tag::Tag};

use super::{EdgeLabel, VertexLabel, NFA, DFA, nfa::NFAWalker};


struct Converter<'a, V, E: Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> {
    walker: NFAWalker<'a, V, E, NFA>,
    dfa: Graph<VertexLabel<V>, E, DFA>,
    mapping: HashMap<Vec<VertexId<NFA>>, VertexId<DFA>>,
    start: VertexId<DFA>,
}

impl<'a, V: Copy + Clone, E: Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> Converter<'a, V, E, NFA, DFA> {
    fn new(nfa: &'a Graph<VertexLabel<V>, EdgeLabel<E>, NFA>, start_vertex: VertexId<NFA>) -> Self {
        let walker = NFAWalker::new(nfa, start_vertex);
        let mut dfa = Graph::new();
        let nfa_start = walker.active_positions();
        let dfa_start = dfa.create_vertex(VertexLabel::NonTerminal);
        let mut mapping = HashMap::new();
        mapping.insert(Self::canonical_vertices(nfa_start), dfa_start);

        Converter {
            walker,
            dfa,
            mapping,
            start: dfa_start
        }
    }

    fn canonical_vertices(nfa_vertices: &HashSet<VertexId<NFA>>) -> Vec<VertexId<NFA>> {
        let mut sorted_vertices: Vec<_> = nfa_vertices.iter().copied().collect();
        sorted_vertices.sort();

        sorted_vertices
    }

    fn map_to_dfa_vertex(&mut self, nfa_vertices: &HashSet<VertexId<NFA>>) -> (VertexId<DFA>, bool) {
        let mut sorted_vertices: Vec<_> = Self::canonical_vertices(nfa_vertices);

        if let Some(dfa_vertex) = self.mapping.get(&sorted_vertices) {
            (*dfa_vertex, false)
        } else {
            let dfa_vertex = self.dfa.create_vertex(VertexLabel::NonTerminal);
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

            let labels: Vec<_> = self.walker.active_vertex_labels().iter().filter_map(|&lbl| {
                match lbl {
                    VertexLabel::NonTerminal => None,
                    VertexLabel::Terminal(_) => Some(lbl)
                }
            }).collect();

            if labels.len() != 0 {
                assert_eq!(1, labels.len());
                let label = labels[0];
                let r = self.dfa.vertex_label_mut(dfa_departure_vertex).expect("Bug");
                *self.dfa.vertex_label_mut(dfa_departure_vertex).expect("Bug") = label.clone();
            }

            for edge_label in self.walker.departing_arcs() {
                match edge_label {
                    EdgeLabel::Char(ch) => {
                        self.walk(ch);

                        let nfa_arrival_vertices = self.walker.active_positions().clone();
                        let (dfa_arrival_vertex, is_new) = self.map_to_dfa_vertex(&nfa_arrival_vertices);
                        self.dfa.create_edge(dfa_departure_vertex, dfa_arrival_vertex, ch).expect("Bug");

                        if is_new {
                            queue.push(nfa_arrival_vertices);
                        }

                        self.walker.set_active_positions(&nfa_departure_vertices);
                    },
                    EdgeLabel::Epsilon => { }
                }
            }
        }
    }

    pub fn eject(&mut self) -> (Graph<VertexLabel<V>, E, DFA>, VertexId<DFA>) {
        let dfa = std::mem::replace(&mut self.dfa, Graph::new());

        (dfa, self.start)
    }
}

fn nfa_to_dfa<V: Copy + Clone, E: Hash + Eq + Copy + Clone>(nfa: &Graph<VertexLabel<V>, EdgeLabel<E>, NFA>, start_vertex: VertexId<NFA>) -> (Graph<VertexLabel<V>, E, DFA>, VertexId<DFA>) {
    let mut converter = Converter::new(nfa, start_vertex);
    converter.convert();
    converter.eject()
}

pub struct DFAWalker<'a, V, E: Hash + Eq + Copy + Clone, T: Tag = ()> {
    walker: GraphWalker<'a, VertexLabel<V>, E, T>,
}

impl<'a, V, E: Hash + Eq + Copy + Clone, T: Tag> DFAWalker<'a, V, E, T> {
    pub fn new(graph: &'a Graph<VertexLabel<V>, E, T>, start_vertex: VertexId<T>) -> Self {
        DFAWalker { walker: GraphWalker::new(graph, start_vertex) }
    }

    pub fn walk(&mut self, ch: E) -> bool {
        if self.walker.walk(&|lbl| *lbl == ch) {
            debug_assert_eq!(1, self.walker.active_positions.len());
            true
        } else {
            false
        }
    }

    pub fn active_vertex_label(&self) -> &VertexLabel<V> {
        let labels = self.walker.active_vertex_labels();
        debug_assert_eq!(1, labels.len());
        labels[0]
    }

    pub fn set_active_position(&mut self, position: VertexId<T>) {
        self.walker.set_active_positions(&HashSet::from([position]));
    }

    pub fn active_positions(&self) -> &HashSet<VertexId<T>> {
        &self.walker.active_positions
    }

    pub fn departing_arcs(&self) -> HashSet<E> {
        self.walker.departing_arcs()
    }
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{assert_same_elements, scripting::regex::{Regex, nfa::NFABuilder}};

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn dfa_literal() {
        let mut nfa_builder = NFABuilder::new();
        nfa_builder.add(&Regex::Literal('a'), 1);
        nfa_builder.add(&Regex::Literal('b'), 2);
        let (mut nfa, start_vertex) = nfa_builder.eject();
        let (dfa, start) = nfa_to_dfa(&nfa, start_vertex);

        {
            let mut walker = DFAWalker::new(&dfa, start);
            assert!(walker.walk('a'));
            assert_eq!(VertexLabel::Terminal(1), *walker.active_vertex_label());
        }

        {
            let mut walker = DFAWalker::new(&dfa, start);
            assert!(walker.walk('b'));
            assert_eq!(VertexLabel::Terminal(2), *walker.active_vertex_label());
        }

        {
            let mut walker = DFAWalker::new(&dfa, start);
            assert!(!walker.walk('c'));
        }
    }

    #[rstest]
    fn dfa_sequence() {
        let mut nfa_builder = NFABuilder::new();
        let regex = Regex::Sequence(vec![ Box::new(Regex::Literal('a')), Box::new(Regex::Literal('b')) ]);
        nfa_builder.add(&regex, 1);
        let regex = Regex::Sequence(vec![ Box::new(Regex::Literal('a')), Box::new(Regex::Literal('c')) ]);
        nfa_builder.add(&regex, 2);
        let regex = Regex::Sequence(vec![ Box::new(Regex::Literal('x')), Box::new(Regex::Literal('y')) ]);
        nfa_builder.add(&regex, 3);
        let (mut nfa, start_vertex) = nfa_builder.eject();
        let (dfa, start) = nfa_to_dfa(&nfa, start_vertex);

        {
            let mut walker = DFAWalker::new(&dfa, start);

            assert_eq!(VertexLabel::NonTerminal, *walker.active_vertex_label());
            assert!(walker.walk('a'));
            assert_eq!(VertexLabel::NonTerminal, *walker.active_vertex_label());
            assert!(walker.walk('b'));
            assert_eq!(VertexLabel::Terminal(1), *walker.active_vertex_label());
        }

        {
            let mut walker = DFAWalker::new(&dfa, start);

            assert_eq!(VertexLabel::NonTerminal, *walker.active_vertex_label());
            assert!(walker.walk('a'));
            assert_eq!(VertexLabel::NonTerminal, *walker.active_vertex_label());
            assert!(walker.walk('c'));
            assert_eq!(VertexLabel::Terminal(2), *walker.active_vertex_label());
        }

        {
            let mut walker = DFAWalker::new(&dfa, start);

            assert_eq!(VertexLabel::NonTerminal, *walker.active_vertex_label());
            assert!(walker.walk('x'));
            assert_eq!(VertexLabel::NonTerminal, *walker.active_vertex_label());
            assert!(walker.walk('y'));
            assert_eq!(VertexLabel::Terminal(3), *walker.active_vertex_label());
        }
    }
}
