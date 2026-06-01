# Animate Logo Plugin

This plugin animates the ASCII logo by applying a moving color sweep across characters. It is designed
to be invoked by xfetch through stdin/stdout JSON.

## Build

```bash
cargo build --release
```

The binary will be created at:

```
plugins/animate-logo/target/release/xfetch-plugin-animate-logo
```

## Install

```bash
cargo install --path plugins/animate-logo
```

## Sample Animated Logo

A sample ASCII logo is included at:

```
plugins/animate-logo/assets/xfetch_logo.txt
```

To test end-to-end animation:

1. Install the plugin (see above).
2. Set the ASCII logo path and the animation plugin in your config:

```jsonc
{
  "ascii": "/path/to/xfetch/plugins/animate-logo/assets/xfetch_logo.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "fps": 12,
    "duration_ms": 1200,
    "loop": false
  }
}
```

3. Run `xfetch` in a TTY-capable terminal.

The plugin animates by sweeping colors across the logo characters. If you prefer a
static logo, remove the `logo_animation` section.

## Protocol

### Request

```json
{
  "version": 1,
  "kind": "logo_animation",
  "lines": ["__  __", " \\ \\/ /"],
  "args": {
    "fps": 12,
    "duration_ms": 1200,
    "loop": false
  }
}
```

### Response

```json
{
  "frames": [
    {
      "delay_ms": 80,
      "lines": ["__  __", " \\ \\/ /"]
    },
    {
      "delay_ms": 80,
      "lines": ["__  __", "  \\/\\ /"]
    }
  ]
}
```

## Notes

- The plugin ignores the `loop` flag and always outputs a finite set of frames.
- Errors should be printed to stderr and the process should exit with a non-zero status.
