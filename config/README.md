# Sample Config (Pacman + Animated Logo)

This folder contains an example config that uses the pacman layout and the animate-logo plugin.

## Files

- config/xfetch_pacman_animate.jsonc
  Example configuration file.

- plugins/animate-logo/assets/xfetch_logo.txt
  Sample ASCII logo file used by the animation.

## Where to put each thing

1. Config file
   - Default location (Linux): ~/.config/xfetch/config.jsonc
   - Optionally run with a custom config path:
     xfetch --config /home/xscriptor/x/xfetch/config/xfetch_pacman_animate.jsonc

2. ASCII logo file
   - Copy the sample logo to:
     ~/.config/xfetch/logos/xfetch_logo.txt
   - The example config points to that path using:
     "ascii": "~/.config/xfetch/logos/xfetch_logo.txt"

3. Plugin binary
   - Build and install the plugin:
     cargo install --path plugins/animate-logo
   - Make sure ~/.cargo/bin is in PATH so xfetch can find:
     xfetch-plugin-animate-logo

## Quick test

1. Install xfetch and the plugin.
2. Copy the logo to ~/.config/xfetch/logos/xfetch_logo.txt.
3. Run:
   xfetch --config /home/xscriptor/x/xfetch/config/xfetch_pacman_animate.jsonc

The animation runs only in TTY terminals.
