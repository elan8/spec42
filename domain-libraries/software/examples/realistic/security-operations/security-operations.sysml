package SecurityOperationsExample {
    import SoftwareCore::*;
    import DistributedSystems::*;
    import IdentitySecurityDomain::*;
    import SoftwareControlPlane::*;
    import ObservabilityDomain::*;
    import CyberAssuranceDomain::*;
    import KubernetesDomain::*;
    import CloudRuntimeDomain::*;

    port def SecurityPort;

    part def PublicGateway :> ExternalEndpoint {
        attribute :>> endpointName = "public-gateway";
        attribute :>> endpointUri = "https://api.example.com";
        port ingress : SecurityPort;
        port upstream : SecurityPort;
    }

    part def PaymentsService :> Service {
        attribute :>> serviceName = "payments-service";
        port apiIngress : SecurityPort;
        port secretIn : SecurityPort;
        port configIn : SecurityPort;
        port authzCheckOut : SecurityPort;
    }

    part def IdentityProviderService :> IdentityProvider {
        attribute :>> providerName = "enterprise-oidc";
        attribute :>> providerUri = "https://idp.example.com";
        port tokenOut : SecurityPort;
    }

    part def PaymentsIdentity :> ServiceIdentity {
        attribute :>> identityName = "spiffe://payments/prod/payments-service";
        port identityIn : SecurityPort;
    }

    part def RuntimeAuthzPolicy :> AuthorizationPolicy {
        attribute :>> policyName = "payments-runtime-authz";
        port policyIn : SecurityPort;
        port policyOut : SecurityPort;
    }

    part def BoundaryInternetToCore :> TrustBoundary {
        attribute :>> boundaryName = "internet-to-core";
    }

    part def PublicExposure :> ExposureBoundary {
        attribute :>> exposureType = "public";
        attribute :>> trafficDirection = "north-south";
    }

    part def PaymentsSecrets :> CredentialStore {
        attribute :>> storeName = "payments-credential-store";
        port credentialOut : SecurityPort;
    }

    part def PaymentsConfig :> ConfigurationSource {
        attribute :>> sourceName = "payments-config";
        attribute :>> sourceUri = "vault://payments/prod/config";
        port configOut : SecurityPort;
    }

    part def TokenSigningKey :> SecretItem {
        attribute :>> key = "TOKEN_SIGNING_KEY";
        port secretOut : SecurityPort;
    }

    part def AuthzDependency :> ServiceDependencyLink {
        attribute :>> dependencyName = "payments-needs-authz";
    }

    part def PaymentsHealth :> HealthSignal {
        attribute :>> signalName = "payments-authz-health";
        attribute :>> healthState = "healthy";
    }

    part def SecurityAuditLogs :> LogSource {
        attribute :>> sourceName = "security-audit-logs";
        port logOut : SecurityPort;
    }

    part def SecurityAlerts :> AlertPolicy {
        attribute :>> policyName = "authz-deny-spike";
        attribute :>> severity = "high";
        port alertIn : SecurityPort;
    }

    part def UnauthorizedAccessIncident :> SecurityIncident {
        attribute :>> incidentId = "INC-SEC-2088";
        attribute :>> severity = "high";
        port incidentIn : SecurityPort;
    }

    part def SecurityOpsCluster :> KubernetesCluster {
        attribute :>> clusterName = "security-ops-prod";
        attribute :>> nodeName = "sec-node-a";
        attribute :>> host = "sec-node-a.internal";
    }

    part def PaymentsDeployment :> KubernetesDeployment {
        attribute :>> deploymentName = "payments-service";
        attribute :>> replicas = "5";
        attribute :>> image = "ghcr.io/example/payments-service:5.2.0";
    }

    part def HomeRegion :> CloudRegion {
        attribute :>> regionName = "eu-west-2";
    }

    part def SecurityOperationsSystem {
        part publicGateway : PublicGateway;
        part paymentsService : PaymentsService;
        part identityProvider : IdentityProviderService;
        part paymentsIdentity : PaymentsIdentity;
        part runtimeAuthzPolicy : RuntimeAuthzPolicy;
        part paymentsSecrets : PaymentsSecrets;
        part paymentsConfig : PaymentsConfig;
        part tokenSigningKey : TokenSigningKey;
        part authzDependency : AuthzDependency;
        part paymentsHealth : PaymentsHealth;
        part securityAuditLogs : SecurityAuditLogs;
        part securityAlerts : SecurityAlerts;
        part unauthorizedAccessIncident : UnauthorizedAccessIncident;

        connect publicGateway.upstream to paymentsService.apiIngress;
        connect identityProvider.tokenOut to paymentsIdentity.identityIn;
        connect paymentsIdentity.identityIn to runtimeAuthzPolicy.policyIn;
        connect runtimeAuthzPolicy.policyOut to paymentsService.authzCheckOut;
        connect paymentsSecrets.credentialOut to tokenSigningKey.secretOut;
        connect tokenSigningKey.secretOut to paymentsService.secretIn;
        connect paymentsConfig.configOut to paymentsService.configIn;
        connect paymentsService.authzCheckOut to authzDependency.dependencyName;
        connect securityAuditLogs.logOut to securityAlerts.alertIn;
        connect securityAlerts.alertIn to unauthorizedAccessIncident.incidentIn;
    }

    allocate PaymentsService to SecurityOpsCluster;
    allocate PaymentsDeployment to SecurityOpsCluster;
}
