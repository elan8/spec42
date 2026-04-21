package RobotStructure {
    import RoboticsCore::*;

    attribute def JointType;
    attribute def LinkLength;
    attribute def PayloadCapacity;

    part def RobotPlatform :> RobotSystem;

    part def Chassis :> Subsystem {
        attribute name : Name;
    }

    part def Manipulator :> Subsystem {
        attribute name : Name;
    }

    part def EndEffector :> Subsystem {
        attribute name : Name;
    }

    part def Joint {
        attribute name : Name;
        attribute jointType : JointType;
    }

    part def Link {
        attribute name : Name;
        attribute linkLength : LinkLength;
    }

    part def MountPoint {
        attribute name : Name;
    }

    part def Payload {
        attribute name : Name;
        attribute capacity : PayloadCapacity;
    }
}
