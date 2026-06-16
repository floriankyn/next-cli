//! One handler file per selected method (`GET.ts`, `POST.ts`, ...).

use crate::domain::http_method::HttpMethod;
use crate::domain::request::GenerationRequest;
use crate::error::CliError;
use crate::generation::artifact::FileArtifact;
use crate::generation::renderer::{NamePlaceholderRenderer, TemplateRenderer};

pub struct HandlerArtifact {
    pub method: HttpMethod,
}

impl FileArtifact for HandlerArtifact {
    fn relative_path(&self) -> String {
        format!("{}.ts", self.method.upper())
    }

    fn render(&self, req: &GenerationRequest) -> Result<String, CliError> {
        Ok(NamePlaceholderRenderer.render(self.method.handler_template(), &req.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::resource_name::ResourceName;
    use std::path::PathBuf;

    fn req() -> GenerationRequest {
        GenerationRequest::new(
            ResourceName::parse("user").unwrap(),
            vec![HttpMethod::Get],
            PathBuf::from("."),
            false,
        )
        .unwrap()
    }

    #[test]
    fn path_uses_upper_method() {
        let a = HandlerArtifact {
            method: HttpMethod::Get,
        };
        assert_eq!(a.relative_path(), "GET.ts");
    }

    #[test]
    fn render_contains_class_and_export() {
        let a = HandlerArtifact {
            method: HttpMethod::Get,
        };
        let out = a.render(&req()).unwrap();
        assert!(out.contains("class GetUserHandler"));
        assert!(out.contains("export const getUserHandler"));
        assert!(!out.contains("$NAME$"));
    }
}
