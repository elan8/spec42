package OrdersKafkaExample {
    import KafkaDomain::*;

    part def OrdersTopic specializes KafkaTopic {
        attribute channelName = "orders";
        attribute retention = "14d";
        attribute partitions = "12";
    }

    part def BillingGroup specializes KafkaConsumerGroup {
        attribute groupName = "billing-service";
    }
}
