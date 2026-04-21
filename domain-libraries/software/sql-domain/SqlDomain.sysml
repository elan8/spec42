package SqlDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def ConsistencyMode;

    part def RelationalStore :> DataStore {
        attribute consistencyMode : ConsistencyMode;
    }

    part def SqlDatabase :> RelationalStore {
        attribute engineName : Name;
    }

    part def ReadReplica :> RelationalStore;
}
