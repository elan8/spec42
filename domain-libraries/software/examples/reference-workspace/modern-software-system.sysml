package ModernSoftwareReference {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SoftwareControlPlane::*;
    import SoftwareDelivery::*;
    import HttpDomain::*;
    import OpenApiDomain::*;
    import SerializationDomain::*;
    import GrpcDomain::*;
    import MessagingDomain::*;
    import KafkaDomain::*;
    import SqlDomain::*;
    import NosqlDomain::*;
    import IdentitySecurityDomain::*;
    import ObservabilityDomain::*;
    import KubernetesDomain::*;
    import CloudRuntimeDomain::*;

    part def OrdersService :> HttpService {
        attribute :>> serviceName = "orders";
    }

    part def BillingService :> GrpcService {
        attribute :>> serviceName = "billing";
    }

    part def OrdersApi :> OpenApiOperation {
        attribute :>> protocol = "https";
        attribute :>> path = "/orders";
        attribute :>> port = "443";
        attribute :>> operationId = "listOrders";
    }

    item def OrderJson :> OpenApiSchema {
        attribute :>> mediaType = "application/json";
        attribute :>> schema = "#/components/schemas/Order";
    }

    item def BillRequest :> GrpcRequest {
        attribute :>> mediaType = "application/protobuf";
        attribute :>> schema = "billing.v1.BillRequest";
        attribute :>> packageName = "billing.v1";
    }

    item def BillResponse :> GrpcResponse {
        attribute :>> mediaType = "application/protobuf";
        attribute :>> schema = "billing.v1.BillResponse";
        attribute :>> packageName = "billing.v1";
    }

    part def BillOrder :> UnaryRpc {
        attribute :>> methodName = "BillOrder";
    }

    part def OrdersTopic :> KafkaTopic {
        attribute :>> channelName = "orders";
        attribute :>> retention = "14d";
        attribute :>> partitions = "6";
    }

    part def OrdersDatabase :> SqlDatabase {
        attribute :>> storeName = "orders-db";
        attribute :>> engineName = "postgres";
        attribute :>> consistencyMode = "strong";
    }

    part def OrdersCache :> KeyValueStore {
        attribute :>> storeName = "orders-cache";
        attribute :>> dataModel = "key-value";
    }

    part def OrdersConfig :> ConfigurationSource {
        attribute :>> sourceName = "orders-config";
        attribute :>> sourceUri = "vault://orders/config";
    }

    part def OrdersSecret :> SecretItem {
        attribute :>> key = "JWT_SIGNING_KEY";
    }

    part def OrdersIdentity :> ServiceIdentity {
        attribute :>> identityName = "orders-service";
    }

    part def OrdersIngress :> ExposureBoundary {
        attribute :>> exposureType = "public";
        attribute :>> trafficDirection = "north-south";
    }

    part def OrdersMetrics :> MetricSource {
        attribute :>> sourceName = "orders-metrics";
    }

    part def OrdersLatency :> ServiceLevelIndicator {
        attribute :>> sliName = "orders-latency";
        attribute :>> signalKind = "latency";
    }

    part def OrdersSlo :> ServiceLevelObjective {
        attribute :>> sloName = "orders-availability";
    }

    part def OrdersRepo :> SourceRepository {
        attribute :>> repositoryName = "orders-service";
        attribute :>> repositoryUrl = "https://git.example.com/orders-service";
    }

    part def OrdersPipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "orders-ci";
    }

    part def OrdersRelease :> ReleaseUnit {
        attribute :>> releaseName = "orders-2026-04";
        attribute :>> version = "2.0.0";
    }

    part def OrdersCluster :> KubernetesCluster {
        attribute :>> clusterName = "orders-prod";
        attribute :>> nodeName = "orders-cluster-a";
        attribute :>> host = "orders-cluster-a.internal";
    }

    part def OrdersDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "orders";
        attribute :>> replicas = "3";
        attribute :>> image = "ghcr.io/example/orders:2.0.0";
    }

    part def PrimaryRegion :> CloudRegion {
        attribute :>> regionName = "eu-west";
    }

    allocate OrdersService to OrdersCluster;
    allocate BillingService to OrdersCluster;
    allocate OrdersDeployment to OrdersCluster;
}
