package GrpcDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SerializationDomain::*;

    attribute def MethodName;

    part def GrpcService :> Service;

    part def GrpcMethod {
        attribute methodName : MethodName;
    }

    part def UnaryRpc :> GrpcMethod;
    part def ServerStreamingRpc :> GrpcMethod;
    part def ClientStreamingRpc :> GrpcMethod;
    part def BidirectionalStreamingRpc :> GrpcMethod;

    item def GrpcRequest :> ProtobufMessage;
    item def GrpcResponse :> ProtobufMessage;
}
