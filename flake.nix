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
          toolchain = fenix.packages.${system}.complete.withComponents [
            "cargo"
            "rustc"
            "rustfmt"
            "clippy"
            "rust-analyzer"
            "rust-src"
          ];
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (craneLib.filterCargoSources path type)
              || pkgs.lib.hasInfix "/ethos/" (toString path);
          };
          # No `cargoVendorDir.outputHashes` — per
          # `~/primary/skills/nix-discipline.md` §"Cargo git deps in
          # crane flakes". Crane fetches git deps from `Cargo.lock`
          # alone; bump revs via
          # `nix run nixpkgs#cargo -- update -p <crate>`.
          commonArgs = {
            inherit src;
            strictDeps = true;
            version = "0.11.0";
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
            pname = "horizon";
            meta.mainProgram = "horizon-cli";
            doCheck = false;
          });
          horizon-compose = ctx.craneLib.buildPackage (ctx.commonArgs // {
            inherit (ctx) cargoArtifacts;
            pname = "horizon-compose";
            cargoExtraArgs = "--bin horizon-compose";
            meta.mainProgram = "horizon-compose";
            doCheck = false;
          });
        });

      checks = forSystems (system:
        let ctx = mkContext system; in
        {
          default = ctx.craneLib.cargoTest (ctx.commonArgs // {
            inherit (ctx) cargoArtifacts;
          });
          no-free-functions = ctx.pkgs.runCommand "horizon-no-free-functions"
            { inherit (ctx.commonArgs) src; }
            (builtins.readFile ./checks/no-free-functions.sh);
          no-inherent-methods = ctx.pkgs.runCommand "horizon-no-inherent-methods"
            { inherit (ctx.commonArgs) src; }
            (builtins.readFile ./checks/no-inherent-methods.sh);
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
      apps = forSystems (system: {
        horizon-compose = {
          type = "app";
          program = "${self.packages.${system}.horizon-compose}/bin/horizon-compose";
          meta.description = "Materialize a validated Horizon definition";
        };
      });
    };
}
