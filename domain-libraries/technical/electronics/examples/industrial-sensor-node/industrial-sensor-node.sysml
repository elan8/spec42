package IndustrialSensorNodeExample {
    import ElectronicsCore::*;
    import ElectricalPowerDomain::*;
    import EmbeddedComputeDomain::*;
    import ElectronicIoDomain::*;
    import ElectronicBusDomain::*;
    import BoardIntegrationDomain::*;

    port def NodePort;

    part def MainRail :> PowerRail {
        attribute :>> railName = "main-24v-rail";
        attribute :>> voltage = "24V";
        attribute :>> currentBudget = "1.2A";
        attribute :>> powerClass = "input";
        port railOut : NodePort;
    }

    part def LogicRegulator :> VoltageRegulator {
        attribute :>> regulatorName = "logic-5v-regulator";
        attribute :>> efficiencyClass = "high";
        port regulatorIn : NodePort;
        port regulatorOut : NodePort;
    }

    part def SensorMcu :> Microcontroller {
        attribute :>> unitName = "sensor-node-mcu";
        attribute :>> computeClass = "arm-cortex-m4";
        port powerIn : NodePort;
        port dataBus : NodePort;
        port analogIn : NodePort;
    }

    part def ApplicationSlot :> FirmwareImageSlot {
        attribute :>> slotName = "slot-a";
        attribute :>> version = "1.3.0";
        port firmwareIn : NodePort;
    }

    part def SensorFrontEnd :> AnalogIoChannel {
        attribute :>> channelName = "pressure-sensor-channel";
        attribute :>> direction = "input";
        attribute :>> signalKind = "analog";
        attribute :>> resolutionClass = "16-bit";
        attribute :>> samplingClass = "2khz";
        port sensorOut : NodePort;
    }

    part def FieldBus :> CanBus {
        attribute :>> busName = "field-can";
        attribute :>> busType = "CAN";
        attribute :>> speedClass = "500kbps";
        attribute :>> topologyClass = "multi-drop";
        port busIn : NodePort;
        port busOut : NodePort;
    }

    part def SensorBoard :> BoardAssembly {
        attribute :>> boardName = "sensor-node-board";
        attribute :>> layerCount = "4";
    }

    part def M12Connector :> BoardConnector {
        attribute :>> connectorName = "field-connector";
        attribute :>> connectorType = "M12";
        port connectorIn : NodePort;
        port connectorOut : NodePort;
    }

    part def SensorNodeSystem {
        part mainRail : MainRail;
        part logicRegulator : LogicRegulator;
        part sensorMcu : SensorMcu;
        part applicationSlot : ApplicationSlot;
        part sensorFrontEnd : SensorFrontEnd;
        part fieldBus : FieldBus;
        part sensorBoard : SensorBoard;
        part m12Connector : M12Connector;

        connect mainRail.railOut to logicRegulator.regulatorIn;
        connect logicRegulator.regulatorOut to sensorMcu.powerIn;
        connect sensorFrontEnd.sensorOut to sensorMcu.analogIn;
        connect applicationSlot.firmwareIn to sensorMcu.dataBus;
        connect sensorMcu.dataBus to fieldBus.busIn;
        connect fieldBus.busOut to m12Connector.connectorIn;
    }
}
