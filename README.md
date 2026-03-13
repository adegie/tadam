# tadam

`tadam` is a tiny Linux-style CLI that waits for piped input to finish and then plays a sound.

Example:

```bash
ls -l | tadam
```

It forwards stdin to stdout by default, so you still see the original command output.

## Build

```bash
cargo build --release
```

Binary path:

```bash
./target/release/tadam
```

## Install

```bash
cargo install --path .
```

This installs `tadam` into Cargo's bin directory (usually `~/.cargo/bin`).

## Usage

```bash
tadam [--sound <FILE>] [--quiet]
```

Generate shell completions:

```bash
tadam completions <bash|zsh|fish>
```

- `--sound <FILE>`: play a custom audio file instead of the embedded default
- `--quiet`: consume piped input without writing it back to stdout

Install completions:

```bash
# Bash
mkdir -p ~/.local/share/bash-completion/completions
tadam completions bash > ~/.local/share/bash-completion/completions/tadam

# Zsh
mkdir -p ~/.zsh/completions
tadam completions zsh > ~/.zsh/completions/_tadam

# Fish
mkdir -p ~/.config/fish/completions
tadam completions fish > ~/.config/fish/completions/tadam.fish
```

## Default Sound

The default embedded sound is `assets/gentle_breeze_and_birds_singing.ogg`.

Source and licensing:

- Wikimedia Commons: <https://commons.wikimedia.org/wiki/File:Gentle_breeze_and_birds_singing.ogg>
- Original source listed there: <http://www.pdsounds.org/audio/download/129/wind_and_birds_singing.mp3>
- License: public domain (as stated on the Wikimedia file page)
