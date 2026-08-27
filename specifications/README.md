# Normative constraint manifest

`constraint_manifest.toml` is a deterministic, checked-in inventory of the closed
`derive*`, `check*`, and `validate*` constraint families. Each rule records its stable XMI
identity and the exact `8.3.*` Abstract Syntax clause heading from the pinned official PDF. The
canonical rule identity is `<spec>-<version>:<exact clause>:<constraint name>`; package and
metaclass remain separate provenance fields. Neither XMI nor PDF artifacts are committed.

Refresh is explicit and local-only. Supply these exact artifacts:

- KerML XMI `ptc/25-04-04`, SHA-256
  `45b18775afe2b2fcdc70e24f37c6d2f344defcc3f38a02075a193354e2d7b466`;
- KerML PDF [`formal/26-03-01`](https://www.omg.org/spec/KerML/1.0/PDF), SHA-256
  `3bcc96f989bfa9d05cd28e026df3351b795fe8d494187b87bff3db7d96373697`;
- SysML XMI `ptc/25-02-15`, SHA-256
  `caa65d54f56798bf7582d173f7567e1eea37a49c45984f8bd7df145011cf8c6f`;
- SysML PDF [`formal/26-03-02`](https://www.omg.org/spec/SysML/2.0/Language/PDF), SHA-256
  `46e6c0476a6f1f34f367d57e039d56659bff75e41d2e4b3d37ca4cadea84a83a`.

The tool verifies all four digests before extraction. It invokes the local `pdftotext` utility to
read the supplied PDFs, then requires each XMI metaclass with an included constraint to resolve
to exactly one normalized `8.3.*` PDF heading. Missing or ambiguous mappings are errors; the tool
never guesses or emits a clause without that evidence.

```sh
cargo run --locked -p spec42-constraint-manifest -- refresh \
  --kerml /path/to/KerML.xmi \
  --sysml /path/to/SysML.xmi \
  --kerml-pdf /path/to/KerML-formal-26-03-01.pdf \
  --sysml-pdf /path/to/SysML-formal-26-03-02.pdf \
  --output specifications/constraint_manifest.toml
```

Audit a checked-in manifest against the same explicit inputs:

```sh
cargo run --locked -p spec42-constraint-manifest -- audit \
  --kerml /path/to/KerML.xmi \
  --sysml /path/to/SysML.xmi \
  --kerml-pdf /path/to/KerML-formal-26-03-01.pdf \
  --sysml-pdf /path/to/SysML-formal-26-03-02.pdf \
  --manifest specifications/constraint_manifest.toml
```

Consumers use the `spec42_constraint_manifest` library API rather than re-parsing or
reclassifying manifest entries.

The manifest also preserves closed, officially reported corrections where a pinned XMI
`specializesFromLibrary` spelling does not name an element in the normative library. Each record
retains the source spelling, corrected concrete anchor, exact rule identity, and OMG issue:

- `KERML11-207` corrects `Performance::enclosedPerformance` to `enclosedPerformances`.
- `KERML11-205` corrects `Performance::subperformance` to `subperformances`.

Refresh derives only these exact rule-scoped corrections; runtime consumers use the corrected
typed contract and never try alternate spellings.
