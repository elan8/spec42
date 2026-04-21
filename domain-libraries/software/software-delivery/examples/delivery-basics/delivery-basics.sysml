package DeliveryBasicsExample {
    import SoftwareCore::*;
    import SoftwareControlPlane::*;
    import SoftwareDelivery::*;

    part def OrdersRepo specializes SourceRepository {
        attribute repositoryName = "orders-service";
        attribute repositoryUrl = "https://git.example.com/orders-service";
    }

    part def OrdersPackage specializes SoftwarePackage {
        attribute moduleName = "orders";
        attribute packageName = "orders-service";
        attribute version = "1.4.0";
    }

    part def OrdersArtifact specializes BuildArtifact {
        attribute artifactName = "orders-image";
        attribute artifactUri = "oci://ghcr.io/example/orders";
        attribute version = "1.4.0";
    }

    part def OrdersTests specializes TestSuite {
        attribute suiteName = "orders-integration";
    }

    part def OrdersPipeline specializes DeliveryPipeline {
        attribute pipelineName = "orders-ci";
    }

    part def BuildStage specializes PipelineStage {
        attribute stageName = "build";
    }

    part def ReleaseStage specializes PipelineStage {
        attribute stageName = "release";
    }

    part def OrdersRelease specializes ReleaseUnit {
        attribute releaseName = "orders-2026-04";
        attribute version = "1.4.0";
    }

    part def OrdersEnv specializes DeploymentEnvironment {
        attribute environmentName = "production";
    }
}
