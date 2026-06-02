<div align="center">
  <h1> Contributing to xfetch</h1>
  <p>Thank you for your interest in contributing to xfetch!</p>
</div>

<br>

<h2>Code of Conduct</h2>

<p>
  This project is open and welcoming. Be respectful, constructive, and collaborative.
  Harassment, trolling, and personal attacks are not tolerated.
</p>

<h2>How to Contribute</h2>

<ol>
  <li><strong>Fork</strong> the repository on GitHub.</li>
  <li><strong>Clone</strong> your fork locally.</li>
  <li>Create a <strong>feature branch</strong> (<code>git checkout -b feature/my-change</code>).</li>
  <li>Make your changes following the project conventions.</li>
  <li>Run <code>cargo build</code> and <code>cargo test</code> to verify everything works.</li>
  <li><strong>Commit</strong> your changes with a clear message.</li>
  <li><strong>Push</strong> to your fork.</li>
  <li>Open a <strong>Pull Request</strong> from your branch to the main repository.</li>
</ol>

<h2>Contributing Plugins</h2>

<p>
  Plugins are standalone executables that extend xfetch's functionality.
  To add a new plugin to the repository, follow this process:
</p>

<h3>1. Fork & Clone</h3>

<p>Fork the repository on GitHub and clone your fork locally:</p>

<pre><code>git clone https://github.com/YOUR_USERNAME/xfetch.git
cd xfetch
git checkout -b plugin-my-plugin</code></pre>

<h3>2. Create the Plugin Directory</h3>

<p>Create your plugin under the <code>plugins/</code> directory following the naming convention:</p>

<pre><code>plugins/my-plugin/
├── Cargo.toml        # name = "xfetch-plugin-&lt;name&gt;"
├── README.md         # Documentation (HTML format, centered titles)
└── src/
    └── main.rs       # Plugin implementation</code></pre>

<h3>3. Implement the Protocol</h3>

<p>Your plugin must communicate with xfetch via stdin/stdout using JSON:</p>

<ul>
  <li>Read exactly one JSON object from stdin.</li>
  <li>Write exactly one JSON response object to stdout.</li>
  <li>Write errors to stderr and exit with non-zero status on failure.</li>
</ul>

<p>See <a href="plugins/docs/README.md">plugins/docs/README.md</a> for the full protocol specification.</p>

<h3>4. Test Locally</h3>

<pre><code># Build your plugin
cargo build --release --manifest-path plugins/my-plugin/Cargo.toml

# Install it locally
xfetch plugin install ./plugins/my-plugin

# Run xfetch and verify it appears
xfetch</code></pre>

<h3>5. Update Documentation</h3>

<ul>
  <li>Add a <code>README.md</code> for your plugin documenting its features, configuration, and output.</li>
  <li>Add an entry for your plugin in <a href="plugins/README.md">plugins/README.md</a> under the "Available Plugins" table.</li>
</ul>

<h3>6. Commit and Push</h3>

<pre><code>git add plugins/my-plugin/ plugins/README.md
git commit -m "feat(plugins): add my-plugin"
git push origin plugin-my-plugin</code></pre>

<h3>7. Open a Pull Request</h3>

<p>
  Open a PR from your branch to the main repository's <code>main</code> branch.
  In the PR description, include:
</p>

<ul>
  <li>A brief description of what your plugin does.</li>
  <li>The kind of plugin (<code>info_provider</code> or <code>logo_animation</code>).</li>
  <li>Any dependencies required (e.g., <code>curl</code>, <code>docker</code> CLI).</li>
  <li>Screenshot or example output (optional but recommended).</li>
</ul>

<h2>Code Standards</h2>

<ul>
  <li>Use <code>cargo build</code> and <code>cargo test</code> before submitting.</li>
  <li>Follow existing code style — no trailing whitespace, consistent indentation.</li>
  <li>Write all code, comments, and documentation in English.</li>
  <li>Keep plugins focused on a single responsibility.</li>
  <li>Minimize dependencies to keep build times fast.</li>
  <li>Do not add network calls or file I/O to animation plugins — they should be pure transformations.</li>
</ul>

<h2>Reporting Issues</h2>

<p>
  Open an issue on GitHub with a clear description of the problem, steps to reproduce,
  and your environment (OS, terminal emulator, xfetch version).
</p>

<h2>License</h2>

<p>
  By contributing, you agree that your contributions will be licensed under the project's
  <a href="LICENSE">LICENSE</a>.
</p>
