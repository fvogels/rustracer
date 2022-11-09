use std::rc::Rc;

mod nfa;
mod dfa;
mod defs;
mod helpers;
mod automaton;

pub use helpers::*;
pub use automaton::*;

#[derive(Debug, Clone)]
pub struct Regex {
    regex: Rc<defs::RegularExpression<char>>,
}
