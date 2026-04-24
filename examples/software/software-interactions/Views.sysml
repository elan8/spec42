package Views {
    import WebShopArchitecture::*;

    view structure : GeneralView {
        expose WebShopArchitecture::SmallWebShopPlatform;
    }

    view connections : InterconnectionView {
        expose WebShopArchitecture::SmallWebShopPlatform;
    }

    view checkoutFlow : SequenceView {
        expose WebShopArchitecture::CheckoutFlow;
    }

    view orderEventFanout : SequenceView {
        expose WebShopArchitecture::OrderEventFanout;
    }
}
