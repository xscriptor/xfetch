<h1 align="center"> Xfetch </h1>

<div align="center">
<p align="center"><img src="./assets/icon.png" width="43" alt="Xscriptor logo" /></p>

![xfetch](https://xscriptor.github.io/badges/software/xfetch.svg) ![linux](https://xscriptor.github.io/badges/os/linux.svg) ![macos](https://xscriptor.github.io/badges/os/macos.svg) ![windows](https://xscriptor.github.io/badges/os/windows.svg) ![rust](https://xscriptor.github.io/badges/languages/rust.svg) ![mit](https://xscriptor.github.io/badges/licenses/mit.svg)

<p>A cross-platform system information fetching tool inspired by fastfetch and neofetch, written in Rust.</p>

<!--Menu-->
<div align="left">
  <h2>Menu</h2>
  <ul>
    <li><a href="#previews">Previews</a></li>
    <li><a href="#quick-install">Quick Install</a></li>
    <li><a href="#features">Features</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#configuration">Configuration</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#related-documents">Related Documents</a></li>
    <li><a href="#contribute">Contribute</a></li>
    <li><a href="#security">Security</a></li>
    <li><a href="#about-the-developer">About</a></li>
  </ul>
</div>


<!-- previews-->
</div>

<h2  id="previews" align="center"> Previews</h2>

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


<h2 id="quick-install" align="center"> Quick Install</h2>

<h3> Linux / macOS </h3>

```bash
curl -fsSL https://raw.githubusercontent.com/xscriptor/xfetch/main/install.sh | bash
```

<h3> Windows (PowerShell)</h3>

```powershell
irm https://raw.githubusercontent.com/xscriptor/xfetch/main/install.ps1 | iex
```

<h2 id="features" align="center"> Features</h2>

- **Cross-platform**: Works on Linux, Windows, and macOS.
- **Customizable**: Configure modules via `config.jsonc`.
- **Fast**: Written in Rust for performance.

<h2 id="installation" align="center"> Installation </h2>

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

<h2 id="configuration" align="center"> Configuration </h2>

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

<h2 id="usage" align="center"> Usage</h2>

Simply run `xfetch` in your terminal.

```bash
xfetch
```

<h2 id="related-documents" align="center">Related Documents</h2>

<div align="left">
  <ul>
    <li><a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/docs/INSTALLATION.md">Installation</a></li>
    <li><a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/docs/CONFIGURATION.md">Config</a></li>
    <li><a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/docs/LAYOUTS.md">Layouts</a></li>
    <li><a href="https://raw.githubusercontent.com/xscriptor/xfetch/main/docs/UNINSTALLATION.md">Uninstall</a></li>
  </ul>
</div>

<h3 align="center">Contribute to the project, report issues, or connect with the developer using the links below</h3>

<div align="center">
<a id= "contribute" href="https://github.com/xscriptor/xfetch/blob/main/LICENSE">MIT-License</a> | <a href="https://github.com/xscriptor/">X</a> | <a id="security" href="https://github.com/xscriptor/xfetch/blob/main/CONTRIBUTING.md">Contribute</a> | <a href="https://github.com/xscriptor/xfetch/blob/main/SECURITY.md">Security</a> | <a id="about-the-developer" href="https://dev.xscriptor.com">DevWeb</a>  
</div>

