package RobotPerception {
    import RoboticsCore::*;

    attribute def SensorType;
    attribute def EstimateType;

    part def Sensor :> Subsystem {
        attribute name : Name;
        attribute sensorType : SensorType;
    }

    part def SensorSuite :> Subsystem {
        attribute name : Name;
    }

    part def CameraSensor :> Sensor;
    part def LidarSensor :> Sensor;
    part def ImuSensor :> Sensor;
    part def EncoderSensor :> Sensor;

    part def PerceptionPipeline {
        attribute name : Name;
    }

    part def StateEstimate {
        attribute name : Name;
        attribute estimateType : EstimateType;
    }
}
