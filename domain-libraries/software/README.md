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
- `software-control-plane/SoftwareControlPlane.sysml` - neutral runtime control-plane concepts
- `software-delivery/SoftwareDelivery.sysml` - neutral delivery and governance concepts
- `http/HttpDomain.sysml` - HTTP/API overlay built on `DistributedSystems`
- `openapi/OpenApiDomain.sysml` - HTTP contract overlay built on `HttpDomain` and `SerializationDomain`
- `serialization/SerializationDomain.sysml` - payload and encoding overlay reusable across protocols
- `grpc/GrpcDomain.sysml` - gRPC overlay built on `DistributedSystems` and `SerializationDomain`
- `messaging/MessagingDomain.sysml` - neutral asynchronous messaging overlay
- `kafka/KafkaDomain.sysml` - streaming/event platform overlay built on `MessagingDomain`
- `sql/SqlDomain.sysml` - relational data overlay
- `nosql/NosqlDomain.sysml` - non-relational data overlay
- `identity-security/IdentitySecurityDomain.sysml` - trust, identity, exposure, and policy overlay
- `cyber-assurance/CyberAssuranceDomain.sysml` - neutral assurance, evidence, risk, vulnerability, and conformity overlay
- `eu-cyber-resilience-overlay/EuCyberResilienceOverlay.sysml` - EU Cyber Resilience Act-oriented product assurance overlay built on `CyberAssuranceDomain`
- `observability/ObservabilityDomain.sysml` - logs, metrics, traces, health, and SLO overlay
- `kubernetes/KubernetesDomain.sysml` - Kubernetes deployment overlay built on `DistributedSystems`
- `cloud-runtime/CloudRuntimeDomain.sysml` - neutral cloud/runtime deployment overlay
- `distributed-systems/rules/distributed-systems-rules.yaml` - external rule catalog for future tooling
- `software-control-plane/rules/software-control-plane-rules.yaml` - runtime control-plane rule catalog
- `software-delivery/rules/software-delivery-rules.yaml` - delivery and governance rule catalog
- `http/rules/http-domain-rules.yaml` - HTTP/API rule catalog
- `openapi/rules/openapi-domain-rules.yaml` - OpenAPI rule catalog
- `serialization/rules/serialization-domain-rules.yaml` - serialization rule catalog
- `grpc/rules/grpc-domain-rules.yaml` - gRPC rule catalog
- `messaging/rules/messaging-domain-rules.yaml` - messaging rule catalog
- `kafka/rules/kafka-domain-rules.yaml` - Kafka rule catalog
- `sql/rules/sql-domain-rules.yaml` - SQL data rule catalog
- `nosql/rules/nosql-domain-rules.yaml` - NoSQL data rule catalog
- `identity-security/rules/identity-security-domain-rules.yaml` - identity and security rule catalog
- `cyber-assurance/rules/cyber-assurance-domain-rules.yaml` - assurance, evidence, and vulnerability-management rule catalog
- `eu-cyber-resilience-overlay/rules/eu-cyber-resilience-overlay-rules.yaml` - CRA-oriented product assurance and reporting rule catalog
- `observability/rules/observability-domain-rules.yaml` - observability rule catalog
- `kubernetes/rules/kubernetes-domain-rules.yaml` - Kubernetes rule catalog
- `cloud-runtime/rules/cloud-runtime-domain-rules.yaml` - cloud runtime rule catalog
- `examples/` - consolidated examples, grouped by library package
- `reference-workspace/` - integrated end-to-end example workspace

## Notes

- The SysML files define reusable domain concepts using ordinary package declarations and specialization.
- The YAML rule catalog is descriptive for now. It is meant to be consumed by future tooling rather than by the current parser.
- The examples are kept intentionally small so they remain easy to parse, inspect, and extend.
- `spec42.toml` is not part of the domain-library concept itself. It is included only because the current Spec42 analysis flow treats a directory as a workspace when that file is present.
- Transport-, serialization-, delivery-, security-, observability-, and platform-specific details should prefer dedicated overlays instead of accumulating in the core packages.
- `IdentitySecurityDomain` remains the place for architecture-oriented security concepts such as trust boundaries, exposure, identities, and authorization.
- `CyberAssuranceDomain` and `EuCyberResilienceOverlay` address assurance, evidence, vulnerability handling, conformity, and regulatory-specialization concerns without pushing those obligations into the identity/security kernel.
- The CRA overlay is the first regulation-oriented specialization in this workspace and is intended to be a pattern that future framework-specific overlays can follow.
