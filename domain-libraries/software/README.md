# Modern Software Domain Libraries

This directory contains a SysML v2 domain-library family for modern software systems and software engineering.

It is organized as a self-contained workspace so it can evolve into:

- reusable library content
- example material
- a future Spec42 analysis fixture

## Structure

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
- `examples/realistic/` - realistic reference scenarios:
  - `commerce-platform/commerce-platform.sysml` - integrated end-to-end system spanning API, messaging, data, platform, security, observability, and delivery.
  - `progressive-delivery/progressive-delivery.sysml` - CI/CD with canary rollout, release gates, rollback, and evidence-driven promotion.
  - `security-operations/security-operations.sysml` - runtime identity, trust boundaries, policy enforcement, secrets/config, and incident-driven security operations.
  - `resilience-failure-modes/resilience-failure-modes.sysml` - failure-aware architecture with retries, timeout/circuit controls, DLQ/replay, and consistency repair.
  - `regulatory-compliance/regulatory-compliance.sysml` - CRA-oriented compliance flow linking risk, evidence, declaration, and post-market reporting.

## Notes

- The SysML files define reusable domain concepts using ordinary package declarations and specialization.
- The YAML rule catalog is descriptive for now. It is meant to be consumed by future tooling rather than by the current parser.
- The examples are intentionally realistic and system-level so they can be used as practical reference models, not only syntax samples.
- Transport-, serialization-, delivery-, security-, observability-, and platform-specific details should prefer dedicated overlays instead of accumulating in the core packages.
- `IdentitySecurityDomain` remains the place for architecture-oriented security concepts such as trust boundaries, exposure, identities, and authorization.
- `CyberAssuranceDomain` and `EuCyberResilienceOverlay` address assurance, evidence, vulnerability handling, conformity, and regulatory-specialization concerns without pushing those obligations into the identity/security kernel.
- The CRA overlay is the first regulation-oriented specialization in this workspace and is intended to be a pattern that future framework-specific overlays can follow.
