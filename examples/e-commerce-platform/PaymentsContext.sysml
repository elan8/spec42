package PaymentsContext {
    import HttpDomain::*;
    import DistributedSystems::*;
    import KafkaDomain::*;
    import SoftwareCore::*;

    part def PaymentsService :> HttpService {
        attribute :>> serviceName = "payments-service";
        port commandIngress;
        port gatewayEgress;
        port eventEgress;
    }

    part def PaymentGateway :> ExternalSystem {
        attribute :>> systemName = "payment-gateway";
        port gatewayIngress;
        port gatewayEgress;
    }

    part def PaymentConsumerGroup :> KafkaConsumerGroup {
        attribute :>> groupName = "payments-workers";
        port consumerIngress;
    }
}
