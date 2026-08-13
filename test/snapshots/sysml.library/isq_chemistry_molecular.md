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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/isq_chemistry_molecular.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 19) (end 20 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 19) (end 21 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 32) (end 24 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 32) (end 42 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 33) (end 43 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 28) (end 44 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 45) (end 47 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 64 36) (end 64 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 77 22) (end 77 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 27) (end 77 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 78 22) (end 78 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 83 35) (end 83 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 34) (end 84 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 84 63) (end 84 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 84 85) (end 84 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 47) (end 85 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 85 76) (end 85 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 85 98) (end 85 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 86 22) (end 86 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 86 46) (end 86 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 90 38) (end 90 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 103 22) (end 103 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 27) (end 103 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 104 22) (end 104 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 109 37) (end 109 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 36) (end 110 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 110 65) (end 110 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 110 87) (end 110 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 47) (end 111 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 111 76) (end 111 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 111 98) (end 111 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 112 22) (end 112 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 112 46) (end 112 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 116 46) (end 116 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 129 22) (end 129 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 27) (end 129 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 130 22) (end 130 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 135 45) (end 135 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 36) (end 136 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 136 65) (end 136 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 136 87) (end 136 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 137 34) (end 137 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 137 63) (end 137 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 137 85) (end 137 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 38) (end 138 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 138 67) (end 138 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 138 89) (end 138 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 47) (end 139 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 139 76) (end 139 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 139 98) (end 139 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 140 22) (end 140 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 140 46) (end 140 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 144 40) (end 144 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 157 22) (end 157 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 157 27) (end 157 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 158 22) (end 158 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 163 39) (end 163 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 36) (end 164 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 164 65) (end 164 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 164 87) (end 164 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 34) (end 165 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 165 63) (end 165 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 165 85) (end 165 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 38) (end 166 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 166 67) (end 166 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 166 89) (end 166 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 47) (end 167 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 167 76) (end 167 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 167 98) (end 167 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 168 22) (end 168 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 168 46) (end 168 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 172 47) (end 172 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 185 22) (end 185 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 27) (end 185 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 186 22) (end 186 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 191 46) (end 191 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 192 36) (end 192 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 192 65) (end 192 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 192 87) (end 192 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 193 34) (end 193 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 193 63) (end 193 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 193 85) (end 193 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 194 38) (end 194 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 194 67) (end 194 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 194 89) (end 194 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 195 47) (end 195 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 195 76) (end 195 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 195 98) (end 195 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 196 22) (end 196 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 196 46) (end 196 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 200 43) (end 200 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 213 22) (end 213 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 27) (end 213 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 214 22) (end 214 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 219 42) (end 219 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 36) (end 220 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 220 65) (end 220 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 220 87) (end 220 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 221 34) (end 221 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 221 63) (end 221 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 221 85) (end 221 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 38) (end 222 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 222 67) (end 222 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 222 89) (end 222 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 223 47) (end 223 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 223 76) (end 223 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 223 98) (end 223 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 224 22) (end 224 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 224 46) (end 224 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 228 44) (end 228 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 241 22) (end 241 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 241 27) (end 241 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 242 22) (end 242 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 247 43) (end 247 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 248 36) (end 248 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 248 65) (end 248 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 248 87) (end 248 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 249 34) (end 249 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 249 63) (end 249 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 249 85) (end 249 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 250 38) (end 250 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 250 67) (end 250 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 250 89) (end 250 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 251 54) (end 251 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 251 83) (end 251 91))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 251 108) (end 251 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 252 47) (end 252 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 252 76) (end 252 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 252 98) (end 252 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 253 22) (end 253 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 253 46) (end 253 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 257 39) (end 257 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 270 22) (end 270 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 270 27) (end 270 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 271 22) (end 271 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 276 38) (end 276 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 277 36) (end 277 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 277 65) (end 277 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 277 87) (end 277 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 278 34) (end 278 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 278 63) (end 278 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 278 85) (end 278 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 279 38) (end 279 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 279 67) (end 279 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 279 89) (end 279 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 280 54) (end 280 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 280 83) (end 280 91))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 280 108) (end 280 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 281 47) (end 281 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 281 76) (end 281 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 281 98) (end 281 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 282 22) (end 282 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 282 46) (end 282 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 286 48) (end 286 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 299 22) (end 299 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 299 27) (end 299 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 300 22) (end 300 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 305 47) (end 305 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 306 36) (end 306 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 306 65) (end 306 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 306 87) (end 306 95))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 307 22) (end 307 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 307 46) (end 307 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 327 44) (end 327 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 340 22) (end 340 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 340 27) (end 340 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 341 22) (end 341 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 346 43) (end 346 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 347 36) (end 347 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 347 65) (end 347 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 347 87) (end 347 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 348 34) (end 348 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 348 63) (end 348 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 348 85) (end 348 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 349 22) (end 349 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 349 46) (end 349 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 353 39) (end 353 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 370 57) (end 370 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 383 22) (end 383 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 383 27) (end 383 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 384 22) (end 384 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 389 56) (end 389 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 390 36) (end 390 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 390 65) (end 390 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 390 87) (end 390 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 391 47) (end 391 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 391 76) (end 391 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 391 98) (end 391 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 392 22) (end 392 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 392 46) (end 392 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 412 64) (end 412 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 429 41) (end 429 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 442 22) (end 442 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 442 27) (end 442 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 443 22) (end 443 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 448 40) (end 448 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 452 35) (end 452 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 465 22) (end 465 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 465 27) (end 465 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 466 22) (end 466 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 471 34) (end 471 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 472 34) (end 472 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 472 63) (end 472 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 472 85) (end 472 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 473 47) (end 473 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 473 76) (end 473 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 473 98) (end 473 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 474 22) (end 474 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 474 46) (end 474 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 478 43) (end 478 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 496 44) (end 496 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 509 22) (end 509 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 509 27) (end 509 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 510 22) (end 510 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 515 43) (end 515 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 516 36) (end 516 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 516 65) (end 516 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 516 87) (end 516 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 517 34) (end 517 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 517 63) (end 517 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 517 85) (end 517 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 518 38) (end 518 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 518 67) (end 518 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 518 89) (end 518 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 519 47) (end 519 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 519 76) (end 519 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 519 98) (end 519 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 520 22) (end 520 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 520 46) (end 520 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 524 43) (end 524 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 541 42) (end 541 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 554 22) (end 554 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 554 27) (end 554 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 555 22) (end 555 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 560 41) (end 560 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 561 36) (end 561 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 561 65) (end 561 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 561 87) (end 561 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 562 34) (end 562 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 562 63) (end 562 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 562 85) (end 562 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 563 38) (end 563 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 563 67) (end 563 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 563 89) (end 563 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 564 22) (end 564 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 564 46) (end 564 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 568 35) (end 568 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 581 22) (end 581 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 581 27) (end 581 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 582 22) (end 582 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 587 34) (end 587 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 588 36) (end 588 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 588 65) (end 588 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 588 87) (end 588 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 589 34) (end 589 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 589 63) (end 589 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 589 85) (end 589 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 590 38) (end 590 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 590 67) (end 590 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 590 89) (end 590 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 591 22) (end 591 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 591 46) (end 591 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 595 52) (end 595 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 608 22) (end 608 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 608 27) (end 608 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 609 22) (end 609 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 614 51) (end 614 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 615 36) (end 615 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 615 65) (end 615 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 615 87) (end 615 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 616 34) (end 616 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 616 63) (end 616 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 616 85) (end 616 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 617 38) (end 617 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 617 67) (end 617 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 617 89) (end 617 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 618 47) (end 618 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 618 76) (end 618 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 618 98) (end 618 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 619 22) (end 619 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 619 46) (end 619 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 623 41) (end 623 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 640 60) (end 640 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 657 43) (end 657 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 676 46) (end 676 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 693 61) (end 693 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 710 44) (end 710 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 729 49) (end 729 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 748 60) (end 748 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 765 42) (end 765 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 778 22) (end 778 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 778 27) (end 778 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 779 22) (end 779 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 784 41) (end 784 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 785 36) (end 785 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 785 65) (end 785 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 785 87) (end 785 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 786 34) (end 786 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 786 63) (end 786 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 786 85) (end 786 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 787 38) (end 787 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 787 67) (end 787 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 787 89) (end 787 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 788 22) (end 788 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 788 46) (end 788 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 792 58) (end 792 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 809 54) (end 809 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 822 22) (end 822 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 822 27) (end 822 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 823 22) (end 823 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 828 53) (end 828 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 829 36) (end 829 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 829 65) (end 829 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 829 87) (end 829 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 830 34) (end 830 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 830 63) (end 830 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 830 85) (end 830 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 831 38) (end 831 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 831 67) (end 831 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 831 89) (end 831 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 832 47) (end 832 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 832 76) (end 832 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 832 98) (end 832 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 833 22) (end 833 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 833 46) (end 833 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 837 32) (end 837 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 853 54) (end 853 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 872 61) (end 872 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 885 22) (end 885 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 885 27) (end 885 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 886 22) (end 886 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 891 60) (end 891 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 892 36) (end 892 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 892 65) (end 892 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 892 87) (end 892 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 893 34) (end 893 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 893 63) (end 893 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 893 85) (end 893 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 894 38) (end 894 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 894 67) (end 894 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 894 89) (end 894 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 895 22) (end 895 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 895 46) (end 895 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 899 66) (end 899 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 912 22) (end 912 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 912 27) (end 912 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 913 22) (end 913 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 918 65) (end 918 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 919 36) (end 919 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 919 65) (end 919 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 919 87) (end 919 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 920 47) (end 920 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 920 76) (end 920 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 920 98) (end 920 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 921 22) (end 921 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 921 46) (end 921 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 925 47) (end 925 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 941 53) (end 941 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 958 58) (end 958 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 977 53) (end 977 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 996 44) (end 996 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1012 37) (end 1012 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1031 43) (end 1031 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1044 22) (end 1044 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1044 27) (end 1044 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1045 22) (end 1045 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1050 42) (end 1050 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1051 36) (end 1051 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1051 65) (end 1051 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1051 87) (end 1051 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1052 34) (end 1052 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1052 63) (end 1052 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1052 85) (end 1052 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1053 38) (end 1053 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1053 67) (end 1053 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1053 89) (end 1053 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1054 54) (end 1054 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1054 83) (end 1054 91))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1054 108) (end 1054 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1055 47) (end 1055 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1055 76) (end 1055 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1055 98) (end 1055 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1056 22) (end 1056 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1056 46) (end 1056 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1063 28) (end 1063 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1079 47) (end 1079 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1092 22) (end 1092 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1092 27) (end 1092 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1093 22) (end 1093 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1098 46) (end 1098 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1099 36) (end 1099 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1099 65) (end 1099 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1099 87) (end 1099 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1100 38) (end 1100 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1100 67) (end 1100 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1100 89) (end 1100 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1101 22) (end 1101 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1101 46) (end 1101 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1105 48) (end 1105 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1122 49) (end 1122 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1139 54) (end 1139 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1152 22) (end 1152 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1152 27) (end 1152 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1153 22) (end 1153 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1158 53) (end 1158 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1159 36) (end 1159 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1159 65) (end 1159 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1159 87) (end 1159 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1160 38) (end 1160 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1160 67) (end 1160 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1160 89) (end 1160 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1161 22) (end 1161 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1161 46) (end 1161 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1165 40) (end 1165 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1178 22) (end 1178 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1178 27) (end 1178 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1179 22) (end 1179 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1184 39) (end 1184 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1185 34) (end 1185 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1185 63) (end 1185 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1185 85) (end 1185 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1186 47) (end 1186 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1186 76) (end 1186 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1186 98) (end 1186 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1187 22) (end 1187 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1187 46) (end 1187 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1191 47) (end 1191 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1210 51) (end 1210 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1223 22) (end 1223 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1223 27) (end 1223 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1224 22) (end 1224 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1229 50) (end 1229 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1230 36) (end 1230 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1230 65) (end 1230 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1230 87) (end 1230 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1231 34) (end 1231 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1231 63) (end 1231 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1231 85) (end 1231 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1232 38) (end 1232 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1232 67) (end 1232 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1232 89) (end 1232 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1233 45) (end 1233 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1233 74) (end 1233 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1233 96) (end 1233 104))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1234 22) (end 1234 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1234 46) (end 1234 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1238 44) (end 1238 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1251 22) (end 1251 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1251 27) (end 1251 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1252 22) (end 1252 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1257 43) (end 1257 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1258 34) (end 1258 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1258 63) (end 1258 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1258 85) (end 1258 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1259 38) (end 1259 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1259 67) (end 1259 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1259 89) (end 1259 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1260 45) (end 1260 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1260 74) (end 1260 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1260 96) (end 1260 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1261 47) (end 1261 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1261 76) (end 1261 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1261 98) (end 1261 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1262 22) (end 1262 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1262 46) (end 1262 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1266 51) (end 1266 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1285 38) (end 1285 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1301 52) (end 1301 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1314 22) (end 1314 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1314 27) (end 1314 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1315 22) (end 1315 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1320 51) (end 1320 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1321 36) (end 1321 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1321 65) (end 1321 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1321 87) (end 1321 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1322 47) (end 1322 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1322 76) (end 1322 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1322 98) (end 1322 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1323 22) (end 1323 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1323 46) (end 1323 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1327 55) (end 1327 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1340 22) (end 1340 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1340 27) (end 1340 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1341 22) (end 1341 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1346 54) (end 1346 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1347 36) (end 1347 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1347 65) (end 1347 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1347 87) (end 1347 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1348 34) (end 1348 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1348 63) (end 1348 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1348 85) (end 1348 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1349 22) (end 1349 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 1349 46) (end 1349 66))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1b76a845b58c298bff89a5b6d72efefad50ebee987087f13acb3359c94d98132") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQBase") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::AngularMeasureValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQThermodynamics::EnergyValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AffinityOfAChemicalReactionUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceConcentrationUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChemicalPotentialUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiffusionCoefficientUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectrolyticConductivityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EquilibriumConstantOnConcentrationBasisUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EquilibriumConstantOnPressureBasisUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FugacityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IonicStrengthUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassConcentrationUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolalityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarConductivityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarEnthalpyUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarEntropyUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarGasConstantUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarGibbsEnergyUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarHeatCapacityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarHelmholtzEnergyUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarInternalEnergyUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarMassUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarOpticalRotatoryPowerUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MolarVolumeUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::NumberOfMolesUnit"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "AmountOfSubstanceUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::NumberOfMolesValue"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "AmountOfSubstanceValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OsmoticPressureUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PartialPressureUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ParticleConcentrationUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpecificOpticalRotatoryPowerUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardChemicalPotentialUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThermalDiffusionCoefficientUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VolumeFractionUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AbsoluteActivityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ActivityCoefficientValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ActivityFactorValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ActivityOfSoluteValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ActivityOfSolventValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AffinityOfAChemicalReactionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceConcentrationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceFractionMoleFractionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AngularMeasureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CanonicalPartitionFunctionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ChemicalPotentialValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "transportNumberOfTheIonB"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DegeneracyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DegreeOfDissociationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DiffusionCoefficientValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "degreeOfDissociation"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ElectrolyticConductivityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "latentHeatOfPhaseTransition"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EquilibriumConstantOnConcentrationBasisValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EquilibriumConstantOnPressureBasisValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FugacityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GrandCanonicalPartitionFunctionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "grandCanonicalPartitionFunction"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "IonicStrengthValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EnergyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MassConcentrationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MassFractionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CountValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molality"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolalityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarConductivityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarEnthalpyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarEntropyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarGasConstantValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarGibbsEnergyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarHeatCapacityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarHelmholtzEnergyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarInternalEnergyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarMassValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarOpticalRotatoryPowerValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolarVolumeValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ParticleConcentrationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MolecularPartitionFunctionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::multiplicity"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "degeneracy"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CountValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::numberOfMoles"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "amountOfSubstance"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "osmoticFactorOfSolvent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "OsmoticFactorOfSolventValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "OsmoticPressureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PartialPressureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ParticleConcentrationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "molecularPartitionFunction"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "activityOfSolute"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "activityOfSolvent"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "RelativeAtomicMassValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SpecificOpticalRotatoryPowerValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StandardAbsoluteActivityInMixtureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StandardAbsoluteActivityInSolutionValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StandardAbsoluteActivityOfSolventValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceConcentrationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StandardChemicalPotentialValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StandardEquilibriumConstantValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CountValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StoichiometricNumberOfSubstanceValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ThermalDiffusionCoefficientValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ThermalDiffusionFactorValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ThermalDiffusionRatioValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "standardEquilibriumConstant"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "TransportNumberOfTheIonBValue"))))
    (declaration (id (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "VolumeFractionValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQBase")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::AngularMeasureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQThermodynamics::EnergyValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "AffinityOfAChemicalReactionUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceConcentrationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "ChemicalPotentialUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "DiffusionCoefficientUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectrolyticConductivityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "EquilibriumConstantOnConcentrationBasisUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "EquilibriumConstantOnPressureBasisUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "FugacityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "IonicStrengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassConcentrationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolalityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarConductivityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarEnthalpyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarEntropyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarGasConstantUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarGibbsEnergyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarHeatCapacityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarHelmholtzEnergyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarInternalEnergyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarMassUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarOpticalRotatoryPowerUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::NumberOfMolesUnit"))) (kind aliasBinding) (ordinal 0))
      (authored-target "AmountOfSubstanceUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::NumberOfMolesValue"))) (kind aliasBinding) (ordinal 0))
      (authored-target "AmountOfSubstanceValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "OsmoticPressureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "PartialPressureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "ParticleConcentrationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SpecificOpticalRotatoryPowerUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardChemicalPotentialUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermalDiffusionCoefficientUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeFractionUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind featureTyping) (ordinal 0))
      (authored-target "AbsoluteActivityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActivityCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActivityFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityFactorValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActivityOfSoluteValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActivityOfSolventValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind featureTyping) (ordinal 0))
      (authored-target "AffinityOfAChemicalReactionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceFractionMoleFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind featureTyping) (ordinal 0))
      (authored-target "AngularMeasureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind featureTyping) (ordinal 0))
      (authored-target "CanonicalPartitionFunctionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChemicalPotentialValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (kind aliasBinding) (ordinal 0))
      (authored-target "transportNumberOfTheIonB")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind featureTyping) (ordinal 0))
      (authored-target "DegeneracyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegeneracyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind featureTyping) (ordinal 0))
      (authored-target "DegreeOfDissociationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiffusionCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (kind aliasBinding) (ordinal 0))
      (authored-target "degreeOfDissociation")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectrolyticConductivityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (kind aliasBinding) (ordinal 0))
      (authored-target "latentHeatOfPhaseTransition")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind featureTyping) (ordinal 0))
      (authored-target "EquilibriumConstantOnConcentrationBasisValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind featureTyping) (ordinal 0))
      (authored-target "EquilibriumConstantOnPressureBasisValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind featureTyping) (ordinal 0))
      (authored-target "FugacityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0))
      (authored-target "GrandCanonicalPartitionFunctionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (kind aliasBinding) (ordinal 0))
      (authored-target "grandCanonicalPartitionFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind featureTyping) (ordinal 0))
      (authored-target "IonicStrengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind featureTyping) (ordinal 0))
      (authored-target "EnergyValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassFractionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0))
      (authored-target "CountValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molality"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolalityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarConductivityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarEnthalpyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarEntropyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarGasConstantValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarGibbsEnergyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarHeatCapacityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarHelmholtzEnergyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarInternalEnergyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarMassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarOpticalRotatoryPowerValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolarVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind featureTyping) (ordinal 0))
      (authored-target "ParticleConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind featureTyping) (ordinal 0))
      (authored-target "MolecularPartitionFunctionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::multiplicity"))) (kind aliasBinding) (ordinal 0))
      (authored-target "degeneracy")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind featureTyping) (ordinal 0))
      (authored-target "CountValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::numberOfMoles"))) (kind aliasBinding) (ordinal 0))
      (authored-target "amountOfSubstance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (kind aliasBinding) (ordinal 0))
      (authored-target "osmoticFactorOfSolvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind featureTyping) (ordinal 0))
      (authored-target "OsmoticFactorOfSolventValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind featureTyping) (ordinal 0))
      (authored-target "OsmoticPressureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind featureTyping) (ordinal 0))
      (authored-target "PartialPressureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind featureTyping) (ordinal 0))
      (authored-target "ParticleConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (kind aliasBinding) (ordinal 0))
      (authored-target "molecularPartitionFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (kind aliasBinding) (ordinal 0))
      (authored-target "activityOfSolute")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (kind aliasBinding) (ordinal 0))
      (authored-target "activityOfSolvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "RelativeAtomicMassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpecificOpticalRotatoryPowerValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardAbsoluteActivityInMixtureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardAbsoluteActivityInSolutionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardAbsoluteActivityOfSolventValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardChemicalPotentialValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardEquilibriumConstantValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "CountValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind featureTyping) (ordinal 0))
      (authored-target "StoichiometricNumberOfSubstanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermalDiffusionCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermalDiffusionFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermalDiffusionRatioValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (kind aliasBinding) (ordinal 0))
      (authored-target "standardEquilibriumConstant")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransportNumberOfTheIonBValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityFactor"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::fugacity"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massConcentration"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massFraction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molality"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molality"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarMass"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarVolume"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::multiplicity"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::multiplicity"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partialPressure"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 15 19) (end 15 32)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 16 19) (end 16 43)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 17 19) (end 17 29)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQBase")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 14 19) (end 14 37)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 20 19) (end 20 52)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::AngularMeasureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 21 19) (end 21 49)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ISQThermodynamics::EnergyValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 524 43) (end 524 60)) (probe (position 524 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 676 46) (end 676 63)) (probe (position 676 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 623 41) (end 623 58)) (probe (position 623 41))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 657 43) (end 657 60)) (probe (position 657 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 710 44) (end 710 61)) (probe (position 710 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 828 53) (end 828 64)) (probe (position 828 53))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 833 22) (end 833 39)) (probe (position 833 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 833 46) (end 833 66)) (probe (position 833 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 832 47) (end 832 66)) (probe (position 832 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 832 76) (end 832 84)) (probe (position 832 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 832 98) (end 832 106)) (probe (position 832 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 831 38) (end 831 57)) (probe (position 831 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 831 67) (end 831 75)) (probe (position 831 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 831 89) (end 831 97)) (probe (position 831 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 829 36) (end 829 55)) (probe (position 829 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 829 65) (end 829 73)) (probe (position 829 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 829 87) (end 829 95)) (probe (position 829 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 830 34) (end 830 53)) (probe (position 830 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 830 63) (end 830 71)) (probe (position 830 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 830 85) (end 830 93)) (probe (position 830 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 809 54) (end 809 73)) (probe (position 809 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 822 27) (end 822 31)) (probe (position 822 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 823 28) (end 823 59)) (probe (position 823 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "AffinityOfAChemicalReactionUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 822 22) (end 822 25)) (probe (position 822 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 823 22) (end 823 26)) (probe (position 823 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 389 56) (end 389 67)) (probe (position 389 56))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 392 22) (end 392 39)) (probe (position 392 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 392 46) (end 392 66)) (probe (position 392 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 391 47) (end 391 66)) (probe (position 391 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 391 76) (end 391 84)) (probe (position 391 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 391 98) (end 391 106)) (probe (position 391 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 390 36) (end 390 55)) (probe (position 390 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 390 65) (end 390 73)) (probe (position 390 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 390 87) (end 390 95)) (probe (position 390 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 370 57) (end 370 76)) (probe (position 370 57))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 383 27) (end 383 31)) (probe (position 383 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 384 28) (end 384 62)) (probe (position 384 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceConcentrationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 383 22) (end 383 25)) (probe (position 383 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 384 22) (end 384 26)) (probe (position 384 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 412 64) (end 412 81)) (probe (position 412 64))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 941 53) (end 941 70)) (probe (position 941 53))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 515 43) (end 515 54)) (probe (position 515 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 520 22) (end 520 39)) (probe (position 520 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 520 46) (end 520 66)) (probe (position 520 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 519 47) (end 519 66)) (probe (position 519 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 519 76) (end 519 84)) (probe (position 519 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 519 98) (end 519 106)) (probe (position 519 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 518 38) (end 518 57)) (probe (position 518 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 518 67) (end 518 75)) (probe (position 518 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 518 89) (end 518 97)) (probe (position 518 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 516 36) (end 516 55)) (probe (position 516 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 516 65) (end 516 73)) (probe (position 516 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 516 87) (end 516 95)) (probe (position 516 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 517 34) (end 517 53)) (probe (position 517 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 517 63) (end 517 71)) (probe (position 517 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 517 85) (end 517 93)) (probe (position 517 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 496 44) (end 496 63)) (probe (position 496 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 509 27) (end 509 31)) (probe (position 509 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 510 28) (end 510 49)) (probe (position 510 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "ChemicalPotentialUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 509 22) (end 509 25)) (probe (position 509 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 510 22) (end 510 26)) (probe (position 510 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1012 37) (end 1012 54)) (probe (position 1012 37))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1191 47) (end 1191 64)) (probe (position 1191 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1098 46) (end 1098 57)) (probe (position 1098 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1101 22) (end 1101 39)) (probe (position 1101 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1101 46) (end 1101 66)) (probe (position 1101 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1100 38) (end 1100 57)) (probe (position 1100 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1100 67) (end 1100 75)) (probe (position 1100 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1100 89) (end 1100 97)) (probe (position 1100 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1099 36) (end 1099 55)) (probe (position 1099 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1099 65) (end 1099 73)) (probe (position 1099 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1099 87) (end 1099 95)) (probe (position 1099 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1079 47) (end 1079 66)) (probe (position 1079 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1092 27) (end 1092 31)) (probe (position 1092 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1093 28) (end 1093 52)) (probe (position 1093 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "DiffusionCoefficientUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1092 22) (end 1092 25)) (probe (position 1092 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1093 22) (end 1093 26)) (probe (position 1093 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1229 50) (end 1229 61)) (probe (position 1229 50))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1234 22) (end 1234 39)) (probe (position 1234 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1234 46) (end 1234 66)) (probe (position 1234 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1232 38) (end 1232 57)) (probe (position 1232 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1232 67) (end 1232 75)) (probe (position 1232 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1232 89) (end 1232 97)) (probe (position 1232 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1233 45) (end 1233 64)) (probe (position 1233 45))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1233 74) (end 1233 82)) (probe (position 1233 74))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1233 96) (end 1233 104)) (probe (position 1233 96))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1230 36) (end 1230 55)) (probe (position 1230 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1230 65) (end 1230 73)) (probe (position 1230 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1230 87) (end 1230 95)) (probe (position 1230 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1231 34) (end 1231 53)) (probe (position 1231 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1231 63) (end 1231 71)) (probe (position 1231 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1231 85) (end 1231 93)) (probe (position 1231 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1210 51) (end 1210 70)) (probe (position 1210 51))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1223 27) (end 1223 31)) (probe (position 1223 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1224 28) (end 1224 56)) (probe (position 1224 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "ElectrolyticConductivityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1223 22) (end 1223 25)) (probe (position 1223 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1224 22) (end 1224 26)) (probe (position 1224 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 918 65) (end 918 76)) (probe (position 918 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 921 22) (end 921 39)) (probe (position 921 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 921 46) (end 921 66)) (probe (position 921 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 920 47) (end 920 66)) (probe (position 920 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 920 76) (end 920 84)) (probe (position 920 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 920 98) (end 920 106)) (probe (position 920 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 919 36) (end 919 55)) (probe (position 919 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 919 65) (end 919 73)) (probe (position 919 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 919 87) (end 919 95)) (probe (position 919 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 899 66) (end 899 85)) (probe (position 899 66))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 912 27) (end 912 31)) (probe (position 912 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 913 28) (end 913 71)) (probe (position 913 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "EquilibriumConstantOnConcentrationBasisUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 912 22) (end 912 25)) (probe (position 912 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 913 22) (end 913 26)) (probe (position 913 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 891 60) (end 891 71)) (probe (position 891 60))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 895 22) (end 895 39)) (probe (position 895 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 895 46) (end 895 66)) (probe (position 895 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 894 38) (end 894 57)) (probe (position 894 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 894 67) (end 894 75)) (probe (position 894 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 894 89) (end 894 97)) (probe (position 894 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 892 36) (end 892 55)) (probe (position 892 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 892 65) (end 892 73)) (probe (position 892 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 892 87) (end 892 95)) (probe (position 892 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 893 34) (end 893 53)) (probe (position 893 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 893 63) (end 893 71)) (probe (position 893 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 893 85) (end 893 93)) (probe (position 893 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 872 61) (end 872 80)) (probe (position 872 61))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 885 27) (end 885 31)) (probe (position 885 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 886 28) (end 886 66)) (probe (position 886 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "EquilibriumConstantOnPressureBasisUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 885 22) (end 885 25)) (probe (position 885 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 886 22) (end 886 26)) (probe (position 886 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 587 34) (end 587 45)) (probe (position 587 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 591 22) (end 591 39)) (probe (position 591 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 591 46) (end 591 66)) (probe (position 591 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 590 38) (end 590 57)) (probe (position 590 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 590 67) (end 590 75)) (probe (position 590 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 590 89) (end 590 97)) (probe (position 590 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 588 36) (end 588 55)) (probe (position 588 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 588 65) (end 588 73)) (probe (position 588 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 588 87) (end 588 95)) (probe (position 588 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 589 34) (end 589 53)) (probe (position 589 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 589 63) (end 589 71)) (probe (position 589 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 589 85) (end 589 93)) (probe (position 589 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 568 35) (end 568 54)) (probe (position 568 35))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 581 27) (end 581 31)) (probe (position 581 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 582 28) (end 582 40)) (probe (position 582 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "FugacityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 581 22) (end 581 25)) (probe (position 581 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 582 22) (end 582 26)) (probe (position 582 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 958 58) (end 958 75)) (probe (position 958 58))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1184 39) (end 1184 50)) (probe (position 1184 39))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1187 22) (end 1187 39)) (probe (position 1187 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1187 46) (end 1187 66)) (probe (position 1187 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1186 47) (end 1186 66)) (probe (position 1186 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1186 76) (end 1186 84)) (probe (position 1186 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1186 98) (end 1186 106)) (probe (position 1186 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1185 34) (end 1185 53)) (probe (position 1185 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1185 63) (end 1185 71)) (probe (position 1185 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1185 85) (end 1185 93)) (probe (position 1185 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1165 40) (end 1165 59)) (probe (position 1165 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1178 27) (end 1178 31)) (probe (position 1178 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1179 28) (end 1179 45)) (probe (position 1179 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "IonicStrengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1178 22) (end 1178 25)) (probe (position 1178 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1179 22) (end 1179 26)) (probe (position 1179 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 346 43) (end 346 54)) (probe (position 346 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 349 22) (end 349 39)) (probe (position 349 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 349 46) (end 349 66)) (probe (position 349 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 347 36) (end 347 55)) (probe (position 347 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 347 65) (end 347 73)) (probe (position 347 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 347 87) (end 347 95)) (probe (position 347 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 348 34) (end 348 53)) (probe (position 348 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 348 63) (end 348 71)) (probe (position 348 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 348 85) (end 348 93)) (probe (position 348 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 327 44) (end 327 63)) (probe (position 327 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 340 27) (end 340 31)) (probe (position 340 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 341 28) (end 341 49)) (probe (position 341 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MassConcentrationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 340 22) (end 340 25)) (probe (position 340 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 341 22) (end 341 26)) (probe (position 341 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 353 39) (end 353 56)) (probe (position 353 39))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 471 34) (end 471 45)) (probe (position 471 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 474 22) (end 474 39)) (probe (position 474 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 474 46) (end 474 66)) (probe (position 474 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 473 47) (end 473 66)) (probe (position 473 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 473 76) (end 473 84)) (probe (position 473 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 473 98) (end 473 106)) (probe (position 473 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 472 34) (end 472 53)) (probe (position 472 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 472 63) (end 472 71)) (probe (position 472 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 472 85) (end 472 93)) (probe (position 472 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 452 35) (end 452 54)) (probe (position 452 35))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 465 27) (end 465 31)) (probe (position 465 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 466 28) (end 466 40)) (probe (position 466 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolalityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 465 22) (end 465 25)) (probe (position 465 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 466 22) (end 466 26)) (probe (position 466 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1257 43) (end 1257 54)) (probe (position 1257 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1262 22) (end 1262 39)) (probe (position 1262 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1262 46) (end 1262 66)) (probe (position 1262 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1261 47) (end 1261 66)) (probe (position 1261 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1261 76) (end 1261 84)) (probe (position 1261 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1261 98) (end 1261 106)) (probe (position 1261 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1259 38) (end 1259 57)) (probe (position 1259 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1259 67) (end 1259 75)) (probe (position 1259 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1259 89) (end 1259 97)) (probe (position 1259 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1260 45) (end 1260 64)) (probe (position 1260 45))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1260 74) (end 1260 82)) (probe (position 1260 74))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1260 96) (end 1260 104)) (probe (position 1260 96))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1258 34) (end 1258 53)) (probe (position 1258 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1258 63) (end 1258 71)) (probe (position 1258 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1258 85) (end 1258 93)) (probe (position 1258 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1238 44) (end 1238 63)) (probe (position 1238 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1251 27) (end 1251 31)) (probe (position 1251 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1252 28) (end 1252 49)) (probe (position 1252 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarConductivityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1251 22) (end 1251 25)) (probe (position 1251 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1252 22) (end 1252 26)) (probe (position 1252 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 163 39) (end 163 50)) (probe (position 163 39))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 168 22) (end 168 39)) (probe (position 168 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 168 46) (end 168 66)) (probe (position 168 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 167 47) (end 167 66)) (probe (position 167 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 167 76) (end 167 84)) (probe (position 167 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 167 98) (end 167 106)) (probe (position 167 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 166 38) (end 166 57)) (probe (position 166 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 166 67) (end 166 75)) (probe (position 166 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 166 89) (end 166 97)) (probe (position 166 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 164 36) (end 164 55)) (probe (position 164 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 164 65) (end 164 73)) (probe (position 164 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 164 87) (end 164 95)) (probe (position 164 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 165 34) (end 165 53)) (probe (position 165 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 165 63) (end 165 71)) (probe (position 165 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 165 85) (end 165 93)) (probe (position 165 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 144 40) (end 144 59)) (probe (position 144 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 157 27) (end 157 31)) (probe (position 157 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 158 28) (end 158 45)) (probe (position 158 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarEnthalpyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 157 22) (end 157 25)) (probe (position 157 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 158 22) (end 158 26)) (probe (position 158 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 276 38) (end 276 49)) (probe (position 276 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 282 22) (end 282 39)) (probe (position 282 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 282 46) (end 282 66)) (probe (position 282 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 281 47) (end 281 66)) (probe (position 281 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 281 76) (end 281 84)) (probe (position 281 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 281 98) (end 281 106)) (probe (position 281 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 279 38) (end 279 57)) (probe (position 279 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 279 67) (end 279 75)) (probe (position 279 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 279 89) (end 279 97)) (probe (position 279 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 277 36) (end 277 55)) (probe (position 277 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 277 65) (end 277 73)) (probe (position 277 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 277 87) (end 277 95)) (probe (position 277 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 278 34) (end 278 53)) (probe (position 278 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 278 63) (end 278 71)) (probe (position 278 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 278 85) (end 278 93)) (probe (position 278 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 280 54) (end 280 73)) (probe (position 280 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 280 83) (end 280 91)) (probe (position 280 83))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 280 108) (end 280 116)) (probe (position 280 108))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 257 39) (end 257 58)) (probe (position 257 39))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 270 27) (end 270 31)) (probe (position 270 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 271 28) (end 271 44)) (probe (position 271 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarEntropyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 270 22) (end 270 25)) (probe (position 270 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 271 22) (end 271 26)) (probe (position 271 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1050 42) (end 1050 53)) (probe (position 1050 42))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1056 22) (end 1056 39)) (probe (position 1056 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1056 46) (end 1056 66)) (probe (position 1056 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1055 47) (end 1055 66)) (probe (position 1055 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1055 76) (end 1055 84)) (probe (position 1055 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1055 98) (end 1055 106)) (probe (position 1055 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1053 38) (end 1053 57)) (probe (position 1053 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1053 67) (end 1053 75)) (probe (position 1053 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1053 89) (end 1053 97)) (probe (position 1053 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1051 36) (end 1051 55)) (probe (position 1051 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1051 65) (end 1051 73)) (probe (position 1051 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1051 87) (end 1051 95)) (probe (position 1051 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1052 34) (end 1052 53)) (probe (position 1052 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1052 63) (end 1052 71)) (probe (position 1052 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1052 85) (end 1052 93)) (probe (position 1052 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1054 54) (end 1054 73)) (probe (position 1054 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1054 83) (end 1054 91)) (probe (position 1054 83))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1054 108) (end 1054 116)) (probe (position 1054 108))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1031 43) (end 1031 62)) (probe (position 1031 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1044 27) (end 1044 31)) (probe (position 1044 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1045 28) (end 1045 48)) (probe (position 1045 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarGasConstantUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1044 22) (end 1044 25)) (probe (position 1044 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1045 22) (end 1045 26)) (probe (position 1045 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 219 42) (end 219 53)) (probe (position 219 42))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 224 22) (end 224 39)) (probe (position 224 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 224 46) (end 224 66)) (probe (position 224 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 223 47) (end 223 66)) (probe (position 223 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 223 76) (end 223 84)) (probe (position 223 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 223 98) (end 223 106)) (probe (position 223 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 222 38) (end 222 57)) (probe (position 222 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 222 67) (end 222 75)) (probe (position 222 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 222 89) (end 222 97)) (probe (position 222 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 220 36) (end 220 55)) (probe (position 220 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 220 65) (end 220 73)) (probe (position 220 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 220 87) (end 220 95)) (probe (position 220 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 221 34) (end 221 53)) (probe (position 221 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 221 63) (end 221 71)) (probe (position 221 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 221 85) (end 221 93)) (probe (position 221 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 200 43) (end 200 62)) (probe (position 200 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 213 27) (end 213 31)) (probe (position 213 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 214 28) (end 214 48)) (probe (position 214 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarGibbsEnergyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 213 22) (end 213 25)) (probe (position 213 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 214 22) (end 214 26)) (probe (position 214 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 247 43) (end 247 54)) (probe (position 247 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 253 22) (end 253 39)) (probe (position 253 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 253 46) (end 253 66)) (probe (position 253 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 252 47) (end 252 66)) (probe (position 252 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 252 76) (end 252 84)) (probe (position 252 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 252 98) (end 252 106)) (probe (position 252 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 250 38) (end 250 57)) (probe (position 250 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 250 67) (end 250 75)) (probe (position 250 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 250 89) (end 250 97)) (probe (position 250 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 248 36) (end 248 55)) (probe (position 248 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 248 65) (end 248 73)) (probe (position 248 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 248 87) (end 248 95)) (probe (position 248 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 249 34) (end 249 53)) (probe (position 249 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 249 63) (end 249 71)) (probe (position 249 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 249 85) (end 249 93)) (probe (position 249 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 251 54) (end 251 73)) (probe (position 251 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 251 83) (end 251 91)) (probe (position 251 83))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 251 108) (end 251 116)) (probe (position 251 108))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 228 44) (end 228 63)) (probe (position 228 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 241 27) (end 241 31)) (probe (position 241 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 242 28) (end 242 49)) (probe (position 242 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarHeatCapacityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 241 22) (end 241 25)) (probe (position 241 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 242 22) (end 242 26)) (probe (position 242 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 191 46) (end 191 57)) (probe (position 191 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 196 22) (end 196 39)) (probe (position 196 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 196 46) (end 196 66)) (probe (position 196 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 195 47) (end 195 66)) (probe (position 195 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 195 76) (end 195 84)) (probe (position 195 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 195 98) (end 195 106)) (probe (position 195 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 194 38) (end 194 57)) (probe (position 194 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 194 67) (end 194 75)) (probe (position 194 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 194 89) (end 194 97)) (probe (position 194 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 192 36) (end 192 55)) (probe (position 192 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 192 65) (end 192 73)) (probe (position 192 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 192 87) (end 192 95)) (probe (position 192 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 193 34) (end 193 53)) (probe (position 193 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 193 63) (end 193 71)) (probe (position 193 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 193 85) (end 193 93)) (probe (position 193 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 172 47) (end 172 66)) (probe (position 172 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 185 27) (end 185 31)) (probe (position 185 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 186 28) (end 186 52)) (probe (position 186 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarHelmholtzEnergyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 185 22) (end 185 25)) (probe (position 185 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 186 22) (end 186 26)) (probe (position 186 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 135 45) (end 135 56)) (probe (position 135 45))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 140 22) (end 140 39)) (probe (position 140 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 140 46) (end 140 66)) (probe (position 140 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 139 47) (end 139 66)) (probe (position 139 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 139 76) (end 139 84)) (probe (position 139 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 139 98) (end 139 106)) (probe (position 139 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 138 38) (end 138 57)) (probe (position 138 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 138 67) (end 138 75)) (probe (position 138 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 138 89) (end 138 97)) (probe (position 138 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 136 36) (end 136 55)) (probe (position 136 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 136 65) (end 136 73)) (probe (position 136 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 136 87) (end 136 95)) (probe (position 136 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 137 34) (end 137 53)) (probe (position 137 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 137 63) (end 137 71)) (probe (position 137 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 137 85) (end 137 93)) (probe (position 137 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 116 46) (end 116 65)) (probe (position 116 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 129 27) (end 129 31)) (probe (position 129 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 130 28) (end 130 51)) (probe (position 130 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarInternalEnergyUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 129 22) (end 129 25)) (probe (position 129 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 130 22) (end 130 26)) (probe (position 130 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 83 35) (end 83 46)) (probe (position 83 35))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 86 22) (end 86 39)) (probe (position 86 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 86 46) (end 86 66)) (probe (position 86 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 85 47) (end 85 66)) (probe (position 85 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 85 76) (end 85 84)) (probe (position 85 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 85 98) (end 85 106)) (probe (position 85 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 84 34) (end 84 53)) (probe (position 84 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 84 63) (end 84 71)) (probe (position 84 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 84 85) (end 84 93)) (probe (position 84 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 64 36) (end 64 55)) (probe (position 64 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 77 27) (end 77 31)) (probe (position 77 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 78 28) (end 78 41)) (probe (position 78 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarMassUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 77 22) (end 77 25)) (probe (position 77 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 78 22) (end 78 26)) (probe (position 78 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1320 51) (end 1320 62)) (probe (position 1320 51))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1323 22) (end 1323 39)) (probe (position 1323 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1323 46) (end 1323 66)) (probe (position 1323 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1322 47) (end 1322 66)) (probe (position 1322 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1322 76) (end 1322 84)) (probe (position 1322 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1322 98) (end 1322 106)) (probe (position 1322 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1321 36) (end 1321 55)) (probe (position 1321 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1321 65) (end 1321 73)) (probe (position 1321 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1321 87) (end 1321 95)) (probe (position 1321 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1301 52) (end 1301 71)) (probe (position 1301 52))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1314 27) (end 1314 31)) (probe (position 1314 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1315 28) (end 1315 57)) (probe (position 1315 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarOpticalRotatoryPowerUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1314 22) (end 1314 25)) (probe (position 1314 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1315 22) (end 1315 26)) (probe (position 1315 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 109 37) (end 109 48)) (probe (position 109 37))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 112 22) (end 112 39)) (probe (position 112 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 112 46) (end 112 66)) (probe (position 112 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 111 47) (end 111 66)) (probe (position 111 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 111 76) (end 111 84)) (probe (position 111 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 111 98) (end 111 106)) (probe (position 111 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 110 36) (end 110 55)) (probe (position 110 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 110 65) (end 110 73)) (probe (position 110 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 110 87) (end 110 95)) (probe (position 110 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 90 38) (end 90 57)) (probe (position 90 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 103 27) (end 103 31)) (probe (position 103 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 104 28) (end 104 43)) (probe (position 104 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MolarVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 103 22) (end 103 25)) (probe (position 103 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 104 22) (end 104 26)) (probe (position 104 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 977 53) (end 977 70)) (probe (position 977 53))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 42 32) (end 42 53)) (probe (position 42 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::NumberOfMolesUnit"))) (kind aliasBinding) (ordinal 0) (authored-target "AmountOfSubstanceUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 43 33) (end 43 55)) (probe (position 43 33))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::NumberOfMolesValue"))) (kind aliasBinding) (ordinal 0) (authored-target "AmountOfSubstanceValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 729 49) (end 729 66)) (probe (position 729 49))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 784 41) (end 784 52)) (probe (position 784 41))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 788 22) (end 788 39)) (probe (position 788 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 788 46) (end 788 66)) (probe (position 788 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 787 38) (end 787 57)) (probe (position 787 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 787 67) (end 787 75)) (probe (position 787 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 787 89) (end 787 97)) (probe (position 787 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 785 36) (end 785 55)) (probe (position 785 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 785 65) (end 785 73)) (probe (position 785 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 785 87) (end 785 95)) (probe (position 785 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 786 34) (end 786 53)) (probe (position 786 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 786 63) (end 786 71)) (probe (position 786 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 786 85) (end 786 93)) (probe (position 786 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 765 42) (end 765 61)) (probe (position 765 42))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 778 27) (end 778 31)) (probe (position 778 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 779 28) (end 779 47)) (probe (position 779 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "OsmoticPressureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 778 22) (end 778 25)) (probe (position 778 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 779 22) (end 779 26)) (probe (position 779 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 560 41) (end 560 52)) (probe (position 560 41))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 564 22) (end 564 39)) (probe (position 564 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 564 46) (end 564 66)) (probe (position 564 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 563 38) (end 563 57)) (probe (position 563 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 563 67) (end 563 75)) (probe (position 563 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 563 89) (end 563 97)) (probe (position 563 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 561 36) (end 561 55)) (probe (position 561 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 561 65) (end 561 73)) (probe (position 561 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 561 87) (end 561 95)) (probe (position 561 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 562 34) (end 562 53)) (probe (position 562 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 562 63) (end 562 71)) (probe (position 562 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 562 85) (end 562 93)) (probe (position 562 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 541 42) (end 541 61)) (probe (position 541 42))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 554 27) (end 554 31)) (probe (position 554 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 555 28) (end 555 47)) (probe (position 555 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "PartialPressureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 554 22) (end 554 25)) (probe (position 554 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 555 22) (end 555 26)) (probe (position 555 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 305 47) (end 305 58)) (probe (position 305 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 307 22) (end 307 39)) (probe (position 307 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 307 46) (end 307 66)) (probe (position 307 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 306 36) (end 306 55)) (probe (position 306 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 306 65) (end 306 73)) (probe (position 306 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 306 87) (end 306 95)) (probe (position 306 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 286 48) (end 286 67)) (probe (position 286 48))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 299 27) (end 299 31)) (probe (position 299 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 300 28) (end 300 53)) (probe (position 300 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "ParticleConcentrationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 299 22) (end 299 25)) (probe (position 299 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 300 22) (end 300 26)) (probe (position 300 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 47 45) (end 47 62)) (probe (position 47 45))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1346 54) (end 1346 65)) (probe (position 1346 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1349 22) (end 1349 39)) (probe (position 1349 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1349 46) (end 1349 66)) (probe (position 1349 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1347 36) (end 1347 55)) (probe (position 1347 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1347 65) (end 1347 73)) (probe (position 1347 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1347 87) (end 1347 95)) (probe (position 1347 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1348 34) (end 1348 53)) (probe (position 1348 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1348 63) (end 1348 71)) (probe (position 1348 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1348 85) (end 1348 93)) (probe (position 1348 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1327 55) (end 1327 74)) (probe (position 1327 55))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1340 27) (end 1340 31)) (probe (position 1340 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1341 28) (end 1341 60)) (probe (position 1341 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SpecificOpticalRotatoryPowerUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1340 22) (end 1340 25)) (probe (position 1340 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1341 22) (end 1341 26)) (probe (position 1341 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 640 60) (end 640 77)) (probe (position 640 60))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 693 61) (end 693 78)) (probe (position 693 61))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 748 60) (end 748 77)) (probe (position 748 60))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 614 51) (end 614 62)) (probe (position 614 51))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 619 22) (end 619 39)) (probe (position 619 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 619 46) (end 619 66)) (probe (position 619 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 618 47) (end 618 66)) (probe (position 618 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 618 76) (end 618 84)) (probe (position 618 76))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 618 98) (end 618 106)) (probe (position 618 98))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 617 38) (end 617 57)) (probe (position 617 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 617 67) (end 617 75)) (probe (position 617 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 617 89) (end 617 97)) (probe (position 617 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 615 36) (end 615 55)) (probe (position 615 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 615 65) (end 615 73)) (probe (position 615 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 615 87) (end 615 95)) (probe (position 615 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 616 34) (end 616 53)) (probe (position 616 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 616 63) (end 616 71)) (probe (position 616 63))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 616 85) (end 616 93)) (probe (position 616 85))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 595 52) (end 595 71)) (probe (position 595 52))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 608 27) (end 608 31)) (probe (position 608 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 609 28) (end 609 57)) (probe (position 609 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "StandardChemicalPotentialUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 608 22) (end 608 25)) (probe (position 608 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 609 22) (end 609 26)) (probe (position 609 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 853 54) (end 853 71)) (probe (position 853 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 792 58) (end 792 75)) (probe (position 792 58))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1158 53) (end 1158 64)) (probe (position 1158 53))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1161 22) (end 1161 39)) (probe (position 1161 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1161 46) (end 1161 66)) (probe (position 1161 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1160 38) (end 1160 57)) (probe (position 1160 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1160 67) (end 1160 75)) (probe (position 1160 67))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1160 89) (end 1160 97)) (probe (position 1160 89))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1159 36) (end 1159 55)) (probe (position 1159 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1159 65) (end 1159 73)) (probe (position 1159 65))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1159 87) (end 1159 95)) (probe (position 1159 87))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1139 54) (end 1139 73)) (probe (position 1139 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1152 27) (end 1152 31)) (probe (position 1152 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1153 28) (end 1153 59)) (probe (position 1153 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "ThermalDiffusionCoefficientUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1152 22) (end 1152 25)) (probe (position 1152 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1153 22) (end 1153 26)) (probe (position 1153 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1122 49) (end 1122 66)) (probe (position 1122 49))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1105 48) (end 1105 65)) (probe (position 1105 48))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1266 51) (end 1266 68)) (probe (position 1266 51))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 448 40) (end 448 56)) (probe (position 448 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 429 41) (end 429 60)) (probe (position 429 41))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 442 27) (end 442 31)) (probe (position 442 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 443 28) (end 443 46)) (probe (position 443 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "VolumeFractionUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 442 22) (end 442 25)) (probe (position 442 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 443 22) (end 443 26)) (probe (position 443 22))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 538 32) (end 538 53)) (probe (position 538 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind featureTyping) (ordinal 0) (authored-target "AbsoluteActivityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 690 35) (end 690 59)) (probe (position 690 35))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind featureTyping) (ordinal 0) (authored-target "ActivityCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 637 30) (end 637 49)) (probe (position 637 30))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind featureTyping) (ordinal 0) (authored-target "ActivityFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityFactorValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 671 32) (end 671 53)) (probe (position 671 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind featureTyping) (ordinal 0) (authored-target "ActivityOfSoluteValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 724 33) (end 724 55)) (probe (position 724 33))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind featureTyping) (ordinal 0) (authored-target "ActivityOfSolventValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 826 43) (end 826 75)) (probe (position 826 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind featureTyping) (ordinal 0) (authored-target "AffinityOfAChemicalReactionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 387 46) (end 387 81)) (probe (position 387 46))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 426 53) (end 426 95)) (probe (position 426 53))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceFractionMoleFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1285 38) (end 1285 57)) (probe (position 1285 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind featureTyping) (ordinal 0) (authored-target "AngularMeasureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 955 42) (end 955 73)) (probe (position 955 42))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind featureTyping) (ordinal 0) (authored-target "CanonicalPartitionFunctionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 513 33) (end 513 55)) (probe (position 513 33))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind featureTyping) (ordinal 0) (authored-target "ChemicalPotentialValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1282 39) (end 1282 63)) (probe (position 1282 39))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (kind aliasBinding) (ordinal 0) (authored-target "transportNumberOfTheIonB")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1026 26) (end 1026 41)) (probe (position 1026 26))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind featureTyping) (ordinal 0) (authored-target "DegeneracyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegeneracyValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1205 36) (end 1205 61)) (probe (position 1205 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind featureTyping) (ordinal 0) (authored-target "DegreeOfDissociationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1096 36) (end 1096 61)) (probe (position 1096 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind featureTyping) (ordinal 0) (authored-target "DiffusionCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1207 35) (end 1207 55)) (probe (position 1207 35))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (kind aliasBinding) (ordinal 0) (authored-target "degreeOfDissociation")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degreeOfDissociation")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1227 40) (end 1227 69)) (probe (position 1227 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind featureTyping) (ordinal 0) (authored-target "ElectrolyticConductivityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 493 40) (end 493 67)) (probe (position 493 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (kind aliasBinding) (ordinal 0) (authored-target "latentHeatOfPhaseTransition")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 916 55) (end 916 99)) (probe (position 916 55))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind featureTyping) (ordinal 0) (authored-target "EquilibriumConstantOnConcentrationBasisValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 889 50) (end 889 89)) (probe (position 889 50))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind featureTyping) (ordinal 0) (authored-target "EquilibriumConstantOnPressureBasisValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 837 32) (end 837 54)) (probe (position 837 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 585 24) (end 585 37)) (probe (position 585 24))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind featureTyping) (ordinal 0) (authored-target "FugacityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 972 47) (end 972 83)) (probe (position 972 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0) (authored-target "GrandCanonicalPartitionFunctionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 974 37) (end 974 68)) (probe (position 974 37))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (kind aliasBinding) (ordinal 0) (authored-target "grandCanonicalPartitionFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1182 29) (end 1182 47)) (probe (position 1182 29))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind featureTyping) (ordinal 0) (authored-target "IonicStrengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 478 43) (end 478 54)) (probe (position 478 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind featureTyping) (ordinal 0) (authored-target "EnergyValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 344 33) (end 344 55)) (probe (position 344 33))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind featureTyping) (ordinal 0) (authored-target "MassConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 367 28) (end 367 45)) (probe (position 367 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind featureTyping) (ordinal 0) (authored-target "MassFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MassFractionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1063 28) (end 1063 39)) (probe (position 1063 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 925 47) (end 925 57)) (probe (position 925 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0) (authored-target "CountValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 469 24) (end 469 37)) (probe (position 469 24))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molality"))) (kind featureTyping) (ordinal 0) (authored-target "MolalityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1255 33) (end 1255 55)) (probe (position 1255 33))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind featureTyping) (ordinal 0) (authored-target "MolarConductivityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 161 29) (end 161 47)) (probe (position 161 29))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind featureTyping) (ordinal 0) (authored-target "MolarEnthalpyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 274 28) (end 274 45)) (probe (position 274 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind featureTyping) (ordinal 0) (authored-target "MolarEntropyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1048 32) (end 1048 53)) (probe (position 1048 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind featureTyping) (ordinal 0) (authored-target "MolarGasConstantValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 217 32) (end 217 53)) (probe (position 217 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind featureTyping) (ordinal 0) (authored-target "MolarGibbsEnergyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 245 33) (end 245 55)) (probe (position 245 33))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind featureTyping) (ordinal 0) (authored-target "MolarHeatCapacityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 189 36) (end 189 61)) (probe (position 189 36))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind featureTyping) (ordinal 0) (authored-target "MolarHelmholtzEnergyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 133 35) (end 133 59)) (probe (position 133 35))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind featureTyping) (ordinal 0) (authored-target "MolarInternalEnergyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 81 25) (end 81 39)) (probe (position 81 25))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind featureTyping) (ordinal 0) (authored-target "MolarMassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1318 41) (end 1318 71)) (probe (position 1318 41))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0) (authored-target "MolarOpticalRotatoryPowerValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 107 27) (end 107 43)) (probe (position 107 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind featureTyping) (ordinal 0) (authored-target "MolarVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 311 38) (end 311 64)) (probe (position 311 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind featureTyping) (ordinal 0) (authored-target "ParticleConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 991 42) (end 991 73)) (probe (position 991 42))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind featureTyping) (ordinal 0) (authored-target "MolecularPartitionFunctionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1028 27) (end 1028 37)) (probe (position 1028 27))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::multiplicity"))) (kind aliasBinding) (ordinal 0) (authored-target "degeneracy")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::degeneracy")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 24 32) (end 24 42)) (probe (position 24 32))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind featureTyping) (ordinal 0) (authored-target "CountValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 44 28) (end 44 45)) (probe (position 44 28))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::numberOfMoles"))) (kind aliasBinding) (ordinal 0) (authored-target "amountOfSubstance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 745 43) (end 745 65)) (probe (position 745 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (kind aliasBinding) (ordinal 0) (authored-target "osmoticFactorOfSolvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 743 38) (end 743 65)) (probe (position 743 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind featureTyping) (ordinal 0) (authored-target "OsmoticFactorOfSolventValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 782 31) (end 782 51)) (probe (position 782 31))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind featureTyping) (ordinal 0) (authored-target "OsmoticPressureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 558 31) (end 558 51)) (probe (position 558 31))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind featureTyping) (ordinal 0) (authored-target "PartialPressureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 303 37) (end 303 63)) (probe (position 303 37))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind featureTyping) (ordinal 0) (authored-target "ParticleConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 993 43) (end 993 69)) (probe (position 993 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (kind aliasBinding) (ordinal 0) (authored-target "molecularPartitionFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 673 39) (end 673 55)) (probe (position 673 39))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (kind aliasBinding) (ordinal 0) (authored-target "activityOfSolute")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolute")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 726 40) (end 726 57)) (probe (position 726 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (kind aliasBinding) (ordinal 0) (authored-target "activityOfSolvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::activityOfSolvent")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 61 34) (end 61 57)) (probe (position 61 34))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind featureTyping) (ordinal 0) (authored-target "RelativeAtomicMassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1344 44) (end 1344 77)) (probe (position 1344 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0) (authored-target "SpecificOpticalRotatoryPowerValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 654 49) (end 654 87)) (probe (position 654 49))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind featureTyping) (ordinal 0) (authored-target "StandardAbsoluteActivityInMixtureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 707 50) (end 707 89)) (probe (position 707 50))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind featureTyping) (ordinal 0) (authored-target "StandardAbsoluteActivityInSolutionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 762 49) (end 762 87)) (probe (position 762 49))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind featureTyping) (ordinal 0) (authored-target "StandardAbsoluteActivityOfSolventValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 396 54) (end 396 89)) (probe (position 396 54))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceConcentrationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 612 41) (end 612 71)) (probe (position 612 41))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind featureTyping) (ordinal 0) (authored-target "StandardChemicalPotentialValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 867 43) (end 867 75)) (probe (position 867 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind featureTyping) (ordinal 0) (authored-target "StandardEquilibriumConstantValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 996 44) (end 996 54)) (probe (position 996 44))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind featureTyping) (ordinal 0) (authored-target "CountValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 806 47) (end 806 83)) (probe (position 806 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind featureTyping) (ordinal 0) (authored-target "StoichiometricNumberOfSubstanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1156 43) (end 1156 75)) (probe (position 1156 43))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind featureTyping) (ordinal 0) (authored-target "ThermalDiffusionCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1136 38) (end 1136 65)) (probe (position 1136 38))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind featureTyping) (ordinal 0) (authored-target "ThermalDiffusionFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1119 37) (end 1119 63)) (probe (position 1119 37))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind featureTyping) (ordinal 0) (authored-target "ThermalDiffusionRatioValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 869 47) (end 869 74)) (probe (position 869 47))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (kind aliasBinding) (ordinal 0) (authored-target "standardEquilibriumConstant")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 1280 40) (end 1280 69)) (probe (position 1280 40))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind featureTyping) (ordinal 0) (authored-target "TransportNumberOfTheIonBValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue")))))
  )
  (query (document "memory://snapshot/isq_chemistry_molecular.md") (range (start 446 30) (end 446 49)) (probe (position 446 30))
    (reference (id (source (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind featureTyping) (ordinal 0) (authored-target "VolumeFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_chemistry_molecular.md") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
  )
)
~~~
