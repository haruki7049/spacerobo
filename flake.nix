{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default-linux";
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
          ...
        }:
        let
          spacerobo = pkgs.stdenv.mkDerivation rec {
            pname = "spacerobo";
            version = "0.1.0-dev";
            src = lib.cleanSource ./.;

            nativeBuildInputs = [
              pkgs.godot_4
              pkgs.autoPatchelfHook
              pkgs.makeWrapper
            ];

            buildInputs = lib.optionals pkgs.stdenv.isLinux [
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXext
              pkgs.xorg.libXinerama
              pkgs.xorg.libXrandr
              pkgs.xorg.libXi
              pkgs.libGL
              pkgs.systemd
              pkgs.libxkbcommon
              pkgs.alsa-lib
              pkgs.libpulseaudio
              pkgs.dbus
              pkgs.fontconfig.lib
            ];

            buildPhase = ''
              runHook preBuild

              # Cannot create directories '/homeless-shelter/.config/godot/projects/...' and '/homeless-shelter/.local/share/godot/export_templates/...'
              export HOME=$TMPDIR

              # Link the export-templates to the expected location. The --export commands
              # expects the template-file at .../export_templates/{godot-version}.stable/linux_x11_64_release
              mkdir -p $HOME/.local/share/godot/export_templates/
              ln -s ${pkgs.godot_4-export-templates} $HOME/.local/share/godot/export_templates/4.4.1.stable

              mkdir -p $out/share/spacerobo

              # The godot exporting for macOS creates universal binary
              godot4 --headless --export-debug "${
                if pkgs.stdenv.isDarwin then "macos" else pkgs.stdenv.system
              }" $out/share/spacerobo/out

              # Add LD_LIBRARY_PATH in runtime environment
              wrapProgram $out/share/spacerobo/out \
                --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath buildInputs}

              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall

              mkdir -p $out/bin
              ln -s $out/share/spacerobo/out $out/bin/spacerobo

              runHook postInstall
            '';

            meta = {
              platforms = [
                "x86_64-linux"
                "aarch64-linux"
                "aarch64-darwin"
              ];
            };
          };
        in
        {
          treefmt = {
            programs.nixfmt.enable = true;
            programs.gdformat.enable = true;
            programs.actionlint.enable = true;
            programs.mdformat.enable = true;
          };

          packages = {
            inherit spacerobo;
            default = spacerobo;
          };

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.nil
              pkgs.godot_4
            ];
          };
        };
    };
}
