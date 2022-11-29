use std::rc::Rc;


enum RegexImp {
    Empty,
    Epsilon,
    Literal(Rc<dyn Fn(char) -> bool>),
    Sequence(Rc<RegexImp>, Rc<RegexImp>),
    Alternatives(Rc<RegexImp>, Rc<RegexImp>),
    Kleene(Rc<RegexImp>),
}

#[derive(Clone, Debug)]
pub struct Regex {
    imp: Rc<RegexImp>
}

impl std::fmt::Debug for RegexImp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexImp::Empty => f.debug_tuple("Emp").finish(),
            RegexImp::Epsilon => f.debug_tuple("Eps").finish(),
            RegexImp::Literal(_) => f.debug_tuple("Lit").finish(),
            RegexImp::Sequence(a, b) => f.debug_tuple("Seq").field(a).field(b).finish(),
            RegexImp::Alternatives(a, b) => f.debug_tuple("Alt").field(a).field(b).finish(),
            RegexImp::Kleene(a) => f.debug_tuple("Kleene").field(a).finish(),
        }
    }
}

impl RegexImp {
    fn is_epsilon(&self) -> bool {
        match self {
            Self::Epsilon => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Epsilon => true,
            Self::Literal(_) => false,
            Self::Sequence(x, y) => x.is_terminal() && y.is_terminal(),
            Self::Alternatives(x, y) => x.is_terminal() || y.is_terminal(),
            Self::Kleene(x) => true,
        }
    }

    fn feed(&self, ch: char) -> Rc<Self> {
        match self {
            Self::Empty => Self::empty(),
            Self::Epsilon => Self::empty(),
            Self::Literal(predicate) => {
                if predicate(ch) {
                    Self::epsilon()
                } else {
                    Self::empty()
                }
            },
            Self::Sequence(x, y) => {
                let mut result = Vec::new();

                result.push(Self::sequence(vec![x.feed(ch), y.clone()].into_iter()));

                if x.is_terminal() {
                    result.push(y.feed(ch));
                }

                Self::alternatives(result.into_iter())
            },
            Self::Alternatives(x, y) => {
                let mut result = vec![
                    x.feed(ch),
                    y.feed(ch),
                ];

                Self::alternatives(result.into_iter())
            },
            Self::Kleene(x) => {
                Self::sequence(vec![x.feed(ch), Self::kleene(x.clone())].into_iter())
            },
        }
    }

    fn empty() -> Rc<Self> {
        Rc::new(RegexImp::Empty)
    }

    fn epsilon() -> Rc<Self> {
        Rc::new(RegexImp::Epsilon)
    }

    fn sequence(children: impl Iterator<Item=Rc<Self>>) -> Rc<Self> {
        let mut result = Rc::new(RegexImp::Epsilon);

        for child in children.into_iter() {
            if child.is_empty() {
                return Self::empty();
            } else if result.is_epsilon() {
                result = child;
            } else if !child.is_epsilon() {
                result = Rc::new(RegexImp::Sequence(result, child));
            }
        }

        result
    }

    fn alternatives(children: impl Iterator<Item=Rc<Self>>) -> Rc<Self> {
        let mut result = Rc::new(RegexImp::Empty);
        let mut contains_epsilon = false;

        for child in children.into_iter() {
            if result.is_empty() {
                result = child
            } else {
                if child.is_epsilon() {
                    if !contains_epsilon {
                        contains_epsilon = true;
                        result = Rc::new(RegexImp::Alternatives(result, child))
                    }
                } else {
                    result = Rc::new(RegexImp::Alternatives(result, child))
                }
            }
        }

        result
    }

    fn kleene(child: Rc<Self>) -> Rc<Self> {
        if child.is_empty() || child.is_epsilon() {
            Self::epsilon()
        } else {
            Rc::new(RegexImp::Kleene(child))
        }
    }

