//! Code for running and managing running code / scripts.

/// The engine for running code blocks.
pub struct Engine {}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Complete,
    Failed,
    Canceled,
}

/// A task wraps a command being executed
/// in a runbook.
pub struct Task {
    status: TaskStatus,
}
