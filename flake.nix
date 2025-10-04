{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
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
          src = lib.cleanSource ./.;
          buildInputs =
            lib.optionals pkgs.stdenv.isLinux [
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
            ]
            ++ [
              pkgs.llvmPackages.libclang.lib
            ];
          nativeBuildInputs = [
            # Build tools
            pkgs.pkg-config
            pkgs.makeWrapper

            # Rust
            rust

            # Nix
            pkgs.nil

            # Linker
            pkgs.llvmPackages.clang
            pkgs.llvmPackages.lld
          ];
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src buildInputs nativeBuildInputs;

            LIBCLANG_PATH = lib.makeLibraryPath buildInputs;
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
          spacerobo = craneLib.buildPackage {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;
            strictDeps = true;
            doCheck = true;

            LIBCLANG_PATH = lib.makeLibraryPath buildInputs;
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            installPhaseCommand = ''
              echo "actually installing contents of $postBuildInstallFromCargoBuildLogOut to $out"
              mkdir -p $out
              find "$postBuildInstallFromCargoBuildLogOut" -mindepth 1 -maxdepth 1 | xargs -r mv -t $out

              echo "Copy assets"
              cp -r assets $out/bin

              wrapProgram $out/bin/spacerobo \
                --set LD_LIBRARY_PATH ${lib.makeLibraryPath buildInputs}
            '';

            meta = {
              licenses = [ lib.licenses.mit ];
              mainProgram = "spacerobo";
            };
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;
            cargoClippyExtraArgs = "--verbose -- --deny warnings";

            LIBCLANG_PATH = lib.makeLibraryPath buildInputs;
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
          cargo-doc = craneLib.cargoDoc {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;

            LIBCLANG_PATH = lib.makeLibraryPath buildInputs;
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
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
            settings.formatter.rustfmt.command = "${rust}/bin/rustfmt";

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
            inherit cargo-clippy;
          };

          devShells.default = pkgs.mkShell {
            inherit buildInputs nativeBuildInputs;

            LIBCLANG_PATH = lib.makeLibraryPath buildInputs;
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
