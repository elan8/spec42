package CraBasicsExample {
    import SoftwareCore::*;
    import SoftwareDelivery::*;
    import CyberAssuranceDomain::*;
    import EuCyberResilienceOverlay::*;

    part def SmartGatewayProduct specializes ProductWithDigitalElements {
        attribute productName = "smart-gateway";
        attribute packageName = "smart-gateway";
        attribute version = "1.4.0";
    }

    part def GatewayRiskAssessment specializes RiskAssessment {
        attribute assessmentName = "gateway-network-risk";
        attribute severity = "high";
    }

    part def GatewaySbom specializes SbomArtifact {
        attribute evidenceName = "smart-gateway-sbom";
        attribute artifactName = "smart-gateway-cyclonedx";
        attribute artifactUri = "https://example.com/compliance/smart-gateway/sbom.json";
        attribute version = "1.4.0";
    }

    part def GatewayVulnerabilityProcess specializes VulnerabilityHandlingProcess {
        attribute processName = "psirt-triage";
    }

    part def GatewayDocs specializes TechnicalDocumentation {
        attribute documentName = "smart-gateway-technical-file";
        attribute evidenceName = "smart-gateway-technical-file";
        attribute artifactName = "technical-documentation";
        attribute artifactUri = "https://example.com/compliance/smart-gateway/technical-documentation";
        attribute version = "2026.04";
    }

    part def GatewayConformity specializes ConformityAssessment {
        attribute assessmentName = "internal-conformity-review";
        attribute assessmentPath = "manufacturer-self-assessment";
    }

    part def GatewayDeclaration specializes DeclarationOfConformity {
        attribute declarationName = "smart-gateway-eu-declaration";
        attribute evidenceName = "smart-gateway-eu-declaration";
        attribute artifactName = "eu-declaration-of-conformity";
        attribute artifactUri = "https://example.com/compliance/smart-gateway/declaration";
        attribute version = "2026.04";
    }

    part def GatewayDisclosure specializes VulnerabilityDisclosureObligation {
        attribute obligationName = "public-vulnerability-disclosure";
        attribute deadline = "without-undue-delay";
    }

    part def GatewayIncidentReporting specializes IncidentReportingDeadline {
        attribute obligationName = "active-exploit-incident-reporting";
        attribute deadline = "24-hours-initial-notice";
    }

    part def GatewayLifetime specializes SupportedProductLifetime {
        attribute lifetime = "5-years";
    }
}
