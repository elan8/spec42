package RobotActuation {
    import RoboticsCore::*;

    attribute def ActuatorType;
    attribute def CommandType;

    part def Actuator :> Subsystem {
        attribute name : Name;
        attribute actuatorType : ActuatorType;
    }

    part def DriveTrain :> Subsystem {
        attribute name : Name;
    }

    part def MotorController :> Subsystem {
        attribute name : Name;
    }

    part def ServoActuator :> Actuator;
    part def BrakeActuator :> Actuator;

    part def ActuationCommand {
        attribute name : Name;
        attribute commandType : CommandType;
    }
}
