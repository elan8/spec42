package OrdersOpenApiExample {
    import HttpDomain::*;
    import SerializationDomain::*;
    import OpenApiDomain::*;

    part def OrdersSpec specializes OpenApiSpecification {
        attribute specificationName = "orders-openapi";
        attribute specificationVersion = "3.1.0";
        attribute specificationUri = "https://api.example.com/orders/openapi.json";
    }

    part def ListOrders specializes OpenApiOperation {
        attribute protocol = "https";
        attribute path = "/orders";
        attribute port = "443";
        attribute operationId = "listOrders";
    }

    item def OrderSchema specializes OpenApiSchema {
        attribute mediaType = "application/json";
        attribute schema = "#/components/schemas/Order";
    }
}
