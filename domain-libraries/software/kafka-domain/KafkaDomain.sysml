package KafkaDomain {
    import SoftwareCore::*;
    import MessagingDomain::*;
    import SerializationDomain::*;

    attribute def PartitionCount;

    part def KafkaTopic specializes EventStream {
        attribute partitions : PartitionCount;
    }

    part def KafkaConsumerGroup {
        attribute groupName : Name;
    }

    part def KafkaBinding specializes MessageBinding;
}
