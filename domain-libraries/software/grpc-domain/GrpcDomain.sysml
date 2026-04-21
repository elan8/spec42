package GrpcDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SerializationDomain::*;

    attribute def MethodName;

    part def GrpcService specializes Service;

    part def GrpcMethod {
        attribute methodName : MethodName;
    }

    part def UnaryRpc specializes GrpcMethod;
    part def ServerStreamingRpc specializes GrpcMethod;
    part def ClientStreamingRpc specializes GrpcMethod;
    part def BidirectionalStreamingRpc specializes GrpcMethod;

    item def GrpcRequest specializes ProtobufMessage;
    item def GrpcResponse specializes ProtobufMessage;
}
