<h1 align="center"> Xfetch </h1>

<div align="center">

![xfetch](https://xscriptor.github.io/badges/software/xfetch.svg) ![linux](https://xscriptor.github.io/badges/os/linux.svg) ![macos](https://xscriptor.github.io/badges/os/macos.svg) ![windows](https://xscriptor.github.io/badges/os/windows.svg) ![rust](https://xscriptor.github.io/badges/languages/rust.svg) ![mit](https://xscriptor.github.io/badges/licenses/mit.svg)

<p>A cross-platform system information fetching tool inspired by fastfetch and neofetch, written in Rust.</p>

</div>


<p align="center"><img src="./assets/icon.png" width="100" alt="Xscriptor logo" /></p>

<h2 align="center"> Previews</h2>

<p align="center">
  <a href="./assets/previews/preview0.jpg">
    <img src="./assets/previews/preview0.jpg" alt="Main preview" width="850"/>
  </a>
</p>

<details>
  <summary>More previews</summary>

  <table>
    <tr>
      <td align="center">
        <a href="./assets/previews/preview1.png">
          <img src="./assets/previews/preview1.png" alt="Preview 2" width="380"/>
        </a>
      </td>
      <td align="center">
        <a href="./assets/previews/preview2.jpg">
          <img src="./assets/previews/preview2.jpg" alt="Preview 3" width="380"/>
        </a>
      </td>
    </tr>
  </table>
</details>


## Quick Install

<h3> Linux / macOS </h3>

```bash
curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/install.sh | bash
```

<h3> Windows (PowerShell)</h3>

```powershell
irm https://raw.githubusercontent.com/xscriptor/xfetch/main/install.ps1 | iex
```

<h2 align="center"> Features</h2>

- **Cross-platform**: Works on Linux, Windows, and macOS.
- **Customizable**: Configure modules via `config.jsonc`.
- **Fast**: Written in Rust for performance.

<h2 align="center"> Installation </h2>

<h3> From Source </h3>

1. Ensure you have Rust installed.
2. Clone the repository.
3. Build and run:

```bash
cargo run --release
```

Or install locally:

```bash
cargo install --path .
```

<h2 align="center"> Configuration </h2>

xfetch looks for a configuration file at:

- **Linux**: `~/.config/xfetch/config.jsonc`
- **Windows**: `%APPDATA%\xfetch\config.jsonc`
- **macOS**: `~/Library/Application Support/xfetch/config.jsonc`

### Example Config (`config.jsonc`)

```jsonc
// Configuration for xfetch
{
  // Path to custom ASCII art file (optional)
  "ascii": null, 
  // Modules to display
  "modules": [
    "os",
    "kernel",
    "wm",
    "packages",
    "shell",
    "cpu",
    "gpu",
    "memory",
    "disk",
    "battery",
    "uptime",
    "terminal"
  ],
  // Enable colors
  "show_colors": true
}
```

<h2 align="center"> Usage</h2>

Simply run `xfetch` in your terminal.

```bash
xfetch
```

<p align="right">You can also specify a config file via CLI args (not yet fully implemented in CLI but supported in code structure).</p>

<p align="center">If you want to contribute to the project, report bugs or security issues, or learn more about the developer, use the links below:</p>
<div align="center">
<a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/LICENSE">MIT-License</a> | <a href="https://github.com/xscriptor/">X</a> | <a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/CONTRIBUTE.md">Contribute</a> | <a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/SECURITY.md">Security</a> | <a href="https://dev.xscriptor.com">DevWeb</a>  
</div>

