//! The main app config / controller.

/// App is the core of the...app. It is responsible
/// for managing the runner engine, loading config,
/// managing the TUI, the runbooks, etc. -- and
/// coordinating between them.
pub struct App;

impl App {
    /// Set up a new instance of the app.
    pub fn setup() -> Self {
        App {}
    }
}
