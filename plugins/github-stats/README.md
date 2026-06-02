<div align="center">
  <h1> GitHub Stats Plugin</h1>
  <p>Displays GitHub user statistics (repos, followers, following) in xfetch.</p>
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
      <td><code>xfetch-plugin-github-stats</code></td>
    </tr>
    <tr>
      <td><strong>Dependencies</strong></td>
      <td><code>curl</code> CLI</td>
    </tr>
  </table>
</div>

<br>

<h2>Build</h2>

<pre><code>cargo build --release --manifest-path plugins/github-stats/Cargo.toml</code></pre>

<p>The binary will be created at:</p>

<pre><code>plugins/github-stats/target/release/xfetch-plugin-github-stats</code></pre>

<h2>Install</h2>

<pre><code>xfetch plugin install ./plugins/github-stats</code></pre>

<h2>Configuration</h2>

<p>The GitHub username is required. Set it via the config file or the <code>GITHUB_USER</code> environment variable.</p>

<h3>Via config.jsonc (recommended)</h3>

<pre><code class="language-jsonc">{
  "info_plugins": [
    {
      "plugin": "github-stats",
      "args": {
        "username": "xscriptor"
      }
    }
  ],
  "modules": [
    "os",
    "kernel",
    "plugin:github-stats",
    "shell",
    "cpu",
    "memory"
  ]
}</code></pre>

<h3>Via environment variable</h3>

<pre><code>export GITHUB_USER="xscriptor"</code></pre>

<p>You can also optionally provide a GitHub personal access token to avoid rate limits:</p>

<pre><code class="language-jsonc">{
  "plugin": "github-stats",
  "args": {
    "username": "xscriptor",
    "token": "ghp_xxxxxxxxxxxxxxxxxxxx"
  }
}</code></pre>

<p>If both config and environment variable are present, the config value takes precedence.</p>

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
      <td>User found</td>
      <td><pre> Xscriptor (@xscriptor)<br>   42 repos<br>   128 followers<br>   12 following</pre></td>
    </tr>
    <tr>
      <td>Username not configured</td>
      <td><code> GitHub: no username configured</code></td>
    </tr>
    <tr>
      <td>User not found or network error</td>
      <td><code> GitHub: could not fetch user 'xscriptor'</code></td>
    </tr>
  </tbody>
</table>

<h2>How It Works</h2>

<ol>
  <li>xfetch sends a JSON request with <code>kind: "info_provider"</code> and the configured <code>args</code>.</li>
  <li>The plugin fetches <code>https://api.github.com/users/&lt;username&gt;</code> using <code>curl</code>.</li>
  <li>The plugin parses the JSON response and formats the statistics.</li>
  <li>The plugin returns a JSON response with the formatted lines.</li>
  <li>xfetch displays them under the <code>plugin:github-stats</code> module key.</li>
</ol>

<h2>Protocol</h2>

<h3>Request</h3>

<pre><code class="language-json">{
  "version": 1,
  "kind": "info_provider",
  "args": {
    "username": "xscriptor",
    "token": null
  }
}</code></pre>

<h3>Response</h3>

<pre><code class="language-json">{
  "lines": [
    " Xscriptor (@xscriptor)",
    "   42 repos",
    "   128 followers",
    "   12 following"
  ]
}</code></pre>

<h2>Rate Limits</h2>

<p>
  Without authentication, GitHub API is limited to 60 requests per hour.
  If you exceed this limit, the plugin will return an error.
  Provide a <a href="https://github.com/settings/tokens">personal access token</a>
  in the <code>args.token</code> field to increase the limit to 5,000 requests per hour.
</p>

<h2>Notes</h2>

<ul>
  <li>Requires <code>curl</code> to be installed and available in <code>PATH</code>.</li>
  <li>Uses the unauthenticated GitHub API unless a token is provided.</li>
  <li>The token must have at least <code>public_access</code> scope.</li>
  <li>Network connectivity is required for this plugin to function.</li>
</ul>
