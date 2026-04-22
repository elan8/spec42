package ElectronicsCore {
    import SoftwareCore::*;

    attribute def SignalKind;
    attribute def VoltageClass;
    attribute def CurrentClass;
    attribute def TimingClass;
    attribute def PinCount;

    part def ElectronicsComponent :> SoftwareComponent {
        attribute componentName : Name;
    }

    part def ElectronicDevice :> ElectronicsComponent;

    part def ElectronicModule :> ElectronicsComponent {
        attribute moduleName : Name;
    }

    part def ElectricalInterface {
        attribute interfaceName : Name;
        attribute signalKind : SignalKind;
        attribute voltageClass : VoltageClass;
        attribute timingClass : TimingClass;
    }

    part def PinGroup {
        attribute groupName : Name;
        attribute pinCount : PinCount;
    }

    item def ElectricalSignal :> MessageType {
        attribute signalName : Name;
        attribute signalKind : SignalKind;
    }
}
