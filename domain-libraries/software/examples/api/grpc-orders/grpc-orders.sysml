package GrpcOrdersExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SerializationDomain::*;
    import GrpcDomain::*;
    import KubernetesDomain::*;

    port def GrpcPort;

    item def GetOrderRequest :> GrpcRequest {
        attribute :>> mediaType = "application/protobuf";
        attribute :>> schema = "orders.v1.GetOrderRequest";
        attribute :>> packageName = "orders.v1";
    }

    item def GetOrderResponse :> GrpcResponse {
        attribute :>> mediaType = "application/protobuf";
        attribute :>> schema = "orders.v1.GetOrderResponse";
        attribute :>> packageName = "orders.v1";
    }

    part def OrdersGrpc :> GrpcService {
        attribute :>> serviceName = "orders-grpc";
        port requestIn : GrpcPort;
        port responseOut : GrpcPort;
    }

    part def GetOrder :> UnaryRpc {
        attribute :>> methodName = "GetOrder";
        port rpcIn : GrpcPort;
        port rpcOut : GrpcPort;
    }

    part def OrdersCluster :> KubernetesCluster {
        attribute :>> clusterName = "orders-cluster";
        attribute :>> nodeName = "orders-node-a";
        attribute :>> host = "orders-node-a.internal";
    }

    part def OrdersDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "orders-grpc";
        attribute :>> replicas = "2";
        attribute :>> image = "ghcr.io/example/orders-grpc:1.0.0";
    }

    part def GrpcOrdersSystem {
        part ordersGrpc : OrdersGrpc;
        part getOrder : GetOrder;
        part ordersCluster : OrdersCluster;
        part ordersDeployment : OrdersDeployment;

        connect ordersGrpc.requestIn to getOrder.rpcIn;
        connect getOrder.rpcOut to ordersGrpc.responseOut;
    }

    allocate OrdersGrpc to OrdersCluster;
    allocate OrdersDeployment to OrdersCluster;
}
