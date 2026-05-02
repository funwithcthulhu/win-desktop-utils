# Release Checklist

Use this checklist before publishing a new `win-desktop-utils` release.

1. Update `Cargo.toml`, `Cargo.lock`, `README.md`, and `CHANGELOG.md` for the new version.
2. Run the local verification suite:

   ```powershell
   cargo fmt --all -- --check
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo check --examples
   cargo check --no-default-features
   cargo check --no-default-features --features app
   cargo check --no-default-features --features elevation
   cargo check --no-default-features --features instance
   cargo check --no-default-features --features paths
   cargo check --no-default-features --features recycle-bin
   cargo check --no-default-features --features shell
   cargo check --no-default-features --features shortcuts
   cargo doc --no-deps
   cargo package
   cargo publish --dry-run
   ```

3. Confirm `main` is clean and synced with `origin/main`.
4. Publish with `cargo publish`.
5. Tag the exact published commit:

   ```powershell
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```

6. Create a GitHub release for the tag with a short change summary and verification list.
7. Confirm crates.io, docs.rs, the GitHub release, and GitHub Actions all reflect the new version.
