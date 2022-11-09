pub enum Regex<T> {
    Epsilon,
    Literal(T),
    Sequence(Vec<Box<Regex<T>>>),
    Alternatives(Vec<Box<Regex<T>>>),
    Kleene(Box<Regex<T>>),
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
