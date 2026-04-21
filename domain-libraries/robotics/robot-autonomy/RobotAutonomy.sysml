package RobotAutonomy {
    import RoboticsCore::*;
    import RobotPerception::*;
    import RobotControl::*;

    attribute def PlannerType;
    attribute def PolicyType;

    part def BehaviorModule {
        attribute behaviorName : Name;
    }

    part def TaskPlan {
        attribute planName : Name;
    }

    part def Planner {
        attribute plannerName : Name;
        attribute plannerType : PlannerType;
    }

    part def NavigationFunction specializes BehaviorModule;
    part def LocalizationFunction specializes BehaviorModule;

    part def DecisionPolicy {
        attribute policyName : Name;
        attribute policyType : PolicyType;
    }
}
