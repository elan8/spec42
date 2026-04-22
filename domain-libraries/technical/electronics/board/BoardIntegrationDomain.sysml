package BoardIntegrationDomain {
    import SoftwareCore::*;
    import ElectronicsCore::*;
    import ElectricalPowerDomain::*;
    import EmbeddedComputeDomain::*;
    import ElectronicIoDomain::*;
    import ElectronicBusDomain::*;

    attribute def ConnectorType;
    attribute def LayerCount;

    part def BoardAssembly :> ElectronicsComponent {
        attribute boardName : Name;
        attribute layerCount : LayerCount;
    }

    part def BoardConnector {
        attribute connectorName : Name;
        attribute connectorType : ConnectorType;
    }

    part def WiringHarness {
        attribute harnessName : Name;
    }

    part def BoardIntegration {
        part board : BoardAssembly;
    }
}
