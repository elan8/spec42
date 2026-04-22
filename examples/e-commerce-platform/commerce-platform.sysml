package CommercePlatformExample {
    import StorefrontContext::*;
    import CheckoutContext::*;
    import PaymentsContext::*;
    import FulfillmentContext::*;
    import PlatformContext::*;
    import CommerceBehavior::*;
    import CommerceRequirements::*;

    part def CommercePlatformSystem {
        part storefrontWebApp : StorefrontWebApp;
        part mobileApp : MobileApp;
        part publicEdge : PublicEdge;
        part bffService : BffService;
        part publicOrdersApi : PublicOrdersApi;

        part ordersService : OrdersService;
        part pricingService : PricingService;
        part priceOrder : PriceOrder;

        part paymentsService : PaymentsService;
        part paymentGateway : PaymentGateway;

        part inventoryService : InventoryService;
        part fulfillmentService : FulfillmentService;
        part notificationService : NotificationService;

        part kafkaCluster : KafkaCluster;
        part ordersEventsTopic : OrdersEventsTopic;
        part ordersRetryTopic : OrdersRetryTopic;
        part ordersDlqTopic : OrdersDlqTopic;
        part ordersKafkaBinding : OrdersKafkaBinding;
        part paymentConsumers : PaymentConsumerGroup;
        part fulfillmentConsumers : FulfillmentConsumerGroup;

        part ordersDatabase : OrdersDatabase;
        part ordersCache : OrdersCache;
        part ordersConfig : OrdersConfig;
        part ordersSigningKey : OrdersSigningKey;

        part ordersIdentity : OrdersIdentity;
        part publicIngress : PublicIngress;

        part ordersMetrics : OrdersMetrics;
        part ordersLogs : OrdersLogs;
        part ordersTraces : OrdersTraces;
        part checkoutLatency : CheckoutLatency;
        part checkoutAvailability : CheckoutAvailability;
        part checkoutAlertPolicy : CheckoutAlertPolicy;
        part checkoutDashboard : CheckoutDashboard;

        part commerceRepo : CommerceRepo;
        part commercePipeline : CommercePipeline;
        part commerceRelease : CommerceRelease;

        part commerceCluster : CommerceCluster;
        part checkoutNamespace : CheckoutNamespace;
        part ordersDeployment : OrdersDeployment;
        part pricingDeployment : PricingDeployment;
        part paymentsDeployment : PaymentsDeployment;
        part fulfillmentDeployment : FulfillmentDeployment;
        part primaryRegion : PrimaryRegion;

        exhibit state orderLifecycle : OrderLifecycleStateMachine;

        connect storefrontWebApp.apiEgress to publicEdge.edgeIngress;
        connect mobileApp.apiEgress to publicEdge.edgeIngress;
        connect publicEdge.edgeEgress to bffService.webIngress;
        connect publicEdge.edgeEgress to bffService.mobileIngress;
        connect bffService.checkoutApiEgress to publicOrdersApi.ingress;
        connect publicOrdersApi.upstream to ordersService.apiIngress;

        connect ordersService.pricingRpcOut to pricingService.grpcIngress;
        connect pricingService.grpcIngress to priceOrder.rpcIn;
        connect priceOrder.rpcOut to ordersService.pricingRpcIn;

        connect ordersService.paymentCommandOut to paymentsService.commandIngress;
        connect paymentsService.gatewayEgress to paymentGateway.gatewayIngress;

        connect ordersService.eventEgress to ordersEventsTopic.topicIngress;
        connect paymentsService.eventEgress to ordersEventsTopic.topicIngress;
        connect inventoryService.reservationEventEgress to ordersEventsTopic.topicIngress;

        connect ordersEventsTopic.topicEgress to paymentConsumers.consumerIngress;
        connect ordersEventsTopic.topicEgress to fulfillmentConsumers.consumerIngress;
        connect ordersEventsTopic.topicEgress to fulfillmentService.orderEventIngress;
        connect ordersEventsTopic.topicEgress to notificationService.orderEventIngress;

        connect fulfillmentService.shippingEventEgress to ordersRetryTopic.topicIngress;
        connect ordersRetryTopic.topicEgress to ordersDlqTopic.topicIngress;

        connect ordersService.sqlEgress to ordersDatabase.dbIngress;
        connect ordersService.cacheEgress to ordersCache.cacheIngress;
        connect ordersConfig.configOut to ordersService.configIn;
        connect ordersSigningKey.secretOut to ordersService.secretIn;

        connect ordersMetrics.metricOut to checkoutLatency.indicatorIn;
        connect checkoutLatency.indicatorOut to checkoutAvailability.objectiveIn;

        connect commerceRepo.repositoryOut to commercePipeline.pipelineIn;
        connect commercePipeline.pipelineOut to commerceRelease.releaseIn;
    }

    requirement checkoutLatency : CheckoutLatencyReq;
    requirement checkoutAvailability : CheckoutAvailabilityReq;
    requirement paymentReliability : PaymentReliabilityReq;
    requirement eventDurability : EventDurabilityReq;
    requirement deadLetterHandling : DeadLetterHandlingReq;
    requirement idempotentConsumer : IdempotentConsumerReq;
    requirement securityBoundary : SecurityBoundaryReq;
    requirement traceability : TraceabilityReq;

    part commercePlatform : CommercePlatformSystem;
    satisfy checkoutLatency by commercePlatform.ordersService;
    satisfy checkoutAvailability by commercePlatform.ordersService;
    satisfy paymentReliability by commercePlatform.paymentsService;
    satisfy eventDurability by commercePlatform.ordersEventsTopic;
    satisfy deadLetterHandling by commercePlatform.ordersDlqTopic;
    satisfy idempotentConsumer by commercePlatform.paymentConsumers;
    satisfy securityBoundary by commercePlatform.publicIngress;
    satisfy traceability by commercePlatform.ordersTraces;

    allocate OrdersService to CommerceCluster;
    allocate PricingService to CommerceCluster;
    allocate PaymentsService to CommerceCluster;
    allocate FulfillmentService to CommerceCluster;
    allocate OrdersDeployment to CommerceCluster;
    allocate PricingDeployment to CommerceCluster;
    allocate PaymentsDeployment to CommerceCluster;
    allocate FulfillmentDeployment to CommerceCluster;
}
