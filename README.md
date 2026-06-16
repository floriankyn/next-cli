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

## Install

### Prebuilt binary (no Rust needed) — Linux, macOS, Windows

Each release ships prebuilt archives on the
[Releases page](https://github.com/floriankyn/next-cli/releases). Download the
one for your platform, unpack it, and put the `next` binary on your `PATH`.

| Platform | Archive |
| --- | --- |
| Linux x86_64 | `next-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz` |
| Linux x86_64 (static) | `next-vX.Y.Z-x86_64-unknown-linux-musl.tar.gz` |
| Linux ARM64 | `next-vX.Y.Z-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Intel | `next-vX.Y.Z-x86_64-apple-darwin.tar.gz` |
| macOS Apple Silicon | `next-vX.Y.Z-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `next-vX.Y.Z-x86_64-pc-windows-msvc.zip` |

```sh
# Linux/macOS example
curl -L https://github.com/floriankyn/next-cli/releases/latest/download/next-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv next /usr/local/bin/
```

Each archive has a matching `.sha256` file to verify the download.

### With `cargo binstall` (fetches the prebuilt binary)

```sh
cargo binstall next
```

### From source with Cargo

Requires a [Rust toolchain](https://rustup.rs) (Rust 1.74+ / edition 2021).

```sh
git clone https://github.com/floriankyn/next-cli next-cli
cd next-cli
cargo install --path .   # -> ~/.cargo/bin/next
```

Verify any of the above:

```sh
next --help
next api create --help
```

> Note: the binary is named `next`, which collides with the Next.js CLI if that
> is also on your `PATH`. Rename it on install if needed
> (e.g. `mv ~/.cargo/bin/next ~/.cargo/bin/next-scaffold`).

## Cutting a release (maintainers)

Releases are built automatically by
[`.github/workflows/release.yml`](.github/workflows/release.yml) when a version
tag is pushed:

```sh
# 1. bump `version` in Cargo.toml, commit
# 2. tag and push — the tag must start with `v`
git tag v0.1.0
git push origin v0.1.0
```

The workflow cross-compiles for all six targets above, packages each binary
with the README and a SHA256 checksum, and uploads them to the GitHub Release
for that tag. No secrets to configure — it uses the built-in `GITHUB_TOKEN`.

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
