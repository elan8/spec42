package RobotActuation {
    import RoboticsCore::*;

    attribute def ActuatorType;
    attribute def CommandType;

    part def Actuator specializes Subsystem {
        attribute actuatorName : Name;
        attribute actuatorType : ActuatorType;
    }

    part def DriveTrain specializes Subsystem {
        attribute driveName : Name;
    }

    part def MotorController specializes Subsystem {
        attribute controllerName : Name;
    }

    part def ServoActuator specializes Actuator;
    part def BrakeActuator specializes Actuator;

    part def ActuationCommand {
        attribute commandName : Name;
        attribute commandType : CommandType;
    }
}
