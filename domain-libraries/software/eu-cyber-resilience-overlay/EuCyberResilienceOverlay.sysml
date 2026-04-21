package EuCyberResilienceOverlay {
    import SoftwareCore::*;
    import SoftwareDelivery::*;
    import IdentitySecurityDomain::*;
    import CyberAssuranceDomain::*;

    attribute def ObligationType;
    attribute def RequirementClass;
    attribute def AssessmentPath;
    attribute def MonitoringCadence;
    attribute def ProductLifetime;

    part def ProductWithDigitalElements :> SoftwarePackage {
        attribute productName : Name;
    }

    part def ManufacturerObligation {
        attribute obligationName : Name;
        attribute obligationType : ObligationType;
    }

    part def EssentialRequirement :> ControlRequirement {
        attribute requirementClass : RequirementClass;
    }

    part def ConformityAssessment {
        attribute assessmentName : Name;
        attribute assessmentPath : AssessmentPath;
    }

    part def TechnicalDocumentation :> EvidenceArtifact {
        attribute documentName : Name;
    }

    part def DeclarationOfConformity :> EvidenceArtifact {
        attribute declarationName : Name;
    }

    part def PostMarketMonitoring {
        attribute monitoringName : Name;
        attribute cadence : MonitoringCadence;
    }

    part def VulnerabilityDisclosureObligation :> ReportingObligation;

    part def IncidentReportingDeadline :> ReportingObligation;

    part def SupportedProductLifetime {
        attribute lifetime : ProductLifetime;
    }
}
