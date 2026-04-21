package CyberAssuranceDomain {
    import SoftwareCore::*;
    import SoftwareDelivery::*;
    import IdentitySecurityDomain::*;
    import ObservabilityDomain::*;

    attribute def RequirementCategory;
    attribute def AssuranceStatus;
    attribute def AssuranceLevelName;
    attribute def RiskSeverity;
    attribute def VulnerabilityStatus;
    attribute def IncidentSeverity;
    attribute def ReportingDeadline;
    attribute def ScopeType;

    part def SecurityRequirementSource {
        attribute sourceName : Name;
        attribute sourceUri : UriString;
        attribute category : RequirementCategory;
    }

    part def ControlRequirement {
        attribute controlName : Name;
        attribute controlId : Identifier;
    }

    part def AssuranceClaim {
        attribute claimName : Name;
        attribute status : AssuranceStatus;
    }

    part def EvidenceArtifact specializes ArtifactReference {
        attribute evidenceName : Name;
    }

    part def AssuranceLevel {
        attribute levelName : AssuranceLevelName;
    }

    part def RiskAssessment {
        attribute assessmentName : Name;
        attribute severity : RiskSeverity;
    }

    part def ThreatScenario {
        attribute scenarioName : Name;
    }

    part def VulnerabilityRecord {
        attribute vulnerabilityId : Identifier;
        attribute status : VulnerabilityStatus;
    }

    part def VulnerabilityHandlingProcess {
        attribute processName : Name;
    }

    part def SbomArtifact specializes EvidenceArtifact;

    part def ProvenanceArtifact specializes EvidenceArtifact;

    part def SecurityIncident {
        attribute incidentId : Identifier;
        attribute severity : IncidentSeverity;
    }

    part def ReportingObligation {
        attribute obligationName : Name;
        attribute deadline : ReportingDeadline;
    }

    part def ConformityScope {
        attribute scopeName : Name;
        attribute scopeType : ScopeType;
    }
}
