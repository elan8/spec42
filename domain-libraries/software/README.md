# Modern Software Domain Libraries

This directory contains a SysML v2 domain-library family for modern software systems and software engineering.

It is organized as a self-contained workspace so it can evolve into:

- reusable library content
- example material
- a future Spec42 analysis fixture

## Structure

- `spec42.toml` - minimal workspace marker for Spec42-style analysis
- `software-core/SoftwareCore.sysml` - small software architecture kernel
- `distributed-systems/DistributedSystems.sysml` - distributed-systems library built on `SoftwareCore`
- `api/` - API and contract overlays (`HttpDomain`, `OpenApiDomain`, `SerializationDomain`, `GrpcDomain`)
- `messaging/` - asynchronous and streaming overlays (`MessagingDomain`, `KafkaDomain`)
- `data/` - relational and non-relational overlays (`SqlDomain`, `NosqlDomain`)
- `security/` - architecture/security + assurance/regulatory overlays (`IdentitySecurityDomain`, `CyberAssuranceDomain`, `EuCyberResilienceOverlay`)
- `platform/` - deployment/runtime platform overlays (`KubernetesDomain`, `CloudRuntimeDomain`)
- `delivery-ops/` - runtime control, delivery governance, and observability overlays (`SoftwareControlPlane`, `SoftwareDelivery`, `ObservabilityDomain`)
- `distributed-systems/rules/distributed-systems-rules.yaml` - external rule catalog for future tooling
- `api/rules/*.yaml` - HTTP/OpenAPI/serialization/gRPC rule catalogs
- `messaging/rules/*.yaml` - messaging and streaming rule catalogs
- `data/rules/*.yaml` - relational/non-relational data rule catalogs
- `security/rules/*.yaml` - identity/assurance/CRA rule catalogs
- `platform/rules/*.yaml` - platform/deployment rule catalogs
- `delivery-ops/rules/*.yaml` - delivery/control-plane/observability rule catalogs
- `examples/` - consolidated examples grouped by concerns (`api`, `messaging`, `data`, `security`, `platform`, `delivery-ops`, `distributed-systems`) plus `reference-workspace/` for the integrated end-to-end example

## Notes

- The SysML files define reusable domain concepts using ordinary package declarations and specialization.
- The YAML rule catalog is descriptive for now. It is meant to be consumed by future tooling rather than by the current parser.
- The examples are kept intentionally small so they remain easy to parse, inspect, and extend.
- `spec42.toml` is not part of the domain-library concept itself. It is included only because the current Spec42 analysis flow treats a directory as a workspace when that file is present.
- Transport-, serialization-, delivery-, security-, observability-, and platform-specific details should prefer dedicated overlays instead of accumulating in the core packages.
- `IdentitySecurityDomain` remains the place for architecture-oriented security concepts such as trust boundaries, exposure, identities, and authorization.
- `CyberAssuranceDomain` and `EuCyberResilienceOverlay` address assurance, evidence, vulnerability handling, conformity, and regulatory-specialization concerns without pushing those obligations into the identity/security kernel.
- The CRA overlay is the first regulation-oriented specialization in this workspace and is intended to be a pattern that future framework-specific overlays can follow.
