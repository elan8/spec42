# Portable SysML reference tools

This directory provides dependency-free grammar and reference query tools. It
includes only the KerML and SysML textual EBNF inputs needed for grammar lookups.

The grammar inputs retain the source attribution in their file headers. They are
derived reference material from the OMG SysML v2 specification; see
[NOTICE.md](NOTICE.md) before redistributing them.

```sh
python3 tools/sysml_reference/query_grammar.py show PartUsage
python3 tools/sysml_reference/query_grammar.py search connector --source sysml
python3 tools/audit_spec42_metamodel.py
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md list
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md show "Element rules"
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md search constraint
python3 tools/sysml_reference/inspect_standard_library.py --library /path/to/library-or-archive list
python3 tools/sysml_reference/inspect_standard_library.py --library /path/to/library-or-archive search PartUsage
python3 -m unittest tools/sysml_reference/test_tools.py
```

`audit_spec42_metamodel.py` checks the Rust `ElementKind -> Metaclass` arms,
the published `Metaclass` enumeration, and the workspace projected
relationship metaclass mapping. The audit is limited to Spec42's own semantic
projection and does not claim to validate external model properties.

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
