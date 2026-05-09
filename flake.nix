{
  description = "horizon-rs — typed schema, projection, and CLI for CriomOS cluster horizons.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";

    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    crane.url = "github:ipetkov/crane";
  };

  outputs =
    { self, nixpkgs, fenix, crane }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forSystems = f: nixpkgs.lib.genAttrs systems (s: f s);

      mkContext = system:
        let
          pkgs = import nixpkgs { inherit system; };
          toolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
          };
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          src = craneLib.cleanCargoSource ./.;
          # No `cargoVendorDir.outputHashes` — per
          # `~/primary/skills/nix-discipline.md` §"Cargo git deps in
          # crane flakes". Crane fetches git deps from `Cargo.lock`
          # alone; bump revs via
          # `nix run nixpkgs#cargo -- update -p <crate>`.
          commonArgs = {
            inherit src;
            strictDeps = true;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        { inherit pkgs toolchain craneLib commonArgs cargoArtifacts; };
    in
    {
      packages = forSystems (system:
        let ctx = mkContext system; in
        {
          default = ctx.craneLib.buildPackage (ctx.commonArgs // {
            inherit (ctx) cargoArtifacts;
            # pname must match Cargo.toml's [[bin]] name so `nix run`
            # finds bin/<pname>.
            pname = "horizon-cli";
            cargoExtraArgs = "--bin horizon-cli";
            meta.mainProgram = "horizon-cli";
            doCheck = false;
          });
        });

      checks = forSystems (system:
        let ctx = mkContext system; in
        {
          default = ctx.craneLib.cargoTest (ctx.commonArgs // {
            inherit (ctx) cargoArtifacts;
          });
        });

      devShells = forSystems (system:
        let ctx = mkContext system; in
        {
          default = ctx.pkgs.mkShell {
            packages = [
              ctx.toolchain
              ctx.pkgs.nixfmt-rfc-style
              ctx.pkgs.jq
            ];
          };
        });

      formatter = forSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
    };
}
