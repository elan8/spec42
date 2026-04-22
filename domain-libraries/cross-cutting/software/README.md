# Software Cross-Cutting Libraries

This directory contains software overlays that cross technical subdomains and support governance, operations, security, and assurance.

## Structure

- `delivery-ops/` - runtime control, delivery governance, and observability overlays (`SoftwareControlPlane`, `SoftwareDelivery`, `ObservabilityDomain`)
- `security/` - architecture/security + assurance/regulatory overlays (`IdentitySecurityDomain`, `CyberAssuranceDomain`, `EuCyberResilienceOverlay`)
- `delivery-ops/rules/*.yaml` - delivery/control-plane/observability rule catalogs
- `security/rules/*.yaml` - identity/assurance/CRA rule catalogs

## Notes

- Cross-cutting libraries may build on technical software capabilities but should not depend on business-domain packages.
- `IdentitySecurityDomain` remains focused on architecture-oriented security (trust boundaries, exposure, identities, authorization).
- `CyberAssuranceDomain` and `EuCyberResilienceOverlay` model assurance, evidence, vulnerability handling, conformity, and regulatory obligations.
