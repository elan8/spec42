package ObservabilityBasicsExample {
    import ObservabilityDomain::*;

    port def ObservabilityPort;

    part def OrdersMetrics :> MetricSource {
        attribute :>> sourceName = "orders-metrics";
        port metricOut : ObservabilityPort;
    }

    part def OrdersLogs :> LogSource {
        attribute :>> sourceName = "orders-logs";
        port logOut : ObservabilityPort;
    }

    part def OrdersLatency :> ServiceLevelIndicator {
        attribute :>> sliName = "latency";
        attribute :>> signalKind = "latency";
        port indicatorIn : ObservabilityPort;
        port indicatorOut : ObservabilityPort;
    }

    part def OrdersAvailability :> ServiceLevelObjective {
        attribute :>> sloName = "availability";
        port objectiveIn : ObservabilityPort;
        port objectiveOut : ObservabilityPort;
    }

    part def PagerDutyAlert :> AlertPolicy {
        attribute :>> policyName = "orders-p1";
        attribute :>> severity = "critical";
        port alertIn : ObservabilityPort;
    }

    part def ObservabilitySystem {
        part ordersMetrics : OrdersMetrics;
        part ordersLogs : OrdersLogs;
        part ordersLatency : OrdersLatency;
        part ordersAvailability : OrdersAvailability;
        part pagerDutyAlert : PagerDutyAlert;

        connect ordersMetrics.metricOut to ordersLatency.indicatorIn;
        connect ordersLogs.logOut to ordersLatency.indicatorIn;
        connect ordersLatency.indicatorOut to ordersAvailability.objectiveIn;
        connect ordersAvailability.objectiveOut to pagerDutyAlert.alertIn;
    }
}
