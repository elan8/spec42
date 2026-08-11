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
  (document "isq_chemistry_molecular.md"
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
        (range (start 15 19) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 26))
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
        (range (start 24 4) (end 24 786))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 4) (end 47 1010))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 4) (end 64 590))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 4) (end 83 372))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 8) (end 84 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 8) (end 85 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 4) (end 90 592))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 4) (end 109 378))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 8) (end 110 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 8) (end 111 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 116 4) (end 116 703))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 4) (end 135 613))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 8) (end 136 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 137 8) (end 137 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 8) (end 138 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 8) (end 139 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 144 4) (end 144 671))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 607))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 8) (end 164 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 8) (end 165 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 8) (end 166 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 8) (end 167 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 172 4) (end 172 712))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 191 4) (end 191 614))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 192 8) (end 192 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 193 8) (end 193 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 194 8) (end 194 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 195 8) (end 195 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 4) (end 200 692))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 219 4) (end 219 610))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 8) (end 220 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 221 8) (end 221 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 8) (end 222 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 223 8) (end 223 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 4) (end 228 696))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 247 4) (end 247 764))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 248 8) (end 248 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 249 8) (end 249 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 250 8) (end 250 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 251 8) (end 251 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 252 8) (end 252 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 257 4) (end 257 669))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 276 4) (end 276 759))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 277 8) (end 277 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 278 8) (end 278 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 279 8) (end 279 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 280 8) (end 280 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 281 8) (end 281 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 4) (end 286 635))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 305 4) (end 305 251))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 306 8) (end 306 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 4) (end 327 748))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 346 4) (end 346 358))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 347 8) (end 347 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 348 8) (end 348 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 353 4) (end 353 552))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 370 4) (end 370 1103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 389 4) (end 389 397))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 390 8) (end 390 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 391 8) (end 391 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 412 4) (end 412 977))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 429 4) (end 429 1052))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 448 4) (end 448 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 452 4) (end 452 842))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 471 4) (end 471 371))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 472 8) (end 472 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 473 8) (end 473 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 496 4) (end 496 935))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 515 4) (end 515 611))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 516 8) (end 516 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 517 8) (end 517 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 518 8) (end 518 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 519 8) (end 519 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 524 4) (end 524 657))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 541 4) (end 541 670))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 560 4) (end 560 474))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 561 8) (end 561 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 562 8) (end 562 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 563 8) (end 563 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 568 4) (end 568 922))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 587 4) (end 587 467))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 588 8) (end 588 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 589 8) (end 589 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 590 8) (end 590 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 595 4) (end 595 989))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 614 4) (end 614 619))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 615 8) (end 615 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 616 8) (end 616 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 617 8) (end 617 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 618 8) (end 618 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 623 4) (end 623 966))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 640 4) (end 640 768))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 657 4) (end 657 1369))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 676 4) (end 676 772))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 693 4) (end 693 886))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 710 4) (end 710 703))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 729 4) (end 729 899))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 748 4) (end 748 693))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 765 4) (end 765 669))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 784 4) (end 784 474))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 785 8) (end 785 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 786 8) (end 786 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 787 8) (end 787 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 792 4) (end 792 845))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 809 4) (end 809 1223))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 828 4) (end 828 621))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 829 8) (end 829 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 830 8) (end 830 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 831 8) (end 831 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 832 8) (end 832 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 837 4) (end 837 705))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 853 4) (end 853 951))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 872 4) (end 872 774))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 891 4) (end 891 493))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 892 8) (end 892 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 893 8) (end 893 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 894 8) (end 894 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 899 4) (end 899 787))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 918 4) (end 918 406))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 919 8) (end 919 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 920 8) (end 920 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 925 4) (end 925 695))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 941 4) (end 941 713))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 958 4) (end 958 921))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 977 4) (end 977 778))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 996 4) (end 996 501))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1012 4) (end 1012 518))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1031 4) (end 1031 650))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1050 4) (end 1050 763))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1051 8) (end 1051 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1052 8) (end 1052 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1053 8) (end 1053 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1054 8) (end 1054 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1055 8) (end 1055 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1063 4) (end 1063 525))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1079 4) (end 1079 861))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1098 4) (end 1098 369))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1099 8) (end 1099 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1100 8) (end 1100 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1105 4) (end 1105 796))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1122 4) (end 1122 651))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1139 4) (end 1139 661))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1158 4) (end 1158 376))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1159 8) (end 1159 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1160 8) (end 1160 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1165 4) (end 1165 674))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1184 4) (end 1184 376))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1185 8) (end 1185 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1186 8) (end 1186 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1191 4) (end 1191 585))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1210 4) (end 1210 796))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1229 4) (end 1229 614))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1230 8) (end 1230 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1231 8) (end 1231 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1232 8) (end 1232 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1233 8) (end 1233 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1238 4) (end 1238 682))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1257 4) (end 1257 629))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1258 8) (end 1258 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1259 8) (end 1259 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1260 8) (end 1260 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1261 8) (end 1261 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1266 4) (end 1266 655))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1301 4) (end 1301 818))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1320 4) (end 1320 392))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1321 8) (end 1321 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1322 8) (end 1322 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1327 4) (end 1327 816))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1346 4) (end 1346 369))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1347 8) (end 1347 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1348 8) (end 1348 101))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "521a456542257c842f89d19905549d7f27bdce687013dc8caa24b2241f7b317c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (kind "package") (name "ISQChemistryMolecular") (declared-name "ISQChemistryMolecular"))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind "attribute def") (name "AbsoluteActivityValue") (declared-name "AbsoluteActivityValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind "attribute def") (name "ActivityCoefficientValue") (declared-name "ActivityCoefficientValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind "attribute def") (name "ActivityFactorValue") (declared-name "ActivityFactorValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind "attribute def") (name "ActivityOfSoluteValue") (declared-name "ActivityOfSoluteValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind "attribute def") (name "ActivityOfSolventValue") (declared-name "ActivityOfSolventValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind "attribute def") (name "AffinityOfAChemicalReactionUnit") (declared-name "AffinityOfAChemicalReactionUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind "attribute def") (name "AffinityOfAChemicalReactionValue") (declared-name "AffinityOfAChemicalReactionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AffinityOfAChemicalReactionUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind "attribute def") (name "AmountOfSubstanceConcentrationUnit") (declared-name "AmountOfSubstanceConcentrationUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind "attribute def") (name "AmountOfSubstanceConcentrationValue") (declared-name "AmountOfSubstanceConcentrationValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmountOfSubstanceConcentrationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind "attribute def") (name "AmountOfSubstanceFractionMoleFractionValue") (declared-name "AmountOfSubstanceFractionMoleFractionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))) (kind "import") (name "AngularMeasureValue") (declared-name "AngularMeasureValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::AngularMeasureValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind "attribute def") (name "CanonicalPartitionFunctionValue") (declared-name "CanonicalPartitionFunctionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind "attribute def") (name "ChemicalPotentialUnit") (declared-name "ChemicalPotentialUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind "attribute def") (name "ChemicalPotentialValue") (declared-name "ChemicalPotentialValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ChemicalPotentialUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind "attribute def") (name "DegeneracyValue") (declared-name "DegeneracyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind "attribute def") (name "DegreeOfDissociationValue") (declared-name "DegreeOfDissociationValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind "attribute def") (name "DiffusionCoefficientUnit") (declared-name "DiffusionCoefficientUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind "attribute def") (name "DiffusionCoefficientValue") (declared-name "DiffusionCoefficientValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiffusionCoefficientUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind "attribute def") (name "ElectrolyticConductivityUnit") (declared-name "ElectrolyticConductivityUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind "attribute def") (name "ElectrolyticConductivityValue") (declared-name "ElectrolyticConductivityValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ElectrolyticConductivityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))) (kind "import") (name "EnergyValue") (declared-name "EnergyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQThermodynamics::EnergyValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind "attribute def") (name "EquilibriumConstantOnConcentrationBasisUnit") (declared-name "EquilibriumConstantOnConcentrationBasisUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind "attribute def") (name "EquilibriumConstantOnConcentrationBasisValue") (declared-name "EquilibriumConstantOnConcentrationBasisValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EquilibriumConstantOnConcentrationBasisUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind "attribute def") (name "EquilibriumConstantOnPressureBasisUnit") (declared-name "EquilibriumConstantOnPressureBasisUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind "attribute def") (name "EquilibriumConstantOnPressureBasisValue") (declared-name "EquilibriumConstantOnPressureBasisValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EquilibriumConstantOnPressureBasisUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind "attribute def") (name "FugacityUnit") (declared-name "FugacityUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind "attribute def") (name "FugacityValue") (declared-name "FugacityValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "FugacityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind "attribute def") (name "GrandCanonicalPartitionFunctionValue") (declared-name "GrandCanonicalPartitionFunctionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind "attribute def") (name "IonicStrengthUnit") (declared-name "IonicStrengthUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind "attribute def") (name "IonicStrengthValue") (declared-name "IonicStrengthValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IonicStrengthUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind "attribute def") (name "MassConcentrationUnit") (declared-name "MassConcentrationUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind "attribute def") (name "MassConcentrationValue") (declared-name "MassConcentrationValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConcentrationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind "attribute def") (name "MassFractionValue") (declared-name "MassFractionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind "attribute def") (name "MolalityUnit") (declared-name "MolalityUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind "attribute def") (name "MolalityValue") (declared-name "MolalityValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolalityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind "attribute def") (name "MolarConductivityUnit") (declared-name "MolarConductivityUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind "attribute def") (name "MolarConductivityValue") (declared-name "MolarConductivityValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarConductivityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind "attribute def") (name "MolarEnthalpyUnit") (declared-name "MolarEnthalpyUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind "attribute def") (name "MolarEnthalpyValue") (declared-name "MolarEnthalpyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarEnthalpyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind "attribute def") (name "MolarEntropyUnit") (declared-name "MolarEntropyUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind "attribute def") (name "MolarEntropyValue") (declared-name "MolarEntropyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarEntropyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind "attribute def") (name "MolarGasConstantUnit") (declared-name "MolarGasConstantUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind "attribute def") (name "MolarGasConstantValue") (declared-name "MolarGasConstantValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarGasConstantUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind "attribute def") (name "MolarGibbsEnergyUnit") (declared-name "MolarGibbsEnergyUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind "attribute def") (name "MolarGibbsEnergyValue") (declared-name "MolarGibbsEnergyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarGibbsEnergyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind "attribute def") (name "MolarHeatCapacityUnit") (declared-name "MolarHeatCapacityUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind "attribute def") (name "MolarHeatCapacityValue") (declared-name "MolarHeatCapacityValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarHeatCapacityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind "attribute def") (name "MolarHelmholtzEnergyUnit") (declared-name "MolarHelmholtzEnergyUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind "attribute def") (name "MolarHelmholtzEnergyValue") (declared-name "MolarHelmholtzEnergyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarHelmholtzEnergyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind "attribute def") (name "MolarInternalEnergyUnit") (declared-name "MolarInternalEnergyUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind "attribute def") (name "MolarInternalEnergyValue") (declared-name "MolarInternalEnergyValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarInternalEnergyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind "attribute def") (name "MolarMassUnit") (declared-name "MolarMassUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind "attribute def") (name "MolarMassValue") (declared-name "MolarMassValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarMassUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind "attribute def") (name "MolarOpticalRotatoryPowerUnit") (declared-name "MolarOpticalRotatoryPowerUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind "attribute def") (name "MolarOpticalRotatoryPowerValue") (declared-name "MolarOpticalRotatoryPowerValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarOpticalRotatoryPowerUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind "attribute def") (name "MolarVolumeUnit") (declared-name "MolarVolumeUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind "attribute def") (name "MolarVolumeValue") (declared-name "MolarVolumeValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarVolumeUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind "attribute def") (name "MolecularPartitionFunctionValue") (declared-name "MolecularPartitionFunctionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::NumberOfMolesUnit"))) (kind "alias") (name "NumberOfMolesUnit") (declared-name "NumberOfMolesUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::NumberOfMolesValue"))) (kind "alias") (name "NumberOfMolesValue") (declared-name "NumberOfMolesValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind "attribute def") (name "OsmoticFactorOfSolventValue") (declared-name "OsmoticFactorOfSolventValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind "attribute def") (name "OsmoticPressureUnit") (declared-name "OsmoticPressureUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind "attribute def") (name "OsmoticPressureValue") (declared-name "OsmoticPressureValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "OsmoticPressureUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind "attribute def") (name "PartialPressureUnit") (declared-name "PartialPressureUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind "attribute def") (name "PartialPressureValue") (declared-name "PartialPressureValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PartialPressureUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind "attribute def") (name "ParticleConcentrationUnit") (declared-name "ParticleConcentrationUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind "attribute def") (name "ParticleConcentrationValue") (declared-name "ParticleConcentrationValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ParticleConcentrationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind "attribute def") (name "RelativeAtomicMassValue") (declared-name "RelativeAtomicMassValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind "attribute def") (name "SpecificOpticalRotatoryPowerUnit") (declared-name "SpecificOpticalRotatoryPowerUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind "attribute def") (name "SpecificOpticalRotatoryPowerValue") (declared-name "SpecificOpticalRotatoryPowerValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificOpticalRotatoryPowerUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind "attribute def") (name "StandardAbsoluteActivityInMixtureValue") (declared-name "StandardAbsoluteActivityInMixtureValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind "attribute def") (name "StandardAbsoluteActivityInSolutionValue") (declared-name "StandardAbsoluteActivityInSolutionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind "attribute def") (name "StandardAbsoluteActivityOfSolventValue") (declared-name "StandardAbsoluteActivityOfSolventValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind "attribute def") (name "StandardChemicalPotentialUnit") (declared-name "StandardChemicalPotentialUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind "attribute def") (name "StandardChemicalPotentialValue") (declared-name "StandardChemicalPotentialValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "StandardChemicalPotentialUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind "attribute def") (name "StandardEquilibriumConstantValue") (declared-name "StandardEquilibriumConstantValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind "attribute def") (name "StoichiometricNumberOfSubstanceValue") (declared-name "StoichiometricNumberOfSubstanceValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind "attribute def") (name "ThermalDiffusionCoefficientUnit") (declared-name "ThermalDiffusionCoefficientUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind "attribute def") (name "ThermalDiffusionCoefficientValue") (declared-name "ThermalDiffusionCoefficientValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalDiffusionCoefficientUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind "attribute def") (name "ThermalDiffusionFactorValue") (declared-name "ThermalDiffusionFactorValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind "attribute def") (name "ThermalDiffusionRatioValue") (declared-name "ThermalDiffusionRatioValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind "attribute def") (name "TransportNumberOfTheIonBValue") (declared-name "TransportNumberOfTheIonBValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind "attribute def") (name "VolumeFractionUnit") (declared-name "VolumeFractionUnit") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind "attribute def") (name "VolumeFractionValue") (declared-name "VolumeFractionValue") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeFractionUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind "attribute def") (name "absoluteActivity") (declared-name "absoluteActivity") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsoluteActivityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind "attribute def") (name "activityCoefficient") (declared-name "activityCoefficient") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityCoefficientValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind "attribute def") (name "activityFactor") (declared-name "activityFactor") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityFactorValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind "attribute def") (name "activityOfSolute") (declared-name "activityOfSolute") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityOfSoluteValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind "attribute def") (name "activityOfSolvent") (declared-name "activityOfSolvent") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityOfSolventValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind "attribute def") (name "affinityOfAChemicalReaction") (declared-name "affinityOfAChemicalReaction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AffinityOfAChemicalReactionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind "attribute def") (name "amountOfSubstanceConcentration") (declared-name "amountOfSubstanceConcentration") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind "attribute def") (name "amountOfSubstanceFractionMoleFraction") (declared-name "amountOfSubstanceFractionMoleFraction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceFractionMoleFractionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind "attribute def") (name "angleOfOpticalRotation") (declared-name "angleOfOpticalRotation") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind "attribute def") (name "canonicalPartitionFunction") (declared-name "canonicalPartitionFunction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CanonicalPartitionFunctionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind "attribute def") (name "chemicalPotential") (declared-name "chemicalPotential") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ChemicalPotentialValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (kind "alias") (name "currentFractionOfTheIonB") (declared-name "currentFractionOfTheIonB") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind "attribute def") (name "degeneracy") (declared-name "degeneracy") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DegeneracyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind "attribute def") (name "degreeOfDissociation") (declared-name "degreeOfDissociation") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DegreeOfDissociationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind "attribute def") (name "diffusionCoefficient") (declared-name "diffusionCoefficient") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DiffusionCoefficientValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (kind "alias") (name "dissociationFraction") (declared-name "dissociationFraction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind "attribute def") (name "electrolyticConductivity") (declared-name "electrolyticConductivity") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectrolyticConductivityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (kind "alias") (name "enthalpyOfPhaseTransition") (declared-name "enthalpyOfPhaseTransition") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind "attribute def") (name "equilibriumConstantOnConcentrationBasis") (declared-name "equilibriumConstantOnConcentrationBasis") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "EquilibriumConstantOnConcentrationBasisValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind "attribute def") (name "equilibriumConstantOnPressureBasis") (declared-name "equilibriumConstantOnPressureBasis") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "EquilibriumConstantOnPressureBasisValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind "attribute def") (name "extentOfReaction") (declared-name "extentOfReaction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind "attribute def") (name "fugacity") (declared-name "fugacity") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "FugacityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind "attribute def") (name "grandCanonicalPartitionFunction") (declared-name "grandCanonicalPartitionFunction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "GrandCanonicalPartitionFunctionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (kind "alias") (name "grandPartitionFunction") (declared-name "grandPartitionFunction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind "attribute def") (name "ionicStrength") (declared-name "ionicStrength") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "IonicStrengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind "attribute def") (name "latentHeatOfPhaseTransition") (declared-name "latentHeatOfPhaseTransition") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind "attribute def") (name "massConcentration") (declared-name "massConcentration") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassConcentrationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind "attribute def") (name "massFraction") (declared-name "massFraction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFractionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind "attribute def") (name "meanFreePath") (declared-name "meanFreePath") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind "attribute def") (name "microcanonicalPartitionFunction") (declared-name "microcanonicalPartitionFunction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (kind "attribute def") (name "molality") (declared-name "molality") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolalityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind "attribute def") (name "molarConductivity") (declared-name "molarConductivity") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarConductivityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind "attribute def") (name "molarEnthalpy") (declared-name "molarEnthalpy") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarEnthalpyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind "attribute def") (name "molarEntropy") (declared-name "molarEntropy") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarEntropyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind "attribute def") (name "molarGasConstant") (declared-name "molarGasConstant") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarGasConstantValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind "attribute def") (name "molarGibbsEnergy") (declared-name "molarGibbsEnergy") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarGibbsEnergyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind "attribute def") (name "molarHeatCapacity") (declared-name "molarHeatCapacity") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHeatCapacityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind "attribute def") (name "molarHelmholtzEnergy") (declared-name "molarHelmholtzEnergy") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHelmholtzEnergyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind "attribute def") (name "molarInternalEnergy") (declared-name "molarInternalEnergy") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarInternalEnergyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind "attribute def") (name "molarMass") (declared-name "molarMass") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarMassValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind "attribute def") (name "molarOpticalRotatoryPower") (declared-name "molarOpticalRotatoryPower") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarOpticalRotatoryPowerValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind "attribute def") (name "molarVolume") (declared-name "molarVolume") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind "attribute def") (name "molecularConcentration") (declared-name "molecularConcentration") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ParticleConcentrationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind "attribute def") (name "molecularPartitionFunction") (declared-name "molecularPartitionFunction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolecularPartitionFunctionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::multiplicity"))) (kind "alias") (name "multiplicity") (declared-name "multiplicity") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind "attribute def") (name "numberOfEntities") (declared-name "numberOfEntities") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfMoles"))) (kind "alias") (name "numberOfMoles") (declared-name "numberOfMoles") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (kind "alias") (name "osmoticCoefficientOfSolventA") (declared-name "osmoticCoefficientOfSolventA") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind "attribute def") (name "osmoticFactorOfSolvent") (declared-name "osmoticFactorOfSolvent") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "OsmoticFactorOfSolventValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind "attribute def") (name "osmoticPressure") (declared-name "osmoticPressure") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "OsmoticPressureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind "attribute def") (name "partialPressure") (declared-name "partialPressure") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "PartialPressureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind "attribute def") (name "particleConcentration") (declared-name "particleConcentration") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ParticleConcentrationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (kind "alias") (name "partitionFunctionOfAMolecule") (declared-name "partitionFunctionOfAMolecule") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (kind "alias") (name "relativeActivityOfSolute") (declared-name "relativeActivityOfSolute") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (kind "alias") (name "relativeActivityOfSolvent") (declared-name "relativeActivityOfSolvent") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind "attribute def") (name "relativeAtomicMass") (declared-name "relativeAtomicMass") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeAtomicMassValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind "attribute def") (name "specificOpticalRotatoryPower") (declared-name "specificOpticalRotatoryPower") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificOpticalRotatoryPowerValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind "attribute def") (name "standardAbsoluteActivityInMixture") (declared-name "standardAbsoluteActivityInMixture") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardAbsoluteActivityInMixtureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind "attribute def") (name "standardAbsoluteActivityInSolution") (declared-name "standardAbsoluteActivityInSolution") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardAbsoluteActivityInSolutionValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind "attribute def") (name "standardAbsoluteActivityOfSolvent") (declared-name "standardAbsoluteActivityOfSolvent") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardAbsoluteActivityOfSolventValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind "attribute def") (name "standardAmountOfSubstanceConcentration") (declared-name "standardAmountOfSubstanceConcentration") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind "attribute def") (name "standardChemicalPotential") (declared-name "standardChemicalPotential") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardChemicalPotentialValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind "attribute def") (name "standardEquilibriumConstant") (declared-name "standardEquilibriumConstant") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardEquilibriumConstantValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind "attribute def") (name "statisticalWeightOfSubsystem") (declared-name "statisticalWeightOfSubsystem") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind "attribute def") (name "stoichiometricNumberOfSubstance") (declared-name "stoichiometricNumberOfSubstance") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StoichiometricNumberOfSubstanceValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind "attribute def") (name "thermalDiffusionCoefficient") (declared-name "thermalDiffusionCoefficient") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusionCoefficientValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind "attribute def") (name "thermalDiffusionFactor") (declared-name "thermalDiffusionFactor") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusionFactorValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind "attribute def") (name "thermalDiffusionRatio") (declared-name "thermalDiffusionRatio") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusionRatioValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (kind "alias") (name "thermodynamicEquilibriumConstant") (declared-name "thermodynamicEquilibriumConstant") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind "attribute def") (name "transportNumberOfTheIonB") (declared-name "transportNumberOfTheIonB") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransportNumberOfTheIonBValue")))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind "attribute def") (name "volumeFraction") (declared-name "volumeFraction") (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFractionValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AffinityOfAChemicalReactionUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::AngularMeasureValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ChemicalPotentialUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffusionCoefficientUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectrolyticConductivityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQThermodynamics::EnergyValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnConcentrationBasisUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnPressureBasisUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "FugacityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IonicStrengthUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolalityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEnthalpyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEntropyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGasConstantUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGibbsEnergyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHelmholtzEnergyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarOpticalRotatoryPowerUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarVolumeUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "OsmoticPressureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PartialPressureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificOpticalRotatoryPowerUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardChemicalPotentialUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionCoefficientUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFractionUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsoluteActivityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityCoefficientValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityFactorValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityOfSoluteValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityOfSolventValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind featureTyping) (ordinal 0)) (authored-target "AffinityOfAChemicalReactionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceFractionMoleFractionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "CanonicalPartitionFunctionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind featureTyping) (ordinal 0)) (authored-target "ChemicalPotentialValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind featureTyping) (ordinal 0)) (authored-target "DegeneracyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind featureTyping) (ordinal 0)) (authored-target "DegreeOfDissociationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffusionCoefficientValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectrolyticConductivityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnConcentrationBasisValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnPressureBasisValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind featureTyping) (ordinal 0)) (authored-target "FugacityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "GrandCanonicalPartitionFunctionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind featureTyping) (ordinal 0)) (authored-target "IonicStrengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (kind featureTyping) (ordinal 0)) (authored-target "MolalityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEnthalpyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEntropyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGasConstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGibbsEnergyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHelmholtzEnergyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarOpticalRotatoryPowerValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "MolecularPartitionFunctionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind featureTyping) (ordinal 0)) (authored-target "OsmoticFactorOfSolventValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "OsmoticPressureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "PartialPressureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeAtomicMassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificOpticalRotatoryPowerValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardAbsoluteActivityInMixtureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardAbsoluteActivityInSolutionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardAbsoluteActivityOfSolventValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardChemicalPotentialValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardEquilibriumConstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind featureTyping) (ordinal 0)) (authored-target "StoichiometricNumberOfSubstanceValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionCoefficientValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionFactorValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionRatioValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind featureTyping) (ordinal 0)) (authored-target "TransportNumberOfTheIonBValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFractionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 77 22) (end 77 25)) (probe (position 77 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 77 22) (end 77 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num") (range (start 77 8) (end 77 32)))
        )
      )
    )
    (query (range (start 103 22) (end 103 25)) (probe (position 103 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 103 22) (end 103 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num") (range (start 103 8) (end 103 32)))
        )
      )
    )
    (query (range (start 129 22) (end 129 25)) (probe (position 129 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 129 22) (end 129 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num") (range (start 129 8) (end 129 32)))
        )
      )
    )
    (query (range (start 157 22) (end 157 25)) (probe (position 157 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 157 22) (end 157 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num") (range (start 157 8) (end 157 32)))
        )
      )
    )
    (query (range (start 185 22) (end 185 25)) (probe (position 185 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 185 22) (end 185 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num") (range (start 185 8) (end 185 32)))
        )
      )
    )
    (query (range (start 213 22) (end 213 25)) (probe (position 213 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 213 22) (end 213 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num") (range (start 213 8) (end 213 32)))
        )
      )
    )
    (query (range (start 241 22) (end 241 25)) (probe (position 241 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 241 22) (end 241 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num") (range (start 241 8) (end 241 32)))
        )
      )
    )
    (query (range (start 270 22) (end 270 25)) (probe (position 270 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 270 22) (end 270 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num") (range (start 270 8) (end 270 32)))
        )
      )
    )
    (query (range (start 299 22) (end 299 25)) (probe (position 299 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 299 22) (end 299 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num") (range (start 299 8) (end 299 32)))
        )
      )
    )
    (query (range (start 340 22) (end 340 25)) (probe (position 340 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 340 22) (end 340 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num") (range (start 340 8) (end 340 32)))
        )
      )
    )
    (query (range (start 383 22) (end 383 25)) (probe (position 383 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 383 22) (end 383 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num") (range (start 383 8) (end 383 32)))
        )
      )
    )
    (query (range (start 442 22) (end 442 25)) (probe (position 442 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 442 22) (end 442 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num") (range (start 442 8) (end 442 32)))
        )
      )
    )
    (query (range (start 465 22) (end 465 25)) (probe (position 465 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 465 22) (end 465 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num") (range (start 465 8) (end 465 32)))
        )
      )
    )
    (query (range (start 509 22) (end 509 25)) (probe (position 509 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 509 22) (end 509 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num") (range (start 509 8) (end 509 32)))
        )
      )
    )
    (query (range (start 554 22) (end 554 25)) (probe (position 554 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 554 22) (end 554 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num") (range (start 554 8) (end 554 32)))
        )
      )
    )
    (query (range (start 581 22) (end 581 25)) (probe (position 581 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 581 22) (end 581 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num") (range (start 581 8) (end 581 32)))
        )
      )
    )
    (query (range (start 608 22) (end 608 25)) (probe (position 608 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 608 22) (end 608 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num") (range (start 608 8) (end 608 32)))
        )
      )
    )
    (query (range (start 778 22) (end 778 25)) (probe (position 778 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 778 22) (end 778 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num") (range (start 778 8) (end 778 32)))
        )
      )
    )
    (query (range (start 822 22) (end 822 25)) (probe (position 822 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 822 22) (end 822 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num") (range (start 822 8) (end 822 32)))
        )
      )
    )
    (query (range (start 885 22) (end 885 25)) (probe (position 885 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 885 22) (end 885 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num") (range (start 885 8) (end 885 32)))
        )
      )
    )
    (query (range (start 912 22) (end 912 25)) (probe (position 912 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 912 22) (end 912 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num") (range (start 912 8) (end 912 32)))
        )
      )
    )
    (query (range (start 1044 22) (end 1044 25)) (probe (position 1044 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1044 22) (end 1044 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num") (range (start 1044 8) (end 1044 32)))
        )
      )
    )
    (query (range (start 1092 22) (end 1092 25)) (probe (position 1092 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1092 22) (end 1092 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num") (range (start 1092 8) (end 1092 32)))
        )
      )
    )
    (query (range (start 1152 22) (end 1152 25)) (probe (position 1152 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1152 22) (end 1152 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num") (range (start 1152 8) (end 1152 32)))
        )
      )
    )
    (query (range (start 1178 22) (end 1178 25)) (probe (position 1178 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1178 22) (end 1178 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num") (range (start 1178 8) (end 1178 32)))
        )
      )
    )
    (query (range (start 1223 22) (end 1223 25)) (probe (position 1223 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1223 22) (end 1223 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num") (range (start 1223 8) (end 1223 32)))
        )
      )
    )
    (query (range (start 1251 22) (end 1251 25)) (probe (position 1251 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1251 22) (end 1251 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num") (range (start 1251 8) (end 1251 32)))
        )
      )
    )
    (query (range (start 1314 22) (end 1314 25)) (probe (position 1314 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1314 22) (end 1314 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num") (range (start 1314 8) (end 1314 32)))
        )
      )
    )
    (query (range (start 1340 22) (end 1340 25)) (probe (position 1340 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1340 22) (end 1340 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num") (range (start 1340 8) (end 1340 32)))
        )
      )
    )
    (query (range (start 78 22) (end 78 26)) (probe (position 78 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 78 22) (end 78 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef") (range (start 78 8) (end 78 45)))
        )
      )
    )
    (query (range (start 104 22) (end 104 26)) (probe (position 104 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 104 22) (end 104 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef") (range (start 104 8) (end 104 47)))
        )
      )
    )
    (query (range (start 130 22) (end 130 26)) (probe (position 130 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 130 22) (end 130 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef") (range (start 130 8) (end 130 55)))
        )
      )
    )
    (query (range (start 158 22) (end 158 26)) (probe (position 158 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 158 22) (end 158 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef") (range (start 158 8) (end 158 49)))
        )
      )
    )
    (query (range (start 186 22) (end 186 26)) (probe (position 186 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 186 22) (end 186 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef") (range (start 186 8) (end 186 56)))
        )
      )
    )
    (query (range (start 214 22) (end 214 26)) (probe (position 214 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 214 22) (end 214 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef") (range (start 214 8) (end 214 52)))
        )
      )
    )
    (query (range (start 242 22) (end 242 26)) (probe (position 242 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 242 22) (end 242 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef") (range (start 242 8) (end 242 53)))
        )
      )
    )
    (query (range (start 271 22) (end 271 26)) (probe (position 271 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 271 22) (end 271 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef") (range (start 271 8) (end 271 48)))
        )
      )
    )
    (query (range (start 300 22) (end 300 26)) (probe (position 300 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 300 22) (end 300 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef") (range (start 300 8) (end 300 57)))
        )
      )
    )
    (query (range (start 341 22) (end 341 26)) (probe (position 341 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 341 22) (end 341 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef") (range (start 341 8) (end 341 53)))
        )
      )
    )
    (query (range (start 384 22) (end 384 26)) (probe (position 384 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 384 22) (end 384 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef") (range (start 384 8) (end 384 66)))
        )
      )
    )
    (query (range (start 443 22) (end 443 26)) (probe (position 443 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 443 22) (end 443 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef") (range (start 443 8) (end 443 50)))
        )
      )
    )
    (query (range (start 466 22) (end 466 26)) (probe (position 466 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 466 22) (end 466 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef") (range (start 466 8) (end 466 44)))
        )
      )
    )
    (query (range (start 510 22) (end 510 26)) (probe (position 510 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 510 22) (end 510 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef") (range (start 510 8) (end 510 53)))
        )
      )
    )
    (query (range (start 555 22) (end 555 26)) (probe (position 555 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 555 22) (end 555 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef") (range (start 555 8) (end 555 51)))
        )
      )
    )
    (query (range (start 582 22) (end 582 26)) (probe (position 582 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 582 22) (end 582 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef") (range (start 582 8) (end 582 44)))
        )
      )
    )
    (query (range (start 609 22) (end 609 26)) (probe (position 609 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 609 22) (end 609 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef") (range (start 609 8) (end 609 61)))
        )
      )
    )
    (query (range (start 779 22) (end 779 26)) (probe (position 779 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 779 22) (end 779 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef") (range (start 779 8) (end 779 51)))
        )
      )
    )
    (query (range (start 823 22) (end 823 26)) (probe (position 823 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 823 22) (end 823 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef") (range (start 823 8) (end 823 63)))
        )
      )
    )
    (query (range (start 886 22) (end 886 26)) (probe (position 886 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 886 22) (end 886 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef") (range (start 886 8) (end 886 70)))
        )
      )
    )
    (query (range (start 913 22) (end 913 26)) (probe (position 913 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 913 22) (end 913 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef") (range (start 913 8) (end 913 75)))
        )
      )
    )
    (query (range (start 1045 22) (end 1045 26)) (probe (position 1045 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1045 22) (end 1045 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef") (range (start 1045 8) (end 1045 52)))
        )
      )
    )
    (query (range (start 1093 22) (end 1093 26)) (probe (position 1093 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1093 22) (end 1093 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef") (range (start 1093 8) (end 1093 56)))
        )
      )
    )
    (query (range (start 1153 22) (end 1153 26)) (probe (position 1153 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1153 22) (end 1153 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef") (range (start 1153 8) (end 1153 63)))
        )
      )
    )
    (query (range (start 1179 22) (end 1179 26)) (probe (position 1179 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1179 22) (end 1179 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef") (range (start 1179 8) (end 1179 49)))
        )
      )
    )
    (query (range (start 1224 22) (end 1224 26)) (probe (position 1224 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1224 22) (end 1224 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef") (range (start 1224 8) (end 1224 60)))
        )
      )
    )
    (query (range (start 1252 22) (end 1252 26)) (probe (position 1252 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1252 22) (end 1252 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef") (range (start 1252 8) (end 1252 53)))
        )
      )
    )
    (query (range (start 1315 22) (end 1315 26)) (probe (position 1315 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1315 22) (end 1315 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef") (range (start 1315 8) (end 1315 61)))
        )
      )
    )
    (query (range (start 1341 22) (end 1341 26)) (probe (position 1341 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1341 22) (end 1341 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef") (range (start 1341 8) (end 1341 64)))
        )
      )
    )
    (query (range (start 17 19) (end 17 26)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQBase::*")
        (range (start 17 19) (end 17 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 29)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 15 19) (end 15 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 86 22) (end 86 39)) (probe (position 86 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 86 22) (end 86 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension") (range (start 86 8) (end 86 101)))
        )
      )
    )
    (query (range (start 112 22) (end 112 39)) (probe (position 112 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 112 22) (end 112 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension") (range (start 112 8) (end 112 103)))
        )
      )
    )
    (query (range (start 140 22) (end 140 39)) (probe (position 140 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 140 22) (end 140 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension") (range (start 140 8) (end 140 123)))
        )
      )
    )
    (query (range (start 168 22) (end 168 39)) (probe (position 168 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 168 22) (end 168 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension") (range (start 168 8) (end 168 123)))
        )
      )
    )
    (query (range (start 196 22) (end 196 39)) (probe (position 196 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 196 22) (end 196 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension") (range (start 196 8) (end 196 123)))
        )
      )
    )
    (query (range (start 224 22) (end 224 39)) (probe (position 224 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 224 22) (end 224 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension") (range (start 224 8) (end 224 123)))
        )
      )
    )
    (query (range (start 253 22) (end 253 39)) (probe (position 253 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 253 22) (end 253 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension") (range (start 253 8) (end 253 151)))
        )
      )
    )
    (query (range (start 282 22) (end 282 39)) (probe (position 282 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 282 22) (end 282 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension") (range (start 282 8) (end 282 151)))
        )
      )
    )
    (query (range (start 307 22) (end 307 39)) (probe (position 307 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 307 22) (end 307 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension") (range (start 307 8) (end 307 80)))
        )
      )
    )
    (query (range (start 349 22) (end 349 39)) (probe (position 349 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 349 22) (end 349 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension") (range (start 349 8) (end 349 90)))
        )
      )
    )
    (query (range (start 392 22) (end 392 39)) (probe (position 392 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 392 22) (end 392 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension") (range (start 392 8) (end 392 103)))
        )
      )
    )
    (query (range (start 474 22) (end 474 39)) (probe (position 474 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 474 22) (end 474 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension") (range (start 474 8) (end 474 101)))
        )
      )
    )
    (query (range (start 520 22) (end 520 39)) (probe (position 520 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 520 22) (end 520 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension") (range (start 520 8) (end 520 123)))
        )
      )
    )
    (query (range (start 564 22) (end 564 39)) (probe (position 564 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 564 22) (end 564 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension") (range (start 564 8) (end 564 102)))
        )
      )
    )
    (query (range (start 591 22) (end 591 39)) (probe (position 591 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 591 22) (end 591 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension") (range (start 591 8) (end 591 102)))
        )
      )
    )
    (query (range (start 619 22) (end 619 39)) (probe (position 619 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 619 22) (end 619 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension") (range (start 619 8) (end 619 123)))
        )
      )
    )
    (query (range (start 788 22) (end 788 39)) (probe (position 788 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 788 22) (end 788 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension") (range (start 788 8) (end 788 102)))
        )
      )
    )
    (query (range (start 833 22) (end 833 39)) (probe (position 833 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 833 22) (end 833 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension") (range (start 833 8) (end 833 123)))
        )
      )
    )
    (query (range (start 895 22) (end 895 39)) (probe (position 895 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 895 22) (end 895 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension") (range (start 895 8) (end 895 102)))
        )
      )
    )
    (query (range (start 921 22) (end 921 39)) (probe (position 921 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 921 22) (end 921 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension") (range (start 921 8) (end 921 103)))
        )
      )
    )
    (query (range (start 1056 22) (end 1056 39)) (probe (position 1056 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1056 22) (end 1056 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension") (range (start 1056 8) (end 1056 151)))
        )
      )
    )
    (query (range (start 1101 22) (end 1101 39)) (probe (position 1101 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1101 22) (end 1101 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension") (range (start 1101 8) (end 1101 94)))
        )
      )
    )
    (query (range (start 1161 22) (end 1161 39)) (probe (position 1161 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1161 22) (end 1161 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension") (range (start 1161 8) (end 1161 94)))
        )
      )
    )
    (query (range (start 1187 22) (end 1187 39)) (probe (position 1187 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1187 22) (end 1187 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension") (range (start 1187 8) (end 1187 101)))
        )
      )
    )
    (query (range (start 1234 22) (end 1234 39)) (probe (position 1234 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1234 22) (end 1234 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension") (range (start 1234 8) (end 1234 121)))
        )
      )
    )
    (query (range (start 1262 22) (end 1262 39)) (probe (position 1262 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1262 22) (end 1262 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension") (range (start 1262 8) (end 1262 132)))
        )
      )
    )
    (query (range (start 1323 22) (end 1323 39)) (probe (position 1323 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1323 22) (end 1323 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension") (range (start 1323 8) (end 1323 103)))
        )
      )
    )
    (query (range (start 1349 22) (end 1349 39)) (probe (position 1349 22))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1349 22) (end 1349 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension") (range (start 1349 8) (end 1349 90)))
        )
      )
    )
    (query (range (start 14 19) (end 14 37)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 14 19) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 40)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 16 19) (end 16 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 19) (end 21 49)) (probe (position 21 19))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQThermodynamics::EnergyValue")
        (range (start 21 19) (end 21 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 19) (end 20 52)) (probe (position 20 19))
      (reference
        (source (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::AngularMeasureValue")
        (range (start 20 19) (end 20 52))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
