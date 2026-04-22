package RegulatoryComplianceExample {
    import SoftwareCore::*;
    import SoftwareDelivery::*;
    import SoftwareControlPlane::*;
    import IdentitySecurityDomain::*;
    import ObservabilityDomain::*;
    import CyberAssuranceDomain::*;
    import EuCyberResilienceOverlay::*;
    import CloudRuntimeDomain::*;

    port def CompliancePort;

    part def ProductPackage :> ProductWithDigitalElements {
        attribute :>> moduleName = "connected-thermostat-platform";
        attribute :>> packageName = "connected-thermostat-platform";
        attribute :>> version = "2.3.0";
        attribute :>> productName = "ThermostatControlSuite";
        port productOut : CompliancePort;
    }

    part def SourceRepo :> SourceRepository {
        attribute :>> repositoryName = "thermostat-control-suite";
        attribute :>> repositoryUrl = "https://git.example.com/thermostat-control-suite";
        port repositoryOut : CompliancePort;
    }

    part def DeliveryPipelineCra :> DeliveryPipeline {
        attribute :>> pipelineName = "cra-evidence-pipeline";
        port pipelineIn : CompliancePort;
        port pipelineOut : CompliancePort;
    }

    part def ProductRelease :> ReleaseUnit {
        attribute :>> releaseName = "thermostat-2026-q2";
        attribute :>> version = "2.3.0";
        port releaseIn : CompliancePort;
    }

    part def EssentialAuthRequirement :> EssentialRequirement {
        attribute :>> controlName = "authenticated-update-channel";
        attribute :>> controlId = "CRA-ER-5.2";
        attribute :>> requirementClass = "essential-cybersecurity";
        port requirementIn : CompliancePort;
    }

    part def ThreatRiskAssessment :> RiskAssessment {
        attribute :>> assessmentName = "remote-tampering-risk";
        attribute :>> severity = "high";
        port riskOut : CompliancePort;
    }

    part def ThreatScenarioRemoteTamper :> ThreatScenario {
        attribute :>> scenarioName = "remote-firmware-tamper";
    }

    part def ProductSbom :> SbomArtifact {
        attribute :>> evidenceName = "thermostat-sbom";
        attribute :>> artifactName = "thermostat-sbom.cdx.json";
        attribute :>> artifactUri = "oci://registry.example.com/thermostat/sbom";
        attribute :>> version = "2.3.0";
        port evidenceOut : CompliancePort;
    }

    part def BuildProvenance :> ProvenanceArtifact {
        attribute :>> evidenceName = "thermostat-build-provenance";
        attribute :>> artifactName = "provenance.intoto.jsonl";
        attribute :>> artifactUri = "oci://registry.example.com/thermostat/provenance";
        attribute :>> version = "2.3.0";
        port evidenceOut : CompliancePort;
    }

    part def TechDocs :> TechnicalDocumentation {
        attribute :>> evidenceName = "technical-documentation";
        attribute :>> artifactName = "tech-docs.pdf";
        attribute :>> artifactUri = "https://docs.example.com/thermostat/2.3.0";
        attribute :>> version = "2.3.0";
        attribute :>> documentName = "Thermostat CRA Technical Documentation";
        port docsOut : CompliancePort;
    }

    part def ConformityAssessmentCra :> ConformityAssessment {
        attribute :>> assessmentName = "cra-self-assessment-2026-q2";
        attribute :>> assessmentPath = "internal-control-path";
        port assessmentIn : CompliancePort;
        port assessmentOut : CompliancePort;
    }

    part def DeclarationCra :> DeclarationOfConformity {
        attribute :>> evidenceName = "declaration-of-conformity";
        attribute :>> artifactName = "eu-declaration.pdf";
        attribute :>> artifactUri = "https://compliance.example.com/thermostat/declaration";
        attribute :>> version = "2.3.0";
        attribute :>> declarationName = "EU Declaration of Conformity - Thermostat";
        port declarationIn : CompliancePort;
    }

    part def OpenVulnerability :> VulnerabilityRecord {
        attribute :>> vulnerabilityId = "CVE-2026-44210";
        attribute :>> status = "open";
        port vulnerabilityOut : CompliancePort;
    }

    part def VulnerabilityProcess :> VulnerabilityHandlingProcess {
        attribute :>> processName = "psirt-triage-and-remediation";
        port processIn : CompliancePort;
        port processOut : CompliancePort;
    }

    part def ReportingDeadlineCra :> IncidentReportingDeadline {
        attribute :>> obligationName = "notify-csirt-within-24h";
        attribute :>> deadline = "24h";
        port reportingIn : CompliancePort;
    }

    part def VulnerabilityDisclosure :> VulnerabilityDisclosureObligation {
        attribute :>> obligationName = "publish-vuln-advisory";
        attribute :>> deadline = "72h";
        port disclosureIn : CompliancePort;
    }

    part def PostMarketOps :> PostMarketMonitoring {
        attribute :>> monitoringName = "security-monitoring";
        attribute :>> cadence = "daily";
        port monitoringOut : CompliancePort;
    }

    part def SupportedLifetime :> SupportedProductLifetime {
        attribute :>> lifetime = "5y";
    }

    part def ComplianceSli :> ServiceLevelIndicator {
        attribute :>> sliName = "vulnerability-remediation-time";
        attribute :>> signalKind = "duration";
        port indicatorOut : CompliancePort;
    }

    part def ComplianceSlo :> ServiceLevelObjective {
        attribute :>> sloName = "remediation-under-15-days";
        port objectiveIn : CompliancePort;
    }

    part def AuditRegion :> CloudRegion {
        attribute :>> regionName = "eu-north-1";
    }

    part def RegulatoryComplianceSystem {
        part productPackage : ProductPackage;
        part sourceRepo : SourceRepo;
        part deliveryPipelineCra : DeliveryPipelineCra;
        part productRelease : ProductRelease;
        part essentialAuthRequirement : EssentialAuthRequirement;
        part threatRiskAssessment : ThreatRiskAssessment;
        part productSbom : ProductSbom;
        part buildProvenance : BuildProvenance;
        part techDocs : TechDocs;
        part conformityAssessmentCra : ConformityAssessmentCra;
        part declarationCra : DeclarationCra;
        part openVulnerability : OpenVulnerability;
        part vulnerabilityProcess : VulnerabilityProcess;
        part reportingDeadlineCra : ReportingDeadlineCra;
        part vulnerabilityDisclosure : VulnerabilityDisclosure;
        part postMarketOps : PostMarketOps;
        part complianceSli : ComplianceSli;
        part complianceSlo : ComplianceSlo;

        connect sourceRepo.repositoryOut to deliveryPipelineCra.pipelineIn;
        connect deliveryPipelineCra.pipelineOut to productRelease.releaseIn;
        connect productPackage.productOut to conformityAssessmentCra.assessmentIn;
        connect essentialAuthRequirement.requirementIn to conformityAssessmentCra.assessmentIn;
        connect threatRiskAssessment.riskOut to conformityAssessmentCra.assessmentIn;
        connect productSbom.evidenceOut to conformityAssessmentCra.assessmentIn;
        connect buildProvenance.evidenceOut to conformityAssessmentCra.assessmentIn;
        connect techDocs.docsOut to conformityAssessmentCra.assessmentIn;
        connect conformityAssessmentCra.assessmentOut to declarationCra.declarationIn;
        connect openVulnerability.vulnerabilityOut to vulnerabilityProcess.processIn;
        connect vulnerabilityProcess.processOut to reportingDeadlineCra.reportingIn;
        connect vulnerabilityProcess.processOut to vulnerabilityDisclosure.disclosureIn;
        connect postMarketOps.monitoringOut to openVulnerability.vulnerabilityOut;
        connect complianceSli.indicatorOut to complianceSlo.objectiveIn;
    }

    allocate ProductPackage to AuditRegion;
    allocate DeliveryPipelineCra to AuditRegion;
}
