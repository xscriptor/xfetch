<h1>Developing Plugins for xfetch</h1>

<p>
  xfetch plugins are standalone executables that communicate with the core via
  stdin/stdout using a JSON protocol. This document describes the protocol, the
  required binary naming conventions, and best practices for creating your own
  plugin.
</p>

<h2>Table of Contents</h2>

<ul>
  <li><a href="#how-it-works">How It Works</a></li>
  <li><a href="#binary-naming">Binary Naming</a></li>
  <li><a href="#discovery">Discovery</a></li>
  <li><a href="#protocol">Protocol</a></li>
  <li><a href="#request-format">Request Format</a></li>
  <li><a href="#response-format">Response Format</a></li>
  <li><a href="#example-plugin">Example Plugin</a></li>
  <li><a href="#testing">Testing</a></li>
  <li><a href="#contribution-guidelines">Contribution Guidelines</a></li>
</ul>

<h2 id="how-it-works">How It Works</h2>

<ol>
  <li>xfetch discovers the plugin binary (see <a href="#discovery">Discovery</a>).</li>
  <li>xfetch spawns the plugin process.</li>
  <li>xfetch writes a JSON request to the plugin's stdin.</li>
  <li>The plugin processes the request and writes a JSON response to stdout.</li>
  <li>xfetch reads the response and uses it for rendering.</li>
  <li>Errors are written to stderr; the plugin exits with a non-zero status on failure.</li>
</ol>

<h2 id="binary-naming">Binary Naming</h2>

<p>
  The plugin binary must follow this naming convention so xfetch can discover it:
</p>

<pre><code>xfetch-plugin-&lt;name&gt;</code></pre>

<table>
  <thead>
    <tr>
      <th>Platform</th>
      <th>Example</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Linux / macOS</td>
      <td><code>xfetch-plugin-animate-logo</code></td>
    </tr>
    <tr>
      <td>Windows</td>
      <td><code>xfetch-plugin-animate-logo.exe</code></td>
    </tr>
  </tbody>
</table>

<p>
  The Cargo.toml <code>name</code> field should follow the same convention:
</p>

<pre><code class="language-toml">[package]
name = "xfetch-plugin-&lt;name&gt;"
version = "0.1.0"
edition = "2024"
</code></pre>

<h2 id="discovery">Discovery</h2>

<p>
  When running a plugin, xfetch searches for the binary in the following order:
</p>

<ol>
  <li>Explicit path from the configuration (if provided).</li>
  <li><code>PATH</code> environment variable, looking for <code>xfetch-plugin-&lt;name&gt;</code>.</li>
  <li>User plugin directory: <code>~/.config/xfetch/plugins/</code>.</li>
  <li>Development path: <code>./plugins/&lt;name&gt;/target/release/</code>.</li>
</ol>

<h2 id="protocol">Protocol</h2>

<p>
  The plugin protocol uses JSON over stdin/stdout. The plugin reads exactly one
  JSON object from stdin and writes exactly one JSON object to stdout.
</p>

<h3 id="request-format">Request Format</h3>

<pre><code class="language-json">{
  "version": 1,
  "kind": "&lt;plugin_kind&gt;",
  "lines": ["line 1", "line 2"],
  "frames": [
    ["frame 1 line 1", "frame 1 line 2"],
    ["frame 2 line 1", "frame 2 line 2"]
  ],
  "args": {
    "fps": 12,
    "duration_ms": 1200,
    "loop": false,
    "style": "sweep"
  }
}
</code></pre>

<table>
  <thead>
    <tr>
      <th>Field</th>
      <th>Type</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>version</code></td>
      <td><code>u32</code></td>
      <td>Protocol version (currently <code>1</code>).</td>
    </tr>
    <tr>
      <td><code>kind</code></td>
      <td><code>string</code></td>
      <td>Plugin type. Currently only <code>"logo_animation"</code> is supported.</td>
    </tr>
    <tr>
      <td><code>lines</code></td>
      <td><code>array[string]</code></td>
      <td>Logo ASCII art, one line per element.</td>
    </tr>
    <tr>
      <td><code>frames</code></td>
      <td><code>array[array[string]]</code> or <code>null</code></td>
      <td>Optional pre-loaded frame sets for frame-based animation.</td>
    </tr>
    <tr>
      <td><code>args</code></td>
      <td><code>object</code></td>
      <td>Plugin-specific arguments from the user config.</td>
    </tr>
  </tbody>
</table>

<h3 id="response-format">Response Format</h3>

<pre><code class="language-json">{
  "frames": [
    {
      "delay_ms": 83,
      "lines": ["colored line 1", "colored line 2"]
    }
  ]
}
</code></pre>

<table>
  <thead>
    <tr>
      <th>Field</th>
      <th>Type</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>frames</code></td>
      <td><code>array[Frame]</code></td>
      <td>Array of animation frames (<strong>required</strong>). Must not be empty.</td>
    </tr>
  </tbody>
