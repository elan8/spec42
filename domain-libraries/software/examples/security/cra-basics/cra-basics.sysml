package CraBasicsExample {
    import SoftwareCore::*;
    import SoftwareDelivery::*;
    import CyberAssuranceDomain::*;
    import EuCyberResilienceOverlay::*;

    port def CraPort;

    part def SmartGatewayProduct :> ProductWithDigitalElements {
        attribute :>> productName = "smart-gateway";
        attribute :>> packageName = "smart-gateway";
        attribute :>> version = "1.4.0";
        port riskIn : CraPort;
    }

    part def GatewayRiskAssessment :> RiskAssessment {
        attribute :>> assessmentName = "gateway-network-risk";
        attribute :>> severity = "high";
        port riskOut : CraPort;
    }

    part def GatewaySbom :> SbomArtifact {
        attribute :>> evidenceName = "smart-gateway-sbom";
        attribute :>> artifactName = "smart-gateway-cyclonedx";
        attribute :>> artifactUri = "https://example.com/compliance/smart-gateway/sbom.json";
        attribute :>> version = "1.4.0";
        port sbomOut : CraPort;
    }

    part def GatewayVulnerabilityProcess :> VulnerabilityHandlingProcess {
        attribute :>> processName = "psirt-triage";
    }

    part def GatewayDocs :> TechnicalDocumentation {
        attribute :>> documentName = "smart-gateway-technical-file";
        attribute :>> evidenceName = "smart-gateway-technical-file";
        attribute :>> artifactName = "technical-documentation";
        attribute :>> artifactUri = "https://example.com/compliance/smart-gateway/technical-documentation";
        attribute :>> version = "2026.04";
    }

    part def GatewayConformity :> ConformityAssessment {
        attribute :>> assessmentName = "internal-conformity-review";
        attribute :>> assessmentPath = "manufacturer-self-assessment";
        port conformityIn : CraPort;
        port conformityOut : CraPort;
    }

    part def GatewayDeclaration :> DeclarationOfConformity {
        attribute :>> declarationName = "smart-gateway-eu-declaration";
        attribute :>> evidenceName = "smart-gateway-eu-declaration";
        attribute :>> artifactName = "eu-declaration-of-conformity";
        attribute :>> artifactUri = "https://example.com/compliance/smart-gateway/declaration";
        attribute :>> version = "2026.04";
        port declarationIn : CraPort;
    }

    part def GatewayDisclosure :> VulnerabilityDisclosureObligation {
        attribute :>> obligationName = "public-vulnerability-disclosure";
        attribute :>> deadline = "without-undue-delay";
    }

    part def GatewayIncidentReporting :> IncidentReportingDeadline {
        attribute :>> obligationName = "active-exploit-incident-reporting";
        attribute :>> deadline = "24-hours-initial-notice";
    }

    part def GatewayLifetime :> SupportedProductLifetime {
        attribute :>> lifetime = "5-years";
    }

    part def CraComplianceSystem {
        part product : SmartGatewayProduct;
        part riskAssessment : GatewayRiskAssessment;
        part sbom : GatewaySbom;
        part conformity : GatewayConformity;
        part declaration : GatewayDeclaration;

        connect riskAssessment.riskOut to product.riskIn;
        connect sbom.sbomOut to conformity.conformityIn;
        connect conformity.conformityOut to declaration.declarationIn;
    }
}
