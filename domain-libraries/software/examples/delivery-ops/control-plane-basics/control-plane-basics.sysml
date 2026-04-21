package ControlPlaneBasicsExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SoftwareControlPlane::*;

    part def OrdersService :> Service {
        attribute :>> serviceName = "orders";
    }

    part def OrdersWorkload :> Workload {
        attribute :>> workloadName = "orders-runtime";
    }

    part def OrdersConfig :> ConfigurationSource {
        attribute :>> sourceName = "orders-config";
        attribute :>> sourceUri = "vault://team/orders/config";
    }

    part def OrdersSecret :> SecretItem {
        attribute :>> key = "DB_PASSWORD";
    }

    part def OrdersHealth :> HealthSignal {
        attribute :>> signalName = "readiness";
        attribute :>> healthState = "ready";
    }

    part def OrdersScaling :> ScalingPolicy {
        attribute :>> target = "cpu";
    }

    part def PaymentsApi :> ExternalEndpoint {
        attribute :>> endpointName = "payments";
        attribute :>> endpointUri = "https://payments.internal";
    }
}
