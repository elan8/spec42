package OrdersOpenApiExample {
    import HttpDomain::*;
    import SerializationDomain::*;
    import OpenApiDomain::*;

    port def ApiModelPort;

    part def OrdersSpec :> OpenApiSpecification {
        attribute :>> specificationName = "orders-openapi";
        attribute :>> specificationVersion = "3.1.0";
        attribute :>> specificationUri = "https://api.example.com/orders/openapi.json";
        port specOut : ApiModelPort;
    }

    part def ListOrders :> OpenApiOperation {
        attribute :>> protocol = "https";
        attribute :>> path = "/orders";
        attribute :>> port = "443";
        attribute :>> encryption = tls;
        attribute :>> operationId = "listOrders";
        port operationIn : ApiModelPort;
        port operationOut : ApiModelPort;
    }

    item def OrderSchema :> OpenApiSchema {
        attribute :>> mediaType = "application/json";
        attribute :>> schema = "#/components/schemas/Order";
    }

    part def OrdersApiModel {
        part ordersSpec : OrdersSpec;
        part listOrders : ListOrders;

        connect ordersSpec.specOut to listOrders.operationIn;
    }
}
