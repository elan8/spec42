package ControlPlaneBasicsExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SoftwareControlPlane::*;

    part def OrdersService specializes Service {
        attribute serviceName = "orders";
    }

    part def OrdersWorkload specializes Workload {
        attribute workloadName = "orders-runtime";
    }

    part def OrdersConfig specializes ConfigurationSource {
        attribute sourceName = "orders-config";
        attribute sourceUri = "vault://team/orders/config";
    }

    part def OrdersSecret specializes SecretItem {
        attribute key = "DB_PASSWORD";
    }

    part def OrdersHealth specializes HealthSignal {
        attribute signalName = "readiness";
        attribute healthState = "ready";
    }

    part def OrdersScaling specializes ScalingPolicy {
        attribute target = "cpu";
    }

    part def PaymentsApi specializes ExternalEndpoint {
        attribute endpointName = "payments";
        attribute endpointUri = "https://payments.internal";
    }
}
