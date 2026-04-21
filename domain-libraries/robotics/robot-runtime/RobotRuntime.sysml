package RobotRuntime {
    import RoboticsCore::*;
    import RobotControl::*;
    import RobotPerception::*;

    attribute def ChannelType;
    attribute def LifecyclePhase;
    attribute def HealthState;

    part def RuntimeNode {
        attribute name : Name;
    }

    part def ExecutionGraph {
        attribute name : Name;
    }

    part def CommunicationChannel {
        attribute name : Name;
        attribute channelType : ChannelType;
    }

    part def ParameterSet {
        attribute name : Name;
    }

    part def LifecycleState {
        attribute phase : LifecyclePhase;
    }

    part def HealthMonitor {
        attribute name : Name;
        attribute healthState : HealthState;
    }
}
