package TransportDomain {
    import CommunicationCore::*;

    attribute def ReliabilityMode;

    part def TransportLink :> CommunicationChannel {
        attribute reliability : ReliabilityMode;
    }

    part def TcpTransport :> TransportLink;
    part def UdpTransport :> TransportLink;
}
