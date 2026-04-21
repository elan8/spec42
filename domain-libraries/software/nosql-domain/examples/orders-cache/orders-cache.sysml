package OrdersCacheExample {
    import NosqlDomain::*;

    part def OrdersCache specializes KeyValueStore {
        attribute storeName = "orders-cache";
        attribute dataModel = "key-value";
    }
}
