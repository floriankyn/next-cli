//! Orchestration: render every artifact, enforce an all-or-nothing collision
//! check, then write. Depends only on the `FileSystem` abstraction.

use crate::domain::request::GenerationRequest;
use crate::error::CliError;
use crate::generation::plan::build_artifacts;
use crate::io::writer::FileSystem;
use std::path::PathBuf;

/// Outcome of a run: the paths that were (or would be) created.
pub struct RunReport {
    pub created: Vec<PathBuf>,
    pub dry_run: bool,
}

pub fn run(
    req: &GenerationRequest,
    fs: &dyn FileSystem,
    dry_run: bool,
) -> Result<RunReport, CliError> {
    let artifacts = build_artifacts(req);

    // Render everything up front so a render error aborts before any write.
    let mut rendered: Vec<(PathBuf, String)> = Vec::with_capacity(artifacts.len());
    for artifact in &artifacts {
        let path = req.target_dir.join(artifact.relative_path());
        let contents = artifact.render(req)?;
        rendered.push((path, contents));
    }

    // All-or-nothing collision check: refuse if any target exists (unless
    // --force). Skipped for --dry-run since nothing is written.
    if !req.force && !dry_run {
        for (path, _) in &rendered {
            if fs.exists(path) {
                return Err(CliError::FileExists(path.display().to_string()));
            }
        }
    }

    let created: Vec<PathBuf> = rendered.iter().map(|(p, _)| p.clone()).collect();

    if dry_run {
        return Ok(RunReport {
            created,
            dry_run: true,
        });
    }

    fs.create_dir_all(&req.target_dir)?;
    for (path, contents) in &rendered {
        fs.write(path, contents)?;
    }

    Ok(RunReport {
        created,
        dry_run: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::http_method::HttpMethod;
    use crate::domain::resource_name::ResourceName;
    use crate::io::writer::InMemoryFs;

    fn req(force: bool) -> GenerationRequest {
        GenerationRequest::new(
            ResourceName::parse("user").unwrap(),
            vec![HttpMethod::Get, HttpMethod::Post],
            PathBuf::from("api"),
            force,
        )
        .unwrap()
    }

    #[test]
    fn writes_expected_file_set() {
        let fs = InMemoryFs::new();
        let report = run(&req(false), &fs, false).unwrap();
        assert!(!report.dry_run);
        let files = fs.files.borrow();
        let mut names: Vec<String> = files
            .keys()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        names.sort();
        assert_eq!(names, vec!["GET.ts", "POST.ts", "controller.ts", "route.ts"]);
        assert!(files
            .get(&PathBuf::from("api/GET.ts"))
            .unwrap()
            .contains("class GetUserHandler"));
    }

    #[test]
    fn collision_without_force_errors_and_writes_nothing() {
        let fs = InMemoryFs::new();
        fs.seed(&PathBuf::from("api/GET.ts"), "old");
        let err = run(&req(false), &fs, false);
        assert!(matches!(err, Err(CliError::FileExists(_))));
        // The pre-existing file is untouched and nothing else was written.
        assert_eq!(fs.files.borrow().len(), 1);
        assert_eq!(fs.files.borrow().get(&PathBuf::from("api/GET.ts")).unwrap(), "old");
    }

    #[test]
    fn force_overwrites() {
        let fs = InMemoryFs::new();
        fs.seed(&PathBuf::from("api/GET.ts"), "old");
        run(&req(true), &fs, false).unwrap();
        assert!(fs
            .files
            .borrow()
            .get(&PathBuf::from("api/GET.ts"))
            .unwrap()
            .contains("class GetUserHandler"));
    }

    #[test]
    fn dry_run_writes_nothing() {
        let fs = InMemoryFs::new();
        let report = run(&req(false), &fs, true).unwrap();
        assert!(report.dry_run);
        assert_eq!(report.created.len(), 4);
        assert!(fs.files.borrow().is_empty());
    }
}
