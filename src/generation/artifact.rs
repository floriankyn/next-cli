//! One output file = one `FileArtifact`. Adding a new file type means a new
//! struct implementing this trait, registered in `plan.rs` — the orchestrator
//! never changes.

use crate::domain::request::GenerationRequest;
use crate::error::CliError;

pub trait FileArtifact {
    /// Path relative to the target directory, e.g. `"GET.ts"`.
    fn relative_path(&self) -> String;

    /// Render the full file contents for this request.
    fn render(&self, req: &GenerationRequest) -> Result<String, CliError>;
}
