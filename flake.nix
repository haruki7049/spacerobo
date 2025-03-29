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
        {
          packages.default = pkgs.stdenv.mkDerivation rec {
            pname = "test-godot3-with-nix";
            version = "0.0.0-dev";
            src = lib.cleanSource ./.;

            nativeBuildInputs = [
              pkgs.godot3-headless
              #pkgs.autoPatchelfHook
            ];

            buildInputs = [
            ];

            buildPhase = ''
              runHook preBuild

              # Cannot create file '/homeless-shelter/.config/godot/projects/...'
              export HOME=$TMPDIR

              # Link the export-templates to the expected location. The --export commands
              # expects the template-file at .../templates/{godot-version}.stable/linux_x11_64_release
              mkdir -p $HOME/.local/share/godot
              ln -s ${pkgs.godot3-export-templates}/share/godot/templates $HOME/.local/share/godot

              mkdir -p $out/share/test-godot3-with-nix

              godot3-headless --export "Linux/X11" $out/share/test-godot3-with-nix/out

              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall

              mkdir -p $out/bin
              ln -s $out/share/test-godot3-with-nix/out $out/bin/test-godot3-with-nix

              # Patch binaries.
              interpreter=$(cat $NIX_CC/nix-support/dynamic-linker)
              #patchelf \
              #  --set-interpreter $interpreter \
              #  --set-rpath ${lib.makeLibraryPath buildInputs} \
              #  $out/share/test-godot3-with-nix/out

              runHook postInstall
            '';

            meta = {
              platforms = lib.platforms.unix;
            };
          };

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.godot_4
            ];
          };
        };
    };
}
