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
    part def DeployableComponent :> SoftwareComponent;
    part def SoftwareModule :> SoftwareComponent {
        attribute moduleName : Name;
    }
    part def ExternalSystem :> SoftwareComponent {
        attribute systemName : Name;
    }

    interface def SoftwareInterface;
    interface def ApiInterface :> SoftwareInterface;
    interface def AsyncInterface :> SoftwareInterface;
    interface def CommandInterface :> AsyncInterface;
    interface def QueryInterface :> ApiInterface;

    item def DataType;
    item def MessageType :> DataType;

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
