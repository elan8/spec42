# Portable SysML reference tools

This directory provides dependency-free grammar and metamodel query tools. It
includes only the KerML and SysML textual EBNF inputs needed for grammar lookups.

The grammar inputs retain the source attribution in their file headers. They are
derived reference material from the OMG SysML v2 specification; see
[NOTICE.md](NOTICE.md) before redistributing them.

```sh
python3 tools/sysml_reference/query_grammar.py show PartUsage
python3 tools/sysml_reference/query_grammar.py search connector --source sysml
python3 tools/audit_spec42_metamodel.py
python3 tools/audit_spec42_metamodel.py --schema /path/to/SysML-abstract-syntax.json
# make raw exact-name mismatches fail (useful while maintaining a normalization map)
python3 tools/audit_spec42_metamodel.py --schema /path/to/SysML-abstract-syntax.json --strict-schema
python3 tools/sysml_reference/query_metamodel.py --schema /path/to/SysML-abstract-syntax.json show PartUsage
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md list
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md show "Element rules"
python3 tools/sysml_reference/query_specification.py --spec /path/to/specification.md search constraint
python3 tools/sysml_reference/inspect_standard_library.py --library /path/to/library-or-archive list
python3 tools/sysml_reference/inspect_standard_library.py --library /path/to/library-or-archive search PartUsage
python3 -m unittest tools/sysml_reference/test_tools.py
```

`audit_spec42_metamodel.py` checks the Rust `ElementKind -> Metaclass` arms,
the published `Metaclass` enumeration, and the workspace projected
relationship metaclass mapping. It does not audit model properties: the
implementation is a semantic projection rather than a stored OMG XMI object
graph. Supplying an external OMG JSON schema additionally checks that every
published metaclass exists in that authority. The raw OMG schema's names do not
always equal deliberate projection names (for example `FlowUsage` versus a
more specific flow-connection type), so unmatched names are reported as `SKIP`
until a reviewed normalization map exists. `--strict-schema` turns those
mismatches into failures. Missing external material is also deliberately
reported as `SKIP`, not silently treated as a passing conformance audit.

`query_metamodel.py` intentionally requires an explicit external schema path.
This keeps large, separately licensed OMG inputs out of the repository while
still supporting a pinned checkout in CI or a developer workstation. It also
supports `--strict-input` when an unavailable schema must fail.

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
