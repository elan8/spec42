package GrpcOrdersExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SerializationDomain::*;
    import GrpcDomain::*;
    import KubernetesDomain::*;

    item def GetOrderRequest :> GrpcRequest {
        attribute mediaType = "application/protobuf";
        attribute schema = "orders.v1.GetOrderRequest";
        attribute packageName = "orders.v1";
    }

    item def GetOrderResponse :> GrpcResponse {
        attribute mediaType = "application/protobuf";
        attribute schema = "orders.v1.GetOrderResponse";
        attribute packageName = "orders.v1";
    }

    part def OrdersGrpc :> GrpcService {
        attribute serviceName = "orders-grpc";
    }

    part def GetOrder :> UnaryRpc {
        attribute methodName = "GetOrder";
    }

    part def OrdersCluster :> KubernetesCluster {
        attribute clusterName = "orders-cluster";
        attribute nodeName = "orders-node-a";
        attribute host = "orders-node-a.internal";
    }

    part def OrdersDeployment :> KubernetesDeployment {
        attribute deploymentName = "orders-grpc";
        attribute replicas = "2";
        attribute image = "ghcr.io/example/orders-grpc:1.0.0";
    }

    allocate OrdersGrpc to OrdersCluster;
    allocate OrdersDeployment to OrdersCluster;
}
