package RobotStructure {
    import RoboticsCore::*;

    attribute def JointType;
    attribute def LinkLength;
    attribute def PayloadCapacity;

    part def RobotPlatform :> RobotSystem;

    part def Chassis :> Subsystem {
        attribute chassisName : Name;
    }

    part def Manipulator :> Subsystem {
        attribute manipulatorName : Name;
    }

    part def EndEffector :> Subsystem {
        attribute effectorName : Name;
    }

    part def Joint {
        attribute jointName : Name;
        attribute jointType : JointType;
    }

    part def Link {
        attribute linkName : Name;
        attribute linkLength : LinkLength;
    }

    part def MountPoint {
        attribute mountName : Name;
    }

    part def Payload {
        attribute payloadName : Name;
        attribute capacity : PayloadCapacity;
    }
}
