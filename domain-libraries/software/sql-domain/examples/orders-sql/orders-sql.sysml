package OrdersSqlExample {
    import SqlDomain::*;

    part def OrdersDb specializes SqlDatabase {
        attribute storeName = "orders-db";
        attribute engineName = "postgres";
        attribute consistencyMode = "strong";
    }
}
