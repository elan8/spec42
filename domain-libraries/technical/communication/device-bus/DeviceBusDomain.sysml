package DeviceBusDomain {
    import CommunicationCore::*;

    attribute def BusSpeedClass;

    part def DeviceBusChannel :> CommunicationChannel {
        attribute speedClass : BusSpeedClass;
    }

    part def UsbBus :> DeviceBusChannel;
}
