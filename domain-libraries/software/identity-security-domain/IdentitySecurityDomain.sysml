package IdentitySecurityDomain {
    import SoftwareCore::*;
    import SoftwareControlPlane::*;

    attribute def ExposureType;
    attribute def TrafficDirection;

    part def TrustBoundary {
        attribute boundaryName : Name;
    }

    part def IdentityProvider {
        attribute providerName : Name;
        attribute providerUri : UriString;
    }

    part def ServiceIdentity {
        attribute identityName : Name;
    }

    part def AuthorizationPolicy {
        attribute policyName : Name;
    }

    part def CredentialStore {
        attribute storeName : Name;
    }

    part def ExposureBoundary {
        attribute exposureType : ExposureType;
        attribute trafficDirection : TrafficDirection;
    }
}
