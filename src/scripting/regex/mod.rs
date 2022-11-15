use std::rc::Rc;

mod automaton;
mod defs;
mod dfa;
mod helpers;
mod nfa;

pub use automaton::*;
pub use helpers::*;

#[derive(Debug, Clone)]
pub struct Regex {
    regex: Rc<defs::RegularExpression<char>>,
}
