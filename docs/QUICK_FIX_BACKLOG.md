# Spec42 Quick Fix Backlog

Potential Quick Fixes that reduce friction for systems architects and systems engineers while modeling SysML/KerML.

## Highest Value (Modeling Flow)

- **Create matching type for untyped usage**
  - Example: `part display;` -> create `part def Display { }` and rewrite to `part display : Display;`
  - Why: keeps model progression fast during exploratory decomposition.

- **Create missing requirement definition from usage**
  - Example: `requirement safetyReq : SafetyRequirement;` where `SafetyRequirement` is unresolved.
  - Quick fix: create `requirement def SafetyRequirement { }` in current package.
  - Why: requirements-first teams frequently sketch usages before formalizing definitions.

- **Specialize selected requirement from existing requirement**
  - Example: `requirement def BrakeDistanceReq;`
  - Quick fix options:
    - `requirement def WetRoadBrakeDistanceReq :> BrakeDistanceReq { }`
    - or update existing selected requirement to specialize a chosen base.
  - Why: supports requirements inheritance trees and variant management.

- **Create satisfy relation to implementation**
  - Example context: selected requirement + selected part/feature.
  - Quick fix: insert `satisfy <Requirement> by <ImplementationElement>;`
  - Why: traceability from requirement to design is a core MBSE task.

- **Create verify relation to test case**
  - Example: requirement has no verification artifact.
  - Quick fix: create/link `test case` (or chosen verification element) and add `verify`.
  - Why: closes requirement verification loop early.

## Traceability and Consistency

- **Add missing requirement ID metadata**
  - Example: requirement without project-required identifier schema.
  - Quick fix: add metadata stub (e.g. `id = "REQ-XXX"`).
  - Why: improves downstream reporting and traceability exports.

- **Normalize satisfy/verify target typing**
  - Example: unresolved satisfy/verify target due to unqualified reference.
  - Quick fix: qualify with package path or import.
  - Why: removes ambiguity and broken links in large workspaces.

- **Generate reverse trace links**
  - Example: have `satisfy` but missing related allocation/reference in implementation package.
  - Quick fix: add backlink/annotation or companion relation per team convention.
  - Why: simplifies impact analysis and audits.

## Architecture and Decomposition

- **Extract inline part to reusable `part def`**
  - Example: repeated inline structure in multiple containers.
  - Quick fix: create new `part def` and retarget usages.
  - Why: promotes reuse and cleaner architecture boundaries.

- **Create interface from repeated connector patterns**
  - Example: repeated port typing/connect usage patterns.
  - Quick fix: generate `interface def` + typed ports.
  - Why: stabilizes subsystem contracts.

- **Promote repeated attributes to `attribute def`**
  - Example: repeated scalar attributes across multiple defs.
  - Quick fix: create `attribute def` and rewrite declarations.
  - Why: consistency for units, value typing, and reuse.

## Safety, Quality, and Reviewability

- **Add missing units/type to numeric attributes**
  - Example: plain numeric attribute where team policy requires typed unit.
  - Quick fix: add recommended type from known unit library.
  - Why: avoids dimensional mistakes.

- **Convert ambiguous multiplicity to explicit interval**
  - Example: shorthand or invalid multiplicity.
  - Quick fix: rewrite to valid canonical form.
  - Why: reduces semantic ambiguity for interface contracts.

- **Wrap top-level declarations into package**
  - Already available in Spec42 (`Wrap in package`).
  - Why: keeps models compliant and indexable.

## Candidate MVP Sequence

1. Create missing requirement definition from unresolved requirement type.
2. Specialize requirement from existing requirement.
3. Create satisfy relation requirement -> implementation.
4. Create verify relation requirement -> test case.
5. Extract inline part to reusable `part def`.

## Design Notes for Implementation

- Prefer diagnostics-driven quick fixes (`code` -> deterministic action mapping).
- Keep edits minimal and local first; provide advanced variants as separate actions.
- Use project naming conventions where available (IDs, package placement, casing).
- Always include no-op guards (do not create duplicates if matching definition already exists).

