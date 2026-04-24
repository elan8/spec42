package WebShopExample {
    import WebShopArchitecture::*;
    import WebShopRequirements::*;
    import WebShopBehavior::*;
    import KubernetesDomain::*;

    part def CommerceCluster :> KubernetesCluster {
        attribute :>> clusterName = "commerce-prod";
        attribute :>> nodeName = "k8s-node-a";
        attribute :>> host = "k8s-node-a.internal";
    }

    part webshopSystem : WebShopSystem {
        exhibit state orderLifecycle : OrderLifecycleStateMachine;
        perform action checkoutPipeline : CheckoutPipeline;
    }
    part commerceCluster : CommerceCluster;

    requirement checkoutLatency : CheckoutLatencyReq;
    requirement checkoutAvailability : CheckoutAvailabilityReq;
    requirement paymentReliability : PaymentReliabilityReq;
    requirement eventDurability : EventDurabilityReq;
    requirement securityBoundary : SecurityBoundaryReq;

    satisfy checkoutLatency by webshopSystem.checkoutService;
    satisfy checkoutAvailability by webshopSystem.apiGateway;
    satisfy paymentReliability by webshopSystem.paymentsService;
    satisfy eventDurability by webshopSystem.ordersEventsTopic;
    satisfy securityBoundary by webshopSystem.apiGateway;

    allocate CheckoutService to CommerceCluster;
}
