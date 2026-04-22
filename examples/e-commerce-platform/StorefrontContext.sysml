package StorefrontContext {
    import SoftwareCore::*;
    import HttpDomain::*;

    part def StorefrontWebApp :> SoftwareModule {
        attribute :>> moduleName = "storefront-web";
        port apiEgress;
    }

    part def MobileApp :> SoftwareModule {
        attribute :>> moduleName = "storefront-mobile";
        port apiEgress;
    }

    part def BffService :> HttpService {
        attribute :>> serviceName = "storefront-bff";
        port webIngress;
        port mobileIngress;
        port checkoutApiEgress;
    }

    part def PublicEdge :> ExternalSystem {
        attribute :>> systemName = "public-edge";
        port edgeIngress;
        port edgeEgress;
    }
}
