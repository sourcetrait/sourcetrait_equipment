//pub(self) use crate::*;

pub(in crate::generated::ai) mod error {
    pub(in crate::generated::ai) mod annotated;
}
pub(in crate::generated::ai) mod encdec;

pub use crate::generated::ai::{
    error::annotated::AnnotatedResult,
};

pub(crate) use crate::generated::ai::{
    encdec::{
        BaseEncDec,
        base32,
        base36,
    },
};

pub(self) use std::{
    fmt,
    fs,
    fmt::Display,
    ops::Range,
    path::Path,
};

pub(self) use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
