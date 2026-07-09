//! Opt-in annotated rendering for [`EquipmentError`].
//!
//! [`EquipmentError::annotate`] returns an `impl Display` that renders
//! [`EquipmentError::TomlFileRead`] as a rustc-style `annotate-snippets`
//! report (re-reading the file for source context) and defers to the normal
//! snafu `Display` for every other variant. [`Annotated::annotated`] adapts
//! an [`EquipmentResult`] so `expect`/`unwrap` panic with that render:
//!
//! ```ignore
//! use equipment::Annotated as _;
//!
//! let lib = equipment::Library::read(path).annotated().expect("reads");
//! ```

use crate::*;
use crate::generated::ai::*;

impl EquipmentError {
    /// Rich rendering: annotated source snippet for TOML parse errors, the
    /// plain `Display` for everything else.
    pub fn annotate(&self) -> impl fmt::Display + '_ {
        Annotate(self)
    }
}

/// Lazy renderer returned by [`EquipmentError::annotate`]; does no work
/// until formatted.
struct Annotate<'a>(&'a EquipmentError);

impl fmt::Display for Annotate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let EquipmentError::TomlFileRead { source, path } = self.0 {
            // The error carries no source text, so re-read the file.
            if let Some(span) = source.span() {
                if let Ok(src) = fs::read_to_string(path) {
                    // Guard against the file having changed since the parse:
                    // `str::get` rejects out-of-bounds and non-char-boundary
                    // ranges, which would otherwise panic the renderer.
                    if src.get(span.clone()).is_some() {
                        return annotate_toml_read(f, span, source.message(), path, &src);
                    }
                }
            }
        }
        // Every other variant — and any TOML error we can't annotate (no
        // span, unreadable or changed file) — uses the snafu Display.
        fmt::Display::fmt(self.0, f)
    }
}

fn annotate_toml_read(
    f: &mut fmt::Formatter<'_>,
    span: Range<usize>,
    msg: &str,
    path: &Path,
    src: &str,
) -> fmt::Result {
    let origin = path.display().to_string();
    let report = &[Level::ERROR.primary_title("Failed to parse TOML").element(
        Snippet::source(src)
            .path(origin.as_str())
            .annotation(AnnotationKind::Primary.span(span).label(msg)),
    )];

    // `styled()` always emits ANSI; swap in `Renderer::plain()` — or gate on
    // `std::io::IsTerminal` — if these renders land in logs that keep escapes.
    write!(f, "{}", Renderer::styled().render(report))
}

// === Result adapter ========================================================

/// Wraps [`EquipmentError`] so that `Debug` — what `expect`/`unwrap` print —
/// routes through [`EquipmentError::annotate`].
pub struct AnnotatedEquipmentError(pub(crate) EquipmentError);

impl fmt::Display for AnnotatedEquipmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.annotate(), f)
    }
}

impl fmt::Debug for AnnotatedEquipmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `expect(msg)` panics with `{msg}: {err:?}`; the leading newline
        // keeps the snippet flush-left instead of trailing the message.
        write!(f, "\n{}", self.0.annotate())
    }
}

impl std::error::Error for AnnotatedEquipmentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<AnnotatedEquipmentError> for EquipmentError {
    fn from(err: AnnotatedEquipmentError) -> Self {
        err.0
    }
}

/// Extension for [`EquipmentResult`]: swaps the error side so failures
/// print annotated.
pub trait AnnotatedResult<T> {
    /// Wraps the error in [`AnnotatedError`].
    fn annotated(self) -> Result<T, AnnotatedEquipmentError>;
}

impl<T> AnnotatedResult<T> for EquipmentResult<T> {
    fn annotated(self) -> Result<T, AnnotatedEquipmentError> {
        self.map_err(AnnotatedEquipmentError)
    }
}