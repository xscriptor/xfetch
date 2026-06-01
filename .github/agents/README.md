<h1>OpenCode Agents</h1>

<p>A collection of ready-to-use <a href="https://opencode.ai/docs/agents">OpenCode agent definitions</a> organized by context and specialization. Each agent is a markdown file with YAML frontmatter that configures its behavior, permissions, and system prompt.</p>

<p><strong>91 agents</strong> across <strong>22 specialization groups</strong>. All agents are <strong>model-agnostic</strong> and work with Claude, GPT, DeepSeek, or any other LLM provider available in OpenCode.</p>

<p>Repository: <a href="https://github.com/xscriptor/ai">github.com/xscriptor/ai</a></p>

<h2>Table of Contents</h2>

<ul>
  <li><a href="#agent-structure">Agent Structure</a></li>
  <li><a href="#installation">Installation</a></li>
  <li><a href="#usage">Usage</a></li>
  <li><a href="#customization">Customization</a></li>
  <li><a href="#general">General</a></li>
  <li><a href="#web-security">Web / Security</a></li>
  <li><a href="#web-architecture">Web / Architecture</a></li>
  <li><a href="#web-frontend">Web / Frontend</a></li>
  <li><a href="#web-backend">Web / Backend</a></li>
  <li><a href="#languages">Languages</a></li>
  <li><a href="#mobile">Mobile</a></li>
  <li><a href="#data-ml">Data & ML</a></li>
  <li><a href="#cloud">Cloud</a></li>
  <li><a href="#testing">Testing</a></li>
  <li><a href="#graphql">GraphQL</a></li>
  <li><a href="#embedded">Embedded</a></li>
  <li><a href="#game-dev">Game Development</a></li>
  <li><a href="#security-recon">Security / Recon</a></li>
  <li><a href="#security-web-pentest">Security / Web Pentest</a></li>
  <li><a href="#security-mobile-pentest">Security / Mobile Pentest</a></li>
  <li><a href="#security-desktop">Security / Desktop Exploitation</a></li>
  <li><a href="#security-red-team">Security / Red Team</a></li>
  <li><a href="#security-blue-team">Security / Blue Team</a></li>
  <li><a href="#content">Content</a></li>
  <li><a href="#observability">Observability</a></li>
  <li><a href="#compliance">Compliance</a></li>
  <li><a href="#installation">Installation</a></li>
</ul>

<h2 id="agent-structure">Agent Structure</h2>

<p>Each agent file uses YAML frontmatter followed by a system prompt in markdown:</p>

<pre><code>---
description: One-line description of what this agent does
mode: subagent
model: anthropic/claude-sonnet-4-20250514
temperature: 0.1
color: accent
permission:
  edit: deny
  bash: deny
---

You are a specialist. Detailed instructions for the AI.
- Bullet points with specific guidance
- Checklists for what to check or do
- Rules to follow
</code></pre>

<h2 id="installation">Installation</h2>

<pre><code># Copy agents for a specific context
cp agents/general/code-reviewer.md ~/.config/opencode/agents/
cp agents/web/backend/api-designer.md ~/.config/opencode/agents/
cp agents/web/frontend/react-specialist.md ~/.config/opencode/agents/

