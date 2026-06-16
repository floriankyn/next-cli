# `next` — Next.js API handler scaffolder

A small Rust CLI that scaffolds Next.js App-Router API handler files from
embedded templates, so you don't hand-write the same boilerplate for every new
endpoint. Give it a resource name and the HTTP methods you want; it writes:

- one handler per method — `GET.ts` / `POST.ts` / `PUT.ts` / `DELETE.ts`
- `controller.ts` — a shared preamble (imports + consts) once, plus a typed
  client fetch function per method
- `route.ts` — imports each handler and re-exports it as the route

The resource name is normalised to PascalCase and substituted for the `$NAME$`
token in every template. Templates are compiled into the binary, so there are
no external files to manage at runtime.

## Setup

Requires a [Rust toolchain](https://rustup.rs) (Rust 1.74+ / edition 2021).

Install the `next` binary onto your `PATH` with Cargo:

```sh
git clone <this-repo> next-cli
cd next-cli
cargo install --path .
```

`cargo install` places the binary in `~/.cargo/bin` (make sure that's on your
`PATH`). Verify:

```sh
next --help
next api create --help
```

Prefer not to install globally? Build a local binary and run it directly:

```sh
cargo build --release
./target/release/next api create User
```

> Note: the binary is named `next`, which collides with the Next.js CLI if that
> is also on your `PATH`. Run it by absolute path, or install under a different
> name (e.g. `cargo install --path . && mv ~/.cargo/bin/next ~/.cargo/bin/next-scaffold`).

## Usage

```
next api create <name> [method flags] [-l <path>] [--force] [--dry-run]
```

- **`<name>`** — resource name in any case (`user`, `user-profile`,
  `userProfile`, `user_profile`). Normalised to **PascalCase** internally and
  substituted for the `$NAME$` token in every template.
- **Method flags** — `--get --post --put --delete --all`.
  No flag → all four. `--all` → all four. Otherwise the chosen subset.
- **`-l, --location <path>`** — target dir (default: `.`). Files are written
  flat into it.
- **`--force`** — overwrite existing files (default: refuse if any exist).
- **`--dry-run`** — print what would be written; write nothing.

### Examples

```sh
next api create User                      # all 6 files in CWD
next api create user-profile --get -l ./api/users
next api create order --post --put --force
next api create order --dry-run
```

Collision handling is **all-or-nothing**: if any target file exists and
`--force` is not set, the command errors and writes nothing.

## Develop

```sh
cargo build                              # debug build at ./target/debug/next
cargo test                               # unit + integration tests
cargo clippy --all-targets -- -D warnings
```

## Architecture

```
cli/        parse argv (clap)             -> CreateArgs
domain/     validate + normalise          -> ResourceName, HttpMethod, GenerationRequest
generation/ render strings from templates -> FileArtifact impls + plan
io/         touch disk                    -> FileSystem (RealFileSystem / InMemoryFs)
run.rs      orchestrate render -> check   -> write
main.rs     composition root + exit code
```

Each layer has one reason to change. The runner depends only on the
`FileArtifact` and `FileSystem` abstractions; `main` is the only place that
constructs the real filesystem. No `unwrap`/`expect`/`panic` outside tests —
everything is `Result<_, CliError>`.

## Adding an HTTP method (Open/Closed checklist)

Adding e.g. `PATCH` touches only:

1. A new `HttpMethod::Patch` variant in `src/domain/http_method.rs` plus its
   `match` arms (`upper`, `has_body`, the four template accessors, `all`).
2. Template files: `handler_patch.ts.tmpl`, `controller_patch.ts.tmpl`,
   `route_import_patch.ts.tmpl`, `route_export_patch.ts.tmpl`.
3. The `--patch` flag in `src/cli/args.rs` and one line in `resolve_methods`.

No changes to `generation/plan.rs`, the artifacts, `io`, or the runner.

## Adding a new output file type

Implement `FileArtifact` for a new struct (e.g. a `__tests__` file) and register
it in `src/generation/plan.rs::build_artifacts`. The orchestrator is untouched.

## Note on route wiring

`route_export_*.ts.tmpl` uses `<handler>.toRoute()` as the adapter. If your
`BaseApiHandler` exposes a different surface (`.handle`, `.GET`, …), change that
one token in the four `route_export_*` templates; nothing else depends on it.
