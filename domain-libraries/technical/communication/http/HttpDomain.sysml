package HttpDomain {
    import CommunicationCore::*;

    attribute def HttpEncryptionMode;

    part def HttpEndpoint :> CommunicationEndpoint {
        attribute encryption : HttpEncryptionMode;
    }

    part def RestApiEndpoint :> HttpEndpoint;
    part def HttpRoute :> HttpEndpoint;
    part def HttpService :> CommunicationService;
}
