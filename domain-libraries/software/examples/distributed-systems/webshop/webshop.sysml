package WebshopExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import HttpDomain::*;
    import KubernetesDomain::*;

    interface def OrdersApi :> ApiInterface;
    item def OrderCreated :> Event;

    part def OrderService :> HttpService {
        attribute :>> serviceName = "orders";
    }

    part def PaymentService :> HttpService {
        attribute :>> serviceName = "payments";
    }

    part def EventBus :> MessageBroker;

    part def Orders :> Topic {
        attribute :>> topicName = "orders.created";
    }

    part def PublicOrdersApi :> RestApiEndpoint {
        attribute :>> protocol = "https";
        attribute :>> path = "/orders";
        attribute :>> port = "443";
    }

    part def BackendCluster :> KubernetesCluster {
        attribute :>> clusterName = "webshop-cluster";
        attribute :>> nodeName = "backend-a";
        attribute :>> host = "backend-a.internal";
    }

    part def OrdersDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "orders";
        attribute :>> replicas = "3";
        attribute :>> image = "ghcr.io/example/orders:1.0.0";
    }

    part def PaymentsDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "payments";
        attribute :>> replicas = "2";
        attribute :>> image = "ghcr.io/example/payments:1.0.0";
    }

    allocate OrderService to BackendCluster;
    allocate PaymentService to BackendCluster;
    allocate EventBus to BackendCluster;
    allocate OrdersDeployment to BackendCluster;
    allocate PaymentsDeployment to BackendCluster;
}
