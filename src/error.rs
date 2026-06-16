//! Single error type for the whole CLI. `main` is the only place that converts
//! a `CliError` into a process exit code.

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("invalid resource name: {0}")]
    InvalidName(String),

    #[error("no HTTP methods selected")]
    NoMethods,

    #[error("file already exists: {0} (use --force to overwrite)")]
    FileExists(String),

    #[error("io error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("update failed: {0}")]
    Update(String),
}
