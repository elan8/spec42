package RobotRuntime {
    import RoboticsCore::*;
    import RobotControl::*;
    import RobotPerception::*;

    attribute def ChannelType;
    attribute def LifecyclePhase;
    attribute def HealthState;

    part def RuntimeNode {
        attribute nodeName : Name;
    }

    part def ExecutionGraph {
        attribute graphName : Name;
    }

    part def CommunicationChannel {
        attribute channelName : Name;
        attribute channelType : ChannelType;
    }

    part def ParameterSet {
        attribute parameterSetName : Name;
    }

    part def LifecycleState {
        attribute phase : LifecyclePhase;
    }

    part def HealthMonitor {
        attribute monitorName : Name;
        attribute healthState : HealthState;
    }
}
