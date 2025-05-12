{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
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
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.rust-analyzer

            # Nix
            pkgs.nil

            # Linker
            pkgs.llvmPackages.clang
            pkgs.llvmPackages.lld
          ];
          spacerobo = pkgs.rustPlatform.buildRustPackage {
            pname = "spacerobo";
            version = "dev";
            src = lib.cleanSource ./.;

            inherit buildInputs nativeBuildInputs;

            cargoLock.lockFile = ./Cargo.lock;

            LIBCLANG_PATH = lib.makeLibraryPath buildInputs;
            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

            postInstall = ''
              wrapProgram $out/bin/spacerobo \
                --set LD_LIBRARY_PATH ${lib.makeLibraryPath buildInputs}
            '';
          };
        in
        {
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
