# Release Checklist

Use this checklist before publishing a new `win-desktop-utils` release.

1. Update `Cargo.toml`, `Cargo.lock`, `README.md`, and `CHANGELOG.md` for the new version.
2. Run the local verification suite:

   ```powershell
   cargo xtask release-check
   ```

   The command runs the release gates below:

   ```powershell
   cargo fmt --all -- --check
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo check --examples
   cargo xtask docs-check
   cargo xtask feature-check
   cargo check --target x86_64-unknown-linux-gnu --all-targets --all-features
   cargo check --target x86_64-unknown-linux-gnu --no-default-features
   cargo deny check
   cargo semver-checks check-release
   cargo xtask release-audit
   cargo xtask package-check
   ```

3. Optionally run manual Windows desktop smoke tests:

   ```powershell
   cargo xtask smoke
   ```

4. Confirm the non-Windows CI job ran `cargo test --lib --all-features` on Linux.
5. Confirm `main` is clean and synced with `origin/main`.
6. Publish with `cargo publish`.
7. Tag the exact published commit:

   ```powershell
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```

8. Create a GitHub release for the tag with a short change summary and verification list.
9. Confirm crates.io, docs.rs, the GitHub release, and GitHub Actions all reflect the new version.

   Useful checks:

   ```powershell
   cargo search win-desktop-utils --limit 1
   cargo info win-desktop-utils
   ```

   Open the versioned docs.rs page for the release, for example
   `https://docs.rs/win-desktop-utils/X.Y.Z/win_desktop_utils/`, and confirm it
   built successfully for the published version.
