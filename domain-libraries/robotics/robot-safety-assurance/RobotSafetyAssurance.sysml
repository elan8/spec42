package RobotSafetyAssurance {
    import RoboticsCore::*;
    import RobotStructure::*;
    import RobotControl::*;

    attribute def HazardSeverity;
    attribute def SafetyCategory;
    attribute def ConstraintType;

    part def Hazard {
        attribute hazardName : Name;
        attribute severity : HazardSeverity;
    }

    part def SafetyFunction {
        attribute functionName : Name;
        attribute category : SafetyCategory;
    }

    part def SafetyBoundary {
        attribute boundaryName : Name;
    }

    part def RiskControl {
        attribute controlName : Name;
    }

    part def FailSafeMode {
        attribute modeName : ModeName;
    }

    part def SafetyCase {
        attribute caseName : Name;
    }

    part def OperationalConstraint {
        attribute constraintName : Name;
        attribute constraintType : ConstraintType;
    }
}
