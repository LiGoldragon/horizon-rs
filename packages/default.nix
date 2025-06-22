{ pkgs, flake, ... }:

# horizon-rs CLI. Produced from the workspace (lib + cli).
# Until schema + methods are ported, this is a stub build.

pkgs.rustPlatform.buildRustPackage {
  pname = "horizon-rs";
  version = "0.0.1";
  src = flake;
  cargoLock.lockFile = flake + "/Cargo.lock";
}
