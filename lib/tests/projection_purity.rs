//! Architectural witness: `lib/src/` is pure.
//!
//! The projection from `ClusterProposal + Viewpoint` to `Horizon` is a
//! deterministic data transform. No environment reads, no filesystem
//! reads, no wall-clock, no stdin/stdout/stderr, no spawning processes,
//! no async runtime. Tests can exercise the full projection on
//! synthetic input with byte-for-byte reproducible output — that
//! property is the reason CriomOS modules can derive their config from
//! horizon at evaluation time.
//!
//! This test reads every `.rs` file under `lib/src/` and asserts that
//! none of the forbidden tokens appear. If a future change reaches for
//! env, fs, time, or process, the test fails and surfaces the
//! architectural rule.
//!
//! Spec: `reports/system-assistant/07-criomos-stack-deep-audit.md`
//! §"Implicit constraints" (projection-is-pure) and §6 (tests proposal).

use std::fs;
use std::path::{Path, PathBuf};

/// Token substrings that must not appear in `lib/src/`. Each one names
/// a class of impurity:
const FORBIDDEN: &[(&str, &str)] = &[
    ("std::env::", "environment reads"),
    ("std::fs::", "filesystem reads"),
    ("std::io::stdin", "stdin reads"),
    ("std::io::stdout", "stdout writes"),
    ("std::io::stderr", "stderr writes"),
    ("std::time::SystemTime", "wall-clock reads"),
    ("std::time::Instant", "monotonic-clock reads"),
    ("std::process::", "subprocess spawning"),
    ("tokio::", "async runtime (horizon-rs is sync)"),
];

fn src_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read src dir") {
        let entry = entry.expect("read dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn line_is_comment_or_empty(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.is_empty() || trimmed.starts_with("//")
}

#[test]
fn projection_is_pure_no_env_fs_time_io_in_src() {
    let mut files = Vec::new();
    collect_rs_files(&src_dir(), &mut files);
    assert!(
        !files.is_empty(),
        "expected at least one .rs file under {:?}",
        src_dir()
    );

    let mut violations: Vec<String> = Vec::new();
    for file in &files {
        let content = fs::read_to_string(file).expect("read src file");
        for (lineno, line) in content.lines().enumerate() {
            if line_is_comment_or_empty(line) {
                continue;
            }
            for (token, what) in FORBIDDEN {
                if line.contains(token) {
                    let rel = file
                        .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                        .unwrap_or(file)
                        .display();
                    violations.push(format!(
                        "{rel}:{}: {what} via {token:?}",
                        lineno + 1
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "projection-purity violation — {} site(s):\n  {}",
        violations.len(),
        violations.join("\n  ")
    );
}
