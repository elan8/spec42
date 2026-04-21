package ObservabilityBasicsExample {
    import ObservabilityDomain::*;

    part def OrdersMetrics :> MetricSource {
        attribute :>> sourceName = "orders-metrics";
    }

    part def OrdersLogs :> LogSource {
        attribute :>> sourceName = "orders-logs";
    }

    part def OrdersLatency :> ServiceLevelIndicator {
        attribute :>> sliName = "latency";
        attribute :>> signalKind = "latency";
    }

    part def OrdersAvailability :> ServiceLevelObjective {
        attribute :>> sloName = "availability";
    }

    part def PagerDutyAlert :> AlertPolicy {
        attribute :>> policyName = "orders-p1";
        attribute :>> severity = "critical";
    }
}
