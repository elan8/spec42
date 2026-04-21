package CloudRuntimeBasicsExample {
    import CloudRuntimeDomain::*;

    part def PrimaryRegion :> CloudRegion {
        attribute :>> regionName = "eu-west";
    }

    part def ComputeCluster :> RuntimeCluster {
        attribute :>> clusterName = "shared-runtime";
        attribute :>> resourceClass = "general-purpose";
    }

    part def ManagedDatabase :> ManagedService {
        attribute :>> serviceName = "managed-postgres";
        attribute :>> serviceUri = "postgres://db.internal";
    }
}
