package WebshopExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import HttpDomain::*;
    import KubernetesDomain::*;

    interface def OrdersApi specializes ApiInterface;
    item def OrderCreated specializes Event;

    part def OrderService specializes HttpService {
        attribute serviceName = "orders";
    }

    part def PaymentService specializes HttpService {
        attribute serviceName = "payments";
    }

    part def EventBus specializes MessageBroker;

    part def Orders specializes Topic {
        attribute topicName = "orders.created";
    }

    part def PublicOrdersApi specializes RestApiEndpoint {
        attribute protocol = "https";
        attribute path = "/orders";
        attribute port = "443";
    }

    part def BackendCluster specializes KubernetesCluster {
        attribute clusterName = "webshop-cluster";
        attribute nodeName = "backend-a";
        attribute host = "backend-a.internal";
    }

    part def OrdersDeployment specializes KubernetesDeployment {
        attribute deploymentName = "orders";
        attribute replicas = "3";
        attribute image = "ghcr.io/example/orders:1.0.0";
    }

    part def PaymentsDeployment specializes KubernetesDeployment {
        attribute deploymentName = "payments";
        attribute replicas = "2";
        attribute image = "ghcr.io/example/payments:1.0.0";
    }

    allocate OrderService to BackendCluster;
    allocate PaymentService to BackendCluster;
    allocate EventBus to BackendCluster;
    allocate OrdersDeployment to BackendCluster;
    allocate PaymentsDeployment to BackendCluster;
}
