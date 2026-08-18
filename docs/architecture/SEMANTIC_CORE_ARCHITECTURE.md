# Immutable semantic core architecture

Spec42 admits source documents into one immutable `PublishedModel`. The publication owns semantic
facts, diagnostics, identities, ordering, completeness, and evaluation results for an exact input
identity. Hosts share that publication through `Arc`; they do not rebuild it or maintain a second
semantic representation.

## Ownership

| Layer | Responsibility |
|---|---|
| `sysml-v2-parser` | Source-fidelity syntax and editor recovery |
| `sysml_resolution` | Semantic construction, resolution, diagnostics, evaluation, and publication identity |
| `sysml_query` | Opaque typed query facade over the immutable publication |
| `workspace` | Source admission and atomic publication ownership |
| `language_service` | Protocol-neutral editor features over typed queries |
| `lsp_server` and `server` | Thin protocol and CLI adapters |
| `generator_api` and `generator_host` | Sandboxed consumers of typed immutable model queries |

```text
sources + configuration
        │
        ▼
 parser-owned syntax/recovery
        │
        ▼
 sysml_resolution publication barrier
        │
        ▼
 Arc<PublishedModel>
   ├── diagnostics/navigation/edits/completion
   ├── inspection/types/evaluation
   └── generator model queries
```

Every consumer of one workspace revision receives the same publication handle. Full rebuilds are
the current correctness path. Incremental graph patching and persistent semantic graph caches were
removed; immutable incremental construction may return only after cold/full equivalence and
supersession behavior are established.

## Deliberately disabled products

- Built-in diagram semantics, view catalogs, render caches, and diagram CLI export are removed. A
  repository-owned generator plugin produces a versioned render artifact for the VS Code renderer.
  State transitions consume their typed projection; every other declared view reports explicit
  incompleteness until its owner-defined query exists.
- Graph-shaped model DTOs and semantic snapshot comparison are removed. A future comparison product
  must compare typed facts by stable identity.
- Call hierarchy and monikers are disabled until the publication owns typed behavior/`perform`
  relationships.
- `model-summary` is validation-only. A bounded structural summary requires its own typed query;
  hosts must not reconstruct one from display names or serialized output.
- Import and ambiguous-name quick fixes are disabled until typed queries provide candidates,
  provenance, and authored replacement/insertion ranges.

These are unsupported states, not compatibility gaps hidden by a fallback. New semantic capability
belongs first in `sysml_resolution`, then in a typed `sysml_query` contract, and only then in a host.
