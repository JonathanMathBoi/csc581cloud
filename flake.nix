{
  description = "CSC 581 cloud project development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      treefmt-nix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        treefmtEval = treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";

          programs.nixfmt = {
            enable = true;
            package = pkgs.nixfmt-rfc-style;
          };

          settings.formatter.rustfmt = {
            command = "${rustToolchain}/bin/rustfmt";
            includes = [ "**/*.rs" ];
            # treefmt runs from repo root, but this repo's Cargo.toml lives in api/.
            # Pass edition explicitly so rustfmt does not fall back to Rust 2015.
            options = [
              "--edition"
              "2024"
            ];
          };
        };
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        checks = {
          formatting = treefmtEval.config.build.check self;
        };

        devShells.default = pkgs.mkShell {
          name = "csc581cloud-dev";

          packages = with pkgs; [
            rustToolchain
            bacon
            pkg-config
            openssl
            docker
            docker-compose
            treefmtEval.config.build.wrapper
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
