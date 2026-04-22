package MessagingDomain {
    import CommunicationCore::*;
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SerializationDomain::*;

    attribute def DeliveryGuarantee;
    attribute def RetentionPolicy;

    part def MessageChannel :> CommunicationChannel;

    part def EventStream :> MessageChannel {
        attribute retention : RetentionPolicy;
    }

    part def Queue {
        attribute queueName : Name;
    }

    part def Publisher {
        attribute publisherName : Name;
    }

    part def Consumer {
        attribute consumerName : Name;
    }

    part def MessageBinding :> CommunicationBinding {
        attribute deliveryGuarantee : DeliveryGuarantee;
    }

    item def StreamEvent :> Event;
}
