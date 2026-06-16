//! `route.ts` — one import line per method, a blank line, then one export line
//! per method.

use crate::domain::http_method::HttpMethod;
use crate::domain::request::GenerationRequest;
use crate::error::CliError;
use crate::generation::artifact::FileArtifact;
use crate::generation::renderer::{NamePlaceholderRenderer, TemplateRenderer};

pub struct RouteArtifact {
    pub methods: Vec<HttpMethod>,
}

impl FileArtifact for RouteArtifact {
    fn relative_path(&self) -> String {
        "route.ts".to_string()
    }

    fn render(&self, req: &GenerationRequest) -> Result<String, CliError> {
        let renderer = NamePlaceholderRenderer;
        let mut out = String::new();
        for method in &self.methods {
            out.push_str(&renderer.render(method.route_import(), &req.name));
        }
        out.push('\n');
        for method in &self.methods {
            out.push_str(&renderer.render(method.route_export(), &req.name));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::resource_name::ResourceName;
    use std::path::PathBuf;

    fn req(methods: Vec<HttpMethod>) -> GenerationRequest {
        GenerationRequest::new(
            ResourceName::parse("user").unwrap(),
            methods,
            PathBuf::from("."),
            false,
        )
        .unwrap()
    }

    #[test]
    fn one_import_and_export_per_method() {
        let methods = vec![HttpMethod::Get, HttpMethod::Post];
        let a = RouteArtifact {
            methods: methods.clone(),
        };
        let out = a.render(&req(methods)).unwrap();
        assert_eq!(out.matches("import {").count(), 2);
        assert_eq!(out.matches("export const").count(), 2);
        assert!(out.contains("import { getUserHandler } from './GET';"));
        assert!(out.contains("export const POST = postUserHandler.toRoute();"));
        assert!(!out.contains("$NAME$"));
    }
}
