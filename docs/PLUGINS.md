<h1>Plugins</h1>
<p>
  xfetch supports external plugins that run as separate executables. The core binary discovers a plugin,
  sends a JSON request on stdin, and reads a JSON response on stdout. This keeps the plugin system
  decoupled from the core and allows plugins to be installed independently.
</p>

<h2>Discovery</h2>
<p>
  The core looks for plugin executables in the following order:
</p>
<ul>
  <li>Explicit path in the config (if provided)</li>
  <li>PATH using the prefix <strong>xfetch-plugin-</strong></li>
  <li>User plugin directory: <strong>~/.config/xfetch/plugins</strong></li>
  <li>Repository development path: <strong>./plugins/&lt;name&gt;/target/release</strong></li>
</ul>

<h2>Configuration</h2>
<p>
  Plugins are configured in the main config file. Each plugin section defines the name to run
  and any plugin-specific arguments. The plugin value can be a short name or a full path to
  an executable.
</p>

<pre><code class="language-jsonc">{
  "logo_animation": {
    "plugin": "animate-logo",
    "fps": 12,
    "duration_ms": 1200,
    "loop": false
  }
}
</code></pre>

<h2>Protocol</h2>
<p>
  xfetch communicates with plugins using JSON over stdin/stdout. The request includes the logo
  lines and plugin arguments. The response returns either a list of frames (for animation) or
  transformed lines (for colorization).
</p>

<h3>Request</h3>
<pre><code class="language-json">{
  "version": 1,
  "kind": "logo_animation",
  "lines": ["__  __", " \\ \\/ /"],
  "args": {
    "fps": 12,
    "duration_ms": 1200,
    "loop": false
  }
}
</code></pre>

<h3>Response</h3>
<pre><code class="language-json">{
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
</code></pre>

<h2>Animate Logo Plugin</h2>
<p>
  The animate-logo plugin lives in <strong>plugins/animate-logo</strong>. It reads the logo lines
  and outputs animated frames. The core will only run the plugin when stdout is a TTY, so
  non-interactive output stays static.
</p>

<h3>Animation Styles</h3>
<p>
  The plugin supports multiple styles controlled by the <code>style</code> field in the
  <code>logo_animation</code> config section:
</p>
<table>
  <thead>
    <tr><th>Style</th><th>Description</th></tr>
  </thead>
  <tbody>
    <tr><td><code>sweep</code></td><td>Color sweep left-to-right (default)</td></tr>
    <tr><td><code>wave</code></td><td>Sine-wave color pattern that moves across</td></tr>
    <tr><td><code>rainbow</code></td><td>Full RGB rainbow gradient shifting over time</td></tr>
    <tr><td><code>sparkle</code></td><td>Random characters light up in bright colors</td></tr>
    <tr><td><code>breathing</code></td><td>Warm amber fade in/out (breathing effect)</td></tr>
    <tr><td><code>frame</code></td><td>Cycles through multiple ASCII art files</td></tr>
    <tr><td><code>none</code></td><td>No coloring, display as-is</td></tr>
  </tbody>
</table>

<h3>Frame Animation</h3>
<p>
  Use <code>style: "frame"</code> con <code>frames_path</code> para animaciones con
  múltiples frames ASCII. Puedes usar un <strong>solo archivo</strong> con las frames
  separadas por <code>===</code> en su propia línea, o un <strong>array de archivos</strong>.
</p>

<pre><code class="language-jsonc">{
  "ascii": "~/.config/xfetch/logos/blink.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "frame",
    "fps": 4,
    "loop": true,
    "frames_path": "~/.config/xfetch/logos/blink.txt"
  }
}
</code></pre>

<p>O con múltiples archivos:</p>

<pre><code class="language-jsonc">{
  "ascii": "~/.config/xfetch/logos/blink_open.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "frame",
    "fps": 4,
    "frames_path": [
      "~/.config/xfetch/logos/blink_open.txt",
      "~/.config/xfetch/logos/blink_half.txt",
      "~/.config/xfetch/logos/blink_closed.txt",
      "~/.config/xfetch/logos/blink_half.txt"
    ]
  }
}
</code></pre>

<h3>Build and Install</h3>
<pre><code class="language-bash">cargo install --path plugins/animate-logo</code></pre>

<h3>Manual Run</h3>
<pre><code class="language-bash">cat request.json | xfetch-plugin-animate-logo</code></pre>

<h2>End-to-End Test</h2>
<p>
  This example uses the sample ASCII logo stored in the repository and enables the
  animate-logo plugin for a full end-to-end test.
</p>
<ol>
  <li>Install the plugin: <code>cargo install --path plugins/animate-logo</code></li>
  <li>Point the ASCII logo to <code>plugins/animate-logo/assets/xfetch_logo.txt</code></li>
  <li>Enable the plugin in your config:</li>
</ol>

<pre><code class="language-jsonc">{
  "ascii": "/path/to/xfetch/plugins/animate-logo/assets/xfetch_logo.txt",
  "logo_animation": {
    "plugin": "animate-logo",
    "style": "sweep",
    "fps": 12,
    "duration_ms": 1200,
    "loop": false
  }
}
</code></pre>

<p>
  Run <code>xfetch</code> in a TTY-capable terminal to see the animated logo.
</p>

<h2>Notes</h2>
<ul>
  <li>Plugins should write errors to stderr and exit with a non-zero status.</li>
  <li>Plugin output must be valid JSON.</li>
  <li>Animations are only rendered on TTY terminals.</li>
  <li>The core only loops animations when a duration is provided.</li>
</ul>
