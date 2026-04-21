package IdentityBasicsExample {
    import IdentitySecurityDomain::*;

    part def CorporateIdp :> IdentityProvider {
        attribute providerName = "corporate-idp";
        attribute providerUri = "https://id.example.com";
    }

    part def OrdersIdentity :> ServiceIdentity {
        attribute identityName = "orders-service";
    }

    part def OrdersAuthz :> AuthorizationPolicy {
        attribute policyName = "orders-api-policy";
    }

    part def PublicIngress :> ExposureBoundary {
        attribute exposureType = "public";
        attribute trafficDirection = "north-south";
    }
}
