# Technical Communication Libraries

This directory contains reusable communication capability libraries that are independent from any single business domain.

Use these packages when a model needs protocol, endpoint, channel, binding, or messaging vocabulary that should be shared across software, electronics, robotics, or other domains.

## Best Starting Points

- Start with `core/CommunicationCore.sysml` for neutral endpoint, channel, operation, session, and binding concepts.
- Add `http/`, `grpc/`, `messaging/`, or `streaming/` when a model needs a concrete communication style.
- Use `transport/`, `device-bus/`, and `industrial/` as scaffolds for lower-level or field-system communication detail.

## Structure

- `core/` - shared abstractions for communication endpoints, channels, operations, sessions, and bindings.
- `http/` - HTTP/REST communication overlays (`HttpDomain`, `OpenApiDomain`).
- `grpc/` - gRPC communication overlays (`GrpcDomain`).
- `messaging/` - asynchronous messaging overlays (`MessagingDomain`).
- `streaming/` - streaming broker specializations (`KafkaDomain`).
- `transport/` - transport protocol scaffolds (TCP/UDP).
- `device-bus/` - device and field bus scaffolds (USB).
- `industrial/` - industrial protocol scaffolds (Modbus/Profinet/EtherCAT).

## Notes

- Communication libraries are technical and business-agnostic.
- Software and business-domain libraries should import these communication libraries rather than redefining protocol concepts.
- Package names for migrated v1 libraries remain stable to minimize import churn.
