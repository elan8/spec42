package SoftwareControlPlane {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def ScalingTarget;
    attribute def HealthState;
    attribute def SliName;
    attribute def SloName;

    part def ConfigurationSource {
        attribute sourceName : Name;
        attribute sourceUri : UriString;
    }

    part def ConfigurationItem {
        attribute key : Identifier;
    }

    part def SecretItem :> ConfigurationItem;

    part def ServiceDependencyLink :> ServiceDependency;

    part def RuntimeDependency {
        attribute dependencyName : Name;
    }

    part def IngressBoundary {
        attribute boundaryName : Name;
    }

    part def EgressBoundary {
        attribute boundaryName : Name;
    }

    part def PolicyBoundary {
        attribute boundaryName : Name;
    }

    part def HealthSignal {
        attribute signalName : Name;
        attribute healthState : HealthState;
    }

    part def ScalingPolicy {
        attribute target : ScalingTarget;
    }

    part def ArtifactReference {
        attribute artifactName : Name;
        attribute artifactUri : UriString;
        attribute version : VersionString;
    }

    part def ExternalEndpoint {
        attribute endpointName : Name;
        attribute endpointUri : UriString;
    }

    part def SliDefinition {
        attribute sliName : SliName;
    }

    part def SloDefinition {
        attribute sloName : SloName;
    }
}
