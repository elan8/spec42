package CheckoutContext {
    import HttpDomain::*;
    import OpenApiDomain::*;
    import GrpcDomain::*;

    part def PublicOrdersApi :> OpenApiOperation {
        attribute :>> protocol = "https";
        attribute :>> path = "/v1/orders";
        attribute :>> port = "443";
        attribute :>> encryption = "tls13";
        attribute :>> operationId = "createOrder";
        port ingress;
        port upstream;
    }

    part def OrdersService :> HttpService {
        attribute :>> serviceName = "orders-service";
        port apiIngress;
        port pricingRpcOut;
        port pricingRpcIn;
        port paymentCommandOut;
        port eventEgress;
        port sqlEgress;
        port cacheEgress;
        port configIn;
        port secretIn;
    }

    part def PricingService :> GrpcService {
        attribute :>> serviceName = "pricing-service";
        port grpcIngress;
    }

    part def PriceOrder :> UnaryRpc {
        attribute :>> methodName = "PriceOrder";
        port rpcIn;
        port rpcOut;
    }
}
