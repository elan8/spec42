package HttpDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def HttpEncryptionMode;

    part def HttpEndpoint {
        attribute protocol : ProtocolName;
        attribute path : PathString;
        attribute port : PortNumber;
        attribute encryption : HttpEncryptionMode;
    }

    part def RestApiEndpoint :> HttpEndpoint;
    part def HttpRoute :> HttpEndpoint;
    part def HttpService :> Service;
}
