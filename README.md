# MeterMouseGarden

**Every agent gets a field check.**

MeterMouseGarden is a local-first AI usage and agent-health dashboard. A tiny field mouse walks through your AI garden, records trustworthy usage notes, and helps you care for the systems inside your own applications.

This first slice is a dependency-free static dashboard prototype that can be deployed to GitHub Pages immediately. It includes five starter pages:

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

The prototype is intentionally static and privacy-safe. The next layer will add a local Rust daemon, SQLite event ledger, provider adapters, and a generated integration wizard without changing the five dashboard surfaces.

The product must keep observed, reported, estimated, unknown, and conflicting values visibly separate. The mascot and garden metaphor make the experience welcoming; they do not replace technical evidence.

## License

Apache-2.0 is the intended open-source license for the project.
