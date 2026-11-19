# Portable SysML reference tools

This directory provides dependency-free grammar and reference query tools. It
includes only the KerML and SysML textual EBNF inputs needed for grammar lookups.

The grammar inputs retain the source attribution in their file headers. They are
derived reference material from the OMG SysML v2 specification; see
[NOTICE.md](NOTICE.md) before redistributing them.

```sh
python3 tools/sysml_reference/query_grammar.py show PartUsage
python3 tools/sysml_reference/query_grammar.py search connector --source sysml
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md list
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md show "Element rules"
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md search constraint
python3 tools/sysml_reference/inspect_standard_library.py --library /path/to/library-or-archive list
python3 tools/sysml_reference/inspect_standard_library.py --library /path/to/library-or-archive search PartUsage
python3 -m unittest tools/sysml_reference/test_tools.py
```

`query_specification.py` accepts an external Markdown specification and offers
section listing, exact heading lookup, and case-insensitive text search.
`inspect_standard_library.py` accepts a source file, directory, or ZIP/TAR
archive; it lists members and searches member paths and UTF-8 text without
extracting archives.
Neither tool follows directory symbolic links. The default search limit is 5
MiB per member, and over-limit content is reported as `SKIP` after its path is
searched. In their default mode, missing external inputs also report an
actionable `SKIP` and exit successfully, which keeps optional local or CI
checks explicit. Use `--strict-input` when an unavailable input must fail.

## Metamodel reference pilot

`check_metamodel.py` audits the OMG 20250201 XMI metamodel files against the SHA-256
digests pinned once in `tools/constraint_manifest/src/lib.rs`, which match
[Open-MBEE's reference toolkit](https://github.com/Open-MBEE/sysml-toolkit/blob/7d740b37c0666be18bee2de1b467c86a234788bc/tools/fetch-spec-refs.pins).
The fetch is explicit and needs `www.omg.org`, so pull-request CI does not run it; the
scheduled conformance gap audit does. Keep the files in a local cache and retain the
existing official release pin for the model corpus.

```sh
python3 tools/sysml_reference/check_metamodel.py .cache/spec-refs --fetch
python3 tools/sysml_reference/check_metamodel.py .cache/spec-refs
cargo snapshot check --fixture item_definition_modifiers.md
cargo snapshot check --fixture conformance/reference_attribute_redefinition.md
cargo test -p server --test integration cli_model_export_preserves_authored_item_abstractness
```

The audit checks the metaclass and property facts used by the pilot. The snapshot and CLI
test check Spec42 behavior; pull-request CI already runs them with the full snapshot corpus
and the `cli_ai_tools` integration tests. The semantic redefinition fixture omits `ref` on the anonymous
redefinition so it runs on the current parser pin. The parser pilot covers the explicit
`ref attribute :>>` spelling. Use that spelling in the Spec42 fixture once its parser pin
includes the parser fix.
