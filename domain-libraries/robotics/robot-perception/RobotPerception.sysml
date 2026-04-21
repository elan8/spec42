package RobotPerception {
    import RoboticsCore::*;

    attribute def SensorType;
    attribute def EstimateType;

    part def Sensor specializes Subsystem {
        attribute sensorName : Name;
        attribute sensorType : SensorType;
    }

    part def SensorSuite specializes Subsystem {
        attribute suiteName : Name;
    }

    part def CameraSensor specializes Sensor;
    part def LidarSensor specializes Sensor;
    part def ImuSensor specializes Sensor;
    part def EncoderSensor specializes Sensor;

    part def PerceptionPipeline {
        attribute pipelineName : Name;
    }

    part def StateEstimate {
        attribute estimateName : Name;
        attribute estimateType : EstimateType;
    }
}