</table>

<h4>Frame Object</h4>

<table>
  <thead>
    <tr>
      <th>Field</th>
      <th>Type</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>delay_ms</code></td>
      <td><code>u64</code></td>
      <td>Milliseconds to display this frame before advancing.</td>
    </tr>
    <tr>
      <td><code>lines</code></td>
      <td><code>array[string]</code></td>
      <td>Frame content (may include ANSI escape codes for color).</td>
    </tr>
  </tbody>
</table>

<h2 id="example-plugin">Example Plugin</h2>

<p>
  The <a href="../animate-logo/"><code>animate-logo</code></a> plugin is the reference
  implementation. Its source is at <code>plugins/animate-logo/src/main.rs</code>.
</p>

<p>Minimal plugin skeleton in Rust:</p>

<pre><code class="language-rust">use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize)]
struct PluginRequest {
    version: Option&lt;u32&gt;,
    kind: Option&lt;String&gt;,
    lines: Vec&lt;String&gt;,
    frames: Option&lt;Vec&lt;Vec&lt;String&gt;&gt;&gt;,
    args: Option&lt;PluginArgs&gt;,
}

#[derive(Debug, Deserialize)]
struct PluginArgs {
    fps: Option&lt;u64&gt;,
    duration_ms: Option&lt;u64&gt;,
    #[serde(rename = "loop")]
    loop_enabled: Option&lt;bool&gt;,
    style: Option&lt;String&gt;,
}

#[derive(Debug, Serialize)]
struct PluginResponse {
    frames: Vec&lt;Frame&gt;,
}

#[derive(Debug, Serialize)]
struct Frame {
    delay_ms: u64,
    lines: Vec&lt;String&gt;,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&amp;mut input).is_err() {
        return;
    }

    let request: PluginRequest = match serde_json::from_str(&amp;input) {
        Ok(value) =&gt; value,
        Err(_) =&gt; return,
    };

    // Your animation logic here...

    let response = PluginResponse { frames };
    if let Ok(body) = serde_json::to_string(&amp;response) {
        let _ = io::stdout().write_all(body.as_bytes());
    }
}
</code></pre>

<h2 id="testing">Testing</h2>

<p>Test your plugin manually by writing a request file and piping it:</p>

<pre><code class="language-bash">echo '{"version":1,"kind":"logo_animation","lines":["hello"],"args":{"fps":12}}' \
  | ./target/release/xfetch-plugin-my-plugin
</code></pre>

<p>
  You can also install it locally and test it with xfetch:
</p>

<pre><code class="language-bash">xfetch plugin install ./plugins/my-plugin
xfetch
</code></pre>

<h2 id="contribution-guidelines">Contribution Guidelines</h2>

<h3>Adding a Plugin</h3>

<ol>
  <li>
    Create a new directory under <code>plugins/</code> named after your plugin
    (e.g., <code>plugins/my-plugin/</code>).
  </li>
  <li>
    Include a <code>Cargo.toml</code> with the package name following the
    <code>xfetch-plugin-&lt;name&gt;</code> convention.
  </li>
  <li>
    Implement the plugin with a <code>src/main.rs</code> or <code>src/lib.rs</code>
    that handles the JSON protocol.
  </li>
  <li>
    Add a <code>README.md</code> documenting the plugin's features, configuration
    options, and available styles.
  </li>
  <li>
    Add sample configs and assets if applicable (see
    <code>plugins/animate-logo/</code> for reference).
  </li>
</ol>

<h3>Code Standards</h3>

<ul>
  <li>Write all code, comments, and documentation in English.</li>
  <li>Use <code>serde</code> and <code>serde_json</code> for JSON serialization.</li>
  <li>Handle errors gracefully — write errors to stderr and exit with non-zero.</li>
  <li>Do not use external display or terminal libraries; the core handles rendering.</li>
  <li>Keep the plugin focused on a single responsibility (e.g., logo animation).</li>
  <li>Minimize dependencies to keep build times fast.</li>
</ul>

<h3>Pull Request Process</h3>

<ol>
  <li>Fork the repository and create a feature branch.</li>
  <li>Implement your plugin following the guidelines above.</li>
  <li>Ensure <code>cargo build --release</code> succeeds for your plugin.</li>
  <li>Add an entry for your plugin in <a href="../README.md">plugins/README.md</a>.</li>
  <li>Submit a pull request with a clear description of your plugin.</li>
  <li>Maintainers will review the protocol compliance and code quality.</li>
</ol>

<h3>What Not to Do</h3>

<ul>
  <li>Do not modify core xfetch source files unless the plugin protocol itself needs changes.</li>
  <li>Do not use network calls or file I/O in animation plugins (they should be pure transformations).</li>
  <li>Do not embed binaries or large assets in the plugin source.</li>
  <li>Do not introduce platform-specific code without fallbacks.</li>
</ul>
