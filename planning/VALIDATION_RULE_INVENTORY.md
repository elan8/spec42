# Normative validation-rule inventory

Every normative validation constraint named by the two specifications now has a traceable snapshot
fixture under `tests/snapshots/validation`, and that corpus is the durable coverage record. This
file holds only what is still open.

Reconciliation of the checked-in corpus against the full normative inventory:

- OMG KerML 1.0 (formal/26-03-01), <https://www.omg.org/spec/KerML/1.0/PDF> -- 88 named
  constraints across clauses 8.3.2 through 8.3.4, all covered.
- OMG SysML 2.0 Language (formal/26-03-02),
  <https://www.omg.org/spec/SysML/2.0/Language/PDF> -- 92 named constraints across clauses 8.3.6
  through 8.3.26, all covered.

180 constraints are covered by 179 fixtures: the two Annotation constraints of KerML 8.3.2.3.3 are
one structural condition and share `kerml_annotation_annotating_element.md`. Each fixture's `META`
carries the specification, its OMG document identifier, the exact clause and the constraint name,
so `grep '^validation_rule=' tests/snapshots/validation/*.md` reproduces the reconciliation above.

Fixtures whose expectation the compiler does not yet meet carry a concrete `skip_validation`
reason, which is where the remaining implementation and parser gaps are recorded. That list is
deliberately not duplicated here; `cargo run -p spec42-snapshot -- check` reports it as `SKIPPED`
lines.

## Active blockers

- `8.3.2.3.3 validateAnnotationAnnotatedElementOwnership`: the publication records a
  `comment about Thing` annotation as documentation of the owning package rather than of `Thing`
  (see `tests/snapshots/validation/kerml_annotation_annotating_element.md`). The constraint itself
  has no textual violating form, so this does not block its fixture, but which element an
  `about` annotation annotates is still open for the annotation clauses generally.
