package OrdersSqlExample {
    import SqlDomain::*;
    import DistributedSystems::*;

    port def SqlPort;

    part def OrdersDb :> SqlDatabase {
        attribute :>> storeName = "orders-db";
        attribute :>> engineName = "postgres";
        attribute :>> consistencyMode = "strong";
        port queryIn : SqlPort;
        port resultOut : SqlPort;
    }

    part def OrdersSqlClient :> Service {
        attribute :>> serviceName = "orders-sql-client";
        port queryOut : SqlPort;
    }

    part def OrdersSqlSystem {
        part ordersClient : OrdersSqlClient;
        part ordersDb : OrdersDb;

        connect ordersClient.queryOut to ordersDb.queryIn;
    }
}
