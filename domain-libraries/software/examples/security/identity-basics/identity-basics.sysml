package IdentityBasicsExample {
    import IdentitySecurityDomain::*;

    port def SecurityPort;

    part def CorporateIdp :> IdentityProvider {
        attribute :>> providerName = "corporate-idp";
        attribute :>> providerUri = "https://id.example.com";
        port tokenOut : SecurityPort;
    }

    part def OrdersIdentity :> ServiceIdentity {
        attribute :>> identityName = "orders-service";
        port identityIn : SecurityPort;
        port authzOut : SecurityPort;
    }

    part def OrdersAuthz :> AuthorizationPolicy {
        attribute :>> policyName = "orders-api-policy";
        port policyIn : SecurityPort;
    }

    part def PublicIngress :> ExposureBoundary {
        attribute :>> exposureType = "public";
        attribute :>> trafficDirection = "north-south";
        port ingressIn : SecurityPort;
    }

    part def IdentitySecuritySystem {
        part corporateIdp : CorporateIdp;
        part ordersIdentity : OrdersIdentity;
        part ordersAuthz : OrdersAuthz;
        part publicIngress : PublicIngress;

        connect corporateIdp.tokenOut to ordersIdentity.identityIn;
        connect ordersIdentity.authzOut to ordersAuthz.policyIn;
        connect publicIngress.ingressIn to ordersIdentity.identityIn;
    }
}
