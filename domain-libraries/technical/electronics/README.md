# Electronics Technical Libraries

This directory contains technical electronics capability libraries intended for reuse across multiple business domains.

Use these packages when a model needs electronics vocabulary for power, compute, I/O, board integration, and bus-level interfaces while keeping business-domain semantics outside the electronics layer.

## Best Starting Points

- Start with `electronics-core/` for common electronics concepts.
- Add `power/` for power sources, loads, conversion, and distribution.
- Add `compute/`, `io/`, `buses/`, and `board/` as the model moves from logical electronics into implementation structure.

## Structure

- `electronics-core/` - base electronics concepts (`ElectronicsCore`).
- `power/` - electrical power and conversion overlays (`ElectricalPowerDomain`).
- `compute/` - embedded compute and firmware overlays (`EmbeddedComputeDomain`).
- `io/` - digital/analog I/O and interface overlays (`ElectronicIoDomain`).
- `buses/` - board-level bus and link overlays (`ElectronicBusDomain`).
- `board/` - board assembly and integration overlays (`BoardIntegrationDomain`).

## Related Layers

- Communication protocol libraries reside in `../communication/`.
- Cross-cutting overlays remain outside electronics in `../../cross-cutting/` for this v1.

## Notes

- Electronics libraries are technical and business-agnostic.
- Foundational types live in `ElectronicsCore`; other packages specialize those concepts.
- Business domains (for example robotics) can compose these electronics libraries without importing business semantics back into technical layers.
