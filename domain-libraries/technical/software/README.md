# Software Technical Libraries

This directory contains technical software capability libraries intended for reuse across multiple business domains.

## Structure

- `software-core/SoftwareCore.sysml` - small software architecture kernel
- `distributed-systems/DistributedSystems.sysml` - distributed-systems library built on `SoftwareCore`
- `api/` - API and contract overlays (`HttpDomain`, `OpenApiDomain`, `SerializationDomain`, `GrpcDomain`)
- `messaging/` - asynchronous and streaming overlays (`MessagingDomain`, `KafkaDomain`)
- `data/` - relational and non-relational overlays (`SqlDomain`, `NosqlDomain`)
- `platform/` - deployment/runtime platform overlays (`KubernetesDomain`, `CloudRuntimeDomain`)
- `distributed-systems/rules/distributed-systems-rules.yaml` - external rule catalog for future tooling
- `api/rules/*.yaml` - HTTP/OpenAPI/serialization/gRPC rule catalogs
- `messaging/rules/*.yaml` - messaging and streaming rule catalogs
- `data/rules/*.yaml` - relational/non-relational data rule catalogs
- `platform/rules/*.yaml` - platform/deployment rule catalogs

## Related Layers

- Cross-cutting overlays for software reside in `../../cross-cutting/software/`.
- Integrated software scenarios reside in `../../examples/software/realistic/`.

## Notes

- The SysML files define reusable domain concepts using ordinary package declarations and specialization.
- Transport-, serialization-, platform-, and data-specific details should prefer dedicated overlays instead of accumulating in the core packages.
