use std::rc::Rc;

use super::{defs::RegularExpression, Regex};

pub fn literal(ch: char) -> Regex {
    wrap(RegularExpression::Literal(ch))
}

pub fn literal_seq(children: impl Iterator<Item=char>) -> Regex {
    sequence(children.map(|c| literal(c)))
}

pub fn kleene(child: Regex) -> Regex {
    wrap(RegularExpression::Kleene(child.regex.clone()))
}

pub fn sequence(children: impl Iterator<Item=Regex>) -> Regex {
    let unwrapped_children = children.map(|r| r.regex.clone()).collect();
    wrap(RegularExpression::Sequence(unwrapped_children))
}

pub fn alternatives(children: impl Iterator<Item=Regex>) -> Regex {
    let unwrapped_children = children.map(|r| r.regex.clone()).collect();
    wrap(RegularExpression::Alternatives(unwrapped_children))
}

fn wrap(regex: RegularExpression<char>) -> Regex {
    Regex { regex: Rc::new(regex) }
}

fn character_class(chars: impl Iterator<Item=char>) -> Regex {
    alternatives(chars.map(|ch| literal(ch)))
}
