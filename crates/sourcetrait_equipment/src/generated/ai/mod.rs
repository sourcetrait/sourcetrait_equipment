//pub(self) use crate::*;

pub(in crate::generated::ai) mod error {
    pub(in crate::generated::ai) mod annotated;
}

pub use crate::generated::ai::{
    error::annotated::AnnotatedResult,
};

pub(self) use std::{
    fmt,
    fs,
    ops::Range,
    path::Path,
};

pub(self) use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
