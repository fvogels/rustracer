use std::{collections::{HashSet, HashMap}, hash::Hash};

use crate::{data::{graph::{Graph, VertexId}, graphwalker::GraphWalker}, util::tag::Tag};

use super::{EdgeLabel, VertexLabel, NFA, DFA};


struct Converter<'a, V, E: Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> {
    walker: GraphWalker<'a, V, E, NFA>,
    dfa: Graph<VertexLabel<V>, E, DFA>,
    mapping: HashMap<Vec<VertexId<NFA>>, VertexId<DFA>>,
}

impl<'a, V, E: Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> Converter<'a, V, E, NFA, DFA> {
    fn new(nfa: &'a Graph<VertexLabel<V>, EdgeLabel<E>, NFA>, start_vertex: VertexId<NFA>) -> Self {
        Converter {
            walker: GraphWalker::new(nfa, start_vertex),
            dfa: Graph::new(),
            mapping: HashMap::new(),
        }
    }

    fn map_to_dfa_vertex(&mut self, nfa_vertices: &HashSet<VertexId<NFA>>) -> (VertexId<DFA>, bool) {
        let mut sorted_vertices: Vec<_> = nfa_vertices.iter().copied().collect();
        sorted_vertices.sort();

        if let Some(dfa_vertex) = self.mapping.get(&sorted_vertices) {
            (*dfa_vertex, false)
        } else {
            let dfa_vertex = self.dfa.create_vertex(VertexLabel::NonTerminal);
            self.mapping.insert(sorted_vertices, dfa_vertex);
            (dfa_vertex, true)
        }
    }

    fn walk(&mut self, ch: E) {
        self.walker.walk(|lbl| *lbl == ch);
    }

    fn convert(&mut self) {
        let mut queue = vec![self.walker.active_positions.clone()];

        while let Some(nfa_departure_vertices) = queue.pop() {
            let (dfa_departure_vertex, _) = self.map_to_dfa_vertex(&nfa_departure_vertices);

            self.walker.set_active_positions(&nfa_departure_vertices);

            for edge_label in self.walker.departing_arcs() {
                match edge_label {
                    EdgeLabel::Char(ch) => {
                        self.walk(ch);

                        let nfa_arrival_vertices = self.walker.active_positions.clone();
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
}

fn nfa_to_dfa<V, E: Hash + Eq + Copy + Clone>(nfa: &Graph<VertexLabel<V>, EdgeLabel<E>, NFA>, start_vertex: VertexId<NFA>) -> Graph<VertexLabel<V>, E, DFA> {
    let mut converter = Converter::new(nfa, start_vertex);
    converter.convert();
    converter.dfa
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
        let dfa = nfa_to_dfa(&nfa, start_vertex);

        // let mut walker = GraphWalker::new(&nfa, start_vertex);
        // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        // walker.follow(|lbl| *lbl == EdgeLabel::Char('a'));
        // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        // assert_same_elements!(vec![&Some(1)], walker.active_vertex_labels());

        // walker.reset();
        // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        // walker.follow(|lbl| *lbl == EdgeLabel::Char('b'));
        // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
        // assert_same_elements!(vec![&Some(2)], walker.active_vertex_labels());
    }
}
