# Software Technical Libraries

This directory contains technical software capability libraries intended for reuse across multiple business domains.

Use these packages when a SysML v2 model needs software architecture vocabulary such as services, databases, topics, deployment platforms, runtime components, and interaction scenarios without tying the model to one business domain.

## Best Starting Points

- Start with `software-core/SoftwareCore.sysml` for basic software component concepts.
- Add `distributed-systems/DistributedSystems.sysml` for service-oriented and distributed architecture patterns.
- Add `data/`, `platform/`, and communication libraries only when the model needs those details.
- Use `interactions/SoftwareInteractions.sysml` together with Spec42 sequence views for software-oriented scenarios.

## Structure

- `software-core/SoftwareCore.sysml` - small software architecture kernel
- `distributed-systems/DistributedSystems.sysml` - distributed-systems library built on `SoftwareCore`
- `data/` - relational and non-relational overlays (`SqlDomain`, `NosqlDomain`)
- `platform/` - deployment/runtime platform overlays (`KubernetesDomain`, `CloudRuntimeDomain`)
- `interactions/SoftwareInteractions.sysml` - Spec42 interaction vocabulary for software-oriented sequence scenarios
- `distributed-systems/rules/distributed-systems-rules.yaml` - external rule catalog for future tooling
- `data/rules/*.yaml` - relational/non-relational data rule catalogs
- `platform/rules/*.yaml` - platform/deployment rule catalogs

## Related Layers

- Cross-cutting overlays for software reside in `../../cross-cutting/software/`.
- Communication-specific protocol libraries reside in `../../technical/communication/`.
- Integrated software scenarios reside in `../../../examples/webshop/`.

## Notes

- The SysML files define reusable domain concepts using ordinary package declarations and specialization.
- `SoftwareInteractions` is a Spec42 extension library meant to pair with `SequenceView`; it is not a claim of full OMG UML/SysML v1 sequence-diagram parity.
- Transport-, serialization-, platform-, and data-specific details should prefer dedicated overlays instead of accumulating in the core packages.
