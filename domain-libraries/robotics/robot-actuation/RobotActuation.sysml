package RobotActuation {
    import RoboticsCore::*;

    attribute def ActuatorType;
    attribute def CommandType;

    part def Actuator :> Subsystem {
        attribute actuatorName : Name;
        attribute actuatorType : ActuatorType;
    }

    part def DriveTrain :> Subsystem {
        attribute driveName : Name;
    }

    part def MotorController :> Subsystem {
        attribute controllerName : Name;
    }

    part def ServoActuator :> Actuator;
    part def BrakeActuator :> Actuator;

    part def ActuationCommand {
        attribute commandName : Name;
        attribute commandType : CommandType;
    }
}
