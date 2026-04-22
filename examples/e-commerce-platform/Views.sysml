package Views {
    import CommercePlatformExample;
    import CommerceBehavior::*;
    import CommerceRequirements::*;

    view structure : GeneralView {
        expose CommercePlatformExample::CommercePlatformSystem;
    }

    view connections : InterconnectionView {
        expose CommercePlatformExample::CommercePlatformSystem;
    }

    view behavior : GeneralView {
        expose CommerceBehavior::OrderLifecycleStateMachine;
        expose CommerceBehavior::CheckoutFlow;
    }

    view requirements : GeneralView {
        expose CommerceRequirements::CheckoutLatencyReq;
        expose CommerceRequirements::CheckoutAvailabilityReq;
        expose CommerceRequirements::PaymentReliabilityReq;
        expose CommerceRequirements::EventDurabilityReq;
        expose CommerceRequirements::DeadLetterHandlingReq;
        expose CommerceRequirements::IdempotentConsumerReq;
        expose CommerceRequirements::SecurityBoundaryReq;
        expose CommerceRequirements::TraceabilityReq;
    }
}