    fn no_inner_empty(&self) -> bool {
        fn does_not_contain_empty(r: Rc<RegexImp>) -> bool {
            match r.as_ref() {
                RegexImp::Empty => false,
                RegexImp::Epsilon => true,
                RegexImp::Literal(_) => true,
                RegexImp::Alternatives(a, b) => does_not_contain_empty(a.clone()) && does_not_contain_empty(b.clone()),
                RegexImp::Sequence(a, b) => does_not_contain_empty(a.clone()) && does_not_contain_empty(b.clone()),
                RegexImp::Kleene(a) => does_not_contain_empty(a.clone()),
            }
        }

        match self {
            Self::Empty => true,
            Self::Epsilon => true,
            Self::Literal(_) => true,
            Self::Alternatives(a, b) => does_not_contain_empty(a.clone()) && does_not_contain_empty(b.clone()),
            Self::Sequence(a, b) => does_not_contain_empty(a.clone()) && does_not_contain_empty(b.clone()),
            Self::Kleene(a) => does_not_contain_empty(a.clone()),
        }
    }
}

impl Regex {
    fn new(imp: Rc<RegexImp>) -> Self {
        debug_assert!(imp.no_inner_empty());

        Regex { imp }
    }

    pub fn empty() -> Self {
        Regex::new(Rc::new(RegexImp::Empty))
    }

    pub fn epsilon() -> Self {
        Regex::new(Rc::new(RegexImp::Epsilon))
    }

    pub fn sequence(children: impl Iterator<Item=Self>) -> Self {
        Regex::new(RegexImp::sequence(Self::unwrap(children)))
    }

    pub fn alternatives(children: impl Iterator<Item=Self>) -> Self {
        Regex::new(RegexImp::alternatives(Self::unwrap(children)))
    }

    fn unwrap(children: impl Iterator<Item=Self>) -> impl Iterator<Item=Rc<RegexImp>> {
        children.map(|c| c.imp.clone())
    }

    pub fn kleene(child: Self) -> Self {
        if child.imp.is_empty() || child.imp.is_epsilon() {
            Self::epsilon()
        } else {
            Regex::new(Rc::new(RegexImp::Kleene(child.imp)))
        }
    }

    pub fn predicate(predicate: Rc<dyn Fn(char) -> bool>) -> Self {
        Regex::new(Rc::new(RegexImp::Literal(predicate)))
    }

    pub fn literal(ch: char) -> Self {
        let closure = Rc::new(move |x| x == ch);

        Self::predicate(closure)
    }

    pub fn optional(child: Regex) -> Self {
        Self::alternatives([Self::epsilon(), child].into_iter())
    }

    pub fn is_terminal(&self) -> bool {
        self.imp.is_terminal()
    }

    pub fn feed_mut(&mut self, ch: char) {
        self.imp = self.imp.feed(ch)
    }

    pub fn feed(&self, ch: char) -> Self {
        let imp = self.imp.feed(ch);

        Regex { imp }
    }

