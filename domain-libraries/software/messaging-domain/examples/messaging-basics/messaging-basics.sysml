package MessagingBasicsExample {
    import MessagingDomain::*;

    part def OrdersEvents specializes EventStream {
        attribute channelName = "orders-events";
        attribute retention = "7d";
    }

    part def OrdersPublisher specializes Publisher {
        attribute publisherName = "orders";
    }

    part def InventoryConsumer specializes Consumer {
        attribute consumerName = "inventory";
    }

    part def OrdersBinding specializes MessageBinding {
        attribute bindingName = "orders-to-inventory";
        attribute deliveryGuarantee = "at-least-once";
    }
}
