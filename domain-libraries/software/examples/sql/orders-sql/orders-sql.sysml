package OrdersSqlExample {
    import SqlDomain::*;

    part def OrdersDb :> SqlDatabase {
        attribute storeName = "orders-db";
        attribute engineName = "postgres";
        attribute consistencyMode = "strong";
    }
}