# Or install all agents from a directory
cp agents/general/*.md ~/.config/opencode/agents/
</code></pre>

<h2 id="usage">Usage</h2>

<p>Invoke any agent in OpenCode with the <code>@</code> mention:</p>

<pre><code>@code-reviewer review this pull request
@web-security-auditor scan the payment module
@software-architect review the system design
@react-specialist review this component
@api-designer design a REST API for orders
</code></pre>

<h2 id="customization">Customization</h2>

<ul>
  <li><strong>model</strong>: Pin a specific provider/model (omit to use the default agent model)</li>
  <li><strong>temperature</strong>: 0.0-0.3 for precise tasks, 0.3-0.7 for creative work</li>
  <li><strong>color</strong>: Hex color or theme token (primary, accent, error, warning, success, info)</li>
  <li><strong>permission</strong>: control tool access per agent (allow, ask, deny)</li>
  <li><strong>steps</strong>: max agentic iterations before forced text response</li>
</ul>

<h2 id="general">General</h2>

<p>Language and framework agnostic agents for any project. See <a href="general/">general/</a>.</p>

<table>
  <thead>
    <tr>
      <th>Agent</th>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>Code Reviewer</td><td><code>general/code-reviewer.md</code></td><td>Reviews code quality, best practices, and potential issues</td></tr>
    <tr><td>Security Auditor</td><td><code>general/security-auditor.md</code></td><td>Security vulnerability analysis with CVE lookup</td></tr>
    <tr><td>Docs Writer</td><td><code>general/docs-writer.md</code></td><td>Creates and maintains project documentation</td></tr>
    <tr><td>API Docs</td><td><code>general/api-docs.md</code></td><td>Generates API documentation in OpenAPI/Swagger format</td></tr>
    <tr><td>Refactor Agent</td><td><code>general/refactor-agent.md</code></td><td>Code refactoring with behavior preservation</td></tr>
    <tr><td>DB Migrator</td><td><code>general/db-migrator.md</code></td><td>Database migrations with reversible up/down patterns</td></tr>
    <tr><td>Test Writer</td><td><code>general/test-writer.md</code></td><td>Unit, integration, and E2E test creation</td></tr>
    <tr><td>Dependency Auditor</td><td><code>general/dependency-auditor.md</code></td><td>Dependency health: CVEs, licenses, maintenance</td></tr>
    <tr><td>Performance Analyzer</td><td><code>general/performance-analyzer.md</code></td><td>Performance bottleneck detection and optimization</td></tr>
    <tr><td>PR Manager</td><td><code>general/pr-manager.md</code></td><td>Pull request creation and changelog generation</td></tr>
    <tr><td>Release Manager</td><td><code>general/release-manager.md</code></td><td>Release planning, versioning, and changelog management</td></tr>
  </tbody>
</table>

<h2 id="web-security">Web / Security</h2>

<p>See <a href="web/security/">web/security/</a>.</p>

<table>
  <thead>
    <tr>
      <th>Agent</th>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>Web Security Auditor</td><td><code>web/security/web-security-auditor.md</code></td><td>Full OWASP Top 10 audit across web application stack</td></tr>
    <tr><td>API Security Specialist</td><td><code>web/security/api-security-specialist.md</code></td><td>API-layer security: REST, GraphQL, gRPC endpoint protection</td></tr>
    <tr><td>Auth Security Specialist</td><td><code>web/security/auth-security-specialist.md</code></td><td>Authentication, authorization, OAuth2, JWT, session management</td></tr>
    <tr><td>AppSec Engineer</td><td><code>web/security/appsec-engineer.md</code></td><td>Secure SDLC: threat modeling, SAST/DAST, cloud security</td></tr>
  </tbody>
</table>

<h2 id="web-architecture">Web / Architecture</h2>

<p>See <a href="web/architecture/">web/architecture/</a>.</p>

<table>
  <thead>
    <tr>
      <th>Agent</th>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>Software Architect</td><td><code>web/architecture/software-architect.md</code></td><td>Architecture styles, patterns, C4 documentation, ADRs</td></tr>
    <tr><td>System Designer</td><td><code>web/architecture/system-designer.md</code></td><td>Distributed system design, scalability, database selection</td></tr>
    <tr><td>Scalability Specialist</td><td><code>web/architecture/scalability-specialist.md</code></td><td>Performance optimization, load testing, database scaling</td></tr>
    <tr><td>Reliability Specialist</td><td><code>web/architecture/reliability-specialist.md</code></td><td>SLO/SLI, circuit breakers, disaster recovery, incident management</td></tr>
  </tbody>
</table>

<h2 id="web-frontend">Web / Frontend</h2>

<p>See <a href="web/frontend/">web/frontend/</a>.</p>

<table>
  <thead>
    <tr>
      <th>Agent</th>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>React Specialist</td><td><code>web/frontend/react-specialist.md</code></td><td>React ecosystem: hooks, state, performance, Server Components</td></tr>
    <tr><td>Vue Specialist</td><td><code>web/frontend/vue-specialist.md</code></td><td>Vue 3 ecosystem: Composition API, Pinia, Nuxt, Vite</td></tr>
    <tr><td>CSS/UI Specialist</td><td><code>web/frontend/css-ui-specialist.md</code></td><td>Modern CSS, design systems, theming, layout, animations</td></tr>
    <tr><td>Frontend Performance</td><td><code>web/frontend/frontend-performance.md</code></td><td>Core Web Vitals, bundle optimization, runtime performance</td></tr>
    <tr><td>Accessibility Specialist</td><td><code>web/frontend/accessibility-specialist.md</code></td><td>WCAG 2.2, ARIA, semantic HTML, screen reader, keyboard navigation</td></tr>
    <tr><td>Next.js Developer</td><td><code>web/frontend/nextjs-developer.md</code></td><td>Next.js App Router, Server Actions, streaming, auth, deployment</td></tr>
    <tr><td>Angular Developer</td><td><code>web/frontend/angular-developer.md</code></td><td>Angular standalone components, signals, state management, testing</td></tr>
  </tbody>
</table>

<h2 id="web-backend">Web / Backend</h2>

<p>See <a href="web/backend/">web/backend/</a>.</p>

<table>
  <thead>
    <tr>
      <th>Agent</th>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>API Designer</td><td><code>web/backend/api-designer.md</code></td><td>REST, GraphQL, gRPC API design with OpenAPI 3.1</td></tr>
    <tr><td>Database Specialist</td><td><code>web/backend/database-specialist.md</code></td><td>Schema design, query optimization, indexing, migrations</td></tr>
    <tr><td>Microservices Architect</td><td><code>web/backend/microservices-architect.md</code></td><td>Service boundaries, communication patterns, distributed data</td></tr>
    <tr><td>DevOps Specialist</td><td><code>web/backend/devops-specialist.md</code></td><td>CI/CD, Docker, Kubernetes, Terraform, cloud infrastructure</td></tr>
    <tr><td>Message Queue Specialist</td><td><code>web/backend/message-queue-specialist.md</code></td><td>Kafka, RabbitMQ, event-driven patterns, outbox, DLQ</td></tr>
    <tr><td>Caching Specialist</td><td><code>web/backend/caching-specialist.md</code></td><td>Redis, CDN, HTTP caching, multi-level cache strategies</td></tr>
  </tbody>
</table>

<h2 id="languages">Languages</h2>

<p>See <a href="languages/">languages/</a>.</p>

<table>
  <thead>
    <tr>
      <th>Agent</th>
      <th>File</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>Python Developer</td><td><code>languages/python-developer.md</code></td><td>Python: async, web frameworks, testing, packaging, data</td></tr>
    <tr><td>TypeScript Developer</td><td><code>languages/typescript-developer.md</code></td><td>TypeScript/JS: type system, runtimes, tooling, async, web</td></tr>
    <tr><td>Go Developer</td><td><code>languages/go-developer.md</code></td><td>Go: concurrency, net/http, CLI, profiling, deployment</td></tr>
    <tr><td>Java Developer</td><td><code>languages/java-developer.md</code></td><td>Java 21+: Spring Boot, JPA, JVM tuning, virtual threads</td></tr>
    <tr><td>Kotlin Developer</td><td><code>languages/kotlin-developer.md</code></td><td>Kotlin: coroutines, Ktor, Exposed, multiplatform, Flow</td></tr>
    <tr><td>Rust Developer</td><td><code>languages/rust-developer.md</code></td><td>Rust: systems, async, Axum, unsafe, FFI, serde</td></tr>
  </tbody>
</table>

<h2 id="mobile">Mobile</h2>

<p>See <a href="mobile/">mobile/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>iOS Developer</td><td><code>mobile/ios-developer.md</code></td><td>Swift, SwiftUI, Swift concurrency, Core Data, testing</td></tr>
    <tr><td>Android Developer</td><td><code>mobile/android-developer.md</code></td><td>Jetpack Compose, ViewModel, Room, Hilt, testing</td></tr>
    <tr><td>React Native Developer</td><td><code>mobile/react-native-developer.md</code></td><td>Expo, Expo Router, NativeWind, FlashList, EAS</td></tr>
    <tr><td>Flutter Developer</td><td><code>mobile/flutter-developer.md</code></td><td>Riverpod, GoRouter, drift, BLoC, testing</td></tr>
  </tbody>
</table>

<h2 id="data-ml">Data & ML</h2>

<p>See <a href="data-ml/">data-ml/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Data Engineer</td><td><code>data-ml/data-engineer.md</code></td><td>Pipelines, ETL/ELT, Kafka, Spark, Airflow, dbt</td></tr>
    <tr><td>ML Engineer</td><td><code>data-ml/ml-engineer.md</code></td><td>PyTorch, XGBoost, training, evaluation, deployment</td></tr>
    <tr><td>MLOps Specialist</td><td><code>data-ml/mlops-specialist.md</code></td><td>Model serving, feature stores, drift monitoring, CI/CD for ML</td></tr>
    <tr><td>Data Scientist</td><td><code>data-ml/data-scientist.md</code></td><td>Statistical analysis, EDA, feature engineering, visualization</td></tr>
  </tbody>
</table>

<h2 id="cloud">Cloud</h2>

<p>See <a href="cloud/">cloud/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Kubernetes Specialist</td><td><code>cloud/kubernetes-specialist.md</code></td><td>Cluster design, workloads, security, autoscaling, GitOps</td></tr>
    <tr><td>SRE Specialist</td><td><code>cloud/sre-specialist.md</code></td><td>SLO/SLI, error budgets, incident response, capacity planning</td></tr>
    <tr><td>GitOps Specialist</td><td><code>cloud/gitops-specialist.md</code></td><td>ArgoCD, Flux, Kustomize, Helm, secrets management</td></tr>
    <tr><td>Service Mesh Specialist</td><td><code>cloud/service-mesh-specialist.md</code></td><td>Istio, Linkerd, Cilium, mTLS, traffic management</td></tr>
    <tr><td>Cloud Architect</td><td><code>cloud/cloud-architect.md</code></td><td>AWS, GCP, Azure comparison, multi-cloud strategy</td></tr>
  </tbody>
</table>

<h2 id="testing">Testing</h2>

<p>See <a href="testing/">testing/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>E2E Testing Specialist</td><td><code>testing/e2e-testing-specialist.md</code></td><td>Playwright, Cypress, page objects, CI integration</td></tr>
    <tr><td>Visual Testing Specialist</td><td><code>testing/visual-testing-specialist.md</code></td><td>Chromatic, Percy, snapshot diff, component states</td></tr>
    <tr><td>Performance Testing Specialist</td><td><code>testing/performance-testing-specialist.md</code></td><td>k6, Locust, Gatling, load/spike/soak tests</td></tr>
    <tr><td>Chaos Engineering Specialist</td><td><code>testing/chaos-engineering-specialist.md</code></td><td>Chaos Mesh, Litmus, Gremlin, blast radius control</td></tr>
  </tbody>
</table>

<h2 id="graphql">GraphQL</h2>

<p>See <a href="graphql/">graphql/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>GraphQL Specialist</td><td><code>graphql/graphql-specialist.md</code></td><td>Schema design, resolvers, DataLoader, caching, Relay</td></tr>
  </tbody>
</table>

<h2 id="embedded">Embedded</h2>

<p>See <a href="embedded/">embedded/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>C/C++ Developer</td><td><code>embedded/c-cpp-developer.md</code></td><td>Systems programming, CMake, embedded, RTOS</td></tr>
    <tr><td>Embedded Rust Developer</td><td><code>embedded/embedded-rust-developer.md</code></td><td>no_std, Zephyr, probe-rs, PAC/HAL, Renode</td></tr>
  </tbody>
</table>

<h2 id="game-dev">Game Development</h2>

<p>See <a href="game-dev/">game-dev/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Unity Developer</td><td><code>game-dev/unity-developer.md</code></td><td>Unity, C#, URP/HDRP, Addressables, DOTS</td></tr>
    <tr><td>Unreal Developer</td><td><code>game-dev/unreal-developer.md</code></td><td>UE5, C++, Blueprint, GAS, Nanite, Lumen</td></tr>
  </tbody>
</table>

<h2 id="security-recon">Security / Recon</h2>

<p>See <a href="security/recon/">security/recon/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Attack Surface Recon</td><td>OSINT, subdomain enumeration, cloud asset discovery, passive/active recon</td></tr>
  </tbody>
</table>

<h2 id="security-web-pentest">Security / Web Pentest</h2>

<p>See <a href="security/web-pentest/">security/web-pentest/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Web Vulnerability Hunter</td><td>SQLi, XSS, SSRF, IDOR, business logic, file upload exploitation</td></tr>
    <tr><td>API Pentester</td><td>REST, GraphQL, gRPC security testing, JWT attacks, introspection abuse</td></tr>
    <tr><td>Auth Bypass Specialist</td><td>Authentication/authorization bypass, OAuth abuse, session attacks</td></tr>
    <tr><td>Server-Side Exploitation</td><td>SSTI, deserialization, command injection, XXE, race conditions</td></tr>
    <tr><td>Cloud Security Assessment</td><td>AWS/GCP/Azure misconfiguration, container escape, IAM abuse</td></tr>
    <tr><td>WAF Bypass Specialist</td><td>Filter evasion for SQLi, XSS, SSRF, LFI across major WAFs</td></tr>
  </tbody>
</table>

<h2 id="security-mobile-pentest">Security / Mobile Pentest</h2>

<p>See <a href="security/mobile-pentest/">security/mobile-pentest/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Mobile App Pentester</td><td>iOS/Android static/dynamic analysis, API testing, data storage flaws</td></tr>
    <tr><td>iOS Security Researcher</td><td>Entitlements, TCC bypasses, XPC exploitation, Mach port abuse</td></tr>
    <tr><td>Android Security Researcher</td><td>Root detection bypass, keystore analysis, IPC abuse, modding</td></tr>
  </tbody>
</table>

<h2 id="security-desktop">Security / Desktop Exploitation</h2>

<p>See <a href="security/desktop/">security/desktop/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Windows Exploit Development</td><td>Stack/heap overflow, kernel exploitation, token stealing, SEH/ROP</td></tr>
    <tr><td>Linux Privilege Escalation</td><td>SUID, capabilities, kernel exploits, container escape, cron abuse</td></tr>
    <tr><td>macOS Security Research</td><td>SIP/TCC bypass, XPC services, code signing, Mach ports, entitlement abuse</td></tr>
    <tr><td>Binary Exploitation</td><td>Reverse engineering, fuzzing, UAF, type confusion (C/C++/Rust, all platforms)</td></tr>
    <tr><td>Python Application Security</td><td>Pickle RCE, sandbox escape, Electron/Node.js, desktop scripting vulns</td></tr>
    <tr><td>Desktop Threat Hunting</td><td>Cross-platform desktop bug hunting, IPC abuse, privilege escalation research</td></tr>
  </tbody>
</table>

<h2 id="security-red-team">Security / Red Team</h2>

<p>See <a href="security/red-team/">security/red-team/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Adversary Simulation</td><td>Full engagement ops: initial access, C2, lateral movement, evasion, persistence</td></tr>
    <tr><td>Social Engineering</td><td>Phishing, vishing, physical tailgating, OSINT targeting, credential harvesting</td></tr>
    <tr><td>Malware Analysis</td><td>Static/dynamic binary analysis, PE/ELF/Mach-O, anti-debug bypass, YARA</td></tr>
    <tr><td>Physical Security Assessment</td><td>RFID cloning, lock bypassing, facility entry, badge system testing</td></tr>
  </tbody>
</table>

<h2 id="security-blue-team">Security / Blue Team</h2>

<p>See <a href="security/blue-team/">security/blue-team/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Threat Hunting</td><td>Hypothesis-driven hunts across endpoints, network, cloud (Windows/Linux/macOS)</td></tr>
    <tr><td>Incident Response</td><td>NIST 800-61 methodology, containment, eradication, IR report generation</td></tr>
    <tr><td>Forensic Analysis</td><td>Memory/disk/mobile/cloud forensics, timeline analysis, anti-forensics detection</td></tr>
    <tr><td>Detection Engineering</td><td>Sigma, KQL, YARA, Splunk rules, behavioral detection, Atomic Red Team</td></tr>
  </tbody>
</table>

<h2 id="content">Content</h2>

<p>See <a href="content/">content/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Technical Writer</td><td><code>content/technical-writer.md</code></td><td>Technical writing, style enforcement, code examples, tutorials</td></tr>
    <tr><td>Content Editor</td><td><code>content/content-editor.md</code></td><td>6-pass editorial review: structure, clarity, grammar, consistency, inclusivity, accuracy</td></tr>
    <tr><td>Content Reviser</td><td><code>content/content-reviser.md</code></td><td>3-level revision: light (grammar), medium (structure), heavy (restructure)</td></tr>
    <tr><td>Translator</td><td><code>content/translator.md</code></td><td>Technical translation: format preservation, locale-specific style, terminology management</td></tr>
  </tbody>
</table>

<h2 id="observability">Observability</h2>

<p>See <a href="observability/">observability/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>Observability Specialist</td><td><code>observability/observability-specialist.md</code></td><td>OpenTelemetry, PromQL, dashboards, alerting, tracing</td></tr>
  </tbody>
</table>

<h2 id="compliance">Compliance</h2>

<p>See <a href="compliance/">compliance/</a>.</p>

<table>
  <thead><tr><th>Agent</th><th>File</th><th>Description</th></tr></thead>
  <tbody>
    <tr><td>SOC 2 Specialist</td><td><code>compliance/soc2-specialist.md</code></td><td>SOC 2 trust criteria, evidence collection, audit readiness</td></tr>
    <tr><td>GDPR Specialist</td><td><code>compliance/gdpr-specialist.md</code></td><td>Data subject rights, consent management, breach notification</td></tr>
  </tbody>
</table>

<h2 id="installation">Installation</h2>

<p>Repository: <a href="https://github.com/xscriptor/ai">github.com/xscriptor/ai</a><br>
Agents: <code>agents/</code> | Skills: <code>skills/</code> | Scripts: <code>scripts/</code></p>

<p>All 91 agents can be installed via script, npx, or manually. Works on macOS, Linux, and Windows WSL.</p>

<h3>Option 1: npx (No Install)</h3>

<pre><code># All 91 agents to OpenCode
npx @xscriptor/ai-agents

# Specific groups
npx @xscriptor/ai-agents --groups general,web/security,languages

# To Claude Code
npx @xscriptor/ai-agents --anthropic

# To project
npx @xscriptor/ai-agents --project</code></pre>

<h3>Option 2: Remote Script (No Clone)</h3>

<pre><code>curl -fsSL https://raw.githubusercontent.com/xscriptor/ai/main/scripts/install-agents.sh | bash
curl -fsSL https://raw.githubusercontent.com/xscriptor/ai/main/scripts/install-agents.sh | bash -s -- --project</code></pre>

<h3>Option 3: Clone and Install</h3>

<pre><code>git clone https://github.com/xscriptor/ai.git
cd ai

# All 91 agents
./scripts/install-agents.sh

# Specific groups
./scripts/install-agents.sh --groups general,web/security

# Interactive
./scripts/install-agents.sh --interactive

# Project-level
./scripts/install-agents.sh --project</code></pre>

<h3>Option 4: Manual Copy</h3>

<pre><code>cp agents/general/code-reviewer.md ~/.config/opencode/agents/
cp agents/web/security/web-security-auditor.md ~/.config/opencode/agents/</code></pre>

<h2>Related Resources</h2>

<ul>
  <li><a href="https://opencode.ai/docs/agents">OpenCode Agents Documentation</a></li>
  <li><a href="https://opencode.ai/docs/permissions">OpenCode Permissions Guide</a></li>
  <li><a href="../skills/">Skills</a></li>
  <li><a href="https://github.com/xscriptor/ai">github.com/xscriptor/ai</a></li>
</ul>

<div id="x" align="center">
<h2>X</h2>

<a href="https://dev.xscriptor.com">
  <img src="https://xscriptor.github.io/icons/icons/code/product-design/xsvg/verified-filled.svg" width="24" alt="X Web" />
</a>
 & 
<a href="https://github.com/xscriptor">
  <img src="https://xscriptor.github.io/icons/icons/code/product-design/xsvg/github.svg" width="24" alt="X Github Profile" />
</a>
 & 
<a href="https://www.xscriptor.com">
  <img src="https://xscriptor.github.io/icons/icons/code/product-design/xsvg/quotes.svg" width="24" alt="Xscriptor web" />
</a>

</div>
