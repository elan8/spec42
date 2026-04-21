package CloudRuntimeDomain {
    import SoftwareCore::*;
    import SoftwareControlPlane::*;

    attribute def RegionName;
    attribute def ZoneName;
    attribute def ResourceClass;

    part def CloudRegion {
        attribute regionName : RegionName;
    }

    part def CloudZone {
        attribute zoneName : ZoneName;
    }

    part def RuntimeCluster {
        attribute clusterName : Name;
        attribute resourceClass : ResourceClass;
    }

    part def RuntimeWorkload specializes ArtifactReference;

    part def ManagedService {
        attribute serviceName : Name;
        attribute serviceUri : UriString;
    }
}
