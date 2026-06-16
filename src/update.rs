//! Self-update: replace the running binary with the latest GitHub release for
//! this platform. Kept isolated so the network/replace concern lives in one
//! place; errors are funnelled into `CliError::Update`.

use crate::error::CliError;

/// Download the latest release archive for the current target and replace the
/// running executable in place.
pub fn update_to_latest() -> Result<(), CliError> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("floriankyn")
        .repo_name("next-cli")
        .bin_name("next")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()
        .map_err(|e| CliError::Update(e.to_string()))?
        .update()
        .map_err(|e| CliError::Update(e.to_string()))?;

    if status.updated() {
        println!("updated to v{}", status.version());
    } else {
        println!("already up to date (v{})", status.version());
    }
    Ok(())
}
