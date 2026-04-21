package RobotAutonomy {
    import RoboticsCore::*;
    import RobotPerception::*;
    import RobotControl::*;

    attribute def PlannerType;
    attribute def PolicyType;

    part def BehaviorModule {
        attribute name : Name;
    }

    part def TaskPlan {
        attribute name : Name;
    }

    part def Planner {
        attribute name : Name;
        attribute plannerType : PlannerType;
    }

    part def NavigationFunction :> BehaviorModule;
    part def LocalizationFunction :> BehaviorModule;

    part def DecisionPolicy {
        attribute name : Name;
        attribute policyType : PolicyType;
    }
}
