package SoftwareCore {
    attribute def Identifier;
    attribute def Name;
    attribute def Hostname;
    attribute def PathString;
    attribute def PortNumber;
    attribute def ProtocolName;
    attribute def UriString;
    attribute def VersionString;
    attribute def EnvironmentName;
    attribute def ResourceName;
    attribute def CriticalityLevel;

    part def SoftwareComponent;
    part def DeployableComponent specializes SoftwareComponent;
    part def SoftwareModule specializes SoftwareComponent {
        attribute moduleName : Name;
    }
    part def ExternalSystem specializes SoftwareComponent {
        attribute systemName : Name;
    }

    interface def SoftwareInterface;
    interface def ApiInterface specializes SoftwareInterface;
    interface def AsyncInterface specializes SoftwareInterface;
    interface def CommandInterface specializes AsyncInterface;
    interface def QueryInterface specializes ApiInterface;

    item def DataType;
    item def MessageType specializes DataType;

    part def ExecutionEnvironment {
        attribute runtimeName : Name;
        attribute runtimeVersion : VersionString;
    }

    part def DeploymentNode {
        attribute nodeName : Name;
        attribute host : Hostname;
    }

    part def Environment {
        attribute environmentName : EnvironmentName;
    }
}
