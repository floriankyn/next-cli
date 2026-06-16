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

Prebuilt binaries are published on every release — no Rust toolchain needed.
Pick your platform below. Replace `0.2.0` with the
[latest version](https://github.com/floriankyn/next-cli/releases/latest) if
newer.

Each archive ships with a matching `.sha256` file you can use to verify the
download.

> The binary is named `next`, which collides with the Next.js CLI. If you use
> Next.js, install it under a different name (each section notes where).

### macOS — Apple Silicon (M1/M2/M3/M4)

```sh
VERSION=0.2.0
curl -L "https://github.com/floriankyn/next-cli/releases/download/v${VERSION}/next-v${VERSION}-aarch64-apple-darwin.tar.gz" | tar xz
sudo mv next /usr/local/bin/        # or: mv next /usr/local/bin/next-scaffold
next --help
```

If macOS Gatekeeper blocks it ("cannot be opened"), clear the quarantine flag:

```sh
sudo xattr -d com.apple.quarantine /usr/local/bin/next
```

Not sure which Mac you have? Run `uname -m` — `arm64` means Apple Silicon,
`x86_64` means Intel (next section).

### macOS — Intel (classic)

```sh
VERSION=0.2.0
curl -L "https://github.com/floriankyn/next-cli/releases/download/v${VERSION}/next-v${VERSION}-x86_64-apple-darwin.tar.gz" | tar xz
sudo mv next /usr/local/bin/        # or: mv next /usr/local/bin/next-scaffold
next --help
```

Same Gatekeeper note as above (`xattr -d com.apple.quarantine ...`).

### Linux

```sh
VERSION=0.2.0
# x86_64 (most desktops/servers). For ARM64 use: aarch64-unknown-linux-gnu
curl -L "https://github.com/floriankyn/next-cli/releases/download/v${VERSION}/next-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz" | tar xz
sudo mv next /usr/local/bin/        # or ~/.local/bin to avoid sudo
next --help
```

- **ARM64** (Raspberry Pi, ARM servers): swap the file for
  `next-v${VERSION}-aarch64-unknown-linux-gnu.tar.gz`.
- **Older / minimal distros** (glibc errors): use the fully static musl build,
  `next-v${VERSION}-x86_64-unknown-linux-musl.tar.gz`.

### Windows

PowerShell (x86_64):

```powershell
$Version = "0.2.0"
$Url = "https://github.com/floriankyn/next-cli/releases/download/v$Version/next-v$Version-x86_64-pc-windows-msvc.zip"
Invoke-WebRequest -Uri $Url -OutFile next.zip
Expand-Archive next.zip -DestinationPath "$env:LOCALAPPDATA\next-cli" -Force
# add it to PATH for the current user
[Environment]::SetEnvironmentVariable(
  "Path",
  [Environment]::GetEnvironmentVariable("Path", "User") + ";$env:LOCALAPPDATA\next-cli",
  "User")
```

Open a new terminal, then run `next --help`. To avoid the Next.js CLI clash,
rename `next.exe` to `next-scaffold.exe` inside `%LOCALAPPDATA%\next-cli`.

### Alternative: install with Cargo

If you already have a [Rust toolchain](https://rustup.rs) (Rust 1.74+):

```sh
cargo binstall next          # downloads the prebuilt binary, any OS
# or build from source:
cargo install --path .       # from a cloned checkout -> ~/.cargo/bin/next
```

## Update

The easiest way — `next` updates itself in place from the latest GitHub
release:

```sh
next --update
```

It detects your platform, downloads the matching release archive, and swaps the
running binary. If `next` lives somewhere root-owned (e.g. `/usr/local/bin`),
run it with `sudo` so it can replace the file:

```sh
sudo next --update
```

Other ways:

```sh
cargo binstall next                       # if installed via Cargo
git pull && cargo install --path . --force  # from an updated checkout
```

Or just re-run your platform's install command above with a newer `VERSION`.

Check the version before/after:

```sh
next --version
```

## Cutting a release (maintainers)

Releases are built automatically by
[`.github/workflows/release.yml`](.github/workflows/release.yml) when a version
tag is pushed:

```sh
# 1. bump `version` in Cargo.toml, commit
# 2. tag and push — the tag must start with `v`
git tag v0.2.0
git push origin v0.2.0
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
