package WebShopArchitecture {
    private import SoftwareInteractions::*;

    port def HttpPort {
        in request : String;
        out response : String;
    }

    port def EventPort {
        in eventIn : String;
        out eventOut : String;
    }

    port def SqlPort {
        in query : String;
        out result : String;
    }

    port def PaymentProviderPort {
        out authorizationRequest : String;
        in authorizationResult : String;
    }

    port def EmailPort {
        out emailCommand : String;
    }

    part def Storefront {
        attribute channelName : String;
        port checkoutApiOut : HttpPort;
        port catalogApiOut : HttpPort;
    }

    part def ApiGateway {
        attribute serviceName : String;
        port publicCheckoutIn : ~HttpPort;
        port publicCatalogIn : ~HttpPort;
        port checkoutApiOut : HttpPort;
        port catalogApiOut : HttpPort;
    }

    part def CatalogService {
        attribute serviceName : String;
        port apiIn : ~HttpPort;
        port catalogDbOut : SqlPort;
    }

    part def CheckoutService {
        attribute serviceName : String;
        port apiIn : ~HttpPort;
        port ordersDbOut : SqlPort;
        port paymentsApiOut : HttpPort;
        port inventoryApiOut : HttpPort;
        port orderEventsOut : EventPort;
    }

    part def PaymentsService {
        attribute serviceName : String;
        port apiIn : ~HttpPort;
        port paymentsDbOut : SqlPort;
        port providerOut : PaymentProviderPort;
    }

    part def InventoryService {
        attribute serviceName : String;
        port apiIn : ~HttpPort;
        port inventoryDbOut : SqlPort;
        port orderEventsIn : ~EventPort;
        port reservationEventsOut : EventPort;
    }

    part def NotificationService {
        attribute serviceName : String;
        port orderEventsIn : ~EventPort;
        port emailOut : EmailPort;
    }

    part def EventBus {
        attribute platformName : String;
        port orderEventsIn : ~EventPort;
        port orderEventsOut : EventPort;
        port reservationEventsIn : ~EventPort;
        port reservationEventsOut : EventPort;
    }

    part def OrdersDatabase {
        attribute storeName : String;
        port sqlIn : ~SqlPort;
    }

    part def CatalogDatabase {
        attribute storeName : String;
        port sqlIn : ~SqlPort;
    }

    part def PaymentsDatabase {
        attribute storeName : String;
        port sqlIn : ~SqlPort;
    }

    part def InventoryDatabase {
        attribute storeName : String;
        port sqlIn : ~SqlPort;
    }

    part def PaymentProvider {
        attribute systemName : String;
        port providerIn : ~PaymentProviderPort;
    }

    part def EmailProvider {
        attribute providerName : String;
        port emailIn : ~EmailPort;
    }

    part def SmallWebShopPlatform {
        part storefront : Storefront {
            attribute :>> channelName = "web-storefront";
        }
        part apiGateway : ApiGateway {
            attribute :>> serviceName = "edge-gateway";
        }
        part catalogService : CatalogService {
            attribute :>> serviceName = "catalog-service";
        }
        part checkoutService : CheckoutService {
            attribute :>> serviceName = "checkout-service";
        }
        part paymentsService : PaymentsService {
            attribute :>> serviceName = "payments-service";
        }
        part inventoryService : InventoryService {
            attribute :>> serviceName = "inventory-service";
        }
        part notificationService : NotificationService {
            attribute :>> serviceName = "notification-service";
        }
        part eventBus : EventBus {
            attribute :>> platformName = "commerce-events";
        }

        part ordersDb : OrdersDatabase {
            attribute :>> storeName = "orders-postgres";
        }
        part catalogDb : CatalogDatabase {
            attribute :>> storeName = "catalog-postgres";
        }
        part paymentsDb : PaymentsDatabase {
            attribute :>> storeName = "payments-postgres";
        }
        part inventoryDb : InventoryDatabase {
            attribute :>> storeName = "inventory-postgres";
        }

        part paymentProvider : PaymentProvider {
            attribute :>> systemName = "stripe";
        }
        part emailProvider : EmailProvider {
            attribute :>> providerName = "customer-email";
        }

        connect storefront.checkoutApiOut to apiGateway.publicCheckoutIn;
        connect storefront.catalogApiOut to apiGateway.publicCatalogIn;

        connect apiGateway.checkoutApiOut to checkoutService.apiIn;
        connect apiGateway.catalogApiOut to catalogService.apiIn;

        connect catalogService.catalogDbOut to catalogDb.sqlIn;

        connect checkoutService.ordersDbOut to ordersDb.sqlIn;
        connect checkoutService.paymentsApiOut to paymentsService.apiIn;
        connect checkoutService.inventoryApiOut to inventoryService.apiIn;
        connect checkoutService.orderEventsOut to eventBus.orderEventsIn;

        connect paymentsService.paymentsDbOut to paymentsDb.sqlIn;
        connect paymentsService.providerOut to paymentProvider.providerIn;

        connect inventoryService.inventoryDbOut to inventoryDb.sqlIn;
        connect eventBus.orderEventsOut to inventoryService.orderEventsIn;
        connect eventBus.orderEventsOut to notificationService.orderEventsIn;
        connect inventoryService.reservationEventsOut to eventBus.reservationEventsIn;

        connect notificationService.emailOut to emailProvider.emailIn;
    }

    part webshopPlatform : SmallWebShopPlatform;

    part def CheckoutFlow :> InteractionScenario {
        part storefront : Lifeline;
        part apiGateway : Lifeline;
        part checkoutService : Lifeline;
        part paymentsService : Lifeline;
        part inventoryService : Lifeline;
        part eventBus : Lifeline;
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
                    ref to : Lifeline = eventBus;
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
        part eventBus : Lifeline;
        part inventoryService : Lifeline;
        part notificationService : Lifeline;
        part orderProjection : Lifeline;

        part publishOrderCreated : AsynchronousMessage {
            ref from : Lifeline = checkoutService;
            ref to : Lifeline = eventBus;
            attribute :>> label = "order-created";
        }

        part createProjection : CreationMessage {
            ref from : Lifeline = eventBus;
            ref to : Lifeline = orderProjection;
            attribute :>> label = "create order projection";
        }

        part fanOutInventory : AsynchronousMessage {
            ref from : Lifeline = eventBus;
            ref to : Lifeline = inventoryService;
            attribute :>> label = "deliver order-created";
        }

        part fanOutNotifications : AsynchronousMessage {
            ref from : Lifeline = eventBus;
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
            ref to : Lifeline = eventBus;
            attribute :>> label = "confirmation-requested";
        }

        part reservationPublished : AsynchronousMessage {
            ref from : Lifeline = inventoryService;
            ref to : Lifeline = eventBus;
            attribute :>> label = "inventory-reserved";
        }
    }
}
