package ObservabilityDomain {
    import SoftwareCore::*;
    import SoftwareControlPlane::*;

    attribute def SignalKind;
    attribute def AlertSeverity;

    part def MetricSource {
        attribute sourceName : Name;
    }

    part def LogSource {
        attribute sourceName : Name;
    }

    part def TraceSource {
        attribute sourceName : Name;
    }

    part def AlertPolicy {
        attribute policyName : Name;
        attribute severity : AlertSeverity;
    }

    part def Dashboard {
        attribute dashboardName : Name;
    }

    part def ServiceLevelIndicator specializes SliDefinition {
        attribute signalKind : SignalKind;
    }

    part def ServiceLevelObjective specializes SloDefinition;
}
