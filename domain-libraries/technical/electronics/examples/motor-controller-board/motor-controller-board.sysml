package MotorControllerBoardExample {
    import ElectronicsCore::*;
    import ElectricalPowerDomain::*;
    import EmbeddedComputeDomain::*;
    import ElectronicIoDomain::*;
    import ElectronicBusDomain::*;
    import BoardIntegrationDomain::*;

    port def MotorPort;

    part def InputRail :> PowerRail {
        attribute :>> railName = "input-48v";
        attribute :>> voltage = "48V";
        attribute :>> currentBudget = "6A";
        attribute :>> powerClass = "input";
        port railOut : MotorPort;
    }

    part def ControlRail :> DcDcConverter {
        attribute :>> converterName = "control-12v-converter";
        attribute :>> efficiencyClass = "high";
        port converterIn : MotorPort;
        port converterOut : MotorPort;
    }

    part def MotorSoC :> SystemOnChip {
        attribute :>> unitName = "motor-control-soc";
        attribute :>> computeClass = "arm-cortex-a53";
        port powerIn : MotorPort;
        port busIo : MotorPort;
        port controlOut : MotorPort;
    }

    part def BootSlot :> FirmwareImageSlot {
        attribute :>> slotName = "boot-slot";
        attribute :>> version = "2.1.4";
        port firmwareOut : MotorPort;
    }

    part def PositionInput :> DigitalIoChannel {
        attribute :>> channelName = "encoder-index";
        attribute :>> direction = "input";
        attribute :>> signalKind = "digital";
        port ioOut : MotorPort;
    }

    part def GateDriveOutput :> DigitalIoChannel {
        attribute :>> channelName = "gate-drive-enable";
        attribute :>> direction = "output";
        attribute :>> signalKind = "digital";
        port ioIn : MotorPort;
    }

    part def DiagnosticBus :> UartBus {
        attribute :>> busName = "diagnostic-uart";
        attribute :>> busType = "UART";
        attribute :>> speedClass = "921600bps";
        attribute :>> topologyClass = "point-to-point";
        port busIn : MotorPort;
        port busOut : MotorPort;
    }

    part def ControllerBoard :> BoardAssembly {
        attribute :>> boardName = "motor-controller-main";
        attribute :>> layerCount = "6";
    }

    part def Harness :> WiringHarness {
        attribute :>> harnessName = "motor-phase-harness";
        port harnessIn : MotorPort;
    }

    part def MotorControllerSystem {
        part inputRail : InputRail;
        part controlRail : ControlRail;
        part motorSoC : MotorSoC;
        part bootSlot : BootSlot;
        part positionInput : PositionInput;
        part gateDriveOutput : GateDriveOutput;
        part diagnosticBus : DiagnosticBus;
        part controllerBoard : ControllerBoard;
        part harness : Harness;

        connect inputRail.railOut to controlRail.converterIn;
        connect controlRail.converterOut to motorSoC.powerIn;
        connect bootSlot.firmwareOut to motorSoC.busIo;
        connect positionInput.ioOut to motorSoC.busIo;
        connect motorSoC.controlOut to gateDriveOutput.ioIn;
        connect motorSoC.busIo to diagnosticBus.busIn;
        connect gateDriveOutput.ioIn to harness.harnessIn;
    }
}
