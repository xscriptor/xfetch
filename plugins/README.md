<h1>Plugins</h1>

<p>
  xfetch supports third-party plugins that extend its functionality. Plugins run as
  separate executables and communicate with xfetch over stdin/stdout using a JSON
  protocol. This keeps the plugin system decoupled, allowing plugins to be installed,
  removed, and updated independently of the core binary.
</p>

<h2>Installing a Plugin</h2>

<p>From a local path:</p>

<pre><code>xfetch plugin install ./plugins/my-plugin</code></pre>

<p>From the default remote repository (GitHub):</p>

<pre><code>xfetch plugin install my-plugin</code></pre>

<h2>Listing Installed Plugins</h2>

<pre><code>xfetch plugin list</code></pre>

<h2>Removing a Plugin</h2>

<pre><code>xfetch plugin remove my-plugin</code></pre>

<h2>Available Plugins</h2>

<table>
  <thead>
    <tr>
      <th>Plugin</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>animate-logo</code></td>
      <td>
        Animates the ASCII logo with multiple styles (sweep, wave, rainbow,
        sparkle, breathing, frame). See
        <a href="./animate-logo/README.md">animate-logo/README.md</a>.
      </td>
    </tr>
  </tbody>
</table>

<h2>Plugin Directory</h2>

<p>
  Installed plugin binaries are stored in the xfetch plugin directory:
</p>

<ul>
  <li><strong>Linux/macOS:</strong> <code>~/.config/xfetch/plugins/</code></li>
  <li><strong>Windows:</strong> <code>%APPDATA%/xfetch/plugins/</code></li>
</ul>

<h2>Creating a Plugin</h2>

<p>
  See <a href="./docs/README.md">plugins/docs/README.md</a> for the full
  plugin development guide, including the protocol specification, conventions,
  and contribution guidelines.
</p>
