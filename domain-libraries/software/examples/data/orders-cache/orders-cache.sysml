package OrdersCacheExample {
    import NosqlDomain::*;

    part def OrdersCache :> KeyValueStore {
        attribute :>> storeName = "orders-cache";
        attribute :>> dataModel = "key-value";
    }
}
