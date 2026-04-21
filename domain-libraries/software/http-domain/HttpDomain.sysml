package HttpDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    part def HttpEndpoint {
        attribute protocol : ProtocolName;
        attribute path : PathString;
        attribute port : PortNumber;
    }

    part def RestApiEndpoint :> HttpEndpoint;
    part def HttpRoute :> HttpEndpoint;
    part def HttpService :> Service;
}
