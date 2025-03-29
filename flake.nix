{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default-linux";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem =
        {
          pkgs,
          lib,
          ...
        }:
        let
          spacerobo = pkgs.stdenv.mkDerivation {
            pname = "spacerobo";
            version = "0.1.0-dev";
            src = lib.cleanSource ./.;

            nativeBuildInputs = [
              pkgs.godot_4
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

              godot4 --headless --export-debug "${pkgs.stdenv.system}" $out/share/spacerobo/out

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
