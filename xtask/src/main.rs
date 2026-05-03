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
    "docs/feature-flags.md",
    "docs/integrations.md",
    "docs/overhead.md",
    "docs/side-effects.md",
    "docs/testing.md",
    "docs/trust.md",
    "docs/which-api.md",
    "examples/README.md",
    "tests/README.md",
];

const PUBLIC_FEATURES: &[&str] = &[
    "app",
    "elevation",
    "instance",
    "paths",
    "recycle-bin",
    "shell",
    "shortcuts",
];

fn main() {
    let result = match env::args().nth(1).as_deref() {
        Some("docs-check") => docs_check(),
        Some("feature-check") => feature_check(),
        Some("package-check") => package_check(),
        Some("release-audit") => release_audit(),
        Some("release-check") => release_check(),
        Some("smoke") => smoke_check(),
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
    println!("  release-audit  check release metadata and package contents");
    println!("  release-check  run the full local release verification suite");
    println!("  smoke          run ignored manual Windows desktop smoke tests");
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
    release_audit()?;
    package_check()
}

fn docs_check() -> Result<(), String> {
    cargo(&["test", "--doc", "--all-features"])?;
    cargo(&["rustdoc", "--all-features", "--", "-D", "warnings"])?;
    check_local_markdown_links()
}

fn feature_check() -> Result<(), String> {
    cargo(&["check", "--no-default-features"])?;

    for feature in PUBLIC_FEATURES {
        cargo(&["check", "--no-default-features", "--features", feature])?;
    }

    for (first, second) in feature_pairs(PUBLIC_FEATURES) {
        let features = format!("{first},{second}");
        cargo(&["check", "--no-default-features", "--features", &features])?;
    }

    Ok(())
}

fn feature_pairs<'a>(features: &'a [&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut pairs = Vec::new();

    for (index, first) in features.iter().enumerate() {
        for second in features.iter().skip(index + 1) {
            pairs.push((*first, *second));
        }
    }

    pairs
}

fn package_check() -> Result<(), String> {
    cargo(&["package"])?;
    cargo(&["publish", "--dry-run"])
}

fn release_audit() -> Result<(), String> {
    let version = package_version()?;

    check_cargo_lock_version(&version)?;
    check_changelog_version(&version)?;
    check_readme_install_snippet(&version)?;
    check_package_contents()?;

    println!("release metadata OK for {version}");
    Ok(())
}

fn smoke_check() -> Result<(), String> {
    if !cfg!(windows) {
        return Err("manual smoke tests require a Windows host".to_owned());
    }

    println!("running ignored manual smoke tests");
    println!("set WIN_DESKTOP_UTILS_SMOKE_UI=1 to open Explorer/browser UI");
    println!("set WIN_DESKTOP_UTILS_SMOKE_ELEVATION=1 to trigger a UAC launch");

    cargo(&[
        "test",
        "--test",
        "smoke",
        "--",
        "--ignored",
        "--test-threads=1",
    ])
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

fn cargo_output(args: &[&str]) -> Result<String, String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .args(args)
        .current_dir(root_dir())
        .output()
        .map_err(|err| format!("failed to start cargo {}: {err}", args.join(" ")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "cargo {} failed with {}\n{}",
            args.join(" "),
            output.status,
            stderr.trim()
        ))
    }
}

fn package_version() -> Result<String, String> {
    let cargo_toml = read_root_file("Cargo.toml")?;
    let mut in_package = false;

    for line in cargo_toml.lines() {
        let trimmed = line.trim();

        if trimmed == "[package]" {
            in_package = true;
            continue;
        }

        if trimmed.starts_with('[') {
            in_package = false;
            continue;
        }

        if in_package && trimmed.starts_with("version") {
            return quoted_value(trimmed)
                .map(str::to_owned)
                .ok_or_else(|| "package version is not a quoted string".to_owned());
        }
    }

    Err("package version not found in Cargo.toml".to_owned())
}

fn check_cargo_lock_version(version: &str) -> Result<(), String> {
    let lock = read_root_file("Cargo.lock")?;
    let mut saw_crate = false;

    for line in lock.lines() {
        let trimmed = line.trim();

        if trimmed == r#"name = "win-desktop-utils""# {
            saw_crate = true;
            continue;
        }

        if saw_crate && trimmed.starts_with("version") {
            let locked_version = quoted_value(trimmed)
                .ok_or_else(|| "Cargo.lock crate version is not a quoted string".to_owned())?;

            if locked_version == version {
                return Ok(());
            }

            return Err(format!(
                "Cargo.lock version {locked_version} does not match Cargo.toml version {version}"
            ));
        }
    }

    Err("win-desktop-utils entry not found in Cargo.lock".to_owned())
}

fn check_changelog_version(version: &str) -> Result<(), String> {
    let changelog = read_root_file("CHANGELOG.md")?;
    let heading = format!("## {version} - ");

    if changelog.contains(&heading) {
        Ok(())
    } else {
        Err(format!("CHANGELOG.md must contain `{heading}YYYY-MM-DD`"))
    }
}

fn check_readme_install_snippet(version: &str) -> Result<(), String> {
    let readme = read_root_file("README.md")?;
    let requirement = readme_version_requirement(version)?;
    let snippet = format!(r#"win-desktop-utils = "{requirement}""#);

    if readme.contains(&snippet) {
        Ok(())
    } else {
        Err(format!(
            "README.md must contain install snippet `{snippet}`"
        ))
    }
}

fn readme_version_requirement(version: &str) -> Result<String, String> {
    let mut parts = version.split('.');
    let major = parts
        .next()
        .ok_or_else(|| format!("version `{version}` is missing a major component"))?;
    let minor = parts
        .next()
        .ok_or_else(|| format!("version `{version}` is missing a minor component"))?;

    Ok(format!("{major}.{minor}"))
}

fn check_package_contents() -> Result<(), String> {
    let package_list = cargo_output(&["package", "--list", "--allow-dirty"])?;
    let forbidden_prefixes = [".cargo/", ".github/", ".git/", "target/", "xtask/"];
    let mut forbidden = Vec::new();

    for line in package_list.lines() {
        let normalized = line.trim().replace('\\', "/");

        if forbidden_prefixes
            .iter()
            .any(|prefix| normalized.starts_with(prefix))
        {
            forbidden.push(normalized);
        }
    }

    if forbidden.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "unexpected files in package:\n{}",
            forbidden.join("\n")
        ))
    }
}

fn read_root_file(path: &str) -> Result<String, String> {
    let path = root_dir().join(path);
    fs::read_to_string(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))
}

fn quoted_value(line: &str) -> Option<&str> {
    let (_, rest) = line.split_once('=')?;
    let rest = rest.trim().strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(&rest[..end])
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
