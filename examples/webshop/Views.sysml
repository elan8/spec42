package Views {
    import WebShopArchitecture::*;
    import WebShopBehavior::*;
    import WebShopRequirements::*;
    import WebShopExample::*;

    view structure : GeneralView {
        expose WebShopExample::webshopSystem;
    }

    view connections : InterconnectionView {
        expose WebShopExample::webshopSystem;
    }

    view checkoutFlow : SequenceView {
        expose WebShopArchitecture::CheckoutFlow;
    }

    view orderEventFanout : SequenceView {
        expose WebShopArchitecture::OrderEventFanout;
    }

    view orderLifecycle : StateTransitionView {
        expose WebShopBehavior::OrderLifecycleStateMachine;
    }

    view checkoutPipeline : ActionFlowView {
        expose WebShopBehavior::CheckoutPipeline;
    }

    view requirements : GeneralView {
        expose WebShopRequirements::CheckoutLatencyReq;
        expose WebShopRequirements::CheckoutAvailabilityReq;
        expose WebShopRequirements::PaymentReliabilityReq;
        expose WebShopRequirements::EventDurabilityReq;
        expose WebShopRequirements::SecurityBoundaryReq;
    }
}
