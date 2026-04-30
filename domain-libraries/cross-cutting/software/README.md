# Software Cross-Cutting Libraries

This directory contains software overlays that cross technical subdomains and support governance, operations, security, and assurance.

Use these packages when the model needs concerns that cut across software architecture: delivery, observability, runtime governance, identity, trust boundaries, cyber assurance, evidence, and regulatory obligations.

## Best Starting Points

- Start with `delivery-ops/` when modeling deployment, runtime control, observability, or delivery governance.
- Start with `security/IdentitySecurityDomain.sysml` for architecture-oriented security concepts.
- Add `CyberAssuranceDomain` and `EuCyberResilienceOverlay` when the model needs assurance, evidence, vulnerability handling, or regulatory obligations.

## Structure

- `delivery-ops/` - runtime control, delivery governance, and observability overlays (`SoftwareControlPlane`, `SoftwareDelivery`, `ObservabilityDomain`)
- `security/` - architecture/security + assurance/regulatory overlays (`IdentitySecurityDomain`, `CyberAssuranceDomain`, `EuCyberResilienceOverlay`)
- `delivery-ops/rules/*.yaml` - delivery/control-plane/observability rule catalogs
- `security/rules/*.yaml` - identity/assurance/CRA rule catalogs

## Notes

- Cross-cutting libraries may build on technical software capabilities but should not depend on business-domain packages.
- `IdentitySecurityDomain` remains focused on architecture-oriented security (trust boundaries, exposure, identities, authorization).
- `CyberAssuranceDomain` and `EuCyberResilienceOverlay` model assurance, evidence, vulnerability handling, conformity, and regulatory obligations.
