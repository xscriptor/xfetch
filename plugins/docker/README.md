<div align="center">
  <h1> Docker Plugin</h1>
  <p>Displays running Docker container statistics in xfetch.</p>
</div>

<br>

<div align="center">
  <table>
    <tr>
      <td><strong>Kind</strong></td>
      <td><code>info_provider</code></td>
    </tr>
    <tr>
      <td><strong>Binary</strong></td>
      <td><code>xfetch-plugin-docker</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>docker</code> CLI</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/docker/Cargo.toml</code></pre>

<p>The binary will be created at:</p>

<pre><code>plugins/docker/target/release/xfetch-plugin-docker</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install ./plugins/docker</code></pre>

<h2>Configuration</h2>

<p>Add the plugin to your <code>config.jsonc</code>:</p>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "docker"
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:docker",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<p>The docker plugin does not require any arguments. It reads directly from the <code>docker</code> CLI.</p>

<h2>Output</h2>

<table>
  <thead>
    <tr>
      <th>State</th>
      <th>Example Output</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Daemon running with containers</td>
      <td><pre> Containers: 15 total<br>  ▶ 3 running<br>  ⏸ 1 paused<br>  ⏹ 11 stopped</pre></td>
    </tr>
    <tr>
      <td>Docker installed but daemon not running</td>
      <td><code> Docker: daemon not running</code></td>
    </tr>
    <tr>
      <td>Docker CLI not found</td>
      <td><code> Docker: not found</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code>.</li>
  <li>The plugin runs <code>docker info --format '{{.Containers}} {{.ContainersRunning}} ...'</code>.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:docker</code> module key.</li>
</ol>

<h2>Protocol</h2>

<h3>Request</h3>

<pre><code class="language-json">{
  "version": 1,
  "kind": "info_provider",
  "args": null
}</code></pre>

<h3>Response</h3>

<pre><code class="language-json">{
  "lines": [
    " Containers: 15 total",
    "  ▶ 3 running",
    "  ⏸ 1 paused",
    "  ⏹ 11 stopped"
  ]
}</code></pre>

<h2>Notes</h2>

<ul>
  <li>Requires the <code>docker</code> CLI to be installed and available in <code>PATH</code>.</li>
  <li>If the Docker daemon is not running, the plugin reports it gracefully.</li>
  <li>No authentication or configuration is needed.</li>
</ul>
