package CloudRuntimeBasicsExample {
    import CloudRuntimeDomain::*;

    part def PrimaryRegion specializes CloudRegion {
        attribute regionName = "eu-west";
    }

    part def ComputeCluster specializes RuntimeCluster {
        attribute clusterName = "shared-runtime";
        attribute resourceClass = "general-purpose";
    }

    part def ManagedDatabase specializes ManagedService {
        attribute serviceName = "managed-postgres";
        attribute serviceUri = "postgres://db.internal";
    }
}
