package OrdersKafkaExample {
    import KafkaDomain::*;

    port def KafkaPort;

    part def OrdersTopic :> KafkaTopic {
        attribute :>> channelName = "orders";
        attribute :>> retention = "14d";
        attribute :>> partitions = "12";
        port topicIngress : KafkaPort;
        port topicEgress : KafkaPort;
    }

    part def BillingGroup :> KafkaConsumerGroup {
        attribute :>> groupName = "billing-service";
        port consumerIngress : KafkaPort;
    }

    part def KafkaMessagingSystem {
        part ordersTopic : OrdersTopic;
        part billingGroup : BillingGroup;

        connect ordersTopic.topicEgress to billingGroup.consumerIngress;
    }
}
