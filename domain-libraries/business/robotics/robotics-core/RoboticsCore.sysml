package RoboticsCore {
    attribute def Identifier;
    attribute def Name;
    attribute def ModeName;
    attribute def EnvironmentKind;
    attribute def InterfaceType;
    attribute def PowerKind;
    attribute def MissionType;
    attribute def CriticalityLevel;

    part def RobotSystem {
        attribute name : Name;
        attribute criticality : CriticalityLevel;
    }

    part def Subsystem {
        attribute name : Name;
    }

    part def RobotMode {
        attribute modeName : ModeName;
    }

    part def MissionProfile {
        attribute name : Name;
        attribute missionType : MissionType;
    }

    part def OperatingEnvironment {
        attribute name : Name;
        attribute environmentKind : EnvironmentKind;
    }

    part def PhysicalInterface {
        attribute name : Name;
        attribute interfaceType : InterfaceType;
    }

    part def PowerSource {
        attribute name : Name;
        attribute powerKind : PowerKind;
    }
}
