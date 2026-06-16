//! Composition root: parse argv, build the request, inject the real
//! filesystem, run, and map any `CliError` to a process exit code. This is the
//! only place that touches `std::process::exit` or prints to stderr.

use clap::{CommandFactory, Parser};
use next::cli::args::{ApiAction, Cli, Command, CreateArgs};
use next::error::CliError;
use next::io::writer::RealFileSystem;
use next::run::{run, RunReport};
use next::update::update_to_latest;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = dispatch(&cli) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn dispatch(cli: &Cli) -> Result<(), CliError> {
    if cli.update {
        return update_to_latest();
    }
    match &cli.command {
        Some(Command::Api { action }) => match action {
            ApiAction::Create(args) => create(args),
        },
        // No subcommand and no --update: show help (and exit non-zero).
        None => {
            let _ = Cli::command().print_help();
            std::process::exit(2);
        }
    }
}

fn create(args: &CreateArgs) -> Result<(), CliError> {
    let request = args.to_request()?;
    let fs = RealFileSystem;
    let report = run(&request, &fs, args.dry_run)?;
    print_summary(&report);
    Ok(())
}

fn print_summary(report: &RunReport) {
    let verb = if report.dry_run {
        "would create"
    } else {
        "created"
    };
    for path in &report.created {
        println!("{verb}: {}", path.display());
    }
    println!("{} file(s) {verb}.", report.created.len());
}
