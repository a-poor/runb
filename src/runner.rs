//! Code for running and managing running code / scripts.

/// The engine for running code blocks.
///
/// It manages running `Task`s based on the runbook
/// configurations.
pub struct Engine {}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    /// The task hasn't started yet
    Pending,
    /// The task is running
    Running,
    /// The task completed
    Complete,
    /// The task failed
    Failed,
    /// The task was canceled
    Canceled,
}

/// A task wraps a command being executed
/// in a runbook.
pub struct Task {
    status: TaskStatus,
}
