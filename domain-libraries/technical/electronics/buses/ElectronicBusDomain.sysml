package ElectronicBusDomain {
    import SoftwareCore::*;
    import ElectronicsCore::*;
    import CommunicationCore::*;

    attribute def BusType;
    attribute def BusSpeedClass;
    attribute def TopologyClass;

    part def ElectronicBus :> CommunicationChannel {
        attribute busName : Name;
        attribute busType : BusType;
        attribute speedClass : BusSpeedClass;
        attribute topologyClass : TopologyClass;
    }

    part def BusNode :> ElectronicsComponent {
        attribute nodeName : Name;
    }

    part def I2cBus :> ElectronicBus;
    part def SpiBus :> ElectronicBus;
    part def UartBus :> ElectronicBus;
    part def CanBus :> ElectronicBus;
}
