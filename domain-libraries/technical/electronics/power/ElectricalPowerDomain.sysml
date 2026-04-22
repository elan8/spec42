package ElectricalPowerDomain {
    import SoftwareCore::*;
    import ElectronicsCore::*;

    attribute def VoltageValue;
    attribute def CurrentValue;
    attribute def PowerClass;
    attribute def EfficiencyClass;

    part def PowerRail :> ElectronicsComponent {
        attribute railName : Name;
        attribute voltage : VoltageValue;
        attribute currentBudget : CurrentValue;
        attribute powerClass : PowerClass;
    }

    part def VoltageRegulator :> ElectronicsComponent {
        attribute regulatorName : Name;
        attribute efficiencyClass : EfficiencyClass;
    }

    part def DcDcConverter :> ElectronicsComponent {
        attribute converterName : Name;
        attribute efficiencyClass : EfficiencyClass;
    }

    part def LoadGroup {
        attribute loadName : Name;
        attribute currentBudget : CurrentValue;
    }
}
