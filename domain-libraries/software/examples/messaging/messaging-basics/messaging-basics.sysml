package MessagingBasicsExample {
    import MessagingDomain::*;

    port def MessagingPort;

    part def OrdersEvents :> EventStream {
        attribute :>> channelName = "orders-events";
        attribute :>> retention = "7d";
        port streamIngress : MessagingPort;
        port streamEgress : MessagingPort;
    }

    part def OrdersPublisher :> Publisher {
        attribute :>> publisherName = "orders";
        port publishOut : MessagingPort;
    }

    part def InventoryConsumer :> Consumer {
        attribute :>> consumerName = "inventory";
        port consumeIn : MessagingPort;
    }

    part def OrdersBinding :> MessageBinding {
        attribute :>> bindingName = "orders-to-inventory";
        attribute :>> deliveryGuarantee = "at-least-once";
    }

    part def MessagingSystem {
        part ordersPublisher : OrdersPublisher;
        part ordersEvents : OrdersEvents;
        part inventoryConsumer : InventoryConsumer;
        part ordersBinding : OrdersBinding;

        connect ordersPublisher.publishOut to ordersEvents.streamIngress;
        connect ordersEvents.streamEgress to inventoryConsumer.consumeIn;
    }
}
