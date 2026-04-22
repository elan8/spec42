package ProgressiveDeliveryExample {
    import SoftwareCore::*;
    import SoftwareDelivery::*;
    import SoftwareControlPlane::*;
    import ObservabilityDomain::*;
    import CyberAssuranceDomain::*;
    import KubernetesDomain::*;
    import CloudRuntimeDomain::*;
    import IdentitySecurityDomain::*;

    port def DeliveryPort;

    part def CheckoutRepo :> SourceRepository {
        attribute :>> repositoryName = "checkout-service";
        attribute :>> repositoryUrl = "https://git.example.com/checkout-service";
        port repositoryOut : DeliveryPort;
    }

    part def CheckoutPipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "checkout-progressive-delivery";
        port pipelineIn : DeliveryPort;
        port pipelineOut : DeliveryPort;
    }

    part def BuildStage :> PipelineStage {
        attribute :>> stageName = "build";
        port stageIn : DeliveryPort;
        port stageOut : DeliveryPort;
    }

    part def SecurityScanStage :> PipelineStage {
        attribute :>> stageName = "security-scan";
        port stageIn : DeliveryPort;
        port stageOut : DeliveryPort;
    }

    part def CanaryStage :> PipelineStage {
        attribute :>> stageName = "canary-rollout";
        port stageIn : DeliveryPort;
        port stageOut : DeliveryPort;
    }

    part def PromoteStage :> PipelineStage {
        attribute :>> stageName = "promote-prod";
        port stageIn : DeliveryPort;
        port stageOut : DeliveryPort;
    }

    part def RollbackRequest :> ChangeRequest {
        attribute :>> requestId = "CHG-2026-047";
        port rollbackIn : DeliveryPort;
    }

    part def CheckoutImage :> BuildArtifact {
        attribute :>> artifactName = "checkout-image";
        attribute :>> artifactUri = "ghcr.io/example/checkout-service";
        attribute :>> version = "4.1.0";
        port artifactOut : DeliveryPort;
    }

    part def CheckoutSbom :> SbomArtifact {
        attribute :>> evidenceName = "checkout-sbom";
        attribute :>> artifactName = "sbom.cdx.json";
        attribute :>> artifactUri = "oci://ghcr.io/example/checkout-service/sbom";
        attribute :>> version = "4.1.0";
        port evidenceOut : DeliveryPort;
    }

    part def CheckoutProvenance :> ProvenanceArtifact {
        attribute :>> evidenceName = "checkout-slsa-provenance";
        attribute :>> artifactName = "provenance.intoto.jsonl";
        attribute :>> artifactUri = "oci://ghcr.io/example/checkout-service/provenance";
        attribute :>> version = "4.1.0";
        port evidenceOut : DeliveryPort;
    }

    part def CheckoutRelease :> ReleaseUnit {
        attribute :>> releaseName = "checkout-2026-04";
        attribute :>> version = "4.1.0";
        port releaseIn : DeliveryPort;
    }

    part def StagingEnvironment :> DeploymentEnvironment {
        attribute :>> environmentName = "staging";
    }

    part def ProductionEnvironment :> DeploymentEnvironment {
        attribute :>> environmentName = "production";
    }

    part def CheckoutLatencySli :> ServiceLevelIndicator {
        attribute :>> sliName = "checkout-p95-latency";
        attribute :>> signalKind = "latency";
        port indicatorIn : DeliveryPort;
        port indicatorOut : DeliveryPort;
    }

    part def CheckoutErrorRateSli :> ServiceLevelIndicator {
        attribute :>> sliName = "checkout-error-rate";
        attribute :>> signalKind = "error-rate";
        port indicatorIn : DeliveryPort;
        port indicatorOut : DeliveryPort;
    }

    part def CheckoutAvailabilitySlo :> ServiceLevelObjective {
        attribute :>> sloName = "checkout-availability-99_95";
        port objectiveIn : DeliveryPort;
    }

    part def ReleaseGatePolicy :> AuthorizationPolicy {
        attribute :>> policyName = "release-gate-policy";
        port policyIn : DeliveryPort;
        port policyOut : DeliveryPort;
    }

    part def ProductionCluster :> KubernetesCluster {
        attribute :>> clusterName = "checkout-prod";
        attribute :>> nodeName = "prod-node-1";
        attribute :>> host = "prod-node-1.internal";
    }

    part def CheckoutCanary :> KubernetesDeployment {
        attribute :>> deploymentName = "checkout-canary";
        attribute :>> replicas = "1";
        attribute :>> image = "ghcr.io/example/checkout-service:4.1.0";
    }

    part def CheckoutPrimary :> KubernetesDeployment {
        attribute :>> deploymentName = "checkout-primary";
        attribute :>> replicas = "8";
        attribute :>> image = "ghcr.io/example/checkout-service:4.1.0";
    }

    part def DeliveryRegion :> CloudRegion {
        attribute :>> regionName = "eu-central-1";
    }

    part def ProgressiveDeliverySystem {
        part checkoutRepo : CheckoutRepo;
        part checkoutPipeline : CheckoutPipeline;
        part buildStage : BuildStage;
        part securityScanStage : SecurityScanStage;
        part canaryStage : CanaryStage;
        part promoteStage : PromoteStage;
        part rollbackRequest : RollbackRequest;
        part checkoutImage : CheckoutImage;
        part checkoutSbom : CheckoutSbom;
        part checkoutProvenance : CheckoutProvenance;
        part checkoutRelease : CheckoutRelease;
        part checkoutLatencySli : CheckoutLatencySli;
        part checkoutErrorRateSli : CheckoutErrorRateSli;
        part checkoutAvailabilitySlo : CheckoutAvailabilitySlo;
        part releaseGatePolicy : ReleaseGatePolicy;

        connect checkoutRepo.repositoryOut to checkoutPipeline.pipelineIn;
        connect checkoutPipeline.pipelineOut to buildStage.stageIn;
        connect buildStage.stageOut to securityScanStage.stageIn;
        connect securityScanStage.stageOut to canaryStage.stageIn;
        connect canaryStage.stageOut to promoteStage.stageIn;
        connect promoteStage.stageOut to checkoutRelease.releaseIn;
        connect checkoutImage.artifactOut to canaryStage.stageIn;
        connect checkoutSbom.evidenceOut to releaseGatePolicy.policyIn;
        connect checkoutProvenance.evidenceOut to releaseGatePolicy.policyIn;
        connect checkoutLatencySli.indicatorOut to checkoutAvailabilitySlo.objectiveIn;
        connect checkoutErrorRateSli.indicatorOut to checkoutAvailabilitySlo.objectiveIn;
        connect releaseGatePolicy.policyOut to promoteStage.stageIn;
        connect checkoutAvailabilitySlo.objectiveIn to rollbackRequest.rollbackIn;
    }

    allocate CheckoutCanary to ProductionCluster;
    allocate CheckoutPrimary to ProductionCluster;
}
