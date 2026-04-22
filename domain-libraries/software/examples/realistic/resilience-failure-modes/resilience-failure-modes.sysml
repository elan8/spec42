package ResilienceFailureModesExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import MessagingDomain::*;
    import KafkaDomain::*;
    import SqlDomain::*;
    import SoftwareControlPlane::*;
    import ObservabilityDomain::*;
    import KubernetesDomain::*;

    port def ResiliencePort;

    part def CheckoutService :> Service {
        attribute :>> serviceName = "checkout-service";
        port requestIn : ResiliencePort;
        port paymentCommandOut : ResiliencePort;
        port orderEventOut : ResiliencePort;
        port persistenceOut : ResiliencePort;
    }

    part def PaymentService :> Service {
        attribute :>> serviceName = "payment-service";
        port commandIn : ResiliencePort;
        port confirmationOut : ResiliencePort;
    }

    part def OrdersTopic :> KafkaTopic {
        attribute :>> channelName = "orders-events";
        attribute :>> retention = "14d";
        attribute :>> partitions = "10";
        port topicIngress : ResiliencePort;
        port topicEgress : ResiliencePort;
    }

    part def PaymentsDlq :> Queue {
        attribute :>> queueName = "payments-dlq";
        port dlqIngress : ResiliencePort;
        port dlqEgress : ResiliencePort;
    }

    part def ReplayQueue :> Queue {
        attribute :>> queueName = "payments-replay";
        port replayIngress : ResiliencePort;
        port replayEgress : ResiliencePort;
    }

    part def PaymentWorkers :> KafkaConsumerGroup {
        attribute :>> groupName = "payment-workers";
        port consumerIngress : ResiliencePort;
    }

    part def ExactlyOnceBinding :> KafkaBinding {
        attribute :>> bindingName = "orders-payment-binding";
        attribute :>> deliveryGuarantee = "at-least-once-idempotent-consumer";
    }

    part def RetryPolicy :> RuntimeDependency {
        attribute :>> dependencyName = "retry-policy-exponential-backoff";
        port retryIn : ResiliencePort;
        port retryOut : ResiliencePort;
    }

    part def TimeoutPolicy :> RuntimeDependency {
        attribute :>> dependencyName = "timeout-policy-800ms";
        port timeoutIn : ResiliencePort;
        port timeoutOut : ResiliencePort;
    }

    part def CircuitPolicy :> PolicyBoundary {
        attribute :>> boundaryName = "payment-circuit-breaker";
        port policyIn : ResiliencePort;
        port policyOut : ResiliencePort;
    }

    part def OrdersDatabase :> SqlDatabase {
        attribute :>> storeName = "orders-primary";
        attribute :>> engineName = "postgres15";
        attribute :>> consistencyMode = "strong";
        port dbIngress : ResiliencePort;
    }

    part def OrdersReadReplica :> ReadReplica {
        attribute :>> storeName = "orders-read-replica";
        attribute :>> consistencyMode = "eventual";
        port replicaIngress : ResiliencePort;
    }

    part def RecoveryWorker :> Workload {
        attribute :>> workloadName = "consistency-repair-worker";
        port replayIn : ResiliencePort;
        port repairOut : ResiliencePort;
    }

    part def FailureRateSli :> ServiceLevelIndicator {
        attribute :>> sliName = "payment-failure-rate";
        attribute :>> signalKind = "error-rate";
        port indicatorIn : ResiliencePort;
        port indicatorOut : ResiliencePort;
    }

    part def PaymentSuccessSlo :> ServiceLevelObjective {
        attribute :>> sloName = "payment-success-99_5";
        port objectiveIn : ResiliencePort;
    }

    part def ResilienceAlert :> AlertPolicy {
        attribute :>> policyName = "payment-failure-burst";
        attribute :>> severity = "critical";
        port alertIn : ResiliencePort;
    }

    part def RuntimeCluster :> KubernetesCluster {
        attribute :>> clusterName = "checkout-runtime";
        attribute :>> nodeName = "runtime-node-a";
        attribute :>> host = "runtime-node-a.internal";
    }

    part def CheckoutDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "checkout-service";
        attribute :>> replicas = "6";
        attribute :>> image = "ghcr.io/example/checkout-service:9.0.1";
    }

    part def PaymentsDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "payment-service";
        attribute :>> replicas = "6";
        attribute :>> image = "ghcr.io/example/payment-service:9.0.1";
    }

    part def ResilienceSystem {
        part checkoutService : CheckoutService;
        part paymentService : PaymentService;
        part ordersTopic : OrdersTopic;
        part paymentsDlq : PaymentsDlq;
        part replayQueue : ReplayQueue;
        part paymentWorkers : PaymentWorkers;
        part retryPolicy : RetryPolicy;
        part timeoutPolicy : TimeoutPolicy;
        part circuitPolicy : CircuitPolicy;
        part ordersDatabase : OrdersDatabase;
        part ordersReadReplica : OrdersReadReplica;
        part recoveryWorker : RecoveryWorker;
        part failureRateSli : FailureRateSli;
        part paymentSuccessSlo : PaymentSuccessSlo;
        part resilienceAlert : ResilienceAlert;

        connect checkoutService.orderEventOut to ordersTopic.topicIngress;
        connect ordersTopic.topicEgress to paymentWorkers.consumerIngress;
        connect paymentWorkers.consumerIngress to paymentService.commandIn;
        connect paymentService.confirmationOut to checkoutService.requestIn;
        connect paymentService.commandIn to timeoutPolicy.timeoutIn;
        connect timeoutPolicy.timeoutOut to retryPolicy.retryIn;
        connect retryPolicy.retryOut to circuitPolicy.policyIn;
        connect circuitPolicy.policyOut to paymentsDlq.dlqIngress;
        connect paymentsDlq.dlqEgress to replayQueue.replayIngress;
        connect replayQueue.replayEgress to recoveryWorker.replayIn;
        connect checkoutService.persistenceOut to ordersDatabase.dbIngress;
        connect recoveryWorker.repairOut to ordersReadReplica.replicaIngress;
        connect failureRateSli.indicatorOut to paymentSuccessSlo.objectiveIn;
        connect failureRateSli.indicatorOut to resilienceAlert.alertIn;
    }

    allocate CheckoutService to RuntimeCluster;
    allocate PaymentService to RuntimeCluster;
    allocate RecoveryWorker to RuntimeCluster;
    allocate CheckoutDeployment to RuntimeCluster;
    allocate PaymentsDeployment to RuntimeCluster;
}
