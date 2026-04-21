package DeliveryBasicsExample {
    import SoftwareCore::*;
    import SoftwareControlPlane::*;
    import SoftwareDelivery::*;

    port def DeliveryPort;

    part def OrdersRepo :> SourceRepository {
        attribute :>> repositoryName = "orders-service";
        attribute :>> repositoryUrl = "https://git.example.com/orders-service";
        port sourceOut : DeliveryPort;
    }

    part def OrdersPackage :> SoftwarePackage {
        attribute :>> moduleName = "orders";
        attribute :>> packageName = "orders-service";
        attribute :>> version = "1.4.0";
        port packageIn : DeliveryPort;
        port packageOut : DeliveryPort;
    }

    part def OrdersArtifact :> BuildArtifact {
        attribute :>> artifactName = "orders-image";
        attribute :>> artifactUri = "oci://ghcr.io/example/orders";
        attribute :>> version = "1.4.0";
        port artifactIn : DeliveryPort;
        port artifactOut : DeliveryPort;
    }

    part def OrdersTests :> TestSuite {
        attribute :>> suiteName = "orders-integration";
    }

    part def OrdersPipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "orders-ci";
        port pipelineIn : DeliveryPort;
        port pipelineOut : DeliveryPort;
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
        port releaseIn : DeliveryPort;
    }

    part def OrdersEnv :> DeploymentEnvironment {
        attribute :>> environmentName = "production";
    }

    part def DeliverySystem {
        part ordersRepo : OrdersRepo;
        part ordersPackage : OrdersPackage;
        part ordersArtifact : OrdersArtifact;
        part ordersTests : OrdersTests;
        part ordersPipeline : OrdersPipeline;
        part buildStage : BuildStage;
        part releaseStage : ReleaseStage;
        part ordersRelease : OrdersRelease;
        part ordersEnv : OrdersEnv;

        connect ordersRepo.sourceOut to ordersPackage.packageIn;
        connect ordersPackage.packageOut to ordersArtifact.artifactIn;
        connect ordersArtifact.artifactOut to ordersPipeline.pipelineIn;
        connect ordersPipeline.pipelineOut to ordersRelease.releaseIn;
    }
}
