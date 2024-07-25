{
  description = "";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
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
    rust-overlay,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    forAllSystems = nixpkgs.lib.genAttrs systems;
    overlays = [
      rust-overlay.overlays.default
      (final: _prev: {
        rustToolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      })
    ];
  in {
    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    packages = forAllSystems (system: let
      pkgs = import nixpkgs {inherit system overlays;};
      rustPlatform = pkgs.makeRustPlatform {
        cargo = pkgs.rustToolchain;
        rustc = pkgs.rustToolchain;
      };
      rev = self.shortRev or self.dirtyShortRev or "dirty";
    in {
      ymsp = pkgs.callPackage ./packages.nix {
        inherit rev rustPlatform;
      };
      default = self.packages.${system}.ymsp;
    });

    checks = forAllSystems (system: {
      inherit (self.packages.${system}) ymsp;
    });
  };
}
