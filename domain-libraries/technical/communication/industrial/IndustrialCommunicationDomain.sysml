package IndustrialCommunicationDomain {
    import CommunicationCore::*;

    attribute def RealTimeClass;

    part def IndustrialNetwork :> CommunicationChannel {
        attribute realTimeClass : RealTimeClass;
    }

    part def ModbusNetwork :> IndustrialNetwork;
    part def ProfinetNetwork :> IndustrialNetwork;
    part def EthercatNetwork :> IndustrialNetwork;
}
