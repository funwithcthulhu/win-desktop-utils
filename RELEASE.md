# Release Checklist

Use this checklist before publishing a new `win-desktop-utils` release.

1. Update `Cargo.toml`, `Cargo.lock`, `README.md`, and `CHANGELOG.md` for the new version.
2. Run the local verification suite:

   ```powershell
   cargo fmt --all -- --check
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo check --examples
   cargo test --doc --all-features
   cargo check --no-default-features
   cargo check --no-default-features --features app
   cargo check --no-default-features --features elevation
   cargo check --no-default-features --features instance
   cargo check --no-default-features --features paths
   cargo check --no-default-features --features recycle-bin
   cargo check --no-default-features --features shell
   cargo check --no-default-features --features shortcuts
   cargo check --target x86_64-unknown-linux-gnu --all-targets --all-features
   cargo check --target x86_64-unknown-linux-gnu --no-default-features
   cargo doc --no-deps
   lychee --offline --no-progress README.md CHANGELOG.md CONTRIBUTING.md SECURITY.md RELEASE.md ROADMAP.md CODE_OF_CONDUCT.md docs/cookbook.md docs/which-api.md docs/side-effects.md docs/compatibility.md
   cargo deny check
   cargo semver-checks check-release
   cargo package
   cargo publish --dry-run
   ```

3. Confirm the non-Windows CI job ran `cargo test --lib --all-features` on Linux.
4. Confirm `main` is clean and synced with `origin/main`.
5. Publish with `cargo publish`.
6. Tag the exact published commit:

   ```powershell
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```

7. Create a GitHub release for the tag with a short change summary and verification list.
8. Confirm crates.io, docs.rs, the GitHub release, and GitHub Actions all reflect the new version.
