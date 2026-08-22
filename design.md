# Spec42 design

This document states the enduring architecture of Spec42: which crate is the authority for what,
how consumers reach the model, and which invariants the repository enforces. It changes only when a
new product requirement changes the architecture. It does not record migration history, open work,
test procedures, or performance figures.

Where things live:

| Concern | Document |
|---|---|
| Repository policy for new and modified code | [`AGENTS.md`](AGENTS.md) |
| Build, test, release, and parser-bump procedures | [`DEVELOPMENT.md`](DEVELOPMENT.md) |
| Contracts of individual query products | [`docs/architecture/`](docs/architecture/) |
| Active decisions, blockers, and remaining work | [`planning/`](planning/) |

## One semantic system

Every semantic fact category has one authoritative owner and representation for a model state, and
every surface uses the same semantic results for identical inputs. `AGENTS.md` states that policy;
this document states the structure that makes it true.

## The authority pipeline

An **authority** is the only crate that may perform a given operation. Spec42 has two, arranged in
one chain, behind one facade:

| Authority | Crate | The only place that may… |
|---|---|---|
| Source | `sysml_source` | read a SysML file from disk, admit text as a document, normalise a URI or line endings, compute a content digest |
| Semantic | `sysml_resolution` | call the parser, hold a parsed tree, lower an AST, resolve names, evaluate, decide a diagnostic, compute a library closure, build or publish a model, own a publication's lifecycle |

`sysml_query` is the **facade**: the only crate a consumer may name for anything SysML. It exposes
the authorities as typed services and contains no semantic logic of its own.

Each link in the chain has exactly one dependant:

```text
sysml-v2-parser ──► sysml_resolution ──► sysml_query ──► consumers
source_identity ──► sysml_source ──────┘
```

Invariants of the pipeline:

- **One parse.** A source revision is parsed once. The editor's syntax queries and the semantic
  build share the same parsed tree through a memoised handle; no consumer can cause a second parse
  of identical content.
- **Memoisation is invisible.** Caches, memos, and stratum reuse are implementation details behind
  service handles. A consumer cannot tell a warm hit from a cold computation, and cannot hold,
  serialise, or rebuild a cached value. A persistent (on-disk) tier may be introduced only with a
  benchmark showing it beats recomputation on the bundled standard-library corpus; at current
  corpus sizes parsing is cheaper than deserialising.
- **No file I/O below or beside the source authority.** `sysml_resolution` never reads a file; hosts
  never read SysML text.
- **Analysis is a service, not a consumer algorithm.** If an answer is derived from SysML syntax or
  semantics — declared packages, import closure, token roles, anything — the derivation lives in the
  semantic authority and is reached through a typed query. A consumer that needs a new answer
  extends the service; it does not compute the answer from facade data, source text, or names.

## Services

Consumers reach the model through five services on the facade. All handles are cheap to clone and
share one set of authorities per host process.

| Service (`sysml_query::…`) | Handle | Owns |
|---|---|---|
| `source::SourceService` | `SourceDocument` | admission of text as an identified document; URI normalisation; line-ending policy; providers (filesystem with ignore rules, in-memory, library roots); content digests |
| `syntax::SyntaxService` | `ParsedSource` | the parser call and parse memo; syntax-fidelity queries over a parsed tree: outline, folding, token roles, package declarations, closure facts, reserved keywords; formatting-preservation checks |
| `library::LibraryClosureService` | `LibraryClosure` | the package index over library roots and the transitive import closure a workspace needs, with its seed signature |
| `publication::PublicationService` and `PublicationSession` | `PublishedModel`, `RelinkToken`, `BuildToken`, `SessionLifecycle` | partitioning admitted documents by provenance, library-stratum reuse, constructing the immutable publication; the publication lifecycle (startup, relink, reindex, reset), its tokens, and the barrier that admits a finished build |
| `PublishedModel` queries | `navigation()`, `types()`, `diagnostics()`, … | typed, opaque semantic queries over one immutable publication |

A host obtains every handle from one `Services` value. `Services::new(config)` is for host
processes; `Services::in_memory()` is the stateless form for WebAssembly, fuzzing, conformance, and
benchmarks. There is exactly one `Services` per host process.

New capability lands in this order: implementation in the owning authority, a typed contract in
`sysml_query`, then use from a host. Never the reverse.

## Crate map

```mermaid
flowchart BT
    parser["sysml-v2-parser<br/><i>grammar (external, git-pinned)</i>"]
    identity["source_identity<br/><i>content digests · std-only guard home</i>"]
    source["sysml_source<br/><b>Source authority</b>"]
    resolution["sysml_resolution<br/><b>Semantic authority</b>"]
    query["sysml_query<br/><b>The facade</b>"]

    diagnostics["sysml_diagnostics"]
    tokens["sysml_tokens"]
    kpar["kpar"]
    langsvc["language_service"]
    catalog["library_catalog"]
    actor["session_actor<br/><i>tokio only</i>"]
    workspace["workspace<br/><i>batch host</i>"]
    lsp["lsp_server<br/><i>editor host</i>"]
    server["server<br/><i>CLI / MCP host</i>"]
    gen["generator_api / generator_host<br/>generator_conformance"]
    tools["tools/* · fuzz/"]

    source --> identity
    resolution --> parser
    resolution --> source
    query --> resolution

    diagnostics --> query
    tokens --> query
    kpar --> query
    langsvc --> query
    gen --> query
    tools --> query
    catalog --> kpar
    workspace --> query
    workspace --> catalog
    workspace --> langsvc
    workspace --> diagnostics
    lsp --> actor
    lsp --> workspace
    lsp --> catalog
    lsp --> langsvc
    lsp --> tokens
    lsp --> diagnostics
    lsp --> query
    server --> lsp
    server --> workspace
    server --> catalog
    server --> query

    classDef authority fill:#fde68a,stroke:#b45309,color:#111
    classDef facade fill:#bfdbfe,stroke:#1d4ed8,color:#111
    classDef consumer fill:#e5e7eb,stroke:#6b7280,color:#111
    classDef host fill:#d1fae5,stroke:#047857,color:#111
    class parser,source,resolution,identity authority
    class query facade
    class diagnostics,tokens,kpar,langsvc,catalog,actor,gen,tools consumer
    class workspace,lsp,server host
```

