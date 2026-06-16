//! clap derive layer. Parses raw argv; produces no domain types directly —
//! conversion to a `GenerationRequest` happens in `to_request`.

use crate::domain::http_method::HttpMethod;
use crate::domain::request::GenerationRequest;
use crate::domain::resource_name::ResourceName;
use crate::error::CliError;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "next",
    version,
    about = "Scaffold Next.js App-Router API handlers"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// API scaffolding commands.
    Api {
        #[command(subcommand)]
        action: ApiAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ApiAction {
    /// Create handler files for a resource.
    Create(CreateArgs),
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    /// Resource name in any case (user, user-profile, userProfile, ...).
    pub name: String,

    #[arg(long)]
    pub get: bool,
    #[arg(long)]
    pub post: bool,
    #[arg(long)]
    pub put: bool,
    #[arg(long)]
    pub delete: bool,
    /// Select all four methods (the default when no flag is given).
    #[arg(long)]
    pub all: bool,

    /// Target directory. Files are written flat into it.
    #[arg(short = 'l', long, default_value = ".")]
    pub location: PathBuf,

    /// Overwrite existing files.
    #[arg(long)]
    pub force: bool,

    /// Print what would be written; write nothing.
    #[arg(long)]
    pub dry_run: bool,
}

/// Resolve the selected method set from the boolean flags.
///
/// No flags -> all four. `--all` -> all four. Otherwise the chosen subset.
pub fn resolve_methods(get: bool, post: bool, put: bool, delete: bool, all: bool) -> Vec<HttpMethod> {
    if all || !(get || post || put || delete) {
        return HttpMethod::all().to_vec();
    }
    let mut v = Vec::new();
    if get {
        v.push(HttpMethod::Get);
    }
    if post {
        v.push(HttpMethod::Post);
    }
    if put {
        v.push(HttpMethod::Put);
    }
    if delete {
        v.push(HttpMethod::Delete);
    }
    v
}

impl CreateArgs {
    /// Validate and resolve into a domain `GenerationRequest`.
    pub fn to_request(&self) -> Result<GenerationRequest, CliError> {
        let name = ResourceName::parse(&self.name)?;
        let methods = resolve_methods(self.get, self.post, self.put, self.delete, self.all);
        GenerationRequest::new(name, methods, self.location.clone(), self.force)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_selected_yields_all() {
        assert_eq!(
            resolve_methods(false, false, false, false, false),
            HttpMethod::all().to_vec()
        );
    }

    #[test]
    fn all_flag_yields_all() {
        assert_eq!(
            resolve_methods(true, false, false, false, true),
            HttpMethod::all().to_vec()
        );
    }

    #[test]
    fn subset_is_respected() {
        assert_eq!(
            resolve_methods(true, false, true, false, false),
            vec![HttpMethod::Get, HttpMethod::Put]
        );
    }
}
