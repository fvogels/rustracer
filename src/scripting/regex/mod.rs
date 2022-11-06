mod nfa;

enum Regex {
    Epsilon,
    Literal(char),
    Sequence(Vec<Box<Regex>>),
    // Alternatives(Vec<Box<Regex>>),
    // Kleene(Box<Regex>),
}
