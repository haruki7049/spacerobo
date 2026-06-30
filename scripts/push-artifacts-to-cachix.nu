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

# Push processes for each packages' caches
nix build --no-link --print-out-paths '.#spacerobo'
  | cachix push haruki7049
nix build --no-link --print-out-paths '.#cargo-doc'
  | cachix push haruki7049
