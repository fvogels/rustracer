// use std::{collections::{HashSet, HashMap}, hash::Hash};

// use crate::{data::{graph::{Graph, VertexId}, graphwalker::GraphWalker}, util::tag::Tag};

// use super::nfa::{VertexLabel, EdgeLabel};


// struct Converter<'a, V: VertexLabel, E: EdgeLabel + Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> {
//     walker: GraphWalker<'a, V, E, NFA>,
//     dfa: Graph<V, E, DFA>,
//     mapping: HashMap<Vec<VertexId<NFA>>, VertexId<DFA>>,
// }

// impl<'a, V: VertexLabel, E: EdgeLabel + Hash + Eq + Copy + Clone, NFA: Tag, DFA: Tag> Converter<'a, V, E, NFA, DFA> {
//     fn new(nfa: &Graph<V, E, NFA>, start_vertex: VertexId<NFA>) -> Self {
//         Converter {
//             walker: GraphWalker::new(nfa, start_vertex),
//             dfa: Graph::new(),
//             mapping: HashMap::new(),
//         }
//     }

//     fn map_to_dfa_vertex(&mut self, nfa_vertices: &HashSet<VertexId<NFA>>) -> (VertexId<DFA>, bool) {
//         let mut sorted_vertices: Vec<_> = nfa_vertices.iter().copied().collect();
//         sorted_vertices.sort();

//         if let Some(dfa_vertex) = self.mapping.get(&sorted_vertices) {
//             (*dfa_vertex, false)
//         } else {
//             let dfa_vertex = self.dfa.create_vertex(V::nonterminal());
//             self.mapping.insert(sorted_vertices, dfa_vertex);
//             (dfa_vertex, true)
//         }
//     }

//     fn walk_epsilons(&mut self) {
//         self.walker.follow_transitively(|lbl| *lbl == E::epsilon());
//     }

//     fn walk(&mut self, ch: char) {
//         self.walker.follow(|lbl| *lbl == E::from_char(ch));
//         self.walk_epsilons();
//     }

//     fn convert(&mut self) {
//         self.walk_epsilons();

//         let mut queue = vec![self.walker.active_positions.clone()];

//         while let Some(nfa_departure_vertices) = queue.pop() {
//             let (dfa_departure_vertex, _) = self.map_to_dfa_vertex(&nfa_departure_vertices);

//             self.walker.set_active_positions(&nfa_departure_vertices);

//             for edge_label in self.walker.departing_arcs() {
//                 match edge_label {
//                     EdgeLabel::Char(ch) => {
//                         self.walk(ch);

//                         let nfa_arrival_vertices = self.walker.active_positions.clone();
//                         let (dfa_arrival_vertex, is_new) = self.map_to_dfa_vertex(&nfa_arrival_vertices);
//                         self.dfa.create_edge(dfa_departure_vertex, dfa_arrival_vertex, EdgeLabel::Char(ch)).expect("Bug");

//                         if is_new {
//                             queue.push(nfa_arrival_vertices);
//                         }

//                         self.walker.set_active_positions(&nfa_departure_vertices);
//                     },
//                     EdgeLabel::Epsilon => { }
//                 }
//             }
//         }
//     }
// }

// fn nfa_to_dfa(nfa: &NFAGraph, start_vertex: VertexId<NFA>) -> DFAGraph {
//     let mut converter = Converter::new(nfa, start_vertex);
//     converter.convert();
//     converter.dfa
// }



// // struct NFAWalker<'a> {
// //     walker: GraphWalker<'a, VertexLabel, EdgeLabel, NFA>,
// // }

// // impl<'a> NFAWalker<'a> {
// //     fn new(graph: &'a NFAGraph, start_vertex: VertexId<NFA>) -> NFAWalker<'a> {
// //         let walker = GraphWalker::new(graph, start_vertex);

// //         NFAWalker { walker }
// //     }

// //     fn walk(&mut self, ch: char) -> bool {
// //         if self.walker.follow(|lbl| *lbl == EdgeLabel::Char(ch)) {
// //             self.walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
// //             true
// //         } else {
// //             false
// //         }
// //     }

// //     fn active_label(&self) -> Option<VertexLabel> {

// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use rstest::rstest;

//     use crate::assert_same_elements;

//     #[cfg(test)]
//     use super::*;


//     #[rstest]
//     fn dfa_literal() {
//         let mut nfa: Graph<_, _, ()> = Graph::new();
//         let start_vertex = nfa.create_vertex(None);
//         add_to_nfa_graph(&mut nfa, &Regex::Literal('a'), start_vertex, None, Some(1));
//         add_to_nfa_graph(&mut nfa, &Regex::Literal('b'), start_vertex, None, Some(2));
//         let dfa = nfa_to_dfa(&nfa, start_vertex);

//         // let mut walker = GraphWalker::new(&nfa, start_vertex);
//         // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         // walker.follow(|lbl| *lbl == EdgeLabel::Char('a'));
//         // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         // assert_same_elements!(vec![&Some(1)], walker.active_vertex_labels());

//         // walker.reset();
//         // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         // walker.follow(|lbl| *lbl == EdgeLabel::Char('b'));
//         // walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         // assert_same_elements!(vec![&Some(2)], walker.active_vertex_labels());
//     }
// }
