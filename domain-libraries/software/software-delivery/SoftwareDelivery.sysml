package SoftwareDelivery {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SoftwareControlPlane::*;

    attribute def RepositoryUrl;
    attribute def StageName;
    attribute def VersionPolicy;

    part def SourceRepository {
        attribute repositoryName : Name;
        attribute repositoryUrl : RepositoryUrl;
    }

    part def SoftwarePackage specializes SoftwareModule {
        attribute packageName : Name;
        attribute version : VersionString;
    }

    part def BuildArtifact specializes ArtifactReference;

    part def TestSuite {
        attribute suiteName : Name;
    }

    part def DeliveryPipeline {
        attribute pipelineName : Name;
    }

    part def PipelineStage {
        attribute stageName : StageName;
    }

    part def ReleaseUnit {
        attribute releaseName : Name;
        attribute version : VersionString;
    }

    part def PromotionPath {
        attribute pathName : Name;
    }

    part def DeploymentEnvironment specializes Environment;

    part def ChangeRequest {
        attribute requestId : Identifier;
    }

    part def CompatibilityContract {
        attribute contractName : Name;
        attribute versionPolicy : VersionPolicy;
    }
}
