package WebshopExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import HttpDomain::*;
    import KubernetesDomain::*;

    port def WebshopPort;

    interface def OrdersApi :> ApiInterface;
    item def OrderCreated :> Event;

    part def OrderService :> HttpService {
        attribute :>> serviceName = "orders";
        port apiIngress : WebshopPort;
        port eventEgress : WebshopPort;
    }

    part def PaymentService :> HttpService {
        attribute :>> serviceName = "payments";
        port commandIngress : WebshopPort;
    }

    part def EventBus :> MessageBroker {
        port ingress : WebshopPort;
        port egress : WebshopPort;
    }

    part def Orders :> Topic {
        attribute :>> topicName = "orders.created";
    }

    part def PublicOrdersApi :> RestApiEndpoint {
        attribute :>> protocol = "https";
        attribute :>> path = "/orders";
        attribute :>> port = "443";
        attribute :>> encryption = tls;
        port publicIngress : WebshopPort;
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

    part def WebshopSystem {
        part ordersService : OrderService;
        part paymentService : PaymentService;
        part eventBus : EventBus;
        part publicApi : PublicOrdersApi;
        part backendCluster : BackendCluster;
        part ordersDeployment : OrdersDeployment;
        part paymentsDeployment : PaymentsDeployment;

        connect publicApi.publicIngress to ordersService.apiIngress;
        connect ordersService.eventEgress to eventBus.ingress;
        connect eventBus.egress to paymentService.commandIngress;
    }

    allocate OrderService to BackendCluster;
    allocate PaymentService to BackendCluster;
    allocate EventBus to BackendCluster;
    allocate OrdersDeployment to BackendCluster;
    allocate PaymentsDeployment to BackendCluster;
}
