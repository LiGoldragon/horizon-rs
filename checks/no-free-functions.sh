set -eu
# horizon-rs keeps production Rust in two crates: the library and the CLI.
# `fn main()` is the one production free function the law allows, so the two
# binary entry points are the only lines excused here.
if grep -R -n -E '^(pub(\([^)]*\))? )?fn ' "$src/lib/src" "$src/cli/src" |
  grep -v -E ':[0-9]+:fn main\('; then
  echo "production Rust must not use module-level free functions" >&2
  exit 1
fi
touch "$out"
