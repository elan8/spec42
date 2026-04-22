package PlatformContext {
    import KafkaDomain::*;
    import SqlDomain::*;
    import NosqlDomain::*;
    import IdentitySecurityDomain::*;
    import SoftwareControlPlane::*;
    import ObservabilityDomain::*;
    import SoftwareDelivery::*;
    import KubernetesDomain::*;
    import CloudRuntimeDomain::*;
    import DistributedSystems::*;

    part def KafkaCluster :> MessageBroker;

    part def OrdersEventsTopic :> KafkaTopic {
        attribute :>> channelName = "commerce.orders.events.v1";
        attribute :>> retention = "7d";
        attribute :>> partitions = "12";
        port topicIngress;
        port topicEgress;
    }

    part def OrdersRetryTopic :> KafkaTopic {
        attribute :>> channelName = "commerce.orders.retry.v1";
        attribute :>> retention = "3d";
        attribute :>> partitions = "6";
        port topicIngress;
        port topicEgress;
    }

    part def OrdersDlqTopic :> KafkaTopic {
        attribute :>> channelName = "commerce.orders.dlq.v1";
        attribute :>> retention = "14d";
        attribute :>> partitions = "3";
        port topicIngress;
        port topicEgress;
    }

    part def OrdersKafkaBinding :> KafkaBinding {
        attribute :>> bindingName = "orders-at-least-once";
        attribute :>> deliveryGuarantee = "at-least-once";
    }

    part def OrdersDatabase :> SqlDatabase {
        attribute :>> storeName = "orders-postgres";
        attribute :>> engineName = "postgres15";
        attribute :>> consistencyMode = "strong";
        port dbIngress;
    }

    part def OrdersCache :> KeyValueStore {
        attribute :>> storeName = "orders-redis";
        attribute :>> dataModel = "key-value";
        port cacheIngress;
    }

    part def OrdersConfig :> ConfigurationSource {
        attribute :>> sourceName = "orders-config";
        attribute :>> sourceUri = "vault://commerce/orders/config";
        port configOut;
    }

    part def OrdersSigningKey :> SecretItem {
        attribute :>> key = "JWT_SIGNING_KEY";
        port secretOut;
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
        port metricOut;
    }

    part def OrdersLogs :> LogSource {
        attribute :>> sourceName = "orders-logs";
    }

    part def OrdersTraces :> TraceSource {
        attribute :>> sourceName = "orders-traces";
    }

    part def CheckoutLatency :> ServiceLevelIndicator {
        attribute :>> sliName = "checkout-latency";
        attribute :>> signalKind = "latency";
        port indicatorIn;
        port indicatorOut;
    }

    part def CheckoutAvailability :> ServiceLevelObjective {
        attribute :>> sloName = "checkout-availability-99_95";
        port objectiveIn;
    }

    part def CheckoutAlertPolicy :> AlertPolicy {
        attribute :>> policyName = "checkout-critical-alerts";
        attribute :>> severity = "critical";
    }

    part def CheckoutDashboard :> Dashboard {
        attribute :>> dashboardName = "checkout-overview";
    }

    part def CommerceRepo :> SourceRepository {
        attribute :>> repositoryName = "commerce-platform";
        attribute :>> repositoryUrl = "https://git.example.com/commerce-platform";
        port repositoryOut;
    }

    part def CommercePipeline :> DeliveryPipeline {
        attribute :>> pipelineName = "commerce-ci-cd";
        port pipelineIn;
        port pipelineOut;
    }

    part def CommerceRelease :> ReleaseUnit {
        attribute :>> releaseName = "commerce-2026-04";
        attribute :>> version = "3.5.0";
        port releaseIn;
    }

    part def CommerceCluster :> KubernetesCluster {
        attribute :>> clusterName = "commerce-prod";
        attribute :>> nodeName = "k8s-node-a";
        attribute :>> host = "k8s-node-a.internal";
    }

    part def CheckoutNamespace :> KubernetesNamespace {
        attribute :>> namespaceName = "checkout";
    }

    part def OrdersDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "orders-service";
        attribute :>> replicas = "6";
        attribute :>> image = "ghcr.io/example/orders-service:3.5.0";
    }

    part def PricingDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "pricing-service";
        attribute :>> replicas = "4";
        attribute :>> image = "ghcr.io/example/pricing-service:2.8.2";
    }

    part def PaymentsDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "payments-service";
        attribute :>> replicas = "4";
        attribute :>> image = "ghcr.io/example/payments-service:1.7.0";
    }

    part def FulfillmentDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "fulfillment-service";
        attribute :>> replicas = "3";
        attribute :>> image = "ghcr.io/example/fulfillment-service:1.2.0";
    }

    part def PrimaryRegion :> CloudRegion {
        attribute :>> regionName = "eu-west-1";
    }
}
