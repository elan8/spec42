package RobotSimulation {
    import RoboticsCore::*;
    import RobotPerception::*;

    attribute def ScenarioType;

    part def SimulationEnvironment {
        attribute simulationName : Name;
    }

    part def WorldModel {
        attribute worldName : Name;
    }

    part def SyntheticSensor specializes Sensor;

    part def ScenarioPlayback {
        attribute playbackName : Name;
        attribute scenarioType : ScenarioType;
    }

    part def HardwareInLoopSetup {
        attribute setupName : Name;
    }
}
