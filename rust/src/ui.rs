//! Sole automatic presentation interface for runtime-owned execution progress.
//!
//! Applications configure and drive no UI objects. Runtime owns the observer
//! and lifecycle-fact contracts; crate composition attaches UI's inferred
//! required interactive dashboard.

pub(crate) mod command;
mod live_log;
mod session;
pub(crate) mod state;
pub(crate) mod terminal;
pub(crate) mod usage;

pub(crate) use session::UiSession;
