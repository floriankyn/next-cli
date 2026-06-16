//! End-to-end test against the real filesystem in a temp dir.

use next::cli::args::resolve_methods;
use next::domain::http_method::HttpMethod;
use next::domain::request::GenerationRequest;
use next::domain::resource_name::ResourceName;
use next::error::CliError;
use next::io::writer::RealFileSystem;
use next::run::run;
use std::path::PathBuf;

fn request(name: &str, methods: Vec<HttpMethod>, dir: PathBuf, force: bool) -> GenerationRequest {
    GenerationRequest::new(ResourceName::parse(name).unwrap(), methods, dir, force).unwrap()
}

#[test]
fn creates_subset_with_kebab_name_into_location() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("api/users");
    let methods = resolve_methods(true, false, false, false, false); // --get
    let req = request("user-profile", methods, dir.clone(), false);

    let report = run(&req, &RealFileSystem, false).unwrap();
    assert!(!report.dry_run);

    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    assert_eq!(names, vec!["GET.ts", "controller.ts", "route.ts"]);

    let handler = std::fs::read_to_string(dir.join("GET.ts")).unwrap();
    assert!(handler.contains("class GetUserProfileHandler"));
}

#[test]
fn default_creates_all_six_files() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_path_buf();
    let methods = resolve_methods(false, false, false, false, false); // none -> all
    let req = request("user", methods, dir.clone(), false);

    run(&req, &RealFileSystem, false).unwrap();

    for f in ["GET.ts", "POST.ts", "PUT.ts", "DELETE.ts", "controller.ts", "route.ts"] {
        assert!(dir.join(f).exists(), "missing {f}");
    }
    let controller = std::fs::read_to_string(dir.join("controller.ts")).unwrap();
    assert_eq!(controller.matches("import { API_V4_URL }").count(), 1);
    assert_eq!(controller.matches("const API_RESSOURCE").count(), 1);
}

#[test]
fn collision_without_force_is_all_or_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().to_path_buf();
    let req = request("user", HttpMethod::all().to_vec(), dir.clone(), false);

    run(&req, &RealFileSystem, false).unwrap();
    std::fs::write(dir.join("route.ts"), "manual edit").unwrap();

    // Re-run without --force must refuse and not clobber route.ts.
    let err = run(&req, &RealFileSystem, false);
    assert!(matches!(err, Err(CliError::FileExists(_))));
    assert_eq!(
        std::fs::read_to_string(dir.join("route.ts")).unwrap(),
        "manual edit"
    );
}

#[test]
fn dry_run_writes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("nope");
    let req = request("user", HttpMethod::all().to_vec(), dir.clone(), false);

    let report = run(&req, &RealFileSystem, true).unwrap();
    assert!(report.dry_run);
    assert!(!dir.exists());
}
