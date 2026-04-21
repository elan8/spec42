package RobotPerception {
    import RoboticsCore::*;

    attribute def SensorType;
    attribute def EstimateType;

    part def Sensor :> Subsystem {
        attribute sensorName : Name;
        attribute sensorType : SensorType;
    }

    part def SensorSuite :> Subsystem {
        attribute suiteName : Name;
    }

    part def CameraSensor :> Sensor;
    part def LidarSensor :> Sensor;
    part def ImuSensor :> Sensor;
    part def EncoderSensor :> Sensor;

    part def PerceptionPipeline {
        attribute pipelineName : Name;
    }

    part def StateEstimate {
        attribute estimateName : Name;
        attribute estimateType : EstimateType;
    }
}
