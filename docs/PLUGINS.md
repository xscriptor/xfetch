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
  and outputs animated frames by applying a moving color sweep. The core will only run the plugin
  when stdout is a TTY, so non-interactive output stays static.
</p>

<h3>Build and Install</h3>
<pre><code class="language-bash">cargo install --path plugins/animate-logo</code></pre>

<h3>Manual Run</h3>
<pre><code class="language-bash">cat request.json | xfetch-plugin-animate-logo</code></pre>

<h2>Notes</h2>
<ul>
  <li>Plugins should write errors to stderr and exit with a non-zero status.</li>
  <li>Plugin output must be valid JSON.</li>
  <li>Animations are only rendered on TTY terminals.</li>
  <li>The core only loops animations when a duration is provided.</li>
</ul>
