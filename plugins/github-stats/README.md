<div align="center">
  <h1> GitHub Stats Plugin</h1>
  <p>Displays GitHub user statistics (stars, repos, PRs, issues, followers) in xfetch.</p>
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
    <tr>
      <td><strong>API calls</strong></td>
      <td><code>/users/:user</code>, <code>/users/:user/repos</code>, <code>/search/issues</code> (x2)</td>
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

<h3>Required — GitHub username</h3>

<p>Set it via config file or <code>GITHUB_USER</code> environment variable.</p>

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
    "plugin:github-stats"
  ]
}</code></pre>

<h3>Optional — Access token</h3>

<p>Without a token, GitHub API is limited to <strong>60 requests/hour</strong>. With a <a href="https://github.com/settings/tokens">personal access token</a> you get <strong>5,000 requests/hour</strong>.</p>

<pre><code class="language-jsonc">{
  "plugin": "github-stats",
  "args": {
    "username": "xscriptor",
    "token": "ghp_xxxxxxxxxxxxxxxxxxxx"
  }
}</code></pre>

<h3>Optional — Limit displayed lines</h3>

<p>Use <code>max_lines</code> to show only the first N stats. The order is always:</p>

<ol>
  <li>Name + username</li>
  <li>Stars</li>
  <li>Repos</li>
  <li>PRs</li>
  <li>Issues</li>
  <li>Followers</li>
  <li>Following</li>
</ol>

<table>
  <thead>
    <tr>
      <th><code>max_lines</code></th>
      <th>Output</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>1</code></td>
      <td><code> X (@xscriptor)</code></td>
    </tr>
    <tr>
      <td><code>3</code></td>
      <td><pre> X (@xscriptor)
 114 stars
 33 repos</pre></td>
    </tr>
    <tr>
      <td><em>omitted</em></td>
      <td>All 7 lines</td>
    </tr>
  </tbody>
</table>

<pre><code class="language-jsonc">{
  "plugin": "github-stats",
  "args": {
    "username": "xscriptor",
    "max_lines": 3
  }
}</code></pre>

<h3>Color</h3>

<p>The plugin uses the <code>plugin:github-stats</code> module key for color and icon lookup in your config.</p>

<pre><code class="language-jsonc">{
  "colors": {
    "plugin:github-stats": "Cyan"
  },
  "icons": {
    "plugin:github-stats": ""
  }
}</code></pre>

<p>Available color names: <code>Black</code>, <code>Red</code>, <code>Green</code>, <code>Yellow</code>, <code>Blue</code>, <code>Magenta</code>, <code>Cyan</code>, <code>White</code>, <code>Grey</code>.</p>

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
      <td>User found (all lines)</td>
      <td><pre> X (@xscriptor)
 114 stars
 33 repos
 0 PRs
 0 issues
 25 followers
 13 following</pre></td>
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
  <li>The plugin fetches from GitHub API:
    <ul>
      <li><code>/users/&lt;username&gt;</code> — name, repos, followers</li>
      <li><code>/users/&lt;username&gt;/repos</code> — stargazers per repo</li>
      <li><code>/search/issues?q=author:&lt;username&gt; type:pr</code> — total PRs</li>
      <li><code>/search/issues?q=author:&lt;username&gt; type:issue</code> — total issues</li>
    </ul>
  </li>
  <li>The plugin parses the JSON responses and formats the statistics.</li>
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
    "token": null,
    "max_lines": null
  }
}</code></pre>

<table>
  <thead>
    <tr>
      <th>Field</th>
      <th>Type</th>
      <th>Required</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>username</code></td>
      <td><code>string</code></td>
      <td>Yes*</td>
      <td>GitHub username. Falls back to <code>GITHUB_USER</code> env var.</td>
    </tr>
    <tr>
      <td><code>token</code></td>
      <td><code>string</code></td>
      <td>No</td>
      <td>GitHub personal access token for higher rate limits.</td>
    </tr>
    <tr>
      <td><code>max_lines</code></td>
      <td><code>number</code></td>
      <td>No</td>
      <td>Limit output to first N lines (1-7).</td>
    </tr>
  </tbody>
</table>

<p><em>* If neither config username nor <code>GITHUB_USER</code> env var is set, the plugin returns an error message.</em></p>

<h3>Response</h3>

<pre><code class="language-json">{
  "lines": [
    " X (@xscriptor)",
    " 114 stars",
    " 33 repos",
    " 0 PRs",
    " 0 issues",
    " 25 followers",
    " 13 following"
  ]
}</code></pre>

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
      <td><code>lines</code></td>
      <td><code>array[string]</code></td>
      <td>Formatted stat lines, one per element.</td>
    </tr>
  </tbody>
</table>

<h2>Rate Limits</h2>

<p>
  Without authentication, GitHub API is limited to <strong>60 requests per hour</strong>
  (this plugin makes 4 requests per run). If you exceed this limit, the plugin will return an error.
  Provide a <a href="https://github.com/settings/tokens">personal access token</a>
  in the <code>args.token</code> field to increase the limit to <strong>5,000 requests per hour</strong>.
</p>

<h2>Tips</h2>

<ul>
  <li>
    <strong>Minimal output</strong> — Set <code>max_lines: 1</code> to show only your GitHub identity line.
  </li>
  <li>
    <strong>Quick overview</strong> — <code>max_lines: 3</code> shows name + stars + repos.
  </li>
  <li>
    <strong>Full profile</strong> — Omit <code>max_lines</code> to show all 7 stats.
  </li>
  <li>
    <strong>Token security</strong> — Never commit your token to version control. Use environment variables or a secrets manager.
  </li>
</ul>

<h2>Notes</h2>

<ul>
  <li>Requires <code>curl</code> to be installed and available in <code>PATH</code>.</li>
  <li>Uses the unauthenticated GitHub API unless a token is provided.</li>
  <li>The token must have at least <code>public_access</code> scope.</li>
  <li>Network connectivity is required for this plugin to function.</li>
  <li>Each xfetch run makes 4 API calls to gather all stats.</li>
</ul>
