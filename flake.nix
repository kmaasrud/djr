{
  description = "Djot command line interface";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust }:
    utils.lib.eachDefaultSystem (system:
      let
        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        pname = toml.package.name;
        version = toml.package.version;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust) ];
        };

        inherit (pkgs) rustPlatform mkShell lib;
      in
      rec {
        # `nix build`
        packages.default = rustPlatform.buildRustPackage {
          inherit pname version;
          src = ./.;
          cargoSha256 = "sha256-S1noIq6mIE38pGx5dj73r1otH5gCAykaCem+2rR8qi0=";
        };

        # `nix run`
        apps.default = utils.lib.mkApp { drv = packages.default; };

        # `nix develop`
        devShells.default = mkShell {
          buildInputs = with pkgs;
            [
              (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            ];
        };
      });
}
