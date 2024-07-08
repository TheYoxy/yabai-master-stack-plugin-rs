{
  description = "";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }: let
    overlays = [
      rust-overlay.overlays.default
      (final: prev: {
        rustToolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      })
    ];
  in
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system overlays;};
      rustPlatform = pkgs.makeRustPlatform {
        cargo = pkgs.rustToolchain;
        rustc = pkgs.rustToolchain;
      };
    in {
      formatter = pkgs.alejandra;

      devShells = {
        default = with pkgs;
          mkShell {
            buildInputs = [
              libiconv
              pkg-config
              rustToolchain
              (cargo-watch.override {
                inherit rustPlatform;
              })
            ];
          };
      };

      packages = let
        inherit (pkgs) lib;
        inherit (lib.importTOML ./Cargo.toml) package;
        rev = self.shortRev or self.dirtyShortRev or "dirty";
      in rec {
        ymsp =
          rustPlatform
          .buildRustPackage {
            pname = package.name;
            version = "${package.version}-${rev}";
            src = lib.fileset.toSource {
              root = ./.;
              fileset =
                lib.fileset.intersection
                (lib.fileset.fromSource (lib.sources.cleanSource ./.))
                (lib.fileset.unions [
                  ./src
                  ./Cargo.toml
                  ./Cargo.lock
                ]);
            };

            cargoLock.lockFile = ./Cargo.lock;

            strictDeps = true;

            nativeBuildInputs = with pkgs; [
              installShellFiles
              makeBinaryWrapper
            ];

            preFixup = ''
              echo "Creating completions directory..."
              mkdir completions
              echo "Generating shell completions..."
              RUST_LOG=trace $out/bin/${package.name} completions bash > completions/${package.name}.bash
              RUST_LOG=trace $out/bin/${package.name} completions zsh > completions/${package.name}.zsh
              RUST_LOG=trace $out/bin/${package.name} completions fish > completions/${package.name}.fish

              installShellCompletion completions/*
            '';

            doCheck = false;
            meta = {
              description = package.description;
              homepage = package.repository;
              license = lib.licenses.mit;
              mainProgram = package.name;
              maintainers = [
                {
                  name = "TheYoxy";
                  email = "floryansimar@gmail.com";
                }
              ];
            };
          };
        default = ymsp;
      };
    });
}