Yellow is the authority chain; blue is the facade; grey crates are library consumers; green crates
are hosts. No grey or green edge reaches yellow.

| Crate | Role | May depend on (SysML crates) |
|---|---|---|
| `source_identity` | typed content digests and manifests; home of the std-only authority guards | — |
| `sysml_source` | source authority | `source_identity` |
| `sysml_resolution` | semantic authority | `sysml-v2-parser`, `sysml_source` |
| `sysml_query` | facade | `sysml_resolution` |
| `sysml_diagnostics` | transport-neutral diagnostic values and reporting policy; decides nothing semantic | `sysml_query` |
| `sysml_tokens` | projection of facade token roles onto editor token indices; host-neutral | `sysml_query` |
| `kpar` | KerML Project Archive read, pack, and validate | `sysml_query` |
| `language_service` | protocol-neutral editor intelligence over typed queries | `sysml_query` |
| `library_catalog` | library provisioning: bundled and managed standard/domain libraries, configuration, data directories; yields library roots | `kpar` |
| `session_actor` | generic asynchronous mailbox over embedder state; knows nothing about SysML | — |
| `workspace` | batch host: engine, directory snapshot, validation, comparison, schema versions | `sysml_query`, `library_catalog`, `language_service`, `sysml_diagnostics` |
| `lsp_server` | editor host: document lifecycle, LSP handlers, host adapters | consumers above |
| `server` | CLI, MCP, and LSP binary | consumers above |
| `generator_api`, `generator_host`, `generator_conformance` | sandboxed generators over typed model queries | `sysml_query` |

## Hosts

A host adapts explicit service inputs and outputs to a transport, an editor, a process, or a
filesystem policy. Hosts may own: where files come from and which are ignored, protocol mapping,
presentation (labels, symbol kinds, snippets, hover prose), scheduling, and configuration. Hosts may
not own: parsing, parsed trees, caches of derived values, library closure, semantic decisions, or
publication lifecycle state outside a `PublicationSession`.

| Host | Shape |
|---|---|
| Batch (`workspace`) | load a directory through the source service, publish once through the publication service, query, compare; no session |
| Editor (`lsp_server`) | keeps `ServerState` inside a `session_actor` mailbox; `ServerState` holds the `PublicationSession` and, per document, a `SourceDocument` and its `ParsedSource`; every parse goes through the syntax service |
| CLI / MCP (`server`) | thin adapters over the batch host and the editor host's validation path; one `Services` shared by both |
| Generators | receive an immutable `PublishedModel` and typed queries; never source text |

## Publication lifecycle and identity

A publication is an immutable `PublishedModel` with a dependency-complete, owner-scoped identity
derived from the content digests of every admitted document, their provenance, and the semantic
contract version. Cold, warm, sequential, and parallel construction of the same inputs produce the
same identity and observably equivalent results.

`PublicationSession` owns the lifecycle for a long-lived host. It separates two token kinds:

- a **relink token** gates editor-staged symbol commits; every edit supersedes the previous one;
- a **build token** carries the identity a build was started for; a finished build is admitted only
  if its owner is still current *and* its identity still matches the session's inputs. Admission is
  independent of relink generation, so an edit arriving while a build runs does not discard a build
  whose inputs it did not change.

The session also carries a monotonically increasing **version** that hosts report to clients and
use to gate diagnostics. Failed, cancelled, stale, identity-mismatched, and out-of-order builds leave
the last coherent publication in place and report a typed outcome.

Library documents are resolved once into a library stratum keyed by their digests and reused across
publications whose library inputs are unchanged.

## Enforcement

Each invariant above is checked in a place the constrained crates cannot disable.

| Invariant | Where it is enforced |
|---|---|
| the parser is a dependency of `sysml_resolution` only; `sysml_resolution` of `sysml_query` only; `sysml_source` of `sysml_resolution` only | `deny.toml` (`cargo deny check bans`, resolved graph, dev-dependencies included); `crates/source_identity/tests/parser_authority.rs` and `authority_chain.rs` (manifest shape, nested `fuzz/` workspace, both lockfiles, local facade renames) |
| exact dependency sets of the chain crates, the facade, `session_actor`, and `workspace`; every downstream crate is a designated consumer; no async runtime in the chain | `crates/sysml_query/tests/architecture.rs` (cargo metadata) |
| the facade publishes no parser or graph type and no text-taking syntax entry point | `crates/sysml_query/tests/architecture.rs` (public API visitor) |
| no consumer re-implements a facade query, revives a retired heuristic, holds a parsed tree or stratum outside the allow-listed host fields, or reads SysML files | `crates/sysml_query/tests/syntax_authority.rs` (retired-name list, shadowed-name set, source-probe detector with predicate-backed exemptions, field and I/O bans) |
| one `Services` per host; library closure never on the edit path | `crates/lsp_server/tests/debt_guardrails.rs` |
| reporting policy decides nothing semantic | `crates/sysml_diagnostics/tests/dependency_guardrails.rs` |
