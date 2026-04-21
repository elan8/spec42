package GrpcOrdersExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SerializationDomain::*;
    import GrpcDomain::*;
    import KubernetesDomain::*;

    item def GetOrderRequest specializes GrpcRequest {
        attribute mediaType = "application/protobuf";
        attribute schema = "orders.v1.GetOrderRequest";
        attribute packageName = "orders.v1";
    }

    item def GetOrderResponse specializes GrpcResponse {
        attribute mediaType = "application/protobuf";
        attribute schema = "orders.v1.GetOrderResponse";
        attribute packageName = "orders.v1";
    }

    part def OrdersGrpc specializes GrpcService {
        attribute serviceName = "orders-grpc";
    }

    part def GetOrder specializes UnaryRpc {
        attribute methodName = "GetOrder";
    }

    part def OrdersCluster specializes KubernetesCluster {
        attribute clusterName = "orders-cluster";
        attribute nodeName = "orders-node-a";
        attribute host = "orders-node-a.internal";
    }

    part def OrdersDeployment specializes KubernetesDeployment {
        attribute deploymentName = "orders-grpc";
        attribute replicas = "2";
        attribute image = "ghcr.io/example/orders-grpc:1.0.0";
    }

    allocate OrdersGrpc to OrdersCluster;
    allocate OrdersDeployment to OrdersCluster;
}
