use std::rc::Rc;

use crate::define_tag;

#[derive(Debug)]
pub enum RegularExpression<T> {
    Epsilon,
    Literal(T),
    Sequence(Vec<Rc<RegularExpression<T>>>),
    Alternatives(Vec<Rc<RegularExpression<T>>>),
    Kleene(Rc<RegularExpression<T>>),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum VertexLabel<T> {
    NonTerminal,
    Terminal(T),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum EdgeLabel<T> {
    Epsilon,
    Char(T),
}

define_tag!(NFA);
define_tag!(DFA);
