# spacerobo

**This game is not stable yet.**

Spacerobo is a game to simulate a robo's moving in space.

## Requirements

Spacerobo is built with [Bevy](https://bevy.org). It needs a display and a GPU with a Vulkan, Metal or DirectX 12
driver. On Linux, the ALSA, udev, X11 or Wayland development libraries are needed to build it; the Nix development
shell provides them.

## Run

With [Nix](https://nixos.org) (flakes enabled):

```bash
nix run github:haruki7049/spacerobo
```

From a clone of this repository, with a Rust toolchain matching `rust-toolchain.toml` (inside the Nix development shell
it is already provided):

```bash
cargo run
```

The binary is called `spr`. Run `spr --help` to list its options.

## Configuration

Spacerobo reads a TOML configuration file. The default location is the `config.toml` of the platform's configuration
directory for `dev.haruki7049.spacerobo`, for example `~/.config/spacerobo/config.toml` on Linux. Use another file
with:

```bash
spr --config-file path/to/config.toml
```

If the file cannot be loaded, the game runs with its default settings. See [docs/README.md](docs/README.md) for every
setting and its default value.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
