//! Assemble the list of files to produce for a request. The single place that
//! knows which artifacts exist; the runner is agnostic.

use crate::domain::request::GenerationRequest;
use crate::generation::artifact::FileArtifact;
use crate::generation::controller::ControllerArtifact;
use crate::generation::handler::HandlerArtifact;
use crate::generation::route::RouteArtifact;

pub fn build_artifacts(req: &GenerationRequest) -> Vec<Box<dyn FileArtifact>> {
    let mut v: Vec<Box<dyn FileArtifact>> = Vec::new();
    for &m in &req.methods {
        v.push(Box::new(HandlerArtifact { method: m }));
    }
    v.push(Box::new(ControllerArtifact {
        methods: req.methods.clone(),
    }));
    v.push(Box::new(RouteArtifact {
        methods: req.methods.clone(),
    }));
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::http_method::HttpMethod;
    use crate::domain::resource_name::ResourceName;
    use std::path::PathBuf;

    #[test]
    fn produces_handlers_plus_controller_and_route() {
        let req = GenerationRequest::new(
            ResourceName::parse("user").unwrap(),
            vec![HttpMethod::Get, HttpMethod::Post],
            PathBuf::from("."),
            false,
        )
        .unwrap();
        let paths: Vec<String> = build_artifacts(&req)
            .iter()
            .map(|a| a.relative_path())
            .collect();
        assert_eq!(
            paths,
            vec!["GET.ts", "POST.ts", "controller.ts", "route.ts"]
        );
    }
}
