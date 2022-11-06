// use std::{collections::{HashSet, HashMap}, hash::Hash};

// use crate::{data::graph::{Graph, VertexId}, util::tag::{Tag, define_tag}};

// define_tag!(NFA);
// define_tag!(DFA);

// pub struct Tokenizer {}

// #[derive(Debug, PartialEq, Clone)]
// pub enum TokenType {
//     LeftParenthesis,
//     RightParenthesis,
//     Identifier(String),
//     Integer(i64),
//     FloatingPointNumber(f64),
// }

// enum Regex {
//     Epsilon,
//     Literal(char),
//     Sequence(Vec<Box<Regex>>),
//     // Alternatives(Vec<Box<Regex>>),
//     // Kleene(Box<Regex>),
// }

// #[derive(Debug, PartialEq)]
// enum VertexLabel {
//     NonTerminal,
//     Terminal(TokenType)
// }

// #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
// enum EdgeLabel {
//     Epsilon,
//     Char(char),
// }

// type NFAGraph = Graph<VertexLabel, EdgeLabel, NFA>;
// type DFAGraph = Graph<VertexLabel, EdgeLabel, DFA>;

// struct Converter<'a> {
//     walker: GraphWalker<'a, VertexLabel, EdgeLabel, NFA>,
//     dfa: DFAGraph,
//     mapping: HashMap<Vec<VertexId<NFA>>, VertexId<DFA>>,
// }

// impl<'a> Converter<'a> {
//     fn new(nfa: &NFAGraph, start_vertex: VertexId<NFA>) -> Self {
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
//             let dfa_vertex = self.dfa.create_vertex(VertexLabel::NonTerminal);
//             self.mapping.insert(sorted_vertices, dfa_vertex);
//             (dfa_vertex, true)
//         }
//     }

//     fn walk_epsilons(&mut self) {
//         self.walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//     }

//     fn walk(&mut self, ch: char) {
//         self.walker.follow(|lbl| *lbl == EdgeLabel::Char(ch));
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
//     fn graphwalker_follow() {
//         fn ps(walker: &GraphWalker<(), char>) -> Vec<VertexId> {
//             walker.active_positions.iter().copied().collect()
//         }

//         let mut graph = Graph::new();

//         let v1 = graph.create_vertex(());
//         let v2 = graph.create_vertex(());
//         let v3 = graph.create_vertex(());
//         let v4 = graph.create_vertex(());

//         for (s, e, c) in vec![
//             (v1, v2, 'a'),
//             (v1, v2, 'b'),
//             (v1, v3, 'b'),
//             (v2, v1, 'c'),
//             (v2, v2, 'd'),
//             (v2, v3, 'a'),
//             (v2, v4, 'b'),
//             (v3, v1, 'c'),
//             (v3, v3, 'b'),
//             (v3, v4, 'b'),
//             (v4, v1, 'c'),
//         ] {
//             graph.create_edge(s, e, c);
//         }

//         let mut walker = GraphWalker::new(&graph, v1);
//         assert_same_elements!(vec![v1], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'a'));
//         assert_same_elements!(vec![v2], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'd'));
//         assert_same_elements!(vec![v2], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'a'));
//         assert_same_elements!(vec![v3], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'b'));
//         assert_same_elements!(vec![v3, v4], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'b'));
//         assert_same_elements!(vec![v3, v4], ps(&walker));

//         assert!(!walker.follow(|lbl| *lbl == 'x'));
//         assert_same_elements!(vec![v3, v4], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'c'));
//         assert_same_elements!(vec![v1], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'b'));
//         assert_same_elements!(vec![v2, v3], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'b'));
//         assert_same_elements!(vec![v3, v4], ps(&walker));
//     }

//     #[rstest]
//     fn graphwalker_follow_transitively() {
//         fn ps(walker: &GraphWalker<(), char>) -> Vec<VertexId> {
//             walker.active_positions.iter().copied().collect()
//         }

//         let mut graph = Graph::new();

//         let v1 = graph.create_vertex(());
//         let v2 = graph.create_vertex(());
//         let v3 = graph.create_vertex(());
//         let v4 = graph.create_vertex(());
//         let v5 = graph.create_vertex(());

//         for (s, e, c) in vec![
//             (v1, v1, 'a'),
//             (v1, v2, 'a'),
//             (v2, v3, 'a'),
//             (v2, v5, 'a'),
//             (v3, v3, 'c'),
//             (v3, v4, 'a'),
//             (v5, v1, 'b'),
//         ] {
//             graph.create_edge(s, e, c).unwrap();
//         }

//         let mut walker = GraphWalker::new(&graph, v1);
//         assert_same_elements!(vec![v1], ps(&walker));

//         walker.follow_transitively(|lbl| *lbl == 'a');
//         assert_same_elements!(vec![v1, v2, v3, v4, v5], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'b'));
//         assert_same_elements!(vec![v1], ps(&walker));

//         walker.follow_transitively(|lbl| *lbl == 'c');
//         assert_same_elements!(vec![v1], ps(&walker));

//         walker.follow_transitively(|lbl| *lbl == 'a');
//         assert_same_elements!(vec![v1, v2, v3, v4, v5], ps(&walker));

//         assert!(walker.follow(|lbl| *lbl == 'c'));
//         assert_same_elements!(vec![v3], ps(&walker));

//         walker.follow_transitively(|lbl| *lbl == 'a');
//         assert_same_elements!(vec![v3, v4], ps(&walker));
//     }

//     #[rstest]
//     fn nfa_literal() {
//         let mut nfa: Graph<_, _, ()> = Graph::new();
//         let start_vertex = nfa.create_vertex(None);
//         add_to_nfa_graph(&mut nfa, &Regex::Literal('a'), start_vertex, None, Some(1));
//         add_to_nfa_graph(&mut nfa, &Regex::Literal('b'), start_vertex, None, Some(2));

//         let mut walker = GraphWalker::new(&nfa, start_vertex);
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         walker.follow(|lbl| *lbl == EdgeLabel::Char('a'));
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         assert_same_elements!(vec![&Some(1)], walker.active_vertex_labels());

//         walker.reset();
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         walker.follow(|lbl| *lbl == EdgeLabel::Char('b'));
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         assert_same_elements!(vec![&Some(2)], walker.active_vertex_labels());
//     }

//     #[rstest]
//     fn nfa_sequence() {
//         let mut nfa: Graph<_, _, ()> = Graph::new();
//         let start_vertex = nfa.create_vertex(None);
//         let regex = Regex::Sequence(vec![Box::new(Regex::Literal('a')), Box::new(Regex::Literal('b')), Box::new(Regex::Literal('c'))]);
//         add_to_nfa_graph(&mut nfa, &regex, start_vertex, None, Some(1));

//         let mut walker = GraphWalker::new(&nfa, start_vertex);
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         walker.follow(|lbl| *lbl == EdgeLabel::Char('a'));
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         walker.follow(|lbl| *lbl == EdgeLabel::Char('b'));
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         walker.follow(|lbl| *lbl == EdgeLabel::Char('c'));
//         walker.follow_transitively(|lbl| *lbl == EdgeLabel::Epsilon);
//         assert_same_elements!(vec![&Some(1)], walker.active_vertex_labels());
//     }

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
