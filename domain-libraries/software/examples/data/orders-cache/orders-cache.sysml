package OrdersCacheExample {
    import NosqlDomain::*;
    import DistributedSystems::*;

    port def CachePort;

    part def OrdersCache :> KeyValueStore {
        attribute :>> storeName = "orders-cache";
        attribute :>> dataModel = "key-value";
        port keyLookupIn : CachePort;
        port valueOut : CachePort;
    }

    part def CacheClient :> Service {
        attribute :>> serviceName = "orders-cache-client";
        port keyLookupOut : CachePort;
    }

    part def OrdersCacheSystem {
        part cacheClient : CacheClient;
        part ordersCache : OrdersCache;

        connect cacheClient.keyLookupOut to ordersCache.keyLookupIn;
    }
}
