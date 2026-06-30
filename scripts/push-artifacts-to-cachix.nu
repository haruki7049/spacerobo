#!/usr/bin/env nu

if ($env | get CACHIX_AUTH_TOKEN? | is-empty) {
  error make { msg: "There are no setting for CACHIX_AUTH_TOKEN environment variable." }
}

print "Pushing flake inputs..."
print

nix flake archive --json
  | from json
  | [ ($in | get path), ($in | get inputs | values | get path) ]
  | flatten
  | str join (char nl)
  | cachix push haruki7049

print "Pushing runtime closure..."
print

# x86_64-linux platform configurations
if ((version | get build_target | find linux | is-not-empty) and (version | get build_target | find x86_64 | is-not-empty)) {
  nix build --no-link --print-out-paths '.#nixosConfigurations.tuf-chan.config.system.build.toplevel'
    | cachix push haruki7049
  nix build --no-link --print-out-paths '.#nixosConfigurations.pana-chama.config.system.build.toplevel'
    | cachix push haruki7049
} else if ((version | get build_target | find darwin | is-not-empty) and (version | get build_target | find aarch64 | is-not-empty)) {
  nix build --no-link --print-out-paths '.#darwinConfigurations.enmac.config.system.build.toplevel'
    | cachix push haruki7049
}
