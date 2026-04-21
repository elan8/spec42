package MessagingBasicsExample {
    import MessagingDomain::*;

    part def OrdersEvents :> EventStream {
        attribute channelName = "orders-events";
        attribute retention = "7d";
    }

    part def OrdersPublisher :> Publisher {
        attribute publisherName = "orders";
    }

    part def InventoryConsumer :> Consumer {
        attribute consumerName = "inventory";
    }

    part def OrdersBinding :> MessageBinding {
        attribute bindingName = "orders-to-inventory";
        attribute deliveryGuarantee = "at-least-once";
    }
}
