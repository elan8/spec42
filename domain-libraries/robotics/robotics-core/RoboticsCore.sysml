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
        attribute robotName : Name;
        attribute criticality : CriticalityLevel;
    }

    part def Subsystem {
        attribute subsystemName : Name;
    }

    part def RobotMode {
        attribute modeName : ModeName;
    }

    part def MissionProfile {
        attribute missionName : Name;
        attribute missionType : MissionType;
    }

    part def OperatingEnvironment {
        attribute environmentName : Name;
        attribute environmentKind : EnvironmentKind;
    }

    part def PhysicalInterface {
        attribute interfaceName : Name;
        attribute interfaceType : InterfaceType;
    }

    part def PowerSource {
        attribute sourceName : Name;
        attribute powerKind : PowerKind;
    }
}
