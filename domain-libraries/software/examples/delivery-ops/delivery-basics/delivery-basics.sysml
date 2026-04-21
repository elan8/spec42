package DeliveryBasicsExample {
    import SoftwareCore::*;
    import SoftwareControlPlane::*;
    import SoftwareDelivery::*;

    part def OrdersRepo :> SourceRepository {
        attribute :>> repositoryName = "orders-service";
        attribute :>> repositoryUrl = "https://git.example.com/orders-service";
    }

    part def OrdersPackage :> SoftwarePackage {
        attribute :>> moduleName = "orders";
        attribute :>> packageName = "orders-service";
        attribute :>> version = "1.4.0";
    }

    part def OrdersArtifact :> BuildArtifact {
        attribute :>> artifactName = "orders-image";
        attribute :>> artifactUri = "oci://ghcr.io/example/orders";
        attribute :>> version = "1.4.0";
    }

    part def OrdersTests :> TestSuite {
        attribute :>> suiteName = "orders-integration";
    }

    part def OrdersPipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "orders-ci";
    }

    part def BuildStage :> PipelineStage {
        attribute :>> stageName = "build";
    }

    part def ReleaseStage :> PipelineStage {
        attribute :>> stageName = "release";
    }

    part def OrdersRelease :> ReleaseUnit {
        attribute :>> releaseName = "orders-2026-04";
        attribute :>> version = "1.4.0";
    }

    part def OrdersEnv :> DeploymentEnvironment {
        attribute :>> environmentName = "production";
    }
}
