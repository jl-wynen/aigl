/// Very basic git functionality.
///
/// This crate is geared towards the AI game installer and launcher and can only work
/// with repositories at HEAD and not with branches.
mod repo;
pub use repo::Repository;
