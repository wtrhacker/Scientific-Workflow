//! Reusable process and terminal tools for independently packaged applications.
//!
//! Enable the `external-tools` feature to use these existing Workflow components
//! without constructing a study or invoking recordings, conversion, or analysis.
//! Scheduling policy and domain-specific resource cleanup belong to the caller.
//!
//! Parameter expansion is available separately as [`crate::config::expand_parameters`].

pub use crate::runtime::process::{ProcessTree, spawn};
pub use crate::ui::command::{CommandInput, EditAction};
pub use crate::ui::state::TaskStatus;
pub use crate::ui::terminal::{TerminalLease, interactive, open_surface, task_status_style};
pub use crate::ui::usage::{UsageMonitor, UsageSnapshot};
