package OrdersKafkaExample {
    import KafkaDomain::*;

    part def OrdersTopic :> KafkaTopic {
        attribute channelName = "orders";
        attribute retention = "14d";
        attribute partitions = "12";
    }

    part def BillingGroup :> KafkaConsumerGroup {
        attribute groupName = "billing-service";
    }
}
