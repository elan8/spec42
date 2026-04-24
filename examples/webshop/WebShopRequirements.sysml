package WebShopRequirements {
    import WebShopArchitecture::*;

    requirement def CheckoutLatencyReq {
        subject checkoutService : CheckoutService;
        doc /* The checkout path shall keep p95 end-user latency below 300 ms. */
        require constraint { doc /* p95 create-order latency remains within target. */ }
    }

    requirement def CheckoutAvailabilityReq {
        subject apiGateway : ApiGateway;
        doc /* The public checkout API shall maintain at least 99.95% monthly availability. */
        require constraint { doc /* Availability budget for checkout stays above SLO. */ }
    }

    requirement def PaymentReliabilityReq {
        subject paymentsService : PaymentsService;
        doc /* Payment authorization calls shall meet the business reliability objective. */
        require constraint { doc /* Payment authorization success rate remains in target range. */ }
    }

    requirement def EventDurabilityReq {
        subject ordersEventsTopic : OrdersEventsTopic;
        doc /* Order events shall be retained for replay and incident recovery. */
        require constraint { doc /* Topic retention remains at least seven days. */ }
    }

    requirement def SecurityBoundaryReq {
        subject apiGateway : ApiGateway;
        doc /* Public traffic shall cross an explicit ingress boundary before business services. */
        require constraint { doc /* Only approved ingress paths expose checkout functionality. */ }
    }
}
