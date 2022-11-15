use std::rc::Rc;

use super::{defs::RegularExpression, Regex};

pub fn literal(ch: char) -> Regex {
    wrap(RegularExpression::Literal(ch))
}

pub fn literal_seq(children: impl Iterator<Item = char>) -> Regex {
    sequence(children.map(|c| literal(c)))
}

pub fn kleene(child: Regex) -> Regex {
    wrap(RegularExpression::Kleene(child.regex.clone()))
}

pub fn sequence(children: impl Iterator<Item = Regex>) -> Regex {
    let unwrapped_children = children.map(|r| r.regex.clone()).collect();
    wrap(RegularExpression::Sequence(unwrapped_children))
}

pub fn alternatives(children: impl Iterator<Item = Regex>) -> Regex {
    let unwrapped_children = children.map(|r| r.regex.clone()).collect();
    wrap(RegularExpression::Alternatives(unwrapped_children))
}

pub fn wrap(regex: RegularExpression<char>) -> Regex {
    Regex {
        regex: Rc::new(regex),
    }
}

pub fn character_class(chars: impl Iterator<Item = char>) -> Regex {
    alternatives(chars.map(|ch| literal(ch)))
}

pub fn one_or_more(child: Regex) -> Regex {
    sequence([child.clone(), kleene(child)].into_iter())
}

pub fn digit() -> Regex {
    character_class("0123456789".chars())
}

pub fn positive_integer() -> Regex {
    one_or_more(digit())
}

pub fn empty() -> Regex {
    wrap(RegularExpression::Epsilon)
}

pub fn optional(child: Regex) -> Regex {
    alternatives([empty(), child].into_iter())
}

pub fn integer() -> Regex {
    sequence([optional(literal('-')), positive_integer()].into_iter())
}
