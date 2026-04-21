package OpenApiDomain {
    import SoftwareCore::*;
    import HttpDomain::*;
    import SerializationDomain::*;

    attribute def SpecificationVersion;

    part def OpenApiSpecification {
        attribute specificationName : Name;
        attribute specificationVersion : SpecificationVersion;
        attribute specificationUri : UriString;
    }

    part def OpenApiOperation specializes RestApiEndpoint {
        attribute operationId : Identifier;
    }

    item def OpenApiSchema specializes JsonPayload;
}
