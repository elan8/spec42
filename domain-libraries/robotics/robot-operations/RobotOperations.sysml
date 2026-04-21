package RobotOperations {
    import RoboticsCore::*;
    import RobotRuntime::*;

    attribute def AlertSeverity;

    part def RobotFleet {
        attribute fleetName : Name;
    }

    part def FleetCoordinator {
        attribute coordinatorName : Name;
    }

    part def MaintenanceProcedure {
        attribute procedureName : Name;
    }

    part def OperationalAlert {
        attribute alertName : Name;
        attribute severity : AlertSeverity;
    }

    part def TelemetryStream {
        attribute streamName : Name;
    }

    part def MissionExecution {
        attribute executionName : Name;
    }
}
