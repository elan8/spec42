# Port nodes experiment (#217)

`port_nodes.rs` compares the shipped interconnection drawing with a graph that
represents each semantic port as an ordinary child node. Edges refer to the exact
port identities supplied by the canonical diagram pipeline. There are no explicit
ELK ports and no changes to the pinned elkrs dependency or production renderer.

Run from the repository root with a schema-5 diagram product:

```powershell
cargo run --offline -p diagram_draw --example port_nodes -- product.json target/port-node-spike/result
```

Open `comparison.html` in the output directory. The tool also writes the baseline
draw input, experimental ELK input and output, SVGs, and a report containing
pairwise shared orthogonal segment counts and lengths. Crossings and point contacts
do not count as shared segments. These measurements are not a general readability
score.

The experiment uses simplified styling and colours per connector. ELK supplies
connector routes and label positions; the renderer does not snap or rewrite them.
Ports are nested boxes rather than symbols fixed to part borders. This is a visual
spike, not a production rendering mode.
