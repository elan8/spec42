package FulfillmentContext {
    import HttpDomain::*;
    import KafkaDomain::*;
    import DistributedSystems::*;

    part def InventoryService :> HttpService {
        attribute :>> serviceName = "inventory-service";
        port reservationIngress;
        port reservationEventEgress;
    }

    part def FulfillmentService :> HttpService {
        attribute :>> serviceName = "fulfillment-service";
        port orderEventIngress;
        port shippingEventEgress;
    }

    part def NotificationService :> HttpService {
        attribute :>> serviceName = "notification-service";
        port orderEventIngress;
    }

    part def FulfillmentConsumerGroup :> KafkaConsumerGroup {
        attribute :>> groupName = "fulfillment-workers";
        port consumerIngress;
    }
}
