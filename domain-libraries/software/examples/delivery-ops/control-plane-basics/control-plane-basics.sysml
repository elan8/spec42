package ControlPlaneBasicsExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import SoftwareControlPlane::*;

    port def ControlPlanePort;

    part def OrdersService :> Service {
        attribute :>> serviceName = "orders";
        port configIn : ControlPlanePort;
        port dependencyOut : ControlPlanePort;
    }

    part def OrdersWorkload :> Workload {
        attribute :>> workloadName = "orders-runtime";
    }

    part def OrdersConfig :> ConfigurationSource {
        attribute :>> sourceName = "orders-config";
        attribute :>> sourceUri = "vault://team/orders/config";
        port configOut : ControlPlanePort;
    }

    part def OrdersSecret :> SecretItem {
        attribute :>> key = "DB_PASSWORD";
        port secretOut : ControlPlanePort;
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
        port endpointIn : ControlPlanePort;
    }

    part def OrdersControlPlaneSystem {
        part ordersService : OrdersService;
        part ordersWorkload : OrdersWorkload;
        part ordersConfig : OrdersConfig;
        part ordersSecret : OrdersSecret;
        part ordersHealth : OrdersHealth;
        part ordersScaling : OrdersScaling;
        part paymentsApi : PaymentsApi;

        connect ordersConfig.configOut to ordersService.configIn;
        connect ordersSecret.secretOut to ordersService.configIn;
        connect ordersService.dependencyOut to paymentsApi.endpointIn;
    }
}
