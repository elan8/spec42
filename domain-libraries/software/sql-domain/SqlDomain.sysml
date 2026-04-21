package SqlDomain {
    import SoftwareCore::*;
    import DistributedSystems::*;

    attribute def ConsistencyMode;

    part def RelationalStore specializes DataStore {
        attribute consistencyMode : ConsistencyMode;
    }

    part def SqlDatabase specializes RelationalStore {
        attribute engineName : Name;
    }

    part def ReadReplica specializes RelationalStore;
}
