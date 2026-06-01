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
