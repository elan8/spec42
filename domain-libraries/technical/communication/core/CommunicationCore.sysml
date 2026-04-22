package CommunicationCore {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def SessionName;
    attribute def OperationName;

    part def CommunicationService :> Service;

    part def CommunicationEndpoint {
        attribute protocol : ProtocolName;
        attribute path : PathString;
        attribute port : PortNumber;
    }

    part def CommunicationChannel {
        attribute channelName : Name;
    }

    part def CommunicationBinding {
        attribute bindingName : Name;
    }

    part def CommunicationSession {
        attribute sessionName : SessionName;
    }

    part def CommunicationOperation {
        attribute operationName : OperationName;
    }
}
