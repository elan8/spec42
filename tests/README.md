# Compiler pipeline integration corpus

This directory owns repository-wide compiler pipeline inputs:

- `snapshots/` contains the source-to-golden corpus exercised by `spec42-snapshot`.
- `benchmarks/` contains the checked-in workload definitions for compiler benchmarks.

Crate-specific integration tests and their support files belong under the owning crate's `tests/`
directory.
