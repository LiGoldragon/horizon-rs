{ pkgs, inputs, system, flake, ... }:

let
  toolchain = inputs.fenix.packages.${system}.fromToolchainFile {
    file = flake + "/rust-toolchain.toml";
    sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
  };

  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

  src = craneLib.cleanCargoSource flake;

  cargoVendorDir = craneLib.vendorCargoDeps {
    inherit src;
    outputHashes = {
      "git+https://github.com/LiGoldragon/nota-serde#fc005d47870a4a17594464251462b57c251f89b8" =
        "sha256-CmycShB+N6JgvFT6xzbgFWw445DTOVj7fbo2jOSUH3I=";
      "git+https://github.com/LiGoldragon/nota-serde-core.git#e553e171b733583758c1351d7a5cd5642e32b5a8" =
        "sha256-XrgepGfd2ADYifj30+hIocKh+Ctycof54E/BBKLKI0g=";
    };
  };

  commonArgs = {
    inherit src cargoVendorDir;
    strictDeps = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (commonArgs // {
  inherit cargoArtifacts;
  pname = "horizon-cli";
  cargoExtraArgs = "--bin horizon-cli";
})
