# ELK layout parity harness

This development-only tool runs identical, checked-in ELK JSON graphs through Spec42's current
ELK.js/QuickJS adapter and the pinned public `elan8/elkrs` revision. The narrow elkrs adapter restores
Spec42's root-authored/root-coordinate edge contract when elkrs publishes an intra-container edge on
its lowest common ancestor. It compares layout geometry, not serialized JSON, so object-key order
and number formatting do not create false differences.

The comparison covers graph and container bounds, nodes, ports, node/port/edge labels, edge-section
start/end points, and bend points. Paths are stable and differences are sorted, making JSON output
suitable for review or CI artifacts.

Run all fixtures:

```sh
cargo run -p elkrs_parity -- --iterations 5
```

Write a machine-readable report:

```sh
cargo run -p elkrs_parity -- --format json --output elkrs-parity.json
```

Pass one or more JSON paths to compare additional captured renderer inputs. Use
`--fail-on-difference` when exact geometry is required. The default is report-only because deciding
which differences are contractually acceptable is an explicit migration decision.

The first timed run includes engine initialization. `median_layout_us` is the median of all timed
runs and is intended for relative local comparison, not as a portable performance guarantee. Peak
memory and final binary/package size are deliberately measured outside this in-process harness;
loading both engines into one process would make engine-attributed memory numbers misleading.
