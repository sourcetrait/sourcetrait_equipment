use crate::*;

pub trait FromFixed<T>: Sized {
    fn fixed(v: T) -> Self;
}

pub trait RepositoryTrait: Clone + PartialEq {
    fn kind(&self) -> RepositoryKind;
}
