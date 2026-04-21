package HttpDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    part def HttpEndpoint {
        attribute protocol : ProtocolName;
        attribute path : PathString;
        attribute port : PortNumber;
    }

    part def RestApiEndpoint specializes HttpEndpoint;
    part def HttpRoute specializes HttpEndpoint;
    part def HttpService specializes Service;
}
