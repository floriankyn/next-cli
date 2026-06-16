//! `controller.ts` — shared preamble (imports + consts) emitted **once**,
//! followed by one client fragment per selected method.

use crate::domain::http_method::HttpMethod;
use crate::domain::request::GenerationRequest;
use crate::error::CliError;
use crate::generation::artifact::FileArtifact;
use crate::generation::renderer::{NamePlaceholderRenderer, TemplateRenderer};

const CONTROLLER_PREAMBLE: &str = include_str!("../templates/controller_preamble.ts.tmpl");

pub struct ControllerArtifact {
    pub methods: Vec<HttpMethod>,
}

impl FileArtifact for ControllerArtifact {
    fn relative_path(&self) -> String {
        "controller.ts".to_string()
    }

    fn render(&self, req: &GenerationRequest) -> Result<String, CliError> {
        let renderer = NamePlaceholderRenderer;
        let mut out = renderer.render(CONTROLLER_PREAMBLE, &req.name);
        for method in &self.methods {
            out.push_str(&renderer.render(method.controller_fragment(), &req.name));
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
    fn preamble_appears_exactly_once_with_all_methods() {
        let methods = HttpMethod::all().to_vec();
        let a = ControllerArtifact {
            methods: methods.clone(),
        };
        let out = a.render(&req(methods)).unwrap();
        assert_eq!(out.matches("import { API_V4_URL }").count(), 1);
        assert_eq!(out.matches("const API_RESSOURCE").count(), 1);
        assert_eq!(out.matches("const DYNAMIC_PARAMS").count(), 1);
    }

    #[test]
    fn one_fetch_fn_per_method() {
        let methods = HttpMethod::all().to_vec();
        let a = ControllerArtifact {
            methods: methods.clone(),
        };
        let out = a.render(&req(methods)).unwrap();
        assert!(out.contains("export const getUser ="));
        assert!(out.contains("export const postUser ="));
        assert!(out.contains("export const putUser ="));
        assert!(out.contains("export const deleteUser ="));
        assert!(!out.contains("$NAME$"));
    }
}
