package CloudRuntimeBasicsExample {
    import CloudRuntimeDomain::*;

    port def RuntimePort;

    part def PrimaryRegion :> CloudRegion {
        attribute :>> regionName = "eu-west";
        port regionOut : RuntimePort;
    }

    part def ComputeCluster :> RuntimeCluster {
        attribute :>> clusterName = "shared-runtime";
        attribute :>> resourceClass = "general-purpose";
        port clusterIn : RuntimePort;
        port serviceOut : RuntimePort;
    }

    part def ManagedDatabase :> ManagedService {
        attribute :>> serviceName = "managed-postgres";
        attribute :>> serviceUri = "postgres://db.internal";
        port serviceIn : RuntimePort;
    }

    part def CloudRuntimeSystem {
        part primaryRegion : PrimaryRegion;
        part computeCluster : ComputeCluster;
        part managedDatabase : ManagedDatabase;

        connect primaryRegion.regionOut to computeCluster.clusterIn;
        connect computeCluster.serviceOut to managedDatabase.serviceIn;
    }
}
