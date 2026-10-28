# MeterMouseGarden

**Every agent gets a field check.**

MeterMouseGarden is a local-first AI usage and agent-health dashboard. A tiny field mouse walks through your AI garden, records trustworthy usage notes, and helps you care for the systems inside your own applications.

The repository now contains a dependency-free static dashboard prototype plus the first local Rust daemon and SQLite usage ledger. The static dashboard can be deployed to GitHub Pages immediately; the daemon is the local-first path for real data.

It includes five starter pages:

1. **Garden** — the first dashboard: health, provider plants, next care task, and field note.
2. **Usage** — requests, tokens, estimates, observed values, and weekly flow.
3. **Agents** — agent routes, health, and last-check state.
4. **Providers** — connected, local, and foggy provider beds.
5. **Field work** — missions that improve reliability, privacy, visibility, and efficiency.

## Run locally

No build step is required:

```bash
open index.html
```

Or serve it over HTTP:

```bash
python3 -m http.server 4173
```

Then open <http://127.0.0.1:4173>.

## Local-first direction

The prototype is intentionally static and privacy-safe. The first daemon layer is now available:

```bash
cargo run -p meter-mouse-daemon
```

It binds only to `127.0.0.1:8787`, stores data in `.meter-mouse/garden.sqlite3`, uses SQLite WAL mode, and exposes:

```text
GET  /api/health
GET  /api/garden/summary
POST /api/usage
```

Record a privacy-safe usage event:

```bash
curl -X POST http://127.0.0.1:8787/api/usage \\
  -H 'content-type: application/json' \\
  -d '{"app":"my-app","provider":"ollama","model":"llama3","input_tokens":120,"output_tokens":42,"latency_ms":820,"status":"succeeded","usage_source":"observed"}'
```

The dashboard remains safe to run without the daemon and currently uses fixture data. The next frontend increment will read `/api/garden/summary` when available, then fall back to the fixture presentation when the local service is offline.

The product must keep observed, reported, estimated, unknown, and conflicting values visibly separate. The mascot and garden metaphor make the experience welcoming; they do not replace technical evidence.

## License

Apache-2.0 is the intended open-source license for the project.
