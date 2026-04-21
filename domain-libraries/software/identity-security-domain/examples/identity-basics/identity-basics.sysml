package IdentityBasicsExample {
    import IdentitySecurityDomain::*;

    part def CorporateIdp specializes IdentityProvider {
        attribute providerName = "corporate-idp";
        attribute providerUri = "https://id.example.com";
    }

    part def OrdersIdentity specializes ServiceIdentity {
        attribute identityName = "orders-service";
    }

    part def OrdersAuthz specializes AuthorizationPolicy {
        attribute policyName = "orders-api-policy";
    }

    part def PublicIngress specializes ExposureBoundary {
        attribute exposureType = "public";
        attribute trafficDirection = "north-south";
    }
}
