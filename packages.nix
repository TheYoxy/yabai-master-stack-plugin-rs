{
  lib,
  rustPlatform,
  installShellFiles,
  rev ? "dirty",
  ...
}: let
  inherit (lib.importTOML ./Cargo.toml) package;
in
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

    nativeBuildInputs = [
      installShellFiles
    ];

    preFixup = ''
      echo "Creating completions directory..."
      mkdir completions
      echo "Generating shell completions..."

      $out/bin/${package.name} completions bash > completions/${package.name}.bash
      $out/bin/${package.name} completions zsh > completions/${package.name}.zsh
      $out/bin/${package.name} completions fish > completions/${package.name}.fish

      installShellCompletion completions/*
    '';

    doCheck = false;
    meta = {
      inherit (package) description;
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
  }
