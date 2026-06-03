# Installation Guide

This guide covers the complete installation process for **xfetch** on Linux, macOS, and Windows.

---

## Prerequisites

- **git** — for cloning the repository (not needed for the remote one-liner install)
- **Rust/Cargo** — the build toolchain. If not installed, the installer can set it up via [rustup](https://rustup.rs/)
- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **Linux**: `build-essential`, `pkg-config`, `libssl-dev` (Debian/Ubuntu) or `base-devel` (Arch)

---

## Quick Install (Recommended)

The fastest way to install xfetch.

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/install.sh | bash
```

If `cargo` is not installed, the script will offer to install Rust via rustup automatically.

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/xscriptor/xfetch/main/install.ps1 | iex
```

### What the Script Does

1. Checks for Rust (offers to install it if missing)
2. Clones the repository
3. Builds the binary with `cargo build --release`
4. Installs it to `~/.local/bin/`
5. Sets up default config files in `~/.config/xfetch/`
6. Adds `~/.local/bin` to your PATH (via `~/.bashrc`, `~/.zshrc`, etc.)

### Install Script Options

The install script supports several flags for customization:

```bash
# Install to a custom prefix
bash <(curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/install.sh) --prefix /usr/local

# Skip PATH modification
bash <(curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/install.sh) --no-modify-path

# Install from a local clone of the repository
git clone https://github.com/xscriptor/xfetch.git
cd xfetch
bash install.sh --local

# Non-interactive install (auto-yes to prompts)
bash install.sh --local --yes
```

For all available flags:
```bash
bash <(curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/install.sh) --help
```

---

## Local Install

If you have already cloned the repository, run the installer directly from the project root:

```bash
cd xfetch
bash install.sh --local
```

This skips the git clone step and builds from your local copy.

---

## Build from Source (Manual)

For full control over the build:

```bash
# Clone
git clone https://github.com/xscriptor/xfetch.git
cd xfetch

# Build release binary
cargo build --release

# The binary is at: target/release/xfetch
# Install it manually:
cp target/release/xfetch ~/.local/bin/

# Set up config
mkdir -p ~/.config/xfetch
cp configs/config.jsonc ~/.config/xfetch/config.jsonc
cp -r logos/* ~/.config/xfetch/logos/
```

---

## Install via Cargo

```bash
cargo install --path .
```

This installs to `~/.cargo/bin/` (ensure it is in your PATH).

---

## Arch Linux (PKGBUILD)

This method installs xfetch as a proper Arch package, making it easy to update and remove.

```bash
git clone https://github.com/xscriptor/xfetch.git
cd xfetch
makepkg -si
```

Installs system-wide to `/usr/bin/xfetch`.

To uninstall the package:
```bash
sudo pacman -R xfetch-git
```

---

## Verifying Installation

After installing, verify xfetch works:

```bash
xfetch --version
```

You should see version output. Then run it to test the display:

```bash
xfetch
```

### Troubleshooting "command not found"

If you get a "command not found" error:

- **Restart your terminal**, or
- Run `source ~/.bashrc` (or `source ~/.zshrc`), or
- Manually add `~/.local/bin` to your PATH:
  ```bash
  export PATH="$HOME/.local/bin:$PATH"
  ```

---

## Uninstallation

See the [Uninstallation Guide](UNINSTALLATION.md) for detailed instructions.

**Quick uninstall:**
```bash
curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/uninstall.sh | bash
```

**Manual removal:**
```bash
rm -f ~/.local/bin/xfetch
rm -rf ~/.config/xfetch
```

Also remove the PATH line from `~/.bashrc`, `~/.zshrc`, or `~/.bash_profile`:
```bash
# xfetch path
export PATH="$HOME/.local/bin:$PATH"
```

---

## Next Steps

- [Configuration Guide](CONFIGURATION.md) — customize modules, logos, colors, and layouts
- [Layouts Guide](LAYOUTS.md) — explore built-in display layouts
- [Plugins Guide](PLUGINS.md) — extend xfetch with external plugins