    pub fn try_feed(&self, ch: char) -> Option<Self> {
        let imp = self.imp.feed(ch);

        if imp.is_empty() {
            None
        } else {
            Some(Regex { imp })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.imp.is_empty()
    }

    pub fn character_class(chars: impl Iterator<Item=char>) -> Self {
        let regexes = chars.map(|c| Self::literal(c));

        Self::alternatives(regexes)
    }

    pub fn one_or_more(r: Regex) -> Regex {
        Self::sequence([r.clone(), Self::kleene(r)].into_iter())
    }

    pub fn lowercase_letter() -> Self {
        Self::predicate(Rc::new(move |ch: char| ch.is_lowercase()))
    }

    pub fn uppercase_letter() -> Self {
        Self::predicate(Rc::new(move |ch: char| ch.is_uppercase()))
    }

    pub fn letter() -> Self {
        Self::predicate(Rc::new(move |ch: char| ch.is_alphabetic()))
    }

    pub fn digit(radix: u32) -> Self {
        Self::predicate(Rc::new(move |ch: char| ch.is_digit(radix)))
    }

    pub fn alphanumeric() -> Self {
        Self::predicate(Rc::new(move |ch: char| ch.is_alphanumeric()))
    }

    pub fn positive_integer(radix: u32) -> Self {
        Self::one_or_more(Self::digit(radix))
    }

    pub fn integer(radix: u32) -> Self {
        Self::sequence([Self::optional(Self::literal('-')), Self::positive_integer(radix)].into_iter())
    }

    pub fn float() -> Self {
        Self::sequence([
            Self::integer(10),
            Self::literal('.'),
            Self::positive_integer(10),
        ].into_iter())
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    #[cfg(test)]
    use super::*;

    #[rstest]
    fn sequence_positive() {
        let mut regex = Regex::sequence([Regex::literal('a'), Regex::literal('b'), Regex::literal('c')].into_iter());

        assert!(!regex.is_terminal());
        assert!(!regex.is_empty());
        regex.feed_mut('a');

        assert!(!regex.is_terminal());
        assert!(!regex.is_empty());
        regex.feed_mut('b');

        assert!(!regex.is_terminal());
        assert!(!regex.is_empty());
        regex.feed_mut('c');

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());

        regex.feed_mut('a');
        assert!(regex.is_empty());
    }

    #[rstest]
    fn sequence_2_negative() {
        let mut regex = Regex::sequence([Regex::literal('a'), Regex::literal('b')].into_iter());

        assert!(!regex.is_terminal());
        assert!(!regex.is_empty());
        regex.feed_mut('b');
        assert!(regex.is_empty());
    }

    #[rstest]
    fn sequence_3_negative() {
        let mut regex = Regex::sequence([Regex::literal('a'), Regex::literal('b'), Regex::literal('c')].into_iter());

        assert!(!regex.is_terminal());
        assert!(!regex.is_empty());
        regex.feed_mut('b');
        assert!(regex.is_empty());
    }

    #[rstest]
    fn alternatives_positive(#[values('a', 'b', 'c')] ch: char) {
        let mut regex = Regex::alternatives([Regex::literal('a'), Regex::literal('b'), Regex::literal('c')].into_iter());

        assert!(!regex.is_terminal());
        regex.feed_mut(ch);
        assert!(regex.is_terminal());
    }

    #[rstest]
    fn alternatives_negative(#[values('x', 'y', 'z')] ch: char) {
        let mut regex = Regex::alternatives([Regex::literal('a'), Regex::literal('b'), Regex::literal('c')].into_iter());

        assert!(!regex.is_terminal());
        regex.feed_mut(ch);
        assert!(!regex.is_terminal());
        assert!(regex.is_empty());
    }

    #[rstest]
    fn kleene_positive(#[values("", "a", "aa", "aaa", "aaaaaaaaa")] string: &str) {
        let mut regex = Regex::kleene(Regex::literal('a'));

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());

        for ch in string.chars() {
            regex.feed_mut(ch);
            assert!(regex.is_terminal());
            assert!(!regex.is_empty());
        }
    }

    #[rstest]
    fn kleene_negative(#[values("b", "ba")] string: &str) {
        let mut regex = Regex::kleene(Regex::literal('a'));

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());

        for ch in string.chars() {
            regex.feed_mut(ch);
            assert!(!regex.is_terminal());
            assert!(regex.is_empty());
        }
    }

    #[rstest]
    fn optional_positive(#[values("", "a")] string: &str) {
        let mut regex = Regex::optional(Regex::literal('a'));

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());
    }

    #[rstest]
    fn optional_negative(#[values("a", "x")] string: &str) {
        let mut regex = Regex::optional(Regex::literal('b'));

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(!regex.is_terminal());
        assert!(regex.is_empty());
    }

    #[rstest]
    fn integer_positive(#[values("0", "1", "1234567890", "-546846")] string: &str) {
        let mut regex = Regex::integer(10);

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());
    }

    #[rstest]
    fn integer_negative(#[values("a", "0a", "1x5")] string: &str) {
        let mut regex = Regex::integer(10);

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(!regex.is_terminal());
        assert!(regex.is_empty());
    }

    #[rstest]
    fn digit_positive(#[values("1", "2", "3", "4", "5", "6", "7", "8", "9", "0")] string: &str) {
        let mut regex = Regex::digit(10);

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());
    }

    #[rstest]
    fn digit_negative(#[values("a", "-")] string: &str) {
        let mut regex = Regex::digit(10);

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(!regex.is_terminal());
        assert!(regex.is_empty());
    }

    #[rstest]
    fn float_positive(#[values("0.0", "1.0", "1234.567890", "-546.846")] string: &str) {
        let mut regex = Regex::float();

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(regex.is_terminal());
        assert!(!regex.is_empty());
    }

    #[rstest]
    fn float_negative(#[values("5", "-", "-2", "78x", "75.", ".92")] string: &str) {
        let mut regex = Regex::float();

        for ch in string.chars() {
            regex.feed_mut(ch);
        }

        assert!(!regex.is_terminal());
    }
}
