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

    port def ReferencePort;

    part def OrdersService :> HttpService {
        attribute :>> serviceName = "orders";
        port httpIngress : ReferencePort;
        port eventEgress : ReferencePort;
    }

    part def BillingService :> GrpcService {
        attribute :>> serviceName = "billing";
        port grpcIngress : ReferencePort;
    }

    part def OrdersApi :> OpenApiOperation {
        attribute :>> protocol = "https";
        attribute :>> path = "/orders";
        attribute :>> port = "443";
        attribute :>> encryption = tls;
        attribute :>> operationId = "listOrders";
        port apiIngress : ReferencePort;
        port apiOut : ReferencePort;
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
        port rpcIn : ReferencePort;
        port rpcOut : ReferencePort;
    }

    part def OrdersTopic :> KafkaTopic {
        attribute :>> channelName = "orders";
        attribute :>> retention = "14d";
        attribute :>> partitions = "6";
        port topicIngress : ReferencePort;
        port topicEgress : ReferencePort;
    }

    part def OrdersDatabase :> SqlDatabase {
        attribute :>> storeName = "orders-db";
        attribute :>> engineName = "postgres";
        attribute :>> consistencyMode = "strong";
        port dbIngress : ReferencePort;
    }

    part def OrdersCache :> KeyValueStore {
        attribute :>> storeName = "orders-cache";
        attribute :>> dataModel = "key-value";
        port cacheIngress : ReferencePort;
    }

    part def OrdersConfig :> ConfigurationSource {
        attribute :>> sourceName = "orders-config";
        attribute :>> sourceUri = "vault://orders/config";
        port configOut : ReferencePort;
    }

    part def OrdersSecret :> SecretItem {
        attribute :>> key = "JWT_SIGNING_KEY";
        port secretOut : ReferencePort;
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
        port metricOut : ReferencePort;
    }

    part def OrdersLatency :> ServiceLevelIndicator {
        attribute :>> sliName = "orders-latency";
        attribute :>> signalKind = "latency";
        port indicatorIn : ReferencePort;
        port indicatorOut : ReferencePort;
    }

    part def OrdersSlo :> ServiceLevelObjective {
        attribute :>> sloName = "orders-availability";
        port objectiveIn : ReferencePort;
    }

    part def OrdersRepo :> SourceRepository {
        attribute :>> repositoryName = "orders-service";
        attribute :>> repositoryUrl = "https://git.example.com/orders-service";
        port repositoryOut : ReferencePort;
    }

    part def OrdersPipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "orders-ci";
        port pipelineIn : ReferencePort;
        port pipelineOut : ReferencePort;
    }

    part def OrdersRelease :> ReleaseUnit {
        attribute :>> releaseName = "orders-2026-04";
        attribute :>> version = "2.0.0";
        port releaseIn : ReferencePort;
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

    part def ModernSoftwareSystem {
        part ordersService : OrdersService;
        part billingService : BillingService;
        part ordersApi : OrdersApi;
        part billOrder : BillOrder;
        part ordersTopic : OrdersTopic;
        part ordersDatabase : OrdersDatabase;
        part ordersCache : OrdersCache;
        part ordersConfig : OrdersConfig;
        part ordersSecret : OrdersSecret;
        part ordersMetrics : OrdersMetrics;
        part ordersLatency : OrdersLatency;
        part ordersSlo : OrdersSlo;
        part ordersRepo : OrdersRepo;
        part ordersPipeline : OrdersPipeline;
        part ordersRelease : OrdersRelease;
        part ordersCluster : OrdersCluster;
        part ordersDeployment : OrdersDeployment;
        part primaryRegion : PrimaryRegion;

        connect ordersApi.apiIngress to ordersService.httpIngress;
        connect ordersService.eventEgress to ordersTopic.topicIngress;
        connect ordersTopic.topicEgress to billingService.grpcIngress;
        connect billingService.grpcIngress to billOrder.rpcIn;
        connect billOrder.rpcOut to ordersService.httpIngress;
        connect ordersService.httpIngress to ordersDatabase.dbIngress;
        connect ordersService.httpIngress to ordersCache.cacheIngress;
        connect ordersConfig.configOut to ordersService.httpIngress;
        connect ordersSecret.secretOut to ordersService.httpIngress;
        connect ordersMetrics.metricOut to ordersLatency.indicatorIn;
        connect ordersLatency.indicatorOut to ordersSlo.objectiveIn;
        connect ordersRepo.repositoryOut to ordersPipeline.pipelineIn;
        connect ordersPipeline.pipelineOut to ordersRelease.releaseIn;
    }

    allocate OrdersService to OrdersCluster;
    allocate BillingService to OrdersCluster;
    allocate OrdersDeployment to OrdersCluster;
}
