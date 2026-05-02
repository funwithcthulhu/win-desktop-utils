use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const MARKDOWN_FILES: &[&str] = &[
    "README.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "RELEASE.md",
    "ROADMAP.md",
    "CODE_OF_CONDUCT.md",
    "docs/adoption.md",
    "docs/compatibility.md",
    "docs/cookbook.md",
    "docs/design.md",
    "docs/side-effects.md",
    "docs/testing.md",
    "docs/which-api.md",
    "tests/README.md",
];

fn main() {
    let result = match env::args().nth(1).as_deref() {
        Some("docs-check") => docs_check(),
        Some("feature-check") => feature_check(),
        Some("package-check") => package_check(),
        Some("release-check") => release_check(),
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(command) => Err(format!("unknown xtask command `{command}`")),
    };

    if let Err(err) = result {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn print_help() {
    println!("cargo xtask <command>");
    println!();
    println!("commands:");
    println!("  docs-check     run doctests, rustdoc lints, and local Markdown link checks");
    println!("  feature-check  check every public feature set");
    println!("  package-check  run cargo package and cargo publish --dry-run");
    println!("  release-check  run the full local release verification suite");
}

fn release_check() -> Result<(), String> {
    cargo(&["fmt", "--all", "--", "--check"])?;
    cargo(&["test"])?;
    cargo(&[
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
    ])?;
    cargo(&["check", "--examples"])?;
    docs_check()?;
    feature_check()?;
    cargo(&[
        "check",
        "--target",
        "x86_64-unknown-linux-gnu",
        "--all-targets",
        "--all-features",
    ])?;
    cargo(&[
        "check",
        "--target",
        "x86_64-unknown-linux-gnu",
        "--no-default-features",
    ])?;
    cargo(&["deny", "check"])?;
    cargo(&["semver-checks", "check-release"])?;
    package_check()
}

fn docs_check() -> Result<(), String> {
    cargo(&["test", "--doc", "--all-features"])?;
    cargo(&["rustdoc", "--all-features", "--", "-D", "warnings"])?;
    check_local_markdown_links()
}

fn feature_check() -> Result<(), String> {
    cargo(&["check", "--no-default-features"])?;

    for feature in [
        "app",
        "elevation",
        "instance",
        "paths",
        "recycle-bin",
        "shell",
        "shortcuts",
    ] {
        cargo(&["check", "--no-default-features", "--features", feature])?;
    }

    Ok(())
}

fn package_check() -> Result<(), String> {
    cargo(&["package"])?;
    cargo(&["publish", "--dry-run"])
}

fn cargo(args: &[&str]) -> Result<(), String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(cargo)
        .args(args)
        .current_dir(root_dir())
        .status()
        .map_err(|err| format!("failed to start cargo {}: {err}", args.join(" ")))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo {} failed with {status}", args.join(" ")))
    }
}

fn check_local_markdown_links() -> Result<(), String> {
    let root = root_dir();
    let mut missing = Vec::new();

    for file in MARKDOWN_FILES {
        let path = root.join(file);
        let text = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;

        for link in markdown_links(&text) {
            if should_skip_link(&link) {
                continue;
            }

            let path_part = link.split('#').next().unwrap_or_default();
            if path_part.trim().is_empty() {
                continue;
            }

            let candidate = path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(trim_angle_brackets(path_part));

            if !candidate.exists() {
                missing.push(format!("{file} -> {link}"));
            }
        }
    }

    if missing.is_empty() {
        println!("local Markdown links OK");
        Ok(())
    } else {
        Err(format!(
            "missing local Markdown links:\n{}",
            missing.join("\n")
        ))
    }
}

fn markdown_links(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut links = Vec::new();
    let mut index = 0;

    while let Some(offset) = find_pair(&bytes[index..], b']', b'(') {
        let start = index + offset + 2;
        if let Some(end_offset) = bytes[start..].iter().position(|byte| *byte == b')') {
            let end = start + end_offset;
            if let Ok(link) = std::str::from_utf8(&bytes[start..end]) {
                links.push(link.trim().to_owned());
            }
            index = end + 1;
        } else {
            break;
        }
    }

    links
}

fn find_pair(bytes: &[u8], first: u8, second: u8) -> Option<usize> {
    bytes
        .windows(2)
        .position(|window| window[0] == first && window[1] == second)
}

fn should_skip_link(link: &str) -> bool {
    link.starts_with('#')
        || link.contains("://")
        || link.starts_with("mailto:")
        || link.starts_with("tel:")
}

fn trim_angle_brackets(value: &str) -> &str {
    value
        .strip_prefix('<')
        .and_then(|without_prefix| without_prefix.strip_suffix('>'))
        .unwrap_or(value)
}

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest should live under the workspace root")
        .to_path_buf()
}
