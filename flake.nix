{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    systems.url = "github:nix-systems/default";
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;
          overlays = [ inputs.rust-overlay.overlays.default ];
          buildInputs = [
            pkgs.pkg-config
            pkgs.udev
            pkgs.alsa-lib
            pkgs.vulkan-loader
            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libXi
            pkgs.xorg.libXrandr
            pkgs.libxkbcommon
            pkgs.wayland
          ];
          src = lib.cleanSource ./.;
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src buildInputs;
          };
          spacerobo = craneLib.buildPackage {
            inherit src cargoArtifacts buildInputs;
            strictDeps = true;
            doCheck = true;
            nativeBuildInputs = [
              pkgs.makeWrapper
            ];

            installPhaseCommand = ''
              # Install commands from:
              # https://github.com/ipetkov/crane/blob/dfd9a8dfd09db9aad544c4d3b6c47b12562544a5/lib/buildPackage.nix

              echo "actually installing contents of $postBuildInstallFromCargoBuildLogOut to $out"
              mkdir -p $out
              find "$postBuildInstallFromCargoBuildLogOut" -mindepth 1 -maxdepth 1 | xargs -r mv -t $out

              # wrapProgram
              wrapProgram $out/bin/spacerobo \
                --set LD_LIBRARY_PATH ${lib.makeLibraryPath buildInputs}
            '';
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts buildInputs;
            cargoClippyExtraArgs = "--verbose -- --deny warning";
          };
          cargo-doc = craneLib.cargoDoc {
            inherit src cargoArtifacts buildInputs;
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          treefmt = {
            projectRootFile = "flake.nix";

            # Nix
            programs.nixfmt.enable = true;

            # Rust
            programs.rustfmt.enable = true;

            # TOML
            programs.taplo.enable = true;

            # GitHub Actions
            programs.actionlint.enable = true;

            # Markdown
            programs.mdformat.enable = true;

            # ShellScript
            programs.shellcheck.enable = true;
            programs.shfmt.enable = true;
          };

          packages = {
            inherit spacerobo;
            default = spacerobo;
            doc = cargo-doc;
          };

          checks = {
            inherit
              spacerobo
              cargo-clippy
              cargo-doc
              ;
          };

          devShells.default = pkgs.mkShell {
            inherit buildInputs;

            nativeBuildInputs = [
              # Rust
              rust

              # Nix
              pkgs.nil
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
