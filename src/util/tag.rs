use std::{hash::Hash, fmt::Debug};

pub trait Tag : Debug + Hash + PartialEq + Eq + Copy + Clone { }

impl Tag for () { }

#[macro_export]
macro_rules! define_tag {
    ($id:ident) => {
        #[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
        struct $id;

        impl $crate::util::tag::Tag for $id { }
    };
}

pub use define_tag;
