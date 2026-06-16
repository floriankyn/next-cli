//! A fully validated, resolved generation request — the boundary between
//! parsing (`cli`) and generation.

use crate::domain::http_method::HttpMethod;
use crate::domain::resource_name::ResourceName;
use crate::error::CliError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GenerationRequest {
    pub name: ResourceName,
    /// Sorted, deduped, guaranteed non-empty.
    pub methods: Vec<HttpMethod>,
    pub target_dir: PathBuf,
    pub force: bool,
}

impl GenerationRequest {
    /// Build a request, normalising the method set (sort + dedup) and
    /// rejecting an empty selection.
    pub fn new(
        name: ResourceName,
        mut methods: Vec<HttpMethod>,
        target_dir: PathBuf,
        force: bool,
    ) -> Result<Self, CliError> {
        methods.sort();
        methods.dedup();
        if methods.is_empty() {
            return Err(CliError::NoMethods);
        }
        Ok(Self {
            name,
            methods,
            target_dir,
            force,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_and_dedups() {
        let req = GenerationRequest::new(
            ResourceName::parse("user").unwrap(),
            vec![HttpMethod::Post, HttpMethod::Get, HttpMethod::Post],
            PathBuf::from("."),
            false,
        )
        .unwrap();
        assert_eq!(req.methods, vec![HttpMethod::Get, HttpMethod::Post]);
    }

    #[test]
    fn rejects_empty_methods() {
        let err = GenerationRequest::new(
            ResourceName::parse("user").unwrap(),
            vec![],
            PathBuf::from("."),
            false,
        );
        assert!(matches!(err, Err(CliError::NoMethods)));
    }
}
