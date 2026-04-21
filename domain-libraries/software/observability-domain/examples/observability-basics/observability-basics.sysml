package ObservabilityBasicsExample {
    import ObservabilityDomain::*;

    part def OrdersMetrics specializes MetricSource {
        attribute sourceName = "orders-metrics";
    }

    part def OrdersLogs specializes LogSource {
        attribute sourceName = "orders-logs";
    }

    part def OrdersLatency specializes ServiceLevelIndicator {
        attribute sliName = "latency";
        attribute signalKind = "latency";
    }

    part def OrdersAvailability specializes ServiceLevelObjective {
        attribute sloName = "availability";
    }

    part def PagerDutyAlert specializes AlertPolicy {
        attribute policyName = "orders-p1";
        attribute severity = "critical";
    }
}
