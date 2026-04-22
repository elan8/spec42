package CommercePlatformExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import HttpDomain::*;
    import OpenApiDomain::*;
    import GrpcDomain::*;
    import KafkaDomain::*;
    import SqlDomain::*;
    import NosqlDomain::*;
    import IdentitySecurityDomain::*;
    import SoftwareControlPlane::*;
    import ObservabilityDomain::*;
    import SoftwareDelivery::*;
    import KubernetesDomain::*;
    import CloudRuntimeDomain::*;

    port def CommercePort;

    part def PublicOrdersApi :> OpenApiOperation {
        attribute :>> protocol = "https";
        attribute :>> path = "/v1/orders";
        attribute :>> port = "443";
        attribute :>> encryption = "tls13";
        attribute :>> operationId = "createOrder";
        port ingress : CommercePort;
        port upstream : CommercePort;
    }

    part def OrdersService :> HttpService {
        attribute :>> serviceName = "orders-service";
        port apiIngress : CommercePort;
        port pricingRpcOut : CommercePort;
        port eventEgress : CommercePort;
        port sqlEgress : CommercePort;
        port cacheEgress : CommercePort;
        port configIn : CommercePort;
        port secretIn : CommercePort;
    }

    part def PricingService :> GrpcService {
        attribute :>> serviceName = "pricing-service";
        port grpcIngress : CommercePort;
    }

    part def PriceOrder :> UnaryRpc {
        attribute :>> methodName = "PriceOrder";
        port rpcIn : CommercePort;
        port rpcOut : CommercePort;
    }

    part def OrdersTopic :> KafkaTopic {
        attribute :>> channelName = "commerce.orders";
        attribute :>> retention = "7d";
        attribute :>> partitions = "12";
        port topicIngress : CommercePort;
        port topicEgress : CommercePort;
    }

    part def BillingConsumerGroup :> KafkaConsumerGroup {
        attribute :>> groupName = "billing-workers";
        port consumerIngress : CommercePort;
    }

    part def OrdersKafkaBinding :> KafkaBinding {
        attribute :>> bindingName = "orders-at-least-once";
        attribute :>> deliveryGuarantee = "at-least-once";
    }

    part def OrdersDatabase :> SqlDatabase {
        attribute :>> storeName = "orders-postgres";
        attribute :>> engineName = "postgres15";
        attribute :>> consistencyMode = "strong";
        port dbIngress : CommercePort;
    }

    part def OrdersCache :> KeyValueStore {
        attribute :>> storeName = "orders-redis";
        attribute :>> dataModel = "key-value";
        port cacheIngress : CommercePort;
    }

    part def OrdersConfig :> ConfigurationSource {
        attribute :>> sourceName = "orders-config";
        attribute :>> sourceUri = "vault://commerce/orders/config";
        port configOut : CommercePort;
    }

    part def OrdersSigningKey :> SecretItem {
        attribute :>> key = "JWT_SIGNING_KEY";
        port secretOut : CommercePort;
    }

    part def OrdersIdentity :> ServiceIdentity {
        attribute :>> identityName = "spiffe://commerce/orders-service";
    }

    part def PublicIngress :> ExposureBoundary {
        attribute :>> exposureType = "public";
        attribute :>> trafficDirection = "north-south";
    }

    part def OrdersMetrics :> MetricSource {
        attribute :>> sourceName = "orders-metrics";
        port metricOut : CommercePort;
    }

    part def OrdersTraces :> TraceSource {
        attribute :>> sourceName = "orders-traces";
    }

    part def CheckoutLatency :> ServiceLevelIndicator {
        attribute :>> sliName = "checkout-latency";
        attribute :>> signalKind = "latency";
        port indicatorIn : CommercePort;
        port indicatorOut : CommercePort;
    }

    part def CheckoutAvailability :> ServiceLevelObjective {
        attribute :>> sloName = "checkout-availability-99_95";
        port objectiveIn : CommercePort;
    }

    part def CommerceRepo :> SourceRepository {
        attribute :>> repositoryName = "commerce-platform";
        attribute :>> repositoryUrl = "https://git.example.com/commerce-platform";
        port repositoryOut : CommercePort;
    }

    part def CommercePipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "commerce-ci-cd";
        port pipelineIn : CommercePort;
        port pipelineOut : CommercePort;
    }

    part def CommerceRelease :> ReleaseUnit {
        attribute :>> releaseName = "commerce-2026-04";
        attribute :>> version = "3.4.0";
        port releaseIn : CommercePort;
    }

    part def CommerceCluster :> KubernetesCluster {
        attribute :>> clusterName = "commerce-prod";
        attribute :>> nodeName = "k8s-node-a";
        attribute :>> host = "k8s-node-a.internal";
    }

    part def OrdersDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "orders-service";
        attribute :>> replicas = "6";
        attribute :>> image = "ghcr.io/example/orders-service:3.4.0";
    }

    part def PricingDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "pricing-service";
        attribute :>> replicas = "4";
        attribute :>> image = "ghcr.io/example/pricing-service:2.8.1";
    }

    part def PrimaryRegion :> CloudRegion {
        attribute :>> regionName = "eu-west-1";
    }

    part def CommercePlatformSystem {
        part publicOrdersApi : PublicOrdersApi;
        part ordersService : OrdersService;
        part pricingService : PricingService;
        part priceOrder : PriceOrder;
        part ordersTopic : OrdersTopic;
        part billingConsumers : BillingConsumerGroup;
        part ordersDatabase : OrdersDatabase;
        part ordersCache : OrdersCache;
        part ordersConfig : OrdersConfig;
        part ordersSigningKey : OrdersSigningKey;
        part ordersMetrics : OrdersMetrics;
        part ordersTraces : OrdersTraces;
        part checkoutLatency : CheckoutLatency;
        part checkoutAvailability : CheckoutAvailability;
        part commerceRepo : CommerceRepo;
        part commercePipeline : CommercePipeline;
        part commerceRelease : CommerceRelease;

        connect publicOrdersApi.upstream to ordersService.apiIngress;
        connect ordersService.pricingRpcOut to pricingService.grpcIngress;
        connect pricingService.grpcIngress to priceOrder.rpcIn;
        connect priceOrder.rpcOut to ordersService.apiIngress;
        connect ordersService.eventEgress to ordersTopic.topicIngress;
        connect ordersTopic.topicEgress to billingConsumers.consumerIngress;
        connect ordersService.sqlEgress to ordersDatabase.dbIngress;
        connect ordersService.cacheEgress to ordersCache.cacheIngress;
        connect ordersConfig.configOut to ordersService.configIn;
        connect ordersSigningKey.secretOut to ordersService.secretIn;
        connect ordersMetrics.metricOut to checkoutLatency.indicatorIn;
        connect checkoutLatency.indicatorOut to checkoutAvailability.objectiveIn;
        connect commerceRepo.repositoryOut to commercePipeline.pipelineIn;
        connect commercePipeline.pipelineOut to commerceRelease.releaseIn;
    }

    allocate OrdersService to CommerceCluster;
    allocate PricingService to CommerceCluster;
    allocate OrdersDeployment to CommerceCluster;
    allocate PricingDeployment to CommerceCluster;
}
