package WebShopArchitecture {
    import SoftwareCore::*;
    import HttpDomain::*;
    import SqlDomain::*;
    import KafkaDomain::*;
    import DistributedSystems::*;
    private import SoftwareInteractions::*;

    part def Storefront :> SoftwareModule {
        attribute :>> moduleName = "web-storefront";
        port checkoutApiOut;
        port catalogApiOut;
    }

    part def ApiGateway :> HttpService {
        attribute :>> serviceName = "edge-gateway";
        port publicCheckoutIn;
        port publicCatalogIn;
        port checkoutApiOut;
        port catalogApiOut;
    }

    part def CatalogService :> HttpService {
        attribute :>> serviceName = "catalog-service";
        port apiIn;
        port catalogDbOut;
    }

    part def CheckoutService :> HttpService {
        attribute :>> serviceName = "checkout-service";
        port apiIn;
        port ordersDbOut;
        port paymentsApiOut;
        port inventoryApiOut;
        port orderEventsOut;
    }

    part def PaymentsService :> HttpService {
        attribute :>> serviceName = "payments-service";
        port apiIn;
        port paymentsDbOut;
        port providerOut;
    }

    part def InventoryService :> HttpService {
        attribute :>> serviceName = "inventory-service";
        port apiIn;
        port inventoryDbOut;
        port orderEventsIn;
        port reservationEventsOut;
    }

    part def NotificationService :> HttpService {
        attribute :>> serviceName = "notification-service";
        port orderEventsIn;
        port emailOut;
    }

    part def OrdersEventsTopic :> KafkaTopic {
        attribute :>> channelName = "commerce.orders.events.v1";
        attribute :>> retention = "7d";
        attribute :>> partitions = "8";
        port topicIngress;
        port topicEgress;
    }

    part def OrdersDatabase :> SqlDatabase {
        attribute :>> storeName = "orders-postgres";
        attribute :>> engineName = "postgres15";
        port dbIngress;
    }

    part def CatalogDatabase :> SqlDatabase {
        attribute :>> storeName = "catalog-postgres";
        attribute :>> engineName = "postgres15";
        port dbIngress;
    }

    part def PaymentsDatabase :> SqlDatabase {
        attribute :>> storeName = "payments-postgres";
        attribute :>> engineName = "postgres15";
        port dbIngress;
    }

    part def InventoryDatabase :> SqlDatabase {
        attribute :>> storeName = "inventory-postgres";
        attribute :>> engineName = "postgres15";
        port dbIngress;
    }

    part def PaymentProvider :> ExternalSystem {
        attribute :>> systemName = "stripe";
        port providerIngress;
    }

    part def EmailProvider :> ExternalSystem {
        attribute :>> systemName = "customer-email";
        port emailIngress;
    }

    part def WebShopSystem {
        part storefront : Storefront;
        part apiGateway : ApiGateway;
        part catalogService : CatalogService;
        part checkoutService : CheckoutService;
        part paymentsService : PaymentsService;
        part inventoryService : InventoryService;
        part notificationService : NotificationService;

        part ordersEventsTopic : OrdersEventsTopic;
        part ordersDb : OrdersDatabase;
        part catalogDb : CatalogDatabase;
        part paymentsDb : PaymentsDatabase;
        part inventoryDb : InventoryDatabase;
        part paymentProvider : PaymentProvider;
        part emailProvider : EmailProvider;

        connect storefront.checkoutApiOut to apiGateway.publicCheckoutIn;
        connect storefront.catalogApiOut to apiGateway.publicCatalogIn;

        connect apiGateway.checkoutApiOut to checkoutService.apiIn;
        connect apiGateway.catalogApiOut to catalogService.apiIn;

        connect catalogService.catalogDbOut to catalogDb.dbIngress;

        connect checkoutService.ordersDbOut to ordersDb.dbIngress;
        connect checkoutService.paymentsApiOut to paymentsService.apiIn;
        connect checkoutService.inventoryApiOut to inventoryService.apiIn;
        connect checkoutService.orderEventsOut to ordersEventsTopic.topicIngress;

        connect paymentsService.paymentsDbOut to paymentsDb.dbIngress;
        connect paymentsService.providerOut to paymentProvider.providerIngress;

        connect inventoryService.inventoryDbOut to inventoryDb.dbIngress;
        connect ordersEventsTopic.topicEgress to inventoryService.orderEventsIn;
        connect ordersEventsTopic.topicEgress to notificationService.orderEventsIn;
        connect inventoryService.reservationEventsOut to ordersEventsTopic.topicIngress;

        connect notificationService.emailOut to emailProvider.emailIngress;
    }

    part def CheckoutFlow :> InteractionScenario {
        part storefront : Lifeline;
        part apiGateway : Lifeline;
        part checkoutService : Lifeline;
        part paymentsService : Lifeline;
        part inventoryService : Lifeline;
        part ordersEventsTopic : Lifeline;
        part notificationService : Lifeline;

        part submitCheckout : SynchronousCall {
            ref from : Lifeline = storefront;
            ref to : Lifeline = apiGateway;
            attribute :>> label = "POST /checkout";
        }

        part forwardCheckout : SynchronousCall {
            ref from : Lifeline = apiGateway;
            ref to : Lifeline = checkoutService;
            attribute :>> label = "forward checkout request";
        }

        part authorizePayment : SynchronousCall {
            ref from : Lifeline = checkoutService;
            ref to : Lifeline = paymentsService;
            attribute :>> label = "authorize payment";
        }

        part reserveInventory : SynchronousCall {
            ref from : Lifeline = checkoutService;
            ref to : Lifeline = inventoryService;
            attribute :>> label = "reserve stock";
        }

        part checkoutExecution : Activation {
            ref on : Lifeline = checkoutService;
            ref startMessage : Message = forwardCheckout;
            ref finishMessage : Message = checkoutAccepted;
        }

        part paymentOutcome : AltFragment {
            part approved : InteractionOperand {
                attribute :>> guard = "payment approved";

                part paymentApproved : ReturnMessage {
                    ref from : Lifeline = paymentsService;
                    ref to : Lifeline = checkoutService;
                    attribute :>> label = "approved";
                }

                part inventoryReserved : ReturnMessage {
                    ref from : Lifeline = inventoryService;
                    ref to : Lifeline = checkoutService;
                    attribute :>> label = "stock reserved";
                }

                part publishOrderCreated : AsynchronousMessage {
                    ref from : Lifeline = checkoutService;
                    ref to : Lifeline = ordersEventsTopic;
                    attribute :>> label = "publish order-created";
                }

                part checkoutAccepted : ReturnMessage {
                    ref from : Lifeline = checkoutService;
                    ref to : Lifeline = apiGateway;
                    attribute :>> label = "201 Created";
                }

                part apiResponse : ReturnMessage {
                    ref from : Lifeline = apiGateway;
                    ref to : Lifeline = storefront;
                    attribute :>> label = "checkout accepted";
                }
            }

            part declined : InteractionOperand {
                attribute :>> guard = "payment declined";

                part paymentDeclined : ReturnMessage {
                    ref from : Lifeline = paymentsService;
                    ref to : Lifeline = checkoutService;
                    attribute :>> label = "declined";
                }

                part checkoutRejected : ReturnMessage {
                    ref from : Lifeline = checkoutService;
                    ref to : Lifeline = apiGateway;
                    attribute :>> label = "402 Payment Required";
                }

                part rejectionResponse : ReturnMessage {
                    ref from : Lifeline = apiGateway;
                    ref to : Lifeline = storefront;
                    attribute :>> label = "payment failed";
                }
            }
        }
    }

    part def OrderEventFanout :> InteractionScenario {
        part checkoutService : Lifeline;
        part ordersEventsTopic : Lifeline;
        part inventoryService : Lifeline;
        part notificationService : Lifeline;
        part orderProjection : Lifeline;

        part publishOrderCreated : AsynchronousMessage {
            ref from : Lifeline = checkoutService;
            ref to : Lifeline = ordersEventsTopic;
            attribute :>> label = "order-created";
        }

        part createProjection : CreationMessage {
            ref from : Lifeline = ordersEventsTopic;
            ref to : Lifeline = orderProjection;
            attribute :>> label = "create order projection";
        }

        part fanOutInventory : AsynchronousMessage {
            ref from : Lifeline = ordersEventsTopic;
            ref to : Lifeline = inventoryService;
            attribute :>> label = "deliver order-created";
        }

        part fanOutNotifications : AsynchronousMessage {
            ref from : Lifeline = ordersEventsTopic;
            ref to : Lifeline = notificationService;
            attribute :>> label = "deliver order-created";
        }

        part inventoryProcessing : Activation {
            ref on : Lifeline = inventoryService;
            ref startMessage : Message = fanOutInventory;
            ref finishMessage : Message = reservationPublished;
        }

        part notifyCustomer : AsynchronousMessage {
            ref from : Lifeline = notificationService;
            ref to : Lifeline = ordersEventsTopic;
            attribute :>> label = "confirmation-requested";
        }

        part reservationPublished : AsynchronousMessage {
            ref from : Lifeline = inventoryService;
            ref to : Lifeline = ordersEventsTopic;
            attribute :>> label = "inventory-reserved";
        }
    }
}
