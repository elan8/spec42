package EmbeddedComputeDomain {
    import SoftwareCore::*;
    import ElectronicsCore::*;

    attribute def FirmwareSlot;
    attribute def MemorySizeClass;
    attribute def ComputeClass;
    attribute def PeripheralClass;

    part def EmbeddedComputeUnit :> ElectronicsComponent {
        attribute unitName : Name;
        attribute computeClass : ComputeClass;
    }

    part def Microcontroller :> EmbeddedComputeUnit;

    part def SystemOnChip :> EmbeddedComputeUnit;

    part def FirmwareImageSlot {
        attribute slotName : FirmwareSlot;
        attribute version : VersionString;
    }

    part def MemoryBlock {
        attribute memoryName : Name;
        attribute sizeClass : MemorySizeClass;
    }

    part def PeripheralBlock {
        attribute peripheralName : Name;
        attribute peripheralClass : PeripheralClass;
    }
}
