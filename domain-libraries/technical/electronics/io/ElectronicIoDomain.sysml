package ElectronicIoDomain {
    import SoftwareCore::*;
    import ElectronicsCore::*;

    attribute def IoDirection;
    attribute def ResolutionClass;
    attribute def SamplingClass;

    part def IoChannel {
        attribute channelName : Name;
        attribute direction : IoDirection;
        attribute signalKind : SignalKind;
    }

    part def DigitalIoChannel :> IoChannel;

    part def AnalogIoChannel :> IoChannel {
        attribute resolutionClass : ResolutionClass;
        attribute samplingClass : SamplingClass;
    }

    part def SensorElectricalInterface :> ElectricalInterface;

    part def ActuatorElectricalInterface :> ElectricalInterface;
}
