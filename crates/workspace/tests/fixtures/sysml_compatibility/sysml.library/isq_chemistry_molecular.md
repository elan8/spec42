# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQChemistryMolecular
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQChemistryMolecular {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-9:2019 "Physical chemistry and molecular physics"
     * see also https://www.iso.org/standard/64979.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;

    /* Quantity definitions referenced from other ISQ packages */
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-9 item 9-1 number of entities */
    attribute numberOfEntities: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 9-1 number of entities
         * symbol(s): `N(X)`, `N_X`
         * application domain: generic
         * name: NumberOfEntities (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of elementary entities of kind `X` in a system
         * remarks: The elementary entities must be specified and can be atoms, molecules, ions, electrons, other particle, or a specified group of such particles. It is important to always give a precise specification of the entity involved; this should preferably be done by the empirical chemical formula of the material involved.
         */
    }

    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    /* See package ISQBase for the declarations of AmountOfSubstanceValue and AmountOfSubstanceUnit */

    alias NumberOfMolesUnit for AmountOfSubstanceUnit;
    alias NumberOfMolesValue for AmountOfSubstanceValue;
    alias numberOfMoles for amountOfSubstance;

    /* ISO-80000-9 item 9-3 relative atomic mass */
    attribute def RelativeAtomicMassValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-3 relative atomic mass
         * symbol(s): `A_r(X)`
         * application domain: generic
         * name: RelativeAtomicMass (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the average mass (ISO 80000-4) of atom `X` and the unified atomic mass (ISO 80000-10)
         * remarks: A similar quantity "relative molecular mass" can be defined for molecules. EXAMPLE `A_r(Cl) ~~ 35.453` `A_r(CO_2) ~~ 44` The relative atomic or relative molecular mass depends on the nuclidic composition. The International Union of Pure and Applied Chemistry (IUPAC) accepts the use of the special names "atomic weight" and "molecular weight" for the quantities "relative atomic mass" and "relative molecular mass", respectively. The use of these traditional names is deprecated.
         */
    }
    attribute relativeAtomicMass: RelativeAtomicMassValue :> scalarQuantities;

    /* ISO-80000-9 item 9-4 molar mass */
    attribute def MolarMassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-4 molar mass
         * symbol(s): `M(X)`
         * application domain: generic
         * name: MolarMass
         * quantity dimension: M^1*N^-1
         * measurement unit(s): g/mol, kg*mol^-1
         * tensor order: 0
         * definition: for a pure substance `X`, quotient of mass `m(X)` (ISO 80000-4) and amount `n` of substance (item 9-2): `M = m/n`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarMassUnit[1];
    }

    attribute molarMass: MolarMassValue[*] nonunique :> scalarQuantities;

    attribute def MolarMassUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-5 molar volume */
    attribute def MolarVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-5 molar volume
         * symbol(s): `V_m`
         * application domain: generic
         * name: MolarVolume
         * quantity dimension: L^3*N^-1
         * measurement unit(s): m^3*mol^-1
         * tensor order: 0
         * definition: for a pure substance, quotient of its volume `V` (ISO 80000-3) and amount `n` of substance (item 9-2): `V_m = V/n`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarVolumeUnit[1];
    }

    attribute molarVolume: MolarVolumeValue[*] nonunique :> scalarQuantities;

    attribute def MolarVolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.1 molar internal energy */
    attribute def MolarInternalEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.1 molar internal energy
         * symbol(s): `U_m`
         * application domain: generic
         * name: MolarInternalEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of internal energy `U` (ISO 80000-5) and amount `n` of substance (item 9-2): `U_m = U/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarInternalEnergyUnit[1];
    }

    attribute molarInternalEnergy: MolarInternalEnergyValue[*] nonunique :> scalarQuantities;

    attribute def MolarInternalEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.2 molar enthalpy */
    attribute def MolarEnthalpyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.2 molar enthalpy
         * symbol(s): `H_m`
         * application domain: generic
         * name: MolarEnthalpy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of enthalpy `H` (ISO 80000-5) and amount `n` of substance (item 9-2): `H_m = H/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarEnthalpyUnit[1];
    }

    attribute molarEnthalpy: MolarEnthalpyValue[*] nonunique :> scalarQuantities;

    attribute def MolarEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.3 molar Helmholtz energy */
    attribute def MolarHelmholtzEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.3 molar Helmholtz energy
         * symbol(s): `F_m`
         * application domain: generic
         * name: MolarHelmholtzEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Helmholtz energy `F` (ISO 80000-5) and amount `n` of substance (item 9-2): `F_m = F/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarHelmholtzEnergyUnit[1];
    }

    attribute molarHelmholtzEnergy: MolarHelmholtzEnergyValue[*] nonunique :> scalarQuantities;

    attribute def MolarHelmholtzEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.4 molar Gibbs energy */
    attribute def MolarGibbsEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.4 molar Gibbs energy
         * symbol(s): `G_m`
         * application domain: generic
         * name: MolarGibbsEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Gibbs energy `G` (ISO 80000-5) and amount `n` of substance (item 9-2): `G_m = G/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarGibbsEnergyUnit[1];
    }

    attribute molarGibbsEnergy: MolarGibbsEnergyValue[*] nonunique :> scalarQuantities;

    attribute def MolarGibbsEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-7 molar heat capacity */
    attribute def MolarHeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-7 molar heat capacity
         * symbol(s): `C_m`
         * application domain: generic
         * name: MolarHeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of heat capacity `C` (ISO 80000-5) and amount of substance `n` (item 9-2): `C_m = C/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarHeatCapacityUnit[1];
    }

    attribute molarHeatCapacity: MolarHeatCapacityValue[*] nonunique :> scalarQuantities;

    attribute def MolarHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-8 molar entropy */
    attribute def MolarEntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-8 molar entropy
         * symbol(s): `S_m`
         * application domain: generic
         * name: MolarEntropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of entropy `S` (ISO 80000-5) and amount `n` of substance (item 9-2): `S_m = S/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarEntropyUnit[1];
    }

    attribute molarEntropy: MolarEntropyValue[*] nonunique :> scalarQuantities;

    attribute def MolarEntropyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-9.1 particle concentration */
    attribute def ParticleConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-9.1 particle concentration
         * symbol(s): `n`, `(C)`
         * application domain: generic
         * name: ParticleConcentration
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number `N` of particles (item 9-1) and volume `V `(ISO 80000-3): `n = N/V`
         * remarks: The term "number density" is also used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleConcentrationUnit[1];
    }

    attribute particleConcentration: ParticleConcentrationValue[*] nonunique :> scalarQuantities;

    attribute def ParticleConcentrationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-9 item 9-9.2 molecular concentration */
    attribute molecularConcentration: ParticleConcentrationValue :> scalarQuantities {
        doc
        /*
         * source: item 9-9.2 molecular concentration
         * symbol(s): `C(X)`, `C_X`
         * application domain: generic
         * name: MolecularConcentration (specializes ParticleConcentration)
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of number `N_X` of molecules of substance `X` and volume `V` (ISO 80000-3) of the mixture: `C_X = N_X/V`
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-10 mass concentration */
    attribute def MassConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-10 mass concentration
         * symbol(s): `γ_X`, `(ρ_X)`
         * application domain: generic
         * name: MassConcentration
         * quantity dimension: L^-3*M^1
         * measurement unit(s): g/l, kg*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and volume `V` (ISO 80000-3) of the mixture: `γ_X = m_X/V`
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassConcentrationUnit[1];
    }

    attribute massConcentration: MassConcentrationValue[*] nonunique :> scalarQuantities;

    attribute def MassConcentrationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-9 item 9-11 mass fraction */
    attribute def MassFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-11 mass fraction
         * symbol(s): `w_X`
         * application domain: generic
         * name: MassFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and total mass `m` of the mixture: `w_X = m_X/m`
         * remarks: None.
         */
    }
    attribute massFraction: MassFractionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-12.1 amount-of-substance concentration */
    attribute def AmountOfSubstanceConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-12.1 amount-of-substance concentration
         * symbol(s): `c_X`
         * application domain: generic
         * name: AmountOfSubstanceConcentration
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount `n_X` of substance (item 9-2) of `X` and volume `V` (ISO 80000-3) of the mixture: `c_X = n_X/V`
         * remarks: In chemistry, the name "amount-of-substance concentration" is generally abbreviated to the single word "concentration", it being assumed that the adjective "amount-of-substance" is intended. For this reason, however, the word "mass" should never be omitted from the name "mass concentration" in item 9-10. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AmountOfSubstanceConcentrationUnit[1];
    }

    attribute amountOfSubstanceConcentration: AmountOfSubstanceConcentrationValue[*] nonunique :> scalarQuantities;

    attribute def AmountOfSubstanceConcentrationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-12.2 standard amount-of-substance concentration */
    attribute standardAmountOfSubstanceConcentration: AmountOfSubstanceConcentrationValue :> scalarQuantities {
        doc
        /*
         * source: item 9-12.2 standard amount-of-substance concentration
         * symbol(s): `c^!(X)`
         * application domain: generic
         * name: StandardAmountOfSubstanceConcentration (specializes AmountOfSubstanceConcentration)
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X`, one mole per litre
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
    }

    /* ISO-80000-9 item 9-13 amount-of-substance fraction mole fraction */
    attribute def AmountOfSubstanceFractionMoleFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-13 amount-of-substance fraction mole fraction
         * symbol(s): `x_X`, `y_X`
         * application domain: generic
         * name: AmountOfSubstanceFractionMoleFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount of substance `n_X` (item 9-2) of `X` and total amount `n` of substance (item 9-2) in the mixture: `x_X = n_X/n`
         * remarks: For condensed phases, `x_X` is used, and for gaseous mixtures `y_X` may be used. The unsystematic name "mole fraction" is still used. However, the use of this name is deprecated. For this quantity, the entity used to define the amount of substance should always be a single molecule for every species in the mixture.
         */
    }
    attribute amountOfSubstanceFractionMoleFraction: AmountOfSubstanceFractionMoleFractionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-14 volume fraction */
    attribute def VolumeFractionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-14 volume fraction
         * symbol(s): `φ_X`
         * application domain: generic
         * name: VolumeFraction
         * quantity dimension: 1
         * measurement unit(s): ml/l, 1
         * tensor order: 0
         * definition: for substance `X`, quotient of product of amount of substance fraction `x_X` (item 9-13) of `X` and molar volume `V_(m,X)` (item 9-5) of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4), and sum over all substances `i` of products of amount-of-substance fractions `x_i` (item 9-13) of substance `i` and their molar volumes `V_(m,i)` (item 9-5): `φ_X = (x_X V_(m,X))/(sum_i x_i V_(m,i))`
         * remarks: Generally, the volume fraction is temperature dependent. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumeFractionUnit[1];
    }

    attribute volumeFraction: VolumeFractionValue[*] nonunique :> scalarQuantities;

    attribute def VolumeFractionUnit :> DimensionOneUnit {
    }

    /* ISO-80000-9 item 9-15 molality */
    attribute def MolalityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-15 molality
         * symbol(s): `b_B`, `m_B`
         * application domain: generic
         * name: Molality
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol/kg
         * tensor order: 0
         * definition: quotient of amount of substance (item 9-2) of solute `B` and mass `m_A` (ISO 80000-4) of the solvent substance `A`: `b_B = n_B/m_A`
         * remarks: The alternative symbol `m_B` should be avoided in situations where it might be mistaken for the mass of substance B. However, the symbol `m_B` is much more commonly used than the symbol `b_B` for molality, despite the possible confusion with mass.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolalityUnit[1];
    }

    attribute molality: MolalityValue[*] nonunique :> scalarQuantities;

    attribute def MolalityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-16 latent heat of phase transition, enthalpy of phase transition */
    attribute latentHeatOfPhaseTransition: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 9-16 latent heat of phase transition, enthalpy of phase transition
         * symbol(s): `C_"pt"`
         * application domain: generic
         * name: LatentHeatOfPhaseTransition (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) necessary to be added or subtracted isothermally and isobarically to a system to completely undergo the phase transition
         * remarks: Mostly, molar or specific quantity is used and phase transition is expressed explicitly, e.g. molar latent heat of evaporation. The subscript "pt" is the qualifier for the phase transition, which may be changed to e.g. "l-g". The term "enthalpy of phase transition" is mainly used in theory.
         */
    }

    alias enthalpyOfPhaseTransition for latentHeatOfPhaseTransition;

    /* ISO-80000-9 item 9-17 chemical potential */
    attribute def ChemicalPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-17 chemical potential
         * symbol(s): `μ_X`
         * application domain: chemistry
         * name: ChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: partial derivative of the Gibbs energy (ISO 80000-5) with respect to amount `n_X` of substance `X` (item 9-2) at constant temperature `T` (ISO 80000-5) and pressure `p `(ISO 80000-4): `μ_X = ((del G)/(del n_X))_(T,p)`
         * remarks: For a pure substance, where `G_m` is the molar Gibbs energy. In a mixture, `μ_B` is the partial molar Gibbs energy. In condensed matter physics, the chemical potential of electrons is energy.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ChemicalPotentialUnit[1];
    }

    attribute chemicalPotential: ChemicalPotentialValue[*] nonunique :> scalarQuantities;

    attribute def ChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-18 absolute activity */
    attribute def AbsoluteActivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-18 absolute activity
         * symbol(s): `λ_X`
         * application domain: generic
         * name: AbsoluteActivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X`, exponential of quotient of chemical potential `μ_X` of substance `B` (item 9-17), and product of molar gas constant `R` (item 9-37.1) and thermodynamic temperature `T` (ISO 80000-5): `λ_X = exp(μ_X/(RT))`
         * remarks: None.
         */
    }
    attribute absoluteActivity: AbsoluteActivityValue :> scalarQuantities;

    /* ISO-80000-9 item 9-19 partial pressure */
    attribute def PartialPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-19 partial pressure
         * symbol(s): `p_X`
         * application domain: generic
         * name: PartialPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X` in a gaseous mixture, product of amount-of-substance fraction `y_X` of substance X (item 9-13) and total pressure `p` (ISO 80000-4): `p_X = y_X p`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PartialPressureUnit[1];
    }

    attribute partialPressure: PartialPressureValue[*] nonunique :> scalarQuantities;

    attribute def PartialPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-20 fugacity */
    attribute def FugacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-20 fugacity
         * symbol(s): `tilde(p)_X`
         * application domain: generic
         * name: Fugacity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X`, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) only, being determined by the condition that, at constant temperature and composition, `p_X/tilde(p)_X` tends to 1 for an indefinitely dilute gas
         * remarks: `tilde(p)_X = λ_X * lim_(p->0) (p_X/λ_X)` where `p` is total pressure (ISO 80000-4). The IUPAC preferred symbol for fugacity is `f`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FugacityUnit[1];
    }

    attribute fugacity: FugacityValue[*] nonunique :> scalarQuantities;

    attribute def FugacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-21 standard chemical potential */
    attribute def StandardChemicalPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-21 standard chemical potential
         * symbol(s): `μ_B^!`, `μ^!`
         * application domain: generic
         * name: StandardChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: for substance `B`, value of the chemical potential (item 9-17) at specified standard conditions
         * remarks: `μ_B^! = RT ln(λ^!)` where `μ_B^!` is a function of temperature `T` at the standard pressure `p = p^!` The standard chemical potential depends on the choice of standard state, which must be specified. In a liquid or solid solution, the standard state is referenced to the ideal dilute behaviour of the solute (substance `B`).
         */
        attribute :>> num: Real;
        attribute :>> mRef: StandardChemicalPotentialUnit[1];
    }

    attribute standardChemicalPotential: StandardChemicalPotentialValue[*] nonunique :> scalarQuantities;

    attribute def StandardChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-22 activity factor */
    attribute def ActivityFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-22 activity factor
         * symbol(s): `f_X`
         * application domain: generic
         * name: ActivityFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, quotient of absolute activity `λ_X` (item 9-18) of substance `X` and the product of absolute activity `λ_X^"*"` of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4) and amount-of-substance fraction `x_X` of substance `X` (item 9-13): `f_X = λ_X/(λ_X^"*" x_X)`
         * remarks: The systematic name is "activity factor", but the name "activity coefficient" is also commonly used (see item 9-25). Activity factors can also be obtained applying Raoult’s law or Henry’s law.
         */
    }
    attribute activityFactor: ActivityFactorValue :> scalarQuantities;

    /* ISO-80000-9 item 9-23 standard absolute activity in mixture */
    attribute def StandardAbsoluteActivityInMixtureValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-23 standard absolute activity in mixture
         * symbol(s): `λ_X^!`
         * application domain: in a mixture
         * name: StandardAbsoluteActivityInMixture (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, absolute activity `λ_X^"*"` (item 9-18) of the pure substance `X` at the same temperature (ISO 80000-5) but at standard pressure (ISO 80000-4) `10^5 ["Pa"]`: `λ_X^! = λ_X"*" (p^!)`
         * remarks: This quantity is a function of temperature only.
         */
    }
    attribute standardAbsoluteActivityInMixture: StandardAbsoluteActivityInMixtureValue :> scalarQuantities;

    /* ISO-80000-9 item 9-24 activity of solute, relative activity of solute */
    attribute def ActivityOfSoluteValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-24 activity of solute, relative activity of solute
         * symbol(s): `a_X`, `a_(m,X)`
         * application domain: generic
         * name: ActivityOfSolute (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `X` in a solution, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) and pressure (ISO 80000-4) only, being determined by the condition that, at constant temperature and pressure, `a_X` divided by the molality (item 9-15) ratio, `b_X/b^!` tends to 1 at infinite dilution; `b_X` is the molality of solute `X` (item 9-15), and `b^!` is standard molality: `a_X = λ_X * lim_(sum b_X -> 0) (b_X//b^!)/λ_X`
         * remarks: The quantity `a_(c,X)` , similarly defined in terms of the concentration ratio `c_X/c^!` , is also called the activity or relative activity of solute `X`; `c^!` is a standard amount-of-substance concentration (item 9-12.2): `a_(c,X) = λ_X * lim_(sum c_X -> 0) (c_X//c^!)/λ_X`, where `sum` denotes summation over all the solute substances. This especially applies to a dilute liquid solution.
         */
    }
    attribute activityOfSolute: ActivityOfSoluteValue :> scalarQuantities;

    alias relativeActivityOfSolute for activityOfSolute;

    /* ISO-80000-9 item 9-25 activity coefficient */
    attribute def ActivityCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-25 activity coefficient
         * symbol(s): `γ_B`
         * application domain: generic
         * name: ActivityCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution, quotient of activity `a_B` of solute `B` (item 9-24), and quotient of the molality (item 9-15) `b_B` of substance `B` and standard molality `b^!`: `γ_B = a_B/(b_B//b^!)`
         * remarks: The name "activity coefficient of solute B" is also used for the quantity `γ_B` defined as: `γ_B = a_(c,B)/(c_B//c^!)` See item 9-22.
         */
    }
    attribute activityCoefficient: ActivityCoefficientValue :> scalarQuantities;

    /* ISO-80000-9 item 9-26 standard absolute activity in solution */
    attribute def StandardAbsoluteActivityInSolutionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-26 standard absolute activity in solution
         * symbol(s): `λ_B^!`
         * application domain: in a solution
         * name: StandardAbsoluteActivityInSolution (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution: `λ_B^! = lim_(sum b_B -> 0) [λ_B ((p^!)b^!)/b_B]` where ∑ denotes summation over all solutes, `p^!` is a standard pressure (ISO 80000-4), `b^!` is standard molality, and `b_B` is the molality of substance `B` (item 9-15)
         * remarks: This quantity is a function of temperature only. It especially applies to a dilute liquid solution. The standard pressure is `10^5 ["Pa"]`.
         */
    }
    attribute standardAbsoluteActivityInSolution: StandardAbsoluteActivityInSolutionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-27.1 activity of solvent, relative activity of solvent */
    attribute def ActivityOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.1 activity of solvent, relative activity of solvent
         * symbol(s): `a_A`
         * application domain: generic
         * name: ActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the solvent `A` in a solution, quotient of the absolute activity of substance `A`, `λ_A` (item 9-18), and that, `λ_A^"*"` , of the pure solvent at the same temperature (ISO 80000-5) and pressure (ISO 80000-4): `a_A = λ_A/λ_A^"*"`
         * remarks: None.
         */
    }
    attribute activityOfSolvent: ActivityOfSolventValue :> scalarQuantities;

    alias relativeActivityOfSolvent for activityOfSolvent;

    /* ISO-80000-9 item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A */
    attribute def OsmoticFactorOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A
         * symbol(s): `φ`
         * application domain: generic
         * name: OsmoticFactorOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `φ = -(M_A sum b_B)^-1 ln(a_A)` where `M_A` is the molar mass (item 9-4) of the solvent A, ∑ denotes summation over all the solutes, `b_B` is the molality of solute B (item 9-15), and `a_A` is the activity of solvent A (item 9-27.1)
         * remarks: The name "osmotic coefficient" is generally used, although the name "osmotic factor" is more systematic. This concept especially applies to a dilute liquid solution.
         */
    }
    attribute osmoticFactorOfSolvent: OsmoticFactorOfSolventValue :> scalarQuantities;

    alias osmoticCoefficientOfSolventA for osmoticFactorOfSolvent;

    /* ISO-80000-9 item 9-27.3 standard absolute activity of solvent */
    attribute def StandardAbsoluteActivityOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.3 standard absolute activity of solvent
         * symbol(s): `λ_A^!`
         * application domain: in a dilute solution
         * name: StandardAbsoluteActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for solvent `A`, standard absolute activity (item 9-23) of the pure substance `A` at the same temperature (ISO 80000-5) and at a standard pressure `p^!` (ISO 80000-4): `λ_A^! = λ_A^"*" p^!`
         * remarks: None.
         */
    }
    attribute standardAbsoluteActivityOfSolvent: StandardAbsoluteActivityOfSolventValue :> scalarQuantities;

    /* ISO-80000-9 item 9-28 osmotic pressure */
    attribute def OsmoticPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-28 osmotic pressure
         * symbol(s): `Π`
         * application domain: generic
         * name: OsmoticPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: excess pressure (ISO 80000-4) required to maintain osmotic equilibrium between a solution and the pure solvent separated by a membrane permeable to the solvent only
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: OsmoticPressureUnit[1];
    }

    attribute osmoticPressure: OsmoticPressureValue[*] nonunique :> scalarQuantities;

    attribute def OsmoticPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-29 stoichiometric number of substance */
    attribute def StoichiometricNumberOfSubstanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-29 stoichiometric number of substance
         * symbol(s): `ν_B`
         * application domain: generic
         * name: StoichiometricNumberOfSubstance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `B`, an integer number or a simple fraction, being negative for a reactant and positive for a product, occurring in the expression for a chemical reaction: `0 = sum ν_B` where the symbol `B` denotes the reactants and products involved in the reaction
         * remarks: EXAMPLE `(1/2)"N"_2 + (3/2)"H"_2 = "N""H"_3` ; `ν("N"_2) = -1/2`, `ν("H"_2) = -3/2`, `ν("N""H"_3) = +1`.
         */
    }
    attribute stoichiometricNumberOfSubstance: StoichiometricNumberOfSubstanceValue :> scalarQuantities;

    /* ISO-80000-9 item 9-30 affinity of a chemical reaction */
    attribute def AffinityOfAChemicalReactionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-30 affinity of a chemical reaction
         * symbol(s): `A`
         * application domain: generic
         * name: AffinityOfAChemicalReaction
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: negative of the sum over all substances `B` of products of stoichiometric number `ν_B` of substance `B` (item 9-29) and chemical potential `μ_B` of substance `B` (item 9-17): `A = -sum ν_B μ_B`
         * remarks: The affinity of a reaction is a measure of the "driving force" of the reaction. When it is positive, the reaction goes spontaneously from reactants to products, and when it is negative, the reaction goes in the opposite direction. Another way to write the definition is: `A = ((del G)/(del ξ))_(p,T)` where `G` is Gibbs energy (ISO 80000-5) and `ξ` is the extent of the reaction (item 9-31). Note that `ν_B` is negative for reactants and positive for products.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AffinityOfAChemicalReactionUnit[1];
    }

    attribute affinityOfAChemicalReaction: AffinityOfAChemicalReactionValue[*] nonunique :> scalarQuantities;

    attribute def AffinityOfAChemicalReactionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-31 extent of reaction */
    attribute extentOfReaction: AmountOfSubstanceValue :> scalarQuantities {
        doc
        /*
         * source: item 9-31 extent of reaction
         * symbol(s): `ξ`
         * application domain: generic
         * name: ExtentOfReaction (specializes AmountOfSubstance)
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: difference of initial amount `n_(B "in")` of substance `B` (item 9-2) and equilibrium amount `n_(B "eq")` of substance `B` (item 9-2) divided by stoichiometric number `ν_B` of substance `B` (item 9-29): `ξ = (n_(B "eq") - n_(B "in"))/ν_B`
         * remarks: See remark to item 9-30.
         */
    }

    /* ISO-80000-9 item 9-32 standard equilibrium constant, thermodynamic equilibrium constant */
    attribute def StandardEquilibriumConstantValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-32 standard equilibrium constant, thermodynamic equilibrium constant
         * symbol(s): `K^!`
         * application domain: generic
         * name: StandardEquilibriumConstant (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a chemical reaction, product for all substances `B` of standard absolute activity `λ_B^!` of substance `B` (item 9-26) in power of minus stoichiometric number `ν_B` of substance `B` (item 9-29): `K^! = prod_B (tilde(a) λ_B^!)^(-ν_B)`
         * remarks: This quantity is a function of temperature only. Others depend on temperature, pressure, and composition. One can define in an analogous way an equilibrium constant in terms of fugacity, `K_f`, molality, `K_m`, etc.
         */
    }
    attribute standardEquilibriumConstant: StandardEquilibriumConstantValue :> scalarQuantities;

    alias thermodynamicEquilibriumConstant for standardEquilibriumConstant;

    /* ISO-80000-9 item 9-33 equilibrium constant on pressure basis */
    attribute def EquilibriumConstantOnPressureBasisValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-33 equilibrium constant on pressure basis
         * symbol(s): `K_p`
         * application domain: pressure basis
         * name: EquilibriumConstantOnPressureBasis
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for gases, product for all substances `B` of partial pressure `p_B` of substance `B` (item 9-19) in power of its stoichiometric number `ν_B` (item 9-29): `K_p = prod_B (p_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EquilibriumConstantOnPressureBasisUnit[1];
    }

    attribute equilibriumConstantOnPressureBasis: EquilibriumConstantOnPressureBasisValue[*] nonunique :> scalarQuantities;

    attribute def EquilibriumConstantOnPressureBasisUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-34 equilibrium constant on concentration basis */
    attribute def EquilibriumConstantOnConcentrationBasisValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-34 equilibrium constant on concentration basis
         * symbol(s): `K_c`
         * application domain: concentration basis
         * name: EquilibriumConstantOnConcentrationBasis
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/m^3
         * tensor order: 0
         * definition: for solutions, product for all substances `B` of concentration `c_B` of substance `B` (item 9-9.1) in power of its stoichiometric number `ν_B` (item 9-29): `K_c = prod_B (c_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EquilibriumConstantOnConcentrationBasisUnit[1];
    }

    attribute equilibriumConstantOnConcentrationBasis: EquilibriumConstantOnConcentrationBasisValue[*] nonunique :> scalarQuantities;

    attribute def EquilibriumConstantOnConcentrationBasisUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-35.1 microcanonical partition function */
    attribute microcanonicalPartitionFunction: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 9-35.1 microcanonical partition function
         * symbol(s): `Ω`
         * application domain: generic
         * name: MicrocanonicalPartitionFunction (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of all quantum states `r` consistent with given energy `E` (ISO 80000-4), volume (ISO 80000-3), and external fields: `Ω = sum_r 1`
         * remarks: `S = k ln(Ω)` where `S` is entropy (ISO 80000-5) and `k` is the Boltzmann constant (ISO 80000-1).
         */
    }

    /* ISO-80000-9 item 9-35.2 canonical partition function */
    attribute def CanonicalPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.2 canonical partition function
         * symbol(s): `Z`
         * application domain: generic
         * name: CanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum over quantum states of energy `E_r` (ISO 80000-4), expressed by: `Z = sum_r exp(-E_r/(kT))` where `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: `A = -kT ln(Z)` where `A` is Helmholtz energy (ISO 80000-5).
         */
    }
    attribute canonicalPartitionFunction: CanonicalPartitionFunctionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-35.3 grand-canonical partition function, grand partition function */
    attribute def GrandCanonicalPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.3 grand-canonical partition function, grand partition function
         * symbol(s): `Ξ`
         * application domain: generic
         * name: GrandCanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum of canonical partition function `Z(N_A,N_B,…)` for the given number of particles `A,B` multiplied by absolute activities (item 9-18) `λ_A, λ_B, ...` of particles `A, B`: `Ξ = sum_(N_A, N_B, ...) Z(N_A, N_B, …) * λ_A^(N_A) * λ_B^(N_B) * ...`
         * remarks: `A - sum μ_B n_B = -kT ln(Ξ)` where `A` is Helmholtz energy (ISO 80000-5), `μ_B` is the chemical potential of substance `B`, and `n_B` is the amount of substance `B`.
         */
    }
    attribute grandCanonicalPartitionFunction: GrandCanonicalPartitionFunctionValue :> scalarQuantities;

    alias grandPartitionFunction for grandCanonicalPartitionFunction;

    /* ISO-80000-9 item 9-35.4 molecular partition function, partition function of a molecule */
    attribute def MolecularPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.4 molecular partition function, partition function of a molecule
         * symbol(s): `q`
         * application domain: generic
         * name: MolecularPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `q = sum_r exp(-ε_r/(kT))` where `ε_r` is the energy (ISO 80000-5) of the `r`-th level of the molecule consistent with given volume (ISO 80000-3) and external fields, `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute molecularPartitionFunction: MolecularPartitionFunctionValue :> scalarQuantities;

    alias partitionFunctionOfAMolecule for molecularPartitionFunction;

    /* ISO-80000-9 item 9-36.1 statistical weight of subsystem */
    attribute statisticalWeightOfSubsystem: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 9-36.1 statistical weight of subsystem
         * symbol(s): `g`
         * application domain: generic
         * name: StatisticalWeightOfSubsystem (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of different microstates in a subsystem
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-36.2 degeneracy, multiplicity */
    attribute def DegeneracyValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-36.2 degeneracy, multiplicity
         * symbol(s): `g`
         * application domain: generic
         * name: Degeneracy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for quantum level, statistical weight of that level
         * remarks: If `g = 1`, the level is called non-degenerate.
         */
    }
    attribute degeneracy: DegeneracyValue :> scalarQuantities;

    alias multiplicity for degeneracy;

    /* ISO-80000-9 item 9-37.1 molar gas constant */
    attribute def MolarGasConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-37.1 molar gas constant
         * symbol(s): `R`
         * application domain: generic
         * name: MolarGasConstant
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: product of the Boltzmann constant (ISO 80000-1) and the Avogadro constant (ISO 80000-1)
         * remarks: For an ideal gas, `pV_m = RT`
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarGasConstantUnit[1];
    }

    attribute molarGasConstant: MolarGasConstantValue[*] nonunique :> scalarQuantities;

    attribute def MolarGasConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-37.2 specific gas constant */
    /* Refer to declaration for SpecificGasConstant in ISQThermodynamics item 5-26 specific gas constant */

    /* ISO-80000-9 item 9-38 mean free path */
    attribute meanFreePath: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 9-38 mean free path
         * symbol(s): `l`, `λ`
         * application domain: chemistry
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: for a particle, the average distance `d` (ISO 80000-3) between two successive collisions with other particles
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-39 diffusion coefficient */
    attribute def DiffusionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-39 diffusion coefficient
         * symbol(s): `D`
         * application domain: chemistry
         * name: DiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: proportionality coefficient of local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture multiplied by the local average velocity (ISO 80000-3) `v_B` of the molecules of `B`, and minus the gradient of the local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture, expressed by: `C_B(v_B) = -D grad C_B`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DiffusionCoefficientUnit[1];
    }

    attribute diffusionCoefficient: DiffusionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def DiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-9 item 9-40.1 thermal diffusion ratio */
    attribute def ThermalDiffusionRatioValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-40.1 thermal diffusion ratio
         * symbol(s): `k_T`
         * application domain: generic
         * name: ThermalDiffusionRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a steady-state of a binary mixture in which thermal diffusion occurs, proportionality factor between gradient of the amount-of-subsstance fraction `x_B` (item 9-13) of the heavier substance `B`, and negative gradient of the local thermodynamic temperature `T` (ISO 80000-5) divided by that temperature (ISO 80000-5): `grad x_B = -(k_T/T) grad T`
         * remarks: None.
         */
    }
    attribute thermalDiffusionRatio: ThermalDiffusionRatioValue :> scalarQuantities;

    /* ISO-80000-9 item 9-40.2 thermal diffusion factor */
    attribute def ThermalDiffusionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-40.2 thermal diffusion factor
         * symbol(s): `α_T`
         * application domain: generic
         * name: ThermalDiffusionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the thermal diffusion ratio `k_T` (item 9-40.1), and the product of the local amount-of-substance fractions `x_A`, `x_B` (item 9-13) of two substances `A` and `B`: `α_T = k_T//(x_A x_B)`
         * remarks: None.
         */
    }
    attribute thermalDiffusionFactor: ThermalDiffusionFactorValue :> scalarQuantities;

    /* ISO-80000-9 item 9-41 thermal diffusion coefficient */
    attribute def ThermalDiffusionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-41 thermal diffusion coefficient
         * symbol(s): `D_T`
         * application domain: generic
         * name: ThermalDiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: product of the thermal diffusion ratio `k_T` (item 9-40.1) and the diffusion coefficient `D` (item 9-39): `D_T = k_T*D`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalDiffusionCoefficientUnit[1];
    }

    attribute thermalDiffusionCoefficient: ThermalDiffusionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def ThermalDiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-9 item 9-42 ionic strength */
    attribute def IonicStrengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-42 ionic strength
         * symbol(s): `I`
         * application domain: generic
         * name: IonicStrength
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol*kg^-1
         * tensor order: 0
         * definition: in a sample, one half of the sum of square of the charge number `z_i` (ISO 80000-10) of `i`-th ion multiplied by its molality `b_i` (item 9-15) over any involved ion: `I = 1/2 sum z_i^2 b_i`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IonicStrengthUnit[1];
    }

    attribute ionicStrength: IonicStrengthValue[*] nonunique :> scalarQuantities;

    attribute def IonicStrengthUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-43 degree of dissociation, dissociation fraction */
    attribute def DegreeOfDissociationValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-43 degree of dissociation, dissociation fraction
         * symbol(s): `α`
         * application domain: generic
         * name: DegreeOfDissociation (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a sample, quotient of the number `n_d` of dissociated molecules and the total number `n` of molecules: `α = n_D / n`
         * remarks: None.
         */
    }
    attribute degreeOfDissociation: DegreeOfDissociationValue :> scalarQuantities;

    alias dissociationFraction for degreeOfDissociation;

    /* ISO-80000-9 item 9-44 electrolytic conductivity */
    attribute def ElectrolyticConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-44 electrolytic conductivity
         * symbol(s): `κ`
         * application domain: generic
         * name: ElectrolyticConductivity
         * quantity dimension: L^-3*M^-1*T^3*I^2
         * measurement unit(s): S/m, kg^-1*m^-3*s^3*A^2
         * tensor order: 0
         * definition: quotient of the magnitude of electric current density `J` (IEC 80000-6) and the magnitude electric field strength `E` (IEC 80000-6) in an electrolyte: `κ = J/E`
         * remarks: For anisotropic media, `κ` is a tensor. In IEC 80000-6 the symbols `σ`, `γ` are used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectrolyticConductivityUnit[1];
    }

    attribute electrolyticConductivity: ElectrolyticConductivityValue[*] nonunique :> scalarQuantities;

    attribute def ElectrolyticConductivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-9 item 9-45 molar conductivity */
    attribute def MolarConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-45 molar conductivity
         * symbol(s): `Λ_m`
         * application domain: generic
         * name: MolarConductivity
         * quantity dimension: M^-1*T^3*I^2*N^-1
         * measurement unit(s): S*m^2/mol, kg^-1*s^3*A^2*mol^-1
         * tensor order: 0
         * definition: in an electrolyte, quotient of electrolytic conductivity `κ` (item 9-44) and amount-of-substance concentration `c_B` (item 9-12.1): `Λ_m = κ/c_B`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarConductivityUnit[1];
    }

    attribute molarConductivity: MolarConductivityValue[*] nonunique :> scalarQuantities;

    attribute def MolarConductivityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-46 transport number of the ion B, current fraction of the ion B */
    attribute def TransportNumberOfTheIonBValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-46 transport number of the ion B, current fraction of the ion B
         * symbol(s): `t_B`
         * application domain: generic
         * name: TransportNumberOfTheIonB (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the ion `B`, quotient of electric current `i_B` (IEC 80000-6) carried by the ion `B` and total electric current `i` (IEC 80000-6) in an electrolyte: `t_B = i_B/i`
         * remarks: None.
         */
    }
    attribute transportNumberOfTheIonB: TransportNumberOfTheIonBValue :> scalarQuantities;

    alias currentFractionOfTheIonB for transportNumberOfTheIonB;

    /* ISO-80000-9 item 9-47 angle of optical rotation */
    attribute angleOfOpticalRotation: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 9-47 angle of optical rotation
         * symbol(s): `α`
         * application domain: generic
         * name: AngleOfOpticalRotation (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: angle through which plane-polarized light is rotated clockwise, as seen when facing the light source, in passing through an optically active medium
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-48 molar optical rotatory power */
    attribute def MolarOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-48 molar optical rotatory power
         * symbol(s): `α_n`
         * application domain: generic
         * name: MolarOpticalRotatoryPower
         * quantity dimension: L^2*N^-1
         * measurement unit(s): rad*m^2/mol, m^2*mol^-1
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the amount of substance `n` (item 9-2) of the optically active component in the path of the beam: `α_n = (α A)/n`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarOpticalRotatoryPowerUnit[1];
    }

    attribute molarOpticalRotatoryPower: MolarOpticalRotatoryPowerValue[*] nonunique :> scalarQuantities;

    attribute def MolarOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-49 specific optical rotatory power */
    attribute def SpecificOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-49 specific optical rotatory power
         * symbol(s): `α_m`
         * application domain: generic
         * name: SpecificOpticalRotatoryPower
         * quantity dimension: L^2*M^-1
         * measurement unit(s): rad*m^2/kg, kg^-1*m^2
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the mass `m` (ISO 80000-4) of the optically active component in the path of the beam: `α_m = (α A)/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificOpticalRotatoryPowerUnit[1];
    }

    attribute specificOpticalRotatoryPower: SpecificOpticalRotatoryPowerValue[*] nonunique :> scalarQuantities;

    attribute def SpecificOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'AmountOfSubstanceValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'AmountOfSubstanceValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
RegularComment,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,KwMultiplicity,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
RegularComment,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQChemistryMolecular'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQSpaceTime::AngularMeasureValue')
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (attribute_usage 'numberOfEntities' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (comment)
    (alias_member 'NumberOfMolesUnit' for 'AmountOfSubstanceUnit')
    (alias_member 'NumberOfMolesValue' for 'AmountOfSubstanceValue')
    (alias_member 'numberOfMoles' for 'amountOfSubstance')
    (comment)
    (attribute_def 'RelativeAtomicMassValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeAtomicMass' : 'RelativeAtomicMassValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MolarMassValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarMassUnit' multiplicity))
    (attribute_usage 'molarMass' : 'MolarMassValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarMassUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarVolumeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarVolumeUnit' multiplicity))
    (attribute_usage 'molarVolume' : 'MolarVolumeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarVolumeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarInternalEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarInternalEnergyUnit' multiplicity))
    (attribute_usage 'molarInternalEnergy' : 'MolarInternalEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarInternalEnergyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarEnthalpyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarEnthalpyUnit' multiplicity))
    (attribute_usage 'molarEnthalpy' : 'MolarEnthalpyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarEnthalpyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarHelmholtzEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarHelmholtzEnergyUnit' multiplicity))
    (attribute_usage 'molarHelmholtzEnergy' : 'MolarHelmholtzEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarHelmholtzEnergyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarGibbsEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarGibbsEnergyUnit' multiplicity))
    (attribute_usage 'molarGibbsEnergy' : 'MolarGibbsEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarGibbsEnergyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarHeatCapacityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarHeatCapacityUnit' multiplicity))
    (attribute_usage 'molarHeatCapacity' : 'MolarHeatCapacityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarHeatCapacityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarEntropyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarEntropyUnit' multiplicity))
    (attribute_usage 'molarEntropy' : 'MolarEntropyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarEntropyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ParticleConcentrationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleConcentrationUnit' multiplicity))
    (attribute_usage 'particleConcentration' : 'ParticleConcentrationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleConcentrationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'molecularConcentration' : 'ParticleConcentrationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'MassConcentrationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassConcentrationUnit' multiplicity))
    (attribute_usage 'massConcentration' : 'MassConcentrationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassConcentrationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassFractionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'massFraction' : 'MassFractionValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AmountOfSubstanceConcentrationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AmountOfSubstanceConcentrationUnit' multiplicity))
    (attribute_usage 'amountOfSubstanceConcentration' : 'AmountOfSubstanceConcentrationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AmountOfSubstanceConcentrationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'standardAmountOfSubstanceConcentration' : 'AmountOfSubstanceConcentrationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'AmountOfSubstanceFractionMoleFractionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'amountOfSubstanceFractionMoleFraction' : 'AmountOfSubstanceFractionMoleFractionValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'VolumeFractionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'VolumeFractionUnit' multiplicity))
    (attribute_usage 'volumeFraction' : 'VolumeFractionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'VolumeFractionUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'MolalityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolalityUnit' multiplicity))
    (attribute_usage 'molality' : 'MolalityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolalityUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'latentHeatOfPhaseTransition' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'enthalpyOfPhaseTransition' for 'latentHeatOfPhaseTransition')
    (comment)
    (attribute_def 'ChemicalPotentialValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ChemicalPotentialUnit' multiplicity))
    (attribute_usage 'chemicalPotential' : 'ChemicalPotentialValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ChemicalPotentialUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AbsoluteActivityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'absoluteActivity' : 'AbsoluteActivityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PartialPressureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PartialPressureUnit' multiplicity))
    (attribute_usage 'partialPressure' : 'PartialPressureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PartialPressureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'FugacityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'FugacityUnit' multiplicity))
    (attribute_usage 'fugacity' : 'FugacityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'FugacityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'StandardChemicalPotentialValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'StandardChemicalPotentialUnit' multiplicity))
    (attribute_usage 'standardChemicalPotential' : 'StandardChemicalPotentialValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'StandardChemicalPotentialUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ActivityFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'activityFactor' : 'ActivityFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StandardAbsoluteActivityInMixtureValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'standardAbsoluteActivityInMixture' : 'StandardAbsoluteActivityInMixtureValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ActivityOfSoluteValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'activityOfSolute' : 'ActivityOfSoluteValue' :> 'scalarQuantities')
    (alias_member 'relativeActivityOfSolute' for 'activityOfSolute')
    (comment)
    (attribute_def 'ActivityCoefficientValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'activityCoefficient' : 'ActivityCoefficientValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StandardAbsoluteActivityInSolutionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'standardAbsoluteActivityInSolution' : 'StandardAbsoluteActivityInSolutionValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ActivityOfSolventValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'activityOfSolvent' : 'ActivityOfSolventValue' :> 'scalarQuantities')
    (alias_member 'relativeActivityOfSolvent' for 'activityOfSolvent')
    (comment)
    (attribute_def 'OsmoticFactorOfSolventValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'osmoticFactorOfSolvent' : 'OsmoticFactorOfSolventValue' :> 'scalarQuantities')
    (alias_member 'osmoticCoefficientOfSolventA' for 'osmoticFactorOfSolvent')
    (comment)
    (attribute_def 'StandardAbsoluteActivityOfSolventValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'standardAbsoluteActivityOfSolvent' : 'StandardAbsoluteActivityOfSolventValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'OsmoticPressureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'OsmoticPressureUnit' multiplicity))
    (attribute_usage 'osmoticPressure' : 'OsmoticPressureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'OsmoticPressureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'StoichiometricNumberOfSubstanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stoichiometricNumberOfSubstance' : 'StoichiometricNumberOfSubstanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AffinityOfAChemicalReactionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AffinityOfAChemicalReactionUnit' multiplicity))
    (attribute_usage 'affinityOfAChemicalReaction' : 'AffinityOfAChemicalReactionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AffinityOfAChemicalReactionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'extentOfReaction' : 'AmountOfSubstanceValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'StandardEquilibriumConstantValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'standardEquilibriumConstant' : 'StandardEquilibriumConstantValue' :> 'scalarQuantities')
    (alias_member 'thermodynamicEquilibriumConstant' for 'standardEquilibriumConstant')
    (comment)
    (attribute_def 'EquilibriumConstantOnPressureBasisValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EquilibriumConstantOnPressureBasisUnit' multiplicity))
    (attribute_usage 'equilibriumConstantOnPressureBasis' : 'EquilibriumConstantOnPressureBasisValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EquilibriumConstantOnPressureBasisUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'EquilibriumConstantOnConcentrationBasisValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EquilibriumConstantOnConcentrationBasisUnit' multiplicity))
    (attribute_usage 'equilibriumConstantOnConcentrationBasis' : 'EquilibriumConstantOnConcentrationBasisValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EquilibriumConstantOnConcentrationBasisUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'microcanonicalPartitionFunction' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'CanonicalPartitionFunctionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'canonicalPartitionFunction' : 'CanonicalPartitionFunctionValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GrandCanonicalPartitionFunctionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'grandCanonicalPartitionFunction' : 'GrandCanonicalPartitionFunctionValue' :> 'scalarQuantities')
    (alias_member 'grandPartitionFunction' for 'grandCanonicalPartitionFunction')
    (comment)
    (attribute_def 'MolecularPartitionFunctionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'molecularPartitionFunction' : 'MolecularPartitionFunctionValue' :> 'scalarQuantities')
    (alias_member 'partitionFunctionOfAMolecule' for 'molecularPartitionFunction')
    (comment)
    (attribute_usage 'statisticalWeightOfSubsystem' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'DegeneracyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'degeneracy' : 'DegeneracyValue' :> 'scalarQuantities')
    (alias_member 'multiplicity' for 'degeneracy')
    (comment)
    (attribute_def 'MolarGasConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarGasConstantUnit' multiplicity))
    (attribute_usage 'molarGasConstant' : 'MolarGasConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarGasConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (comment)
    (comment)
    (attribute_usage 'meanFreePath' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'DiffusionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DiffusionCoefficientUnit' multiplicity))
    (attribute_usage 'diffusionCoefficient' : 'DiffusionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DiffusionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermalDiffusionRatioValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'thermalDiffusionRatio' : 'ThermalDiffusionRatioValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ThermalDiffusionFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'thermalDiffusionFactor' : 'ThermalDiffusionFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ThermalDiffusionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalDiffusionCoefficientUnit' multiplicity))
    (attribute_usage 'thermalDiffusionCoefficient' : 'ThermalDiffusionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalDiffusionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'IonicStrengthValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IonicStrengthUnit' multiplicity))
    (attribute_usage 'ionicStrength' : 'IonicStrengthValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IonicStrengthUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'DegreeOfDissociationValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'degreeOfDissociation' : 'DegreeOfDissociationValue' :> 'scalarQuantities')
    (alias_member 'dissociationFraction' for 'degreeOfDissociation')
    (comment)
    (attribute_def 'ElectrolyticConductivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectrolyticConductivityUnit' multiplicity))
    (attribute_usage 'electrolyticConductivity' : 'ElectrolyticConductivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectrolyticConductivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarConductivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarConductivityUnit' multiplicity))
    (attribute_usage 'molarConductivity' : 'MolarConductivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarConductivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'TransportNumberOfTheIonBValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'transportNumberOfTheIonB' : 'TransportNumberOfTheIonBValue' :> 'scalarQuantities')
    (alias_member 'currentFractionOfTheIonB' for 'transportNumberOfTheIonB')
    (comment)
    (attribute_usage 'angleOfOpticalRotation' : 'AngularMeasureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'MolarOpticalRotatoryPowerValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarOpticalRotatoryPowerUnit' multiplicity))
    (attribute_usage 'molarOpticalRotatoryPower' : 'MolarOpticalRotatoryPowerValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarOpticalRotatoryPowerUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificOpticalRotatoryPowerValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificOpticalRotatoryPowerUnit' multiplicity))
    (attribute_usage 'specificOpticalRotatoryPower' : 'SpecificOpticalRotatoryPowerValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificOpticalRotatoryPowerUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))))
~~~
# FORMAT
~~~sysml
standard library package ISQChemistryMolecular {
    doc /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-9:2019 "Physical chemistry and molecular physics"
     * see also https://www.iso.org/standard/64979.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;

    /* Quantity definitions referenced from other ISQ packages */
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-9 item 9-1 number of entities */
    attribute numberOfEntities : CountValue :> scalarQuantities {
        doc /*
         * source: item 9-1 number of entities
         * symbol(s): `N(X)`, `N_X`
         * application domain: generic
         * name: NumberOfEntities (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of elementary entities of kind `X` in a system
         * remarks: The elementary entities must be specified and can be atoms, molecules, ions, electrons, other particle, or a specified group of such particles. It is important to always give a precise specification of the entity involved; this should preferably be done by the empirical chemical formula of the material involved.
         */
    }

    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    /* See package ISQBase for the declarations of AmountOfSubstanceValue and AmountOfSubstanceUnit */

    alias NumberOfMolesUnit for AmountOfSubstanceUnit;
    alias NumberOfMolesValue for AmountOfSubstanceValue;
    alias numberOfMoles for amountOfSubstance;

    /* ISO-80000-9 item 9-3 relative atomic mass */
    attribute def RelativeAtomicMassValue :> DimensionOneValue {
        doc /*
         * source: item 9-3 relative atomic mass
         * symbol(s): `A_r(X)`
         * application domain: generic
         * name: RelativeAtomicMass (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the average mass (ISO 80000-4) of atom `X` and the unified atomic mass (ISO 80000-10)
         * remarks: A similar quantity "relative molecular mass" can be defined for molecules. EXAMPLE `A_r(Cl) ~~ 35.453` `A_r(CO_2) ~~ 44` The relative atomic or relative molecular mass depends on the nuclidic composition. The International Union of Pure and Applied Chemistry (IUPAC) accepts the use of the special names "atomic weight" and "molecular weight" for the quantities "relative atomic mass" and "relative molecular mass", respectively. The use of these traditional names is deprecated.
         */
    }
    attribute relativeAtomicMass : RelativeAtomicMassValue :> scalarQuantities;

    /* ISO-80000-9 item 9-4 molar mass */
    attribute def MolarMassValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-4 molar mass
         * symbol(s): `M(X)`
         * application domain: generic
         * name: MolarMass
         * quantity dimension: M^1*N^-1
         * measurement unit(s): g/mol, kg*mol^-1
         * tensor order: 0
         * definition: for a pure substance `X`, quotient of mass `m(X)` (ISO 80000-4) and amount `n` of substance (item 9-2): `M = m/n`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarMassUnit [1];
    }

    attribute molarMass : MolarMassValue :> scalarQuantities [*] nonunique;

    attribute def MolarMassUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-5 molar volume */
    attribute def MolarVolumeValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-5 molar volume
         * symbol(s): `V_m`
         * application domain: generic
         * name: MolarVolume
         * quantity dimension: L^3*N^-1
         * measurement unit(s): m^3*mol^-1
         * tensor order: 0
         * definition: for a pure substance, quotient of its volume `V` (ISO 80000-3) and amount `n` of substance (item 9-2): `V_m = V/n`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarVolumeUnit [1];
    }

    attribute molarVolume : MolarVolumeValue :> scalarQuantities [*] nonunique;

    attribute def MolarVolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-6.1 molar internal energy */
    attribute def MolarInternalEnergyValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-6.1 molar internal energy
         * symbol(s): `U_m`
         * application domain: generic
         * name: MolarInternalEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of internal energy `U` (ISO 80000-5) and amount `n` of substance (item 9-2): `U_m = U/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarInternalEnergyUnit [1];
    }

    attribute molarInternalEnergy : MolarInternalEnergyValue :> scalarQuantities [*] nonunique;

    attribute def MolarInternalEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-6.2 molar enthalpy */
    attribute def MolarEnthalpyValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-6.2 molar enthalpy
         * symbol(s): `H_m`
         * application domain: generic
         * name: MolarEnthalpy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of enthalpy `H` (ISO 80000-5) and amount `n` of substance (item 9-2): `H_m = H/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarEnthalpyUnit [1];
    }

    attribute molarEnthalpy : MolarEnthalpyValue :> scalarQuantities [*] nonunique;

    attribute def MolarEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-6.3 molar Helmholtz energy */
    attribute def MolarHelmholtzEnergyValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-6.3 molar Helmholtz energy
         * symbol(s): `F_m`
         * application domain: generic
         * name: MolarHelmholtzEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Helmholtz energy `F` (ISO 80000-5) and amount `n` of substance (item 9-2): `F_m = F/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarHelmholtzEnergyUnit [1];
    }

    attribute molarHelmholtzEnergy : MolarHelmholtzEnergyValue :> scalarQuantities [*] nonunique;

    attribute def MolarHelmholtzEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-6.4 molar Gibbs energy */
    attribute def MolarGibbsEnergyValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-6.4 molar Gibbs energy
         * symbol(s): `G_m`
         * application domain: generic
         * name: MolarGibbsEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Gibbs energy `G` (ISO 80000-5) and amount `n` of substance (item 9-2): `G_m = G/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarGibbsEnergyUnit [1];
    }

    attribute molarGibbsEnergy : MolarGibbsEnergyValue :> scalarQuantities [*] nonunique;

    attribute def MolarGibbsEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-7 molar heat capacity */
    attribute def MolarHeatCapacityValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-7 molar heat capacity
         * symbol(s): `C_m`
         * application domain: generic
         * name: MolarHeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of heat capacity `C` (ISO 80000-5) and amount of substance `n` (item 9-2): `C_m = C/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarHeatCapacityUnit [1];
    }

    attribute molarHeatCapacity : MolarHeatCapacityValue :> scalarQuantities [*] nonunique;

    attribute def MolarHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-8 molar entropy */
    attribute def MolarEntropyValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-8 molar entropy
         * symbol(s): `S_m`
         * application domain: generic
         * name: MolarEntropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of entropy `S` (ISO 80000-5) and amount `n` of substance (item 9-2): `S_m = S/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarEntropyUnit [1];
    }

    attribute molarEntropy : MolarEntropyValue :> scalarQuantities [*] nonunique;

    attribute def MolarEntropyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-9.1 particle concentration */
    attribute def ParticleConcentrationValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-9.1 particle concentration
         * symbol(s): `n`, `(C)`
         * application domain: generic
         * name: ParticleConcentration
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number `N` of particles (item 9-1) and volume `V `(ISO 80000-3): `n = N/V`
         * remarks: The term "number density" is also used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleConcentrationUnit [1];
    }

    attribute particleConcentration : ParticleConcentrationValue :> scalarQuantities [*] nonunique;

    attribute def ParticleConcentrationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-9 item 9-9.2 molecular concentration */
    attribute molecularConcentration : ParticleConcentrationValue :> scalarQuantities {
        doc /*
         * source: item 9-9.2 molecular concentration
         * symbol(s): `C(X)`, `C_X`
         * application domain: generic
         * name: MolecularConcentration (specializes ParticleConcentration)
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of number `N_X` of molecules of substance `X` and volume `V` (ISO 80000-3) of the mixture: `C_X = N_X/V`
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-10 mass concentration */
    attribute def MassConcentrationValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-10 mass concentration
         * symbol(s): `γ_X`, `(ρ_X)`
         * application domain: generic
         * name: MassConcentration
         * quantity dimension: L^-3*M^1
         * measurement unit(s): g/l, kg*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and volume `V` (ISO 80000-3) of the mixture: `γ_X = m_X/V`
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassConcentrationUnit [1];
    }

    attribute massConcentration : MassConcentrationValue :> scalarQuantities [*] nonunique;

    attribute def MassConcentrationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }

    /* ISO-80000-9 item 9-11 mass fraction */
    attribute def MassFractionValue :> DimensionOneValue {
        doc /*
         * source: item 9-11 mass fraction
         * symbol(s): `w_X`
         * application domain: generic
         * name: MassFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and total mass `m` of the mixture: `w_X = m_X/m`
         * remarks: None.
         */
    }
    attribute massFraction : MassFractionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-12.1 amount-of-substance concentration */
    attribute def AmountOfSubstanceConcentrationValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-12.1 amount-of-substance concentration
         * symbol(s): `c_X`
         * application domain: generic
         * name: AmountOfSubstanceConcentration
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount `n_X` of substance (item 9-2) of `X` and volume `V` (ISO 80000-3) of the mixture: `c_X = n_X/V`
         * remarks: In chemistry, the name "amount-of-substance concentration" is generally abbreviated to the single word "concentration", it being assumed that the adjective "amount-of-substance" is intended. For this reason, however, the word "mass" should never be omitted from the name "mass concentration" in item 9-10. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AmountOfSubstanceConcentrationUnit [1];
    }

    attribute amountOfSubstanceConcentration : AmountOfSubstanceConcentrationValue :> scalarQuantities [*] nonunique;

    attribute def AmountOfSubstanceConcentrationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-12.2 standard amount-of-substance concentration */
    attribute standardAmountOfSubstanceConcentration : AmountOfSubstanceConcentrationValue :> scalarQuantities {
        doc /*
         * source: item 9-12.2 standard amount-of-substance concentration
         * symbol(s): `c^!(X)`
         * application domain: generic
         * name: StandardAmountOfSubstanceConcentration (specializes AmountOfSubstanceConcentration)
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X`, one mole per litre
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
    }

    /* ISO-80000-9 item 9-13 amount-of-substance fraction mole fraction */
    attribute def AmountOfSubstanceFractionMoleFractionValue :> DimensionOneValue {
        doc /*
         * source: item 9-13 amount-of-substance fraction mole fraction
         * symbol(s): `x_X`, `y_X`
         * application domain: generic
         * name: AmountOfSubstanceFractionMoleFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount of substance `n_X` (item 9-2) of `X` and total amount `n` of substance (item 9-2) in the mixture: `x_X = n_X/n`
         * remarks: For condensed phases, `x_X` is used, and for gaseous mixtures `y_X` may be used. The unsystematic name "mole fraction" is still used. However, the use of this name is deprecated. For this quantity, the entity used to define the amount of substance should always be a single molecule for every species in the mixture.
         */
    }
    attribute amountOfSubstanceFractionMoleFraction : AmountOfSubstanceFractionMoleFractionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-14 volume fraction */
    attribute def VolumeFractionValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-14 volume fraction
         * symbol(s): `φ_X`
         * application domain: generic
         * name: VolumeFraction
         * quantity dimension: 1
         * measurement unit(s): ml/l, 1
         * tensor order: 0
         * definition: for substance `X`, quotient of product of amount of substance fraction `x_X` (item 9-13) of `X` and molar volume `V_(m,X)` (item 9-5) of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4), and sum over all substances `i` of products of amount-of-substance fractions `x_i` (item 9-13) of substance `i` and their molar volumes `V_(m,i)` (item 9-5): `φ_X = (x_X V_(m,X))/(sum_i x_i V_(m,i))`
         * remarks: Generally, the volume fraction is temperature dependent. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumeFractionUnit [1];
    }

    attribute volumeFraction : VolumeFractionValue :> scalarQuantities [*] nonunique;

    attribute def VolumeFractionUnit :> DimensionOneUnit { }

    /* ISO-80000-9 item 9-15 molality */
    attribute def MolalityValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-15 molality
         * symbol(s): `b_B`, `m_B`
         * application domain: generic
         * name: Molality
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol/kg
         * tensor order: 0
         * definition: quotient of amount of substance (item 9-2) of solute `B` and mass `m_A` (ISO 80000-4) of the solvent substance `A`: `b_B = n_B/m_A`
         * remarks: The alternative symbol `m_B` should be avoided in situations where it might be mistaken for the mass of substance B. However, the symbol `m_B` is much more commonly used than the symbol `b_B` for molality, despite the possible confusion with mass.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolalityUnit [1];
    }

    attribute molality : MolalityValue :> scalarQuantities [*] nonunique;

    attribute def MolalityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-16 latent heat of phase transition, enthalpy of phase transition */
    attribute latentHeatOfPhaseTransition : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 9-16 latent heat of phase transition, enthalpy of phase transition
         * symbol(s): `C_"pt"`
         * application domain: generic
         * name: LatentHeatOfPhaseTransition (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) necessary to be added or subtracted isothermally and isobarically to a system to completely undergo the phase transition
         * remarks: Mostly, molar or specific quantity is used and phase transition is expressed explicitly, e.g. molar latent heat of evaporation. The subscript "pt" is the qualifier for the phase transition, which may be changed to e.g. "l-g". The term "enthalpy of phase transition" is mainly used in theory.
         */
    }

    alias enthalpyOfPhaseTransition for latentHeatOfPhaseTransition;

    /* ISO-80000-9 item 9-17 chemical potential */
    attribute def ChemicalPotentialValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-17 chemical potential
         * symbol(s): `μ_X`
         * application domain: chemistry
         * name: ChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: partial derivative of the Gibbs energy (ISO 80000-5) with respect to amount `n_X` of substance `X` (item 9-2) at constant temperature `T` (ISO 80000-5) and pressure `p `(ISO 80000-4): `μ_X = ((del G)/(del n_X))_(T,p)`
         * remarks: For a pure substance, where `G_m` is the molar Gibbs energy. In a mixture, `μ_B` is the partial molar Gibbs energy. In condensed matter physics, the chemical potential of electrons is energy.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ChemicalPotentialUnit [1];
    }

    attribute chemicalPotential : ChemicalPotentialValue :> scalarQuantities [*] nonunique;

    attribute def ChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-18 absolute activity */
    attribute def AbsoluteActivityValue :> DimensionOneValue {
        doc /*
         * source: item 9-18 absolute activity
         * symbol(s): `λ_X`
         * application domain: generic
         * name: AbsoluteActivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X`, exponential of quotient of chemical potential `μ_X` of substance `B` (item 9-17), and product of molar gas constant `R` (item 9-37.1) and thermodynamic temperature `T` (ISO 80000-5): `λ_X = exp(μ_X/(RT))`
         * remarks: None.
         */
    }
    attribute absoluteActivity : AbsoluteActivityValue :> scalarQuantities;

    /* ISO-80000-9 item 9-19 partial pressure */
    attribute def PartialPressureValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-19 partial pressure
         * symbol(s): `p_X`
         * application domain: generic
         * name: PartialPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X` in a gaseous mixture, product of amount-of-substance fraction `y_X` of substance X (item 9-13) and total pressure `p` (ISO 80000-4): `p_X = y_X p`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PartialPressureUnit [1];
    }

    attribute partialPressure : PartialPressureValue :> scalarQuantities [*] nonunique;

    attribute def PartialPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-9 item 9-20 fugacity */
    attribute def FugacityValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-20 fugacity
         * symbol(s): `tilde(p)_X`
         * application domain: generic
         * name: Fugacity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X`, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) only, being determined by the condition that, at constant temperature and composition, `p_X/tilde(p)_X` tends to 1 for an indefinitely dilute gas
         * remarks: `tilde(p)_X = λ_X * lim_(p->0) (p_X/λ_X)` where `p` is total pressure (ISO 80000-4). The IUPAC preferred symbol for fugacity is `f`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : FugacityUnit [1];
    }

    attribute fugacity : FugacityValue :> scalarQuantities [*] nonunique;

    attribute def FugacityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-9 item 9-21 standard chemical potential */
    attribute def StandardChemicalPotentialValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-21 standard chemical potential
         * symbol(s): `μ_B^!`, `μ^!`
         * application domain: generic
         * name: StandardChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: for substance `B`, value of the chemical potential (item 9-17) at specified standard conditions
         * remarks: `μ_B^! = RT ln(λ^!)` where `μ_B^!` is a function of temperature `T` at the standard pressure `p = p^!` The standard chemical potential depends on the choice of standard state, which must be specified. In a liquid or solid solution, the standard state is referenced to the ideal dilute behaviour of the solute (substance `B`).
         */
        attribute :>> num : Real;
        attribute :>> mRef : StandardChemicalPotentialUnit [1];
    }

    attribute standardChemicalPotential : StandardChemicalPotentialValue :> scalarQuantities [*] nonunique;

    attribute def StandardChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-22 activity factor */
    attribute def ActivityFactorValue :> DimensionOneValue {
        doc /*
         * source: item 9-22 activity factor
         * symbol(s): `f_X`
         * application domain: generic
         * name: ActivityFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, quotient of absolute activity `λ_X` (item 9-18) of substance `X` and the product of absolute activity `λ_X^"*"` of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4) and amount-of-substance fraction `x_X` of substance `X` (item 9-13): `f_X = λ_X/(λ_X^"*" x_X)`
         * remarks: The systematic name is "activity factor", but the name "activity coefficient" is also commonly used (see item 9-25). Activity factors can also be obtained applying Raoult’s law or Henry’s law.
         */
    }
    attribute activityFactor : ActivityFactorValue :> scalarQuantities;

    /* ISO-80000-9 item 9-23 standard absolute activity in mixture */
    attribute def StandardAbsoluteActivityInMixtureValue :> DimensionOneValue {
        doc /*
         * source: item 9-23 standard absolute activity in mixture
         * symbol(s): `λ_X^!`
         * application domain: in a mixture
         * name: StandardAbsoluteActivityInMixture (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, absolute activity `λ_X^"*"` (item 9-18) of the pure substance `X` at the same temperature (ISO 80000-5) but at standard pressure (ISO 80000-4) `10^5 ["Pa"]`: `λ_X^! = λ_X"*" (p^!)`
         * remarks: This quantity is a function of temperature only.
         */
    }
    attribute standardAbsoluteActivityInMixture : StandardAbsoluteActivityInMixtureValue :> scalarQuantities;

    /* ISO-80000-9 item 9-24 activity of solute, relative activity of solute */
    attribute def ActivityOfSoluteValue :> DimensionOneValue {
        doc /*
         * source: item 9-24 activity of solute, relative activity of solute
         * symbol(s): `a_X`, `a_(m,X)`
         * application domain: generic
         * name: ActivityOfSolute (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `X` in a solution, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) and pressure (ISO 80000-4) only, being determined by the condition that, at constant temperature and pressure, `a_X` divided by the molality (item 9-15) ratio, `b_X/b^!` tends to 1 at infinite dilution; `b_X` is the molality of solute `X` (item 9-15), and `b^!` is standard molality: `a_X = λ_X * lim_(sum b_X -> 0) (b_X//b^!)/λ_X`
         * remarks: The quantity `a_(c,X)` , similarly defined in terms of the concentration ratio `c_X/c^!` , is also called the activity or relative activity of solute `X`; `c^!` is a standard amount-of-substance concentration (item 9-12.2): `a_(c,X) = λ_X * lim_(sum c_X -> 0) (c_X//c^!)/λ_X`, where `sum` denotes summation over all the solute substances. This especially applies to a dilute liquid solution.
         */
    }
    attribute activityOfSolute : ActivityOfSoluteValue :> scalarQuantities;

    alias relativeActivityOfSolute for activityOfSolute;

    /* ISO-80000-9 item 9-25 activity coefficient */
    attribute def ActivityCoefficientValue :> DimensionOneValue {
        doc /*
         * source: item 9-25 activity coefficient
         * symbol(s): `γ_B`
         * application domain: generic
         * name: ActivityCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution, quotient of activity `a_B` of solute `B` (item 9-24), and quotient of the molality (item 9-15) `b_B` of substance `B` and standard molality `b^!`: `γ_B = a_B/(b_B//b^!)`
         * remarks: The name "activity coefficient of solute B" is also used for the quantity `γ_B` defined as: `γ_B = a_(c,B)/(c_B//c^!)` See item 9-22.
         */
    }
    attribute activityCoefficient : ActivityCoefficientValue :> scalarQuantities;

    /* ISO-80000-9 item 9-26 standard absolute activity in solution */
    attribute def StandardAbsoluteActivityInSolutionValue :> DimensionOneValue {
        doc /*
         * source: item 9-26 standard absolute activity in solution
         * symbol(s): `λ_B^!`
         * application domain: in a solution
         * name: StandardAbsoluteActivityInSolution (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution: `λ_B^! = lim_(sum b_B -> 0) [λ_B ((p^!)b^!)/b_B]` where ∑ denotes summation over all solutes, `p^!` is a standard pressure (ISO 80000-4), `b^!` is standard molality, and `b_B` is the molality of substance `B` (item 9-15)
         * remarks: This quantity is a function of temperature only. It especially applies to a dilute liquid solution. The standard pressure is `10^5 ["Pa"]`.
         */
    }
    attribute standardAbsoluteActivityInSolution : StandardAbsoluteActivityInSolutionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-27.1 activity of solvent, relative activity of solvent */
    attribute def ActivityOfSolventValue :> DimensionOneValue {
        doc /*
         * source: item 9-27.1 activity of solvent, relative activity of solvent
         * symbol(s): `a_A`
         * application domain: generic
         * name: ActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the solvent `A` in a solution, quotient of the absolute activity of substance `A`, `λ_A` (item 9-18), and that, `λ_A^"*"` , of the pure solvent at the same temperature (ISO 80000-5) and pressure (ISO 80000-4): `a_A = λ_A/λ_A^"*"`
         * remarks: None.
         */
    }
    attribute activityOfSolvent : ActivityOfSolventValue :> scalarQuantities;

    alias relativeActivityOfSolvent for activityOfSolvent;

    /* ISO-80000-9 item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A */
    attribute def OsmoticFactorOfSolventValue :> DimensionOneValue {
        doc /*
         * source: item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A
         * symbol(s): `φ`
         * application domain: generic
         * name: OsmoticFactorOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `φ = -(M_A sum b_B)^-1 ln(a_A)` where `M_A` is the molar mass (item 9-4) of the solvent A, ∑ denotes summation over all the solutes, `b_B` is the molality of solute B (item 9-15), and `a_A` is the activity of solvent A (item 9-27.1)
         * remarks: The name "osmotic coefficient" is generally used, although the name "osmotic factor" is more systematic. This concept especially applies to a dilute liquid solution.
         */
    }
    attribute osmoticFactorOfSolvent : OsmoticFactorOfSolventValue :> scalarQuantities;

    alias osmoticCoefficientOfSolventA for osmoticFactorOfSolvent;

    /* ISO-80000-9 item 9-27.3 standard absolute activity of solvent */
    attribute def StandardAbsoluteActivityOfSolventValue :> DimensionOneValue {
        doc /*
         * source: item 9-27.3 standard absolute activity of solvent
         * symbol(s): `λ_A^!`
         * application domain: in a dilute solution
         * name: StandardAbsoluteActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for solvent `A`, standard absolute activity (item 9-23) of the pure substance `A` at the same temperature (ISO 80000-5) and at a standard pressure `p^!` (ISO 80000-4): `λ_A^! = λ_A^"*" p^!`
         * remarks: None.
         */
    }
    attribute standardAbsoluteActivityOfSolvent : StandardAbsoluteActivityOfSolventValue :> scalarQuantities;

    /* ISO-80000-9 item 9-28 osmotic pressure */
    attribute def OsmoticPressureValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-28 osmotic pressure
         * symbol(s): `Π`
         * application domain: generic
         * name: OsmoticPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: excess pressure (ISO 80000-4) required to maintain osmotic equilibrium between a solution and the pure solvent separated by a membrane permeable to the solvent only
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : OsmoticPressureUnit [1];
    }

    attribute osmoticPressure : OsmoticPressureValue :> scalarQuantities [*] nonunique;

    attribute def OsmoticPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-9 item 9-29 stoichiometric number of substance */
    attribute def StoichiometricNumberOfSubstanceValue :> DimensionOneValue {
        doc /*
         * source: item 9-29 stoichiometric number of substance
         * symbol(s): `ν_B`
         * application domain: generic
         * name: StoichiometricNumberOfSubstance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `B`, an integer number or a simple fraction, being negative for a reactant and positive for a product, occurring in the expression for a chemical reaction: `0 = sum ν_B` where the symbol `B` denotes the reactants and products involved in the reaction
         * remarks: EXAMPLE `(1/2)"N"_2 + (3/2)"H"_2 = "N""H"_3` ; `ν("N"_2) = -1/2`, `ν("H"_2) = -3/2`, `ν("N""H"_3) = +1`.
         */
    }
    attribute stoichiometricNumberOfSubstance : StoichiometricNumberOfSubstanceValue :> scalarQuantities;

    /* ISO-80000-9 item 9-30 affinity of a chemical reaction */
    attribute def AffinityOfAChemicalReactionValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-30 affinity of a chemical reaction
         * symbol(s): `A`
         * application domain: generic
         * name: AffinityOfAChemicalReaction
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: negative of the sum over all substances `B` of products of stoichiometric number `ν_B` of substance `B` (item 9-29) and chemical potential `μ_B` of substance `B` (item 9-17): `A = -sum ν_B μ_B`
         * remarks: The affinity of a reaction is a measure of the "driving force" of the reaction. When it is positive, the reaction goes spontaneously from reactants to products, and when it is negative, the reaction goes in the opposite direction. Another way to write the definition is: `A = ((del G)/(del ξ))_(p,T)` where `G` is Gibbs energy (ISO 80000-5) and `ξ` is the extent of the reaction (item 9-31). Note that `ν_B` is negative for reactants and positive for products.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AffinityOfAChemicalReactionUnit [1];
    }

    attribute affinityOfAChemicalReaction : AffinityOfAChemicalReactionValue :> scalarQuantities [*] nonunique;

    attribute def AffinityOfAChemicalReactionUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-31 extent of reaction */
    attribute extentOfReaction : AmountOfSubstanceValue :> scalarQuantities {
        doc /*
         * source: item 9-31 extent of reaction
         * symbol(s): `ξ`
         * application domain: generic
         * name: ExtentOfReaction (specializes AmountOfSubstance)
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: difference of initial amount `n_(B "in")` of substance `B` (item 9-2) and equilibrium amount `n_(B "eq")` of substance `B` (item 9-2) divided by stoichiometric number `ν_B` of substance `B` (item 9-29): `ξ = (n_(B "eq") - n_(B "in"))/ν_B`
         * remarks: See remark to item 9-30.
         */
    }

    /* ISO-80000-9 item 9-32 standard equilibrium constant, thermodynamic equilibrium constant */
    attribute def StandardEquilibriumConstantValue :> DimensionOneValue {
        doc /*
         * source: item 9-32 standard equilibrium constant, thermodynamic equilibrium constant
         * symbol(s): `K^!`
         * application domain: generic
         * name: StandardEquilibriumConstant (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a chemical reaction, product for all substances `B` of standard absolute activity `λ_B^!` of substance `B` (item 9-26) in power of minus stoichiometric number `ν_B` of substance `B` (item 9-29): `K^! = prod_B (tilde(a) λ_B^!)^(-ν_B)`
         * remarks: This quantity is a function of temperature only. Others depend on temperature, pressure, and composition. One can define in an analogous way an equilibrium constant in terms of fugacity, `K_f`, molality, `K_m`, etc.
         */
    }
    attribute standardEquilibriumConstant : StandardEquilibriumConstantValue :> scalarQuantities;

    alias thermodynamicEquilibriumConstant for standardEquilibriumConstant;

    /* ISO-80000-9 item 9-33 equilibrium constant on pressure basis */
    attribute def EquilibriumConstantOnPressureBasisValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-33 equilibrium constant on pressure basis
         * symbol(s): `K_p`
         * application domain: pressure basis
         * name: EquilibriumConstantOnPressureBasis
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for gases, product for all substances `B` of partial pressure `p_B` of substance `B` (item 9-19) in power of its stoichiometric number `ν_B` (item 9-29): `K_p = prod_B (p_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EquilibriumConstantOnPressureBasisUnit [1];
    }

    attribute equilibriumConstantOnPressureBasis : EquilibriumConstantOnPressureBasisValue :> scalarQuantities [*] nonunique;

    attribute def EquilibriumConstantOnPressureBasisUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-9 item 9-34 equilibrium constant on concentration basis */
    attribute def EquilibriumConstantOnConcentrationBasisValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-34 equilibrium constant on concentration basis
         * symbol(s): `K_c`
         * application domain: concentration basis
         * name: EquilibriumConstantOnConcentrationBasis
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/m^3
         * tensor order: 0
         * definition: for solutions, product for all substances `B` of concentration `c_B` of substance `B` (item 9-9.1) in power of its stoichiometric number `ν_B` (item 9-29): `K_c = prod_B (c_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EquilibriumConstantOnConcentrationBasisUnit [1];
    }

    attribute equilibriumConstantOnConcentrationBasis : EquilibriumConstantOnConcentrationBasisValue :> scalarQuantities [*] nonunique;

    attribute def EquilibriumConstantOnConcentrationBasisUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-35.1 microcanonical partition function */
    attribute microcanonicalPartitionFunction : CountValue :> scalarQuantities {
        doc /*
         * source: item 9-35.1 microcanonical partition function
         * symbol(s): `Ω`
         * application domain: generic
         * name: MicrocanonicalPartitionFunction (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of all quantum states `r` consistent with given energy `E` (ISO 80000-4), volume (ISO 80000-3), and external fields: `Ω = sum_r 1`
         * remarks: `S = k ln(Ω)` where `S` is entropy (ISO 80000-5) and `k` is the Boltzmann constant (ISO 80000-1).
         */
    }

    /* ISO-80000-9 item 9-35.2 canonical partition function */
    attribute def CanonicalPartitionFunctionValue :> DimensionOneValue {
        doc /*
         * source: item 9-35.2 canonical partition function
         * symbol(s): `Z`
         * application domain: generic
         * name: CanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum over quantum states of energy `E_r` (ISO 80000-4), expressed by: `Z = sum_r exp(-E_r/(kT))` where `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: `A = -kT ln(Z)` where `A` is Helmholtz energy (ISO 80000-5).
         */
    }
    attribute canonicalPartitionFunction : CanonicalPartitionFunctionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-35.3 grand-canonical partition function, grand partition function */
    attribute def GrandCanonicalPartitionFunctionValue :> DimensionOneValue {
        doc /*
         * source: item 9-35.3 grand-canonical partition function, grand partition function
         * symbol(s): `Ξ`
         * application domain: generic
         * name: GrandCanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum of canonical partition function `Z(N_A,N_B,…)` for the given number of particles `A,B` multiplied by absolute activities (item 9-18) `λ_A, λ_B, ...` of particles `A, B`: `Ξ = sum_(N_A, N_B, ...) Z(N_A, N_B, …) * λ_A^(N_A) * λ_B^(N_B) * ...`
         * remarks: `A - sum μ_B n_B = -kT ln(Ξ)` where `A` is Helmholtz energy (ISO 80000-5), `μ_B` is the chemical potential of substance `B`, and `n_B` is the amount of substance `B`.
         */
    }
    attribute grandCanonicalPartitionFunction : GrandCanonicalPartitionFunctionValue :> scalarQuantities;

    alias grandPartitionFunction for grandCanonicalPartitionFunction;

    /* ISO-80000-9 item 9-35.4 molecular partition function, partition function of a molecule */
    attribute def MolecularPartitionFunctionValue :> DimensionOneValue {
        doc /*
         * source: item 9-35.4 molecular partition function, partition function of a molecule
         * symbol(s): `q`
         * application domain: generic
         * name: MolecularPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `q = sum_r exp(-ε_r/(kT))` where `ε_r` is the energy (ISO 80000-5) of the `r`-th level of the molecule consistent with given volume (ISO 80000-3) and external fields, `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute molecularPartitionFunction : MolecularPartitionFunctionValue :> scalarQuantities;

    alias partitionFunctionOfAMolecule for molecularPartitionFunction;

    /* ISO-80000-9 item 9-36.1 statistical weight of subsystem */
    attribute statisticalWeightOfSubsystem : CountValue :> scalarQuantities {
        doc /*
         * source: item 9-36.1 statistical weight of subsystem
         * symbol(s): `g`
         * application domain: generic
         * name: StatisticalWeightOfSubsystem (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of different microstates in a subsystem
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-36.2 degeneracy, multiplicity */
    attribute def DegeneracyValue :> DimensionOneValue {
        doc /*
         * source: item 9-36.2 degeneracy, multiplicity
         * symbol(s): `g`
         * application domain: generic
         * name: Degeneracy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for quantum level, statistical weight of that level
         * remarks: If `g = 1`, the level is called non-degenerate.
         */
    }
    attribute degeneracy : DegeneracyValue :> scalarQuantities;

    alias multiplicity for degeneracy;

    /* ISO-80000-9 item 9-37.1 molar gas constant */
    attribute def MolarGasConstantValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-37.1 molar gas constant
         * symbol(s): `R`
         * application domain: generic
         * name: MolarGasConstant
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: product of the Boltzmann constant (ISO 80000-1) and the Avogadro constant (ISO 80000-1)
         * remarks: For an ideal gas, `pV_m = RT`
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarGasConstantUnit [1];
    }

    attribute molarGasConstant : MolarGasConstantValue :> scalarQuantities [*] nonunique;

    attribute def MolarGasConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-37.2 specific gas constant */
    /* Refer to declaration for SpecificGasConstant in ISQThermodynamics item 5-26 specific gas constant */

    /* ISO-80000-9 item 9-38 mean free path */
    attribute meanFreePath : LengthValue :> scalarQuantities {
        doc /*
         * source: item 9-38 mean free path
         * symbol(s): `l`, `λ`
         * application domain: chemistry
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: for a particle, the average distance `d` (ISO 80000-3) between two successive collisions with other particles
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-39 diffusion coefficient */
    attribute def DiffusionCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-39 diffusion coefficient
         * symbol(s): `D`
         * application domain: chemistry
         * name: DiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: proportionality coefficient of local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture multiplied by the local average velocity (ISO 80000-3) `v_B` of the molecules of `B`, and minus the gradient of the local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture, expressed by: `C_B(v_B) = -D grad C_B`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DiffusionCoefficientUnit [1];
    }

    attribute diffusionCoefficient : DiffusionCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def DiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-9 item 9-40.1 thermal diffusion ratio */
    attribute def ThermalDiffusionRatioValue :> DimensionOneValue {
        doc /*
         * source: item 9-40.1 thermal diffusion ratio
         * symbol(s): `k_T`
         * application domain: generic
         * name: ThermalDiffusionRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a steady-state of a binary mixture in which thermal diffusion occurs, proportionality factor between gradient of the amount-of-subsstance fraction `x_B` (item 9-13) of the heavier substance `B`, and negative gradient of the local thermodynamic temperature `T` (ISO 80000-5) divided by that temperature (ISO 80000-5): `grad x_B = -(k_T/T) grad T`
         * remarks: None.
         */
    }
    attribute thermalDiffusionRatio : ThermalDiffusionRatioValue :> scalarQuantities;

    /* ISO-80000-9 item 9-40.2 thermal diffusion factor */
    attribute def ThermalDiffusionFactorValue :> DimensionOneValue {
        doc /*
         * source: item 9-40.2 thermal diffusion factor
         * symbol(s): `α_T`
         * application domain: generic
         * name: ThermalDiffusionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the thermal diffusion ratio `k_T` (item 9-40.1), and the product of the local amount-of-substance fractions `x_A`, `x_B` (item 9-13) of two substances `A` and `B`: `α_T = k_T//(x_A x_B)`
         * remarks: None.
         */
    }
    attribute thermalDiffusionFactor : ThermalDiffusionFactorValue :> scalarQuantities;

    /* ISO-80000-9 item 9-41 thermal diffusion coefficient */
    attribute def ThermalDiffusionCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-41 thermal diffusion coefficient
         * symbol(s): `D_T`
         * application domain: generic
         * name: ThermalDiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: product of the thermal diffusion ratio `k_T` (item 9-40.1) and the diffusion coefficient `D` (item 9-39): `D_T = k_T*D`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalDiffusionCoefficientUnit [1];
    }

    attribute thermalDiffusionCoefficient : ThermalDiffusionCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def ThermalDiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-9 item 9-42 ionic strength */
    attribute def IonicStrengthValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-42 ionic strength
         * symbol(s): `I`
         * application domain: generic
         * name: IonicStrength
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol*kg^-1
         * tensor order: 0
         * definition: in a sample, one half of the sum of square of the charge number `z_i` (ISO 80000-10) of `i`-th ion multiplied by its molality `b_i` (item 9-15) over any involved ion: `I = 1/2 sum z_i^2 b_i`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IonicStrengthUnit [1];
    }

    attribute ionicStrength : IonicStrengthValue :> scalarQuantities [*] nonunique;

    attribute def IonicStrengthUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-43 degree of dissociation, dissociation fraction */
    attribute def DegreeOfDissociationValue :> DimensionOneValue {
        doc /*
         * source: item 9-43 degree of dissociation, dissociation fraction
         * symbol(s): `α`
         * application domain: generic
         * name: DegreeOfDissociation (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a sample, quotient of the number `n_d` of dissociated molecules and the total number `n` of molecules: `α = n_D / n`
         * remarks: None.
         */
    }
    attribute degreeOfDissociation : DegreeOfDissociationValue :> scalarQuantities;

    alias dissociationFraction for degreeOfDissociation;

    /* ISO-80000-9 item 9-44 electrolytic conductivity */
    attribute def ElectrolyticConductivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-44 electrolytic conductivity
         * symbol(s): `κ`
         * application domain: generic
         * name: ElectrolyticConductivity
         * quantity dimension: L^-3*M^-1*T^3*I^2
         * measurement unit(s): S/m, kg^-1*m^-3*s^3*A^2
         * tensor order: 0
         * definition: quotient of the magnitude of electric current density `J` (IEC 80000-6) and the magnitude electric field strength `E` (IEC 80000-6) in an electrolyte: `κ = J/E`
         * remarks: For anisotropic media, `κ` is a tensor. In IEC 80000-6 the symbols `σ`, `γ` are used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectrolyticConductivityUnit [1];
    }

    attribute electrolyticConductivity : ElectrolyticConductivityValue :> scalarQuantities [*] nonunique;

    attribute def ElectrolyticConductivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* ISO-80000-9 item 9-45 molar conductivity */
    attribute def MolarConductivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-45 molar conductivity
         * symbol(s): `Λ_m`
         * application domain: generic
         * name: MolarConductivity
         * quantity dimension: M^-1*T^3*I^2*N^-1
         * measurement unit(s): S*m^2/mol, kg^-1*s^3*A^2*mol^-1
         * tensor order: 0
         * definition: in an electrolyte, quotient of electrolytic conductivity `κ` (item 9-44) and amount-of-substance concentration `c_B` (item 9-12.1): `Λ_m = κ/c_B`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarConductivityUnit [1];
    }

    attribute molarConductivity : MolarConductivityValue :> scalarQuantities [*] nonunique;

    attribute def MolarConductivityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-46 transport number of the ion B, current fraction of the ion B */
    attribute def TransportNumberOfTheIonBValue :> DimensionOneValue {
        doc /*
         * source: item 9-46 transport number of the ion B, current fraction of the ion B
         * symbol(s): `t_B`
         * application domain: generic
         * name: TransportNumberOfTheIonB (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the ion `B`, quotient of electric current `i_B` (IEC 80000-6) carried by the ion `B` and total electric current `i` (IEC 80000-6) in an electrolyte: `t_B = i_B/i`
         * remarks: None.
         */
    }
    attribute transportNumberOfTheIonB : TransportNumberOfTheIonBValue :> scalarQuantities;

    alias currentFractionOfTheIonB for transportNumberOfTheIonB;

    /* ISO-80000-9 item 9-47 angle of optical rotation */
    attribute angleOfOpticalRotation : AngularMeasureValue :> scalarQuantities {
        doc /*
         * source: item 9-47 angle of optical rotation
         * symbol(s): `α`
         * application domain: generic
         * name: AngleOfOpticalRotation (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: angle through which plane-polarized light is rotated clockwise, as seen when facing the light source, in passing through an optically active medium
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-48 molar optical rotatory power */
    attribute def MolarOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-48 molar optical rotatory power
         * symbol(s): `α_n`
         * application domain: generic
         * name: MolarOpticalRotatoryPower
         * quantity dimension: L^2*N^-1
         * measurement unit(s): rad*m^2/mol, m^2*mol^-1
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the amount of substance `n` (item 9-2) of the optically active component in the path of the beam: `α_n = (α A)/n`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarOpticalRotatoryPowerUnit [1];
    }

    attribute molarOpticalRotatoryPower : MolarOpticalRotatoryPowerValue :> scalarQuantities [*] nonunique;

    attribute def MolarOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor [1] {
            :>> quantity = isq.N;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }

    /* ISO-80000-9 item 9-49 specific optical rotatory power */
    attribute def SpecificOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc /*
         * source: item 9-49 specific optical rotatory power
         * symbol(s): `α_m`
         * application domain: generic
         * name: SpecificOpticalRotatoryPower
         * quantity dimension: L^2*M^-1
         * measurement unit(s): rad*m^2/kg, kg^-1*m^2
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the mass `m` (ISO 80000-4) of the optically active component in the path of the beam: `α_m = (α A)/m`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificOpticalRotatoryPowerUnit [1];
    }

    attribute specificOpticalRotatoryPower : SpecificOpticalRotatoryPowerValue :> scalarQuantities [*] nonunique;

    attribute def SpecificOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (name "ISQChemistryMolecular") (declared-name "ISQChemistryMolecular")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (name "AbsoluteActivityValue") (declared-name "AbsoluteActivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (name "ActivityCoefficientValue") (declared-name "ActivityCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (name "ActivityFactorValue") (declared-name "ActivityFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (name "ActivityOfSoluteValue") (declared-name "ActivityOfSoluteValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (name "ActivityOfSolventValue") (declared-name "ActivityOfSolventValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (name "AffinityOfAChemicalReactionUnit") (declared-name "AffinityOfAChemicalReactionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (name "AffinityOfAChemicalReactionValue") (declared-name "AffinityOfAChemicalReactionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (name "AmountOfSubstanceConcentrationUnit") (declared-name "AmountOfSubstanceConcentrationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (name "AmountOfSubstanceConcentrationValue") (declared-name "AmountOfSubstanceConcentrationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (name "AmountOfSubstanceFractionMoleFractionValue") (declared-name "AmountOfSubstanceFractionMoleFractionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))) (name "AngularMeasureValue") (declared-name "AngularMeasureValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (name "CanonicalPartitionFunctionValue") (declared-name "CanonicalPartitionFunctionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (name "ChemicalPotentialUnit") (declared-name "ChemicalPotentialUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (name "ChemicalPotentialValue") (declared-name "ChemicalPotentialValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (name "DegeneracyValue") (declared-name "DegeneracyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (name "DegreeOfDissociationValue") (declared-name "DegreeOfDissociationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (name "DiffusionCoefficientUnit") (declared-name "DiffusionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (name "DiffusionCoefficientValue") (declared-name "DiffusionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (name "ElectrolyticConductivityUnit") (declared-name "ElectrolyticConductivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (name "ElectrolyticConductivityValue") (declared-name "ElectrolyticConductivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (name "EquilibriumConstantOnConcentrationBasisUnit") (declared-name "EquilibriumConstantOnConcentrationBasisUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (name "EquilibriumConstantOnConcentrationBasisValue") (declared-name "EquilibriumConstantOnConcentrationBasisValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (name "EquilibriumConstantOnPressureBasisUnit") (declared-name "EquilibriumConstantOnPressureBasisUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (name "EquilibriumConstantOnPressureBasisValue") (declared-name "EquilibriumConstantOnPressureBasisValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (name "FugacityUnit") (declared-name "FugacityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (name "FugacityValue") (declared-name "FugacityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (name "GrandCanonicalPartitionFunctionValue") (declared-name "GrandCanonicalPartitionFunctionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (name "IonicStrengthUnit") (declared-name "IonicStrengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (name "IonicStrengthValue") (declared-name "IonicStrengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (name "MassConcentrationUnit") (declared-name "MassConcentrationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (name "MassConcentrationValue") (declared-name "MassConcentrationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (name "MassFractionValue") (declared-name "MassFractionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (name "MolalityUnit") (declared-name "MolalityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (name "MolalityValue") (declared-name "MolalityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (name "MolarConductivityUnit") (declared-name "MolarConductivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (name "MolarConductivityValue") (declared-name "MolarConductivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (name "MolarEnthalpyUnit") (declared-name "MolarEnthalpyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (name "MolarEnthalpyValue") (declared-name "MolarEnthalpyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (name "MolarEntropyUnit") (declared-name "MolarEntropyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (name "MolarEntropyValue") (declared-name "MolarEntropyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (name "MolarGasConstantUnit") (declared-name "MolarGasConstantUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (name "MolarGasConstantValue") (declared-name "MolarGasConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (name "MolarGibbsEnergyUnit") (declared-name "MolarGibbsEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (name "MolarGibbsEnergyValue") (declared-name "MolarGibbsEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (name "MolarHeatCapacityUnit") (declared-name "MolarHeatCapacityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (name "MolarHeatCapacityValue") (declared-name "MolarHeatCapacityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (name "MolarHelmholtzEnergyUnit") (declared-name "MolarHelmholtzEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (name "MolarHelmholtzEnergyValue") (declared-name "MolarHelmholtzEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (name "MolarInternalEnergyUnit") (declared-name "MolarInternalEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (name "MolarInternalEnergyValue") (declared-name "MolarInternalEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (name "MolarMassUnit") (declared-name "MolarMassUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (name "MolarMassValue") (declared-name "MolarMassValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (name "MolarOpticalRotatoryPowerUnit") (declared-name "MolarOpticalRotatoryPowerUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (name "MolarOpticalRotatoryPowerValue") (declared-name "MolarOpticalRotatoryPowerValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (name "MolarVolumeUnit") (declared-name "MolarVolumeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (name "MolarVolumeValue") (declared-name "MolarVolumeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (name "MolecularPartitionFunctionValue") (declared-name "MolecularPartitionFunctionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::NumberOfMolesUnit"))) (name "NumberOfMolesUnit") (declared-name "NumberOfMolesUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::NumberOfMolesValue"))) (name "NumberOfMolesValue") (declared-name "NumberOfMolesValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (name "OsmoticFactorOfSolventValue") (declared-name "OsmoticFactorOfSolventValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (name "OsmoticPressureUnit") (declared-name "OsmoticPressureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (name "OsmoticPressureValue") (declared-name "OsmoticPressureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (name "PartialPressureUnit") (declared-name "PartialPressureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (name "PartialPressureValue") (declared-name "PartialPressureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (name "ParticleConcentrationUnit") (declared-name "ParticleConcentrationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (name "ParticleConcentrationValue") (declared-name "ParticleConcentrationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (name "RelativeAtomicMassValue") (declared-name "RelativeAtomicMassValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (name "SpecificOpticalRotatoryPowerUnit") (declared-name "SpecificOpticalRotatoryPowerUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (name "SpecificOpticalRotatoryPowerValue") (declared-name "SpecificOpticalRotatoryPowerValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (name "StandardAbsoluteActivityInMixtureValue") (declared-name "StandardAbsoluteActivityInMixtureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (name "StandardAbsoluteActivityInSolutionValue") (declared-name "StandardAbsoluteActivityInSolutionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (name "StandardAbsoluteActivityOfSolventValue") (declared-name "StandardAbsoluteActivityOfSolventValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (name "StandardChemicalPotentialUnit") (declared-name "StandardChemicalPotentialUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (name "StandardChemicalPotentialValue") (declared-name "StandardChemicalPotentialValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (name "StandardEquilibriumConstantValue") (declared-name "StandardEquilibriumConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (name "StoichiometricNumberOfSubstanceValue") (declared-name "StoichiometricNumberOfSubstanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (name "ThermalDiffusionCoefficientUnit") (declared-name "ThermalDiffusionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (name "ThermalDiffusionCoefficientValue") (declared-name "ThermalDiffusionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (name "ThermalDiffusionFactorValue") (declared-name "ThermalDiffusionFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (name "ThermalDiffusionRatioValue") (declared-name "ThermalDiffusionRatioValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (name "TransportNumberOfTheIonBValue") (declared-name "TransportNumberOfTheIonBValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (name "VolumeFractionUnit") (declared-name "VolumeFractionUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (name "VolumeFractionValue") (declared-name "VolumeFractionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (name "absoluteActivity") (declared-name "absoluteActivity") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (name "activityCoefficient") (declared-name "activityCoefficient") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (name "activityFactor") (declared-name "activityFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (name "activityOfSolute") (declared-name "activityOfSolute") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (name "activityOfSolvent") (declared-name "activityOfSolvent") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (name "affinityOfAChemicalReaction") (declared-name "affinityOfAChemicalReaction") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (name "amountOfSubstanceConcentration") (declared-name "amountOfSubstanceConcentration") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (name "amountOfSubstanceFractionMoleFraction") (declared-name "amountOfSubstanceFractionMoleFraction") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (name "angleOfOpticalRotation") (declared-name "angleOfOpticalRotation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (name "canonicalPartitionFunction") (declared-name "canonicalPartitionFunction") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (name "chemicalPotential") (declared-name "chemicalPotential") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (name "currentFractionOfTheIonB") (declared-name "currentFractionOfTheIonB"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (name "degeneracy") (declared-name "degeneracy") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (name "degreeOfDissociation") (declared-name "degreeOfDissociation") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (name "diffusionCoefficient") (declared-name "diffusionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (name "dissociationFraction") (declared-name "dissociationFraction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (name "electrolyticConductivity") (declared-name "electrolyticConductivity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (name "enthalpyOfPhaseTransition") (declared-name "enthalpyOfPhaseTransition"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (name "equilibriumConstantOnConcentrationBasis") (declared-name "equilibriumConstantOnConcentrationBasis") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (name "equilibriumConstantOnPressureBasis") (declared-name "equilibriumConstantOnPressureBasis") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (name "extentOfReaction") (declared-name "extentOfReaction") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (name "fugacity") (declared-name "fugacity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (name "grandCanonicalPartitionFunction") (declared-name "grandCanonicalPartitionFunction") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (name "grandPartitionFunction") (declared-name "grandPartitionFunction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (name "ionicStrength") (declared-name "ionicStrength") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (name "latentHeatOfPhaseTransition") (declared-name "latentHeatOfPhaseTransition") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (name "massConcentration") (declared-name "massConcentration") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (name "massFraction") (declared-name "massFraction") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (name "meanFreePath") (declared-name "meanFreePath") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (name "microcanonicalPartitionFunction") (declared-name "microcanonicalPartitionFunction") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (name "molality") (declared-name "molality") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (name "molarConductivity") (declared-name "molarConductivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (name "molarEnthalpy") (declared-name "molarEnthalpy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (name "molarEntropy") (declared-name "molarEntropy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (name "molarGasConstant") (declared-name "molarGasConstant") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (name "molarGibbsEnergy") (declared-name "molarGibbsEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (name "molarHeatCapacity") (declared-name "molarHeatCapacity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (name "molarHelmholtzEnergy") (declared-name "molarHelmholtzEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (name "molarInternalEnergy") (declared-name "molarInternalEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (name "molarMass") (declared-name "molarMass") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (name "molarOpticalRotatoryPower") (declared-name "molarOpticalRotatoryPower") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (name "molarVolume") (declared-name "molarVolume") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (name "molecularConcentration") (declared-name "molecularConcentration") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (name "molecularPartitionFunction") (declared-name "molecularPartitionFunction") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::multiplicity"))) (name "multiplicity") (declared-name "multiplicity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (name "numberOfEntities") (declared-name "numberOfEntities") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfMoles"))) (name "numberOfMoles") (declared-name "numberOfMoles"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (name "osmoticCoefficientOfSolventA") (declared-name "osmoticCoefficientOfSolventA"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (name "osmoticFactorOfSolvent") (declared-name "osmoticFactorOfSolvent") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (name "osmoticPressure") (declared-name "osmoticPressure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (name "partialPressure") (declared-name "partialPressure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (name "particleConcentration") (declared-name "particleConcentration") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (name "partitionFunctionOfAMolecule") (declared-name "partitionFunctionOfAMolecule"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (name "relativeActivityOfSolute") (declared-name "relativeActivityOfSolute"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (name "relativeActivityOfSolvent") (declared-name "relativeActivityOfSolvent"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (name "relativeAtomicMass") (declared-name "relativeAtomicMass") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (name "specificOpticalRotatoryPower") (declared-name "specificOpticalRotatoryPower") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (name "standardAbsoluteActivityInMixture") (declared-name "standardAbsoluteActivityInMixture") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (name "standardAbsoluteActivityInSolution") (declared-name "standardAbsoluteActivityInSolution") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (name "standardAbsoluteActivityOfSolvent") (declared-name "standardAbsoluteActivityOfSolvent") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (name "standardAmountOfSubstanceConcentration") (declared-name "standardAmountOfSubstanceConcentration") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (name "standardChemicalPotential") (declared-name "standardChemicalPotential") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (name "standardEquilibriumConstant") (declared-name "standardEquilibriumConstant") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (name "statisticalWeightOfSubsystem") (declared-name "statisticalWeightOfSubsystem") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (name "stoichiometricNumberOfSubstance") (declared-name "stoichiometricNumberOfSubstance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (name "thermalDiffusionCoefficient") (declared-name "thermalDiffusionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (name "thermalDiffusionFactor") (declared-name "thermalDiffusionFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (name "thermalDiffusionRatio") (declared-name "thermalDiffusionRatio") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (name "thermodynamicEquilibriumConstant") (declared-name "thermodynamicEquilibriumConstant"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (name "transportNumberOfTheIonB") (declared-name "transportNumberOfTheIonB") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (name "volumeFraction") (declared-name "volumeFraction") (declared (properties (ordered false) (unique false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem::_documentation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (to (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
