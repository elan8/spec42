# Normative validation-rule inventory

Every normative validation constraint named by the two specifications now has a traceable snapshot
fixture under `tests/snapshots/validation`, and that corpus is the durable coverage record. This
file holds only what is still open.

## What counts as a validation rule

KerML 1.0 clause 7 (repeated in SysML 2.0) divides the constraints in the abstract-syntax clauses
into exactly three named families, and only the third is the validation contract:

1. **Derivation constraints**, named `derive*` (KerML 90, SysML 145). They define the value of a
   derived property as an equality. They state no condition a model can violate, so they are not
   validation rules and are deliberately out of scope.
2. **Semantic constraints**, named `check*` (KerML 98, SysML 138). These are the semantically
   required relationships, mostly implied specializations from the Kernel Semantic Library. The
   specification explicitly permits a conformant tool to satisfy them by introducing implied
   relationships, to report them, *or to ignore them*, so they are not a validation contract
   either. They are out of scope here, but they are the natural next corpus if implied-relationship
   conformance is ever put under test -- that would be a separate piece of work with a different
   expected-diagnostic story, because "no diagnostic, relationship implied" is a conforming outcome.
3. **Validation constraints**, named `validate*` (KerML 88, SysML 92). Of these the specification
   says a conformant tool "should report violations". These 180 are the corpus below.

This split is why the inventory is 180 and not the 415 constraint names the two documents contain.

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
