//! Composition root: parse argv, build the request, inject the real
//! filesystem, run, and map any `CliError` to a process exit code. This is the
//! only place that touches `std::process::exit` or prints to stderr.

use clap::Parser;
use next::cli::args::{ApiAction, Cli, Command, CreateArgs};
use next::error::CliError;
use next::io::writer::RealFileSystem;
use next::run::{run, RunReport};

fn main() {
    let cli = Cli::parse();
    if let Err(err) = dispatch(&cli) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn dispatch(cli: &Cli) -> Result<(), CliError> {
    match &cli.command {
        Command::Api { action } => match action {
            ApiAction::Create(args) => create(args),
        },
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
