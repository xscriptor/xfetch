# Animate Logo Plugin

This plugin animates the ASCII logo. It supports multiple animation styles and
frame-based ASCII art. It is designed to be invoked by xfetch through stdin/stdout JSON.

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

## Animation Styles

Add a `"style"` field to `logo_animation` in your config:

### `sweep` (default)

Colors sweep left-to-right across characters using the ANSI palette (red, green,
yellow, blue, magenta, cyan).

```jsonc
{
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "sweep",
    "fps": 12,
    "duration_ms": 1200,
    "loop": false
  }
}
```

### `wave`

A sine-wave color pattern that moves across the logo.

```jsonc
"style": "wave"
```

### `rainbow`

Full RGB rainbow gradient that shifts over time.

```jsonc
"style": "rainbow"
```

### `sparkle`

Random characters light up in bright colors.

```jsonc
"style": "sparkle"
```

### `breathing`

All characters fade in and out in a warm amber tone (breathing effect).

```jsonc
"style": "breathing"
```

### `frame` — ASCII Frame Animation

Cycles through multiple ASCII art frames to create animations. You can define them
as an **array of files** or in a **single file** separated by `===`:

#### Single file with `===` separator

Create a file (e.g. `blink.txt`) with all frames separated by `===`:

```
  .---.
 /     \
| O   O |
|   ^   |
 \_____/
===
  .---.
 /     \
| O   O |
|  ---  |
 \_____/
===
  .---.
 /     \
| O   O |
|       |
 \_____/
===
  .---.
 /     \
| O   O |
|  ---  |
 \_____/
```

```jsonc
{
  "ascii": "~/.config/xfetch/logos/blink.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "frame",
    "fps": 4,
    "loop": true,
    "frames_path": "~/.config/xfetch/logos/blink.txt"
  }
}
```

#### Multiple files

```jsonc
{
  "ascii": "~/.config/xfetch/logos/blink_open.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "frame",
    "fps": 4,
    "loop": true,
    "frames_path": [
      "~/.config/xfetch/logos/blink_open.txt",
      "~/.config/xfetch/logos/blink_half.txt",
      "~/.config/xfetch/logos/blink_closed.txt",
      "~/.config/xfetch/logos/blink_half.txt"
    ]
  }
}
```

> The `ascii` field is used as a static fallback.
> Use low `fps` (2-6) for frame animations. The delimiter must be on its own line.

### `none`

No coloring — displays the logo as-is (useful for logos that already have ANSI
colors embedded).

```jsonc
"style": "none"
```

## Sample Animations

### xfetch Logo (color sweep)

```
plugins/animate-logo/assets/xfetch_logo.txt
```

```jsonc
{
  "ascii": "~/.config/xfetch/logos/xfetch_logo.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "sweep",
    "fps": 12,
    "duration_ms": 1200
  }
}
```

### Blinking Eye (frame animation)

Included in a single file:

```
plugins/animate-logo/assets/blink.txt
```

```jsonc
{
  "ascii": "~/.config/xfetch/logos/blink.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "frame",
    "fps": 4,
    "loop": true,
    "frames_path": "~/.config/xfetch/logos/blink.txt"
  }
}
```

## Protocol

### Request

```json
{
  "version": 1,
  "kind": "logo_animation",
  "lines": ["__  __", " \\ \\/ /"],
  "frames": [
    ["frame1_line1", "frame1_line2"],
    ["frame2_line1", "frame2_line2"]
  ],
  "args": {
    "fps": 12,
    "duration_ms": 1200,
    "loop": false,
    "style": "sweep"
  }
}
```

### Response

```json
{
  "frames": [
    {
      "delay_ms": 83,
      "lines": ["\u001b[31m__\u001b[0m\u001b[32m  \u001b[0m\u001b[33m__\u001b[0m"]
    }
  ]
}
```

## Notes

- The `frames` field in the request is only populated when `style: "frame"` and
  `frames_path` is configured.
- The plugin ignores the `loop` flag and always outputs a finite set of frames.
- Errors should be printed to stderr and the process should exit with a non-zero status.
