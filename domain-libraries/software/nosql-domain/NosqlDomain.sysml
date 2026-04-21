package NosqlDomain {
    import DistributedSystems::*;

    attribute def DataModelType;

    part def DocumentStore specializes DataStore {
        attribute dataModel : DataModelType;
    }

    part def KeyValueStore specializes DataStore {
        attribute dataModel : DataModelType;
    }

    part def ObjectStore specializes DataStore;
}
