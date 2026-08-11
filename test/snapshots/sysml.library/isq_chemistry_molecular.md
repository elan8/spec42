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
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (kind "package") (name "ISQChemistryMolecular") (declared-name "ISQChemistryMolecular") (range (start (line 0) (character 0)) (end (line 0) (character 71442))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind "attribute def") (name "AbsoluteActivityValue") (declared-name "AbsoluteActivityValue") (range (start (line 524) (character 4)) (end (line 524) (character 657))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 524) (character 4)) (end (line 524) (character 657))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind "attribute def") (name "ActivityCoefficientValue") (declared-name "ActivityCoefficientValue") (range (start (line 676) (character 4)) (end (line 676) (character 772))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 676) (character 4)) (end (line 676) (character 772))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind "attribute def") (name "ActivityFactorValue") (declared-name "ActivityFactorValue") (range (start (line 623) (character 4)) (end (line 623) (character 966))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 623) (character 4)) (end (line 623) (character 966))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind "attribute def") (name "ActivityOfSoluteValue") (declared-name "ActivityOfSoluteValue") (range (start (line 657) (character 4)) (end (line 657) (character 1369))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue::_documentation"))) (kind "documentation") (name "") (range (start (line 657) (character 4)) (end (line 657) (character 1369))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind "attribute def") (name "ActivityOfSolventValue") (declared-name "ActivityOfSolventValue") (range (start (line 710) (character 4)) (end (line 710) (character 703))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue::_documentation"))) (kind "documentation") (name "") (range (start (line 710) (character 4)) (end (line 710) (character 703))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind "attribute def") (name "AffinityOfAChemicalReactionUnit") (declared-name "AffinityOfAChemicalReactionUnit") (range (start (line 828) (character 4)) (end (line 828) (character 621))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 832) (character 8)) (end (line 832) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 831) (character 8)) (end (line 831) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 829) (character 8)) (end (line 829) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 830) (character 8)) (end (line 830) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 833) (character 8)) (end (line 833) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 833) (character 22)) (end (line 833) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind "attribute def") (name "AffinityOfAChemicalReactionValue") (declared-name "AffinityOfAChemicalReactionValue") (range (start (line 809) (character 4)) (end (line 809) (character 1223))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 809) (character 4)) (end (line 809) (character 1223))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 823) (character 8)) (end (line 823) (character 63))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AffinityOfAChemicalReactionUnit") (range none)) (redefinition (reference "mRef") (range (start (line 823) (character 22)) (end (line 823) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 822) (character 8)) (end (line 822) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 822) (character 22)) (end (line 822) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind "attribute def") (name "AmountOfSubstanceConcentrationUnit") (declared-name "AmountOfSubstanceConcentrationUnit") (range (start (line 389) (character 4)) (end (line 389) (character 397))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 391) (character 8)) (end (line 391) (character 113))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 390) (character 8)) (end (line 390) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 392) (character 8)) (end (line 392) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 392) (character 22)) (end (line 392) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind "attribute def") (name "AmountOfSubstanceConcentrationValue") (declared-name "AmountOfSubstanceConcentrationValue") (range (start (line 370) (character 4)) (end (line 370) (character 1103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 370) (character 4)) (end (line 370) (character 1103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 384) (character 8)) (end (line 384) (character 66))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmountOfSubstanceConcentrationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 384) (character 22)) (end (line 384) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 383) (character 8)) (end (line 383) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 383) (character 22)) (end (line 383) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind "attribute def") (name "AmountOfSubstanceFractionMoleFractionValue") (declared-name "AmountOfSubstanceFractionMoleFractionValue") (range (start (line 412) (character 4)) (end (line 412) (character 977))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 412) (character 4)) (end (line 412) (character 977))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))) (kind "import") (name "AngularMeasureValue") (declared-name "AngularMeasureValue") (range (start (line 20) (character 4)) (end (line 20) (character 53))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::AngularMeasureValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 19)) (end (line 20) (character 52))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind "attribute def") (name "CanonicalPartitionFunctionValue") (declared-name "CanonicalPartitionFunctionValue") (range (start (line 941) (character 4)) (end (line 941) (character 713))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 941) (character 4)) (end (line 941) (character 713))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind "attribute def") (name "ChemicalPotentialUnit") (declared-name "ChemicalPotentialUnit") (range (start (line 515) (character 4)) (end (line 515) (character 611))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 519) (character 8)) (end (line 519) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 518) (character 8)) (end (line 518) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 516) (character 8)) (end (line 516) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 517) (character 8)) (end (line 517) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 520) (character 8)) (end (line 520) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 520) (character 22)) (end (line 520) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind "attribute def") (name "ChemicalPotentialValue") (declared-name "ChemicalPotentialValue") (range (start (line 496) (character 4)) (end (line 496) (character 935))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::_documentation"))) (kind "documentation") (name "") (range (start (line 496) (character 4)) (end (line 496) (character 935))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 510) (character 8)) (end (line 510) (character 53))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ChemicalPotentialUnit") (range none)) (redefinition (reference "mRef") (range (start (line 510) (character 22)) (end (line 510) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 509) (character 8)) (end (line 509) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 509) (character 22)) (end (line 509) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind "attribute def") (name "DegeneracyValue") (declared-name "DegeneracyValue") (range (start (line 1012) (character 4)) (end (line 1012) (character 518))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1012) (character 4)) (end (line 1012) (character 518))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind "attribute def") (name "DegreeOfDissociationValue") (declared-name "DegreeOfDissociationValue") (range (start (line 1191) (character 4)) (end (line 1191) (character 585))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1191) (character 4)) (end (line 1191) (character 585))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind "attribute def") (name "DiffusionCoefficientUnit") (declared-name "DiffusionCoefficientUnit") (range (start (line 1098) (character 4)) (end (line 1098) (character 369))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1100) (character 8)) (end (line 1100) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1099) (character 8)) (end (line 1099) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1101) (character 8)) (end (line 1101) (character 94))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1101) (character 22)) (end (line 1101) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind "attribute def") (name "DiffusionCoefficientValue") (declared-name "DiffusionCoefficientValue") (range (start (line 1079) (character 4)) (end (line 1079) (character 861))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1079) (character 4)) (end (line 1079) (character 861))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1093) (character 8)) (end (line 1093) (character 56))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiffusionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1093) (character 22)) (end (line 1093) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1092) (character 8)) (end (line 1092) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1092) (character 22)) (end (line 1092) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind "attribute def") (name "ElectrolyticConductivityUnit") (declared-name "ElectrolyticConductivityUnit") (range (start (line 1229) (character 4)) (end (line 1229) (character 614))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1232) (character 8)) (end (line 1232) (character 104))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 1233) (character 8)) (end (line 1233) (character 111))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1230) (character 8)) (end (line 1230) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1231) (character 8)) (end (line 1231) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1234) (character 8)) (end (line 1234) (character 121))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1234) (character 22)) (end (line 1234) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind "attribute def") (name "ElectrolyticConductivityValue") (declared-name "ElectrolyticConductivityValue") (range (start (line 1210) (character 4)) (end (line 1210) (character 796))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1210) (character 4)) (end (line 1210) (character 796))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1224) (character 8)) (end (line 1224) (character 60))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ElectrolyticConductivityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1224) (character 22)) (end (line 1224) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1223) (character 8)) (end (line 1223) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1223) (character 22)) (end (line 1223) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))) (kind "import") (name "EnergyValue") (declared-name "EnergyValue") (range (start (line 21) (character 4)) (end (line 21) (character 50))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQThermodynamics::EnergyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 19)) (end (line 21) (character 49))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind "attribute def") (name "EquilibriumConstantOnConcentrationBasisUnit") (declared-name "EquilibriumConstantOnConcentrationBasisUnit") (range (start (line 918) (character 4)) (end (line 918) (character 406))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 920) (character 8)) (end (line 920) (character 113))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 919) (character 8)) (end (line 919) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 921) (character 8)) (end (line 921) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 921) (character 22)) (end (line 921) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind "attribute def") (name "EquilibriumConstantOnConcentrationBasisValue") (declared-name "EquilibriumConstantOnConcentrationBasisValue") (range (start (line 899) (character 4)) (end (line 899) (character 787))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::_documentation"))) (kind "documentation") (name "") (range (start (line 899) (character 4)) (end (line 899) (character 787))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 913) (character 8)) (end (line 913) (character 75))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EquilibriumConstantOnConcentrationBasisUnit") (range none)) (redefinition (reference "mRef") (range (start (line 913) (character 22)) (end (line 913) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 912) (character 8)) (end (line 912) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 912) (character 22)) (end (line 912) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind "attribute def") (name "EquilibriumConstantOnPressureBasisUnit") (declared-name "EquilibriumConstantOnPressureBasisUnit") (range (start (line 891) (character 4)) (end (line 891) (character 493))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 894) (character 8)) (end (line 894) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 892) (character 8)) (end (line 892) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 893) (character 8)) (end (line 893) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 895) (character 8)) (end (line 895) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 895) (character 22)) (end (line 895) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind "attribute def") (name "EquilibriumConstantOnPressureBasisValue") (declared-name "EquilibriumConstantOnPressureBasisValue") (range (start (line 872) (character 4)) (end (line 872) (character 774))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::_documentation"))) (kind "documentation") (name "") (range (start (line 872) (character 4)) (end (line 872) (character 774))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 886) (character 8)) (end (line 886) (character 70))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EquilibriumConstantOnPressureBasisUnit") (range none)) (redefinition (reference "mRef") (range (start (line 886) (character 22)) (end (line 886) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 885) (character 8)) (end (line 885) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 885) (character 22)) (end (line 885) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind "attribute def") (name "FugacityUnit") (declared-name "FugacityUnit") (range (start (line 587) (character 4)) (end (line 587) (character 467))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 590) (character 8)) (end (line 590) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 588) (character 8)) (end (line 588) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 589) (character 8)) (end (line 589) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 591) (character 8)) (end (line 591) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 591) (character 22)) (end (line 591) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind "attribute def") (name "FugacityValue") (declared-name "FugacityValue") (range (start (line 568) (character 4)) (end (line 568) (character 922))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 568) (character 4)) (end (line 568) (character 922))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 582) (character 8)) (end (line 582) (character 44))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "FugacityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 582) (character 22)) (end (line 582) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 581) (character 8)) (end (line 581) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 581) (character 22)) (end (line 581) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind "attribute def") (name "GrandCanonicalPartitionFunctionValue") (declared-name "GrandCanonicalPartitionFunctionValue") (range (start (line 958) (character 4)) (end (line 958) (character 921))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 958) (character 4)) (end (line 958) (character 921))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind "attribute def") (name "IonicStrengthUnit") (declared-name "IonicStrengthUnit") (range (start (line 1184) (character 4)) (end (line 1184) (character 376))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 1186) (character 8)) (end (line 1186) (character 113))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1185) (character 8)) (end (line 1185) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1187) (character 8)) (end (line 1187) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1187) (character 22)) (end (line 1187) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind "attribute def") (name "IonicStrengthValue") (declared-name "IonicStrengthValue") (range (start (line 1165) (character 4)) (end (line 1165) (character 674))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1165) (character 4)) (end (line 1165) (character 674))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1179) (character 8)) (end (line 1179) (character 49))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IonicStrengthUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1179) (character 22)) (end (line 1179) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1178) (character 8)) (end (line 1178) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1178) (character 22)) (end (line 1178) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind "attribute def") (name "MassConcentrationUnit") (declared-name "MassConcentrationUnit") (range (start (line 346) (character 4)) (end (line 346) (character 358))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 347) (character 8)) (end (line 347) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 348) (character 8)) (end (line 348) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 349) (character 8)) (end (line 349) (character 90))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 349) (character 22)) (end (line 349) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind "attribute def") (name "MassConcentrationValue") (declared-name "MassConcentrationValue") (range (start (line 327) (character 4)) (end (line 327) (character 748))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 327) (character 4)) (end (line 327) (character 748))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 341) (character 8)) (end (line 341) (character 53))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConcentrationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 341) (character 22)) (end (line 341) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 340) (character 8)) (end (line 340) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 340) (character 22)) (end (line 340) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind "attribute def") (name "MassFractionValue") (declared-name "MassFractionValue") (range (start (line 353) (character 4)) (end (line 353) (character 552))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 353) (character 4)) (end (line 353) (character 552))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind "attribute def") (name "MolalityUnit") (declared-name "MolalityUnit") (range (start (line 471) (character 4)) (end (line 471) (character 371))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 473) (character 8)) (end (line 473) (character 113))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 472) (character 8)) (end (line 472) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 474) (character 8)) (end (line 474) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 474) (character 22)) (end (line 474) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind "attribute def") (name "MolalityValue") (declared-name "MolalityValue") (range (start (line 452) (character 4)) (end (line 452) (character 842))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 452) (character 4)) (end (line 452) (character 842))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 466) (character 8)) (end (line 466) (character 44))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolalityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 466) (character 22)) (end (line 466) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 465) (character 8)) (end (line 465) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 465) (character 22)) (end (line 465) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind "attribute def") (name "MolarConductivityUnit") (declared-name "MolarConductivityUnit") (range (start (line 1257) (character 4)) (end (line 1257) (character 629))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 1261) (character 8)) (end (line 1261) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1259) (character 8)) (end (line 1259) (character 104))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 1260) (character 8)) (end (line 1260) (character 111))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1258) (character 8)) (end (line 1258) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1262) (character 8)) (end (line 1262) (character 132))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1262) (character 22)) (end (line 1262) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind "attribute def") (name "MolarConductivityValue") (declared-name "MolarConductivityValue") (range (start (line 1238) (character 4)) (end (line 1238) (character 682))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1238) (character 4)) (end (line 1238) (character 682))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1252) (character 8)) (end (line 1252) (character 53))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarConductivityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1252) (character 22)) (end (line 1252) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1251) (character 8)) (end (line 1251) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1251) (character 22)) (end (line 1251) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind "attribute def") (name "MolarEnthalpyUnit") (declared-name "MolarEnthalpyUnit") (range (start (line 163) (character 4)) (end (line 163) (character 607))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 167) (character 8)) (end (line 167) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 166) (character 8)) (end (line 166) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 164) (character 8)) (end (line 164) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 165) (character 8)) (end (line 165) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 168) (character 8)) (end (line 168) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 168) (character 22)) (end (line 168) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind "attribute def") (name "MolarEnthalpyValue") (declared-name "MolarEnthalpyValue") (range (start (line 144) (character 4)) (end (line 144) (character 671))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 144) (character 4)) (end (line 144) (character 671))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 158) (character 8)) (end (line 158) (character 49))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarEnthalpyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 158) (character 22)) (end (line 158) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 157) (character 8)) (end (line 157) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 157) (character 22)) (end (line 157) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind "attribute def") (name "MolarEntropyUnit") (declared-name "MolarEntropyUnit") (range (start (line 276) (character 4)) (end (line 276) (character 759))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 281) (character 8)) (end (line 281) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 279) (character 8)) (end (line 279) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 277) (character 8)) (end (line 277) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 278) (character 8)) (end (line 278) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 282) (character 8)) (end (line 282) (character 151))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 282) (character 22)) (end (line 282) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 280) (character 8)) (end (line 280) (character 124))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind "attribute def") (name "MolarEntropyValue") (declared-name "MolarEntropyValue") (range (start (line 257) (character 4)) (end (line 257) (character 669))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 257) (character 4)) (end (line 257) (character 669))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 271) (character 8)) (end (line 271) (character 48))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarEntropyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 271) (character 22)) (end (line 271) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 270) (character 8)) (end (line 270) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 270) (character 22)) (end (line 270) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind "attribute def") (name "MolarGasConstantUnit") (declared-name "MolarGasConstantUnit") (range (start (line 1050) (character 4)) (end (line 1050) (character 763))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 1055) (character 8)) (end (line 1055) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1053) (character 8)) (end (line 1053) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1051) (character 8)) (end (line 1051) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1052) (character 8)) (end (line 1052) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1056) (character 8)) (end (line 1056) (character 151))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1056) (character 22)) (end (line 1056) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 1054) (character 8)) (end (line 1054) (character 124))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind "attribute def") (name "MolarGasConstantValue") (declared-name "MolarGasConstantValue") (range (start (line 1031) (character 4)) (end (line 1031) (character 650))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1031) (character 4)) (end (line 1031) (character 650))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1045) (character 8)) (end (line 1045) (character 52))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarGasConstantUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1045) (character 22)) (end (line 1045) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1044) (character 8)) (end (line 1044) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1044) (character 22)) (end (line 1044) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind "attribute def") (name "MolarGibbsEnergyUnit") (declared-name "MolarGibbsEnergyUnit") (range (start (line 219) (character 4)) (end (line 219) (character 610))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 223) (character 8)) (end (line 223) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 222) (character 8)) (end (line 222) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 220) (character 8)) (end (line 220) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 221) (character 8)) (end (line 221) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 224) (character 8)) (end (line 224) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 224) (character 22)) (end (line 224) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind "attribute def") (name "MolarGibbsEnergyValue") (declared-name "MolarGibbsEnergyValue") (range (start (line 200) (character 4)) (end (line 200) (character 692))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 200) (character 4)) (end (line 200) (character 692))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 214) (character 8)) (end (line 214) (character 52))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarGibbsEnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 214) (character 22)) (end (line 214) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 213) (character 8)) (end (line 213) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 213) (character 22)) (end (line 213) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind "attribute def") (name "MolarHeatCapacityUnit") (declared-name "MolarHeatCapacityUnit") (range (start (line 247) (character 4)) (end (line 247) (character 764))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 252) (character 8)) (end (line 252) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 250) (character 8)) (end (line 250) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 248) (character 8)) (end (line 248) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 249) (character 8)) (end (line 249) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 253) (character 8)) (end (line 253) (character 151))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 253) (character 22)) (end (line 253) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 251) (character 8)) (end (line 251) (character 124))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind "attribute def") (name "MolarHeatCapacityValue") (declared-name "MolarHeatCapacityValue") (range (start (line 228) (character 4)) (end (line 228) (character 696))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 228) (character 4)) (end (line 228) (character 696))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 242) (character 8)) (end (line 242) (character 53))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarHeatCapacityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 242) (character 22)) (end (line 242) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 241) (character 8)) (end (line 241) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 241) (character 22)) (end (line 241) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind "attribute def") (name "MolarHelmholtzEnergyUnit") (declared-name "MolarHelmholtzEnergyUnit") (range (start (line 191) (character 4)) (end (line 191) (character 614))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 195) (character 8)) (end (line 195) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 194) (character 8)) (end (line 194) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 192) (character 8)) (end (line 192) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 193) (character 8)) (end (line 193) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 196) (character 8)) (end (line 196) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 196) (character 22)) (end (line 196) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind "attribute def") (name "MolarHelmholtzEnergyValue") (declared-name "MolarHelmholtzEnergyValue") (range (start (line 172) (character 4)) (end (line 172) (character 712))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 172) (character 4)) (end (line 172) (character 712))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 186) (character 8)) (end (line 186) (character 56))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarHelmholtzEnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 186) (character 22)) (end (line 186) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 185) (character 8)) (end (line 185) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 185) (character 22)) (end (line 185) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind "attribute def") (name "MolarInternalEnergyUnit") (declared-name "MolarInternalEnergyUnit") (range (start (line 135) (character 4)) (end (line 135) (character 613))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 139) (character 8)) (end (line 139) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 138) (character 8)) (end (line 138) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 136) (character 8)) (end (line 136) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 137) (character 8)) (end (line 137) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 140) (character 8)) (end (line 140) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 140) (character 22)) (end (line 140) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind "attribute def") (name "MolarInternalEnergyValue") (declared-name "MolarInternalEnergyValue") (range (start (line 116) (character 4)) (end (line 116) (character 703))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 116) (character 4)) (end (line 116) (character 703))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 130) (character 8)) (end (line 130) (character 55))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarInternalEnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 130) (character 22)) (end (line 130) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 129) (character 8)) (end (line 129) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 129) (character 22)) (end (line 129) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind "attribute def") (name "MolarMassUnit") (declared-name "MolarMassUnit") (range (start (line 83) (character 4)) (end (line 83) (character 372))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 85) (character 8)) (end (line 85) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 84) (character 8)) (end (line 84) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 86) (character 8)) (end (line 86) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 86) (character 22)) (end (line 86) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind "attribute def") (name "MolarMassValue") (declared-name "MolarMassValue") (range (start (line 64) (character 4)) (end (line 64) (character 590))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::_documentation"))) (kind "documentation") (name "") (range (start (line 64) (character 4)) (end (line 64) (character 590))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 78) (character 8)) (end (line 78) (character 45))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarMassUnit") (range none)) (redefinition (reference "mRef") (range (start (line 78) (character 22)) (end (line 78) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 77) (character 8)) (end (line 77) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 77) (character 22)) (end (line 77) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind "attribute def") (name "MolarOpticalRotatoryPowerUnit") (declared-name "MolarOpticalRotatoryPowerUnit") (range (start (line 1320) (character 4)) (end (line 1320) (character 392))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 1322) (character 8)) (end (line 1322) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1321) (character 8)) (end (line 1321) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1323) (character 8)) (end (line 1323) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1323) (character 22)) (end (line 1323) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind "attribute def") (name "MolarOpticalRotatoryPowerValue") (declared-name "MolarOpticalRotatoryPowerValue") (range (start (line 1301) (character 4)) (end (line 1301) (character 818))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1301) (character 4)) (end (line 1301) (character 818))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1315) (character 8)) (end (line 1315) (character 61))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarOpticalRotatoryPowerUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1315) (character 22)) (end (line 1315) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1314) (character 8)) (end (line 1314) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1314) (character 22)) (end (line 1314) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind "attribute def") (name "MolarVolumeUnit") (declared-name "MolarVolumeUnit") (range (start (line 109) (character 4)) (end (line 109) (character 378))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 111) (character 8)) (end (line 111) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 110) (character 8)) (end (line 110) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 112) (character 8)) (end (line 112) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 112) (character 22)) (end (line 112) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind "attribute def") (name "MolarVolumeValue") (declared-name "MolarVolumeValue") (range (start (line 90) (character 4)) (end (line 90) (character 592))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 90) (character 4)) (end (line 90) (character 592))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 104) (character 8)) (end (line 104) (character 47))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarVolumeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 104) (character 22)) (end (line 104) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 103) (character 8)) (end (line 103) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 103) (character 22)) (end (line 103) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind "attribute def") (name "MolecularPartitionFunctionValue") (declared-name "MolecularPartitionFunctionValue") (range (start (line 977) (character 4)) (end (line 977) (character 778))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 977) (character 4)) (end (line 977) (character 778))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::NumberOfMolesUnit"))) (kind "alias") (name "NumberOfMolesUnit") (declared-name "NumberOfMolesUnit") (range (start (line 42) (character 4)) (end (line 42) (character 54))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::NumberOfMolesValue"))) (kind "alias") (name "NumberOfMolesValue") (declared-name "NumberOfMolesValue") (range (start (line 43) (character 4)) (end (line 43) (character 56))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind "attribute def") (name "OsmoticFactorOfSolventValue") (declared-name "OsmoticFactorOfSolventValue") (range (start (line 729) (character 4)) (end (line 729) (character 899))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue::_documentation"))) (kind "documentation") (name "") (range (start (line 729) (character 4)) (end (line 729) (character 899))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind "attribute def") (name "OsmoticPressureUnit") (declared-name "OsmoticPressureUnit") (range (start (line 784) (character 4)) (end (line 784) (character 474))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 787) (character 8)) (end (line 787) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 785) (character 8)) (end (line 785) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 786) (character 8)) (end (line 786) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 788) (character 8)) (end (line 788) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 788) (character 22)) (end (line 788) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind "attribute def") (name "OsmoticPressureValue") (declared-name "OsmoticPressureValue") (range (start (line 765) (character 4)) (end (line 765) (character 669))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 765) (character 4)) (end (line 765) (character 669))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 779) (character 8)) (end (line 779) (character 51))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "OsmoticPressureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 779) (character 22)) (end (line 779) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 778) (character 8)) (end (line 778) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 778) (character 22)) (end (line 778) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind "attribute def") (name "PartialPressureUnit") (declared-name "PartialPressureUnit") (range (start (line 560) (character 4)) (end (line 560) (character 474))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 563) (character 8)) (end (line 563) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 561) (character 8)) (end (line 561) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 562) (character 8)) (end (line 562) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 564) (character 8)) (end (line 564) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 564) (character 22)) (end (line 564) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind "attribute def") (name "PartialPressureValue") (declared-name "PartialPressureValue") (range (start (line 541) (character 4)) (end (line 541) (character 670))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 541) (character 4)) (end (line 541) (character 670))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 555) (character 8)) (end (line 555) (character 51))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PartialPressureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 555) (character 22)) (end (line 555) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 554) (character 8)) (end (line 554) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 554) (character 22)) (end (line 554) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind "attribute def") (name "ParticleConcentrationUnit") (declared-name "ParticleConcentrationUnit") (range (start (line 305) (character 4)) (end (line 305) (character 251))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 306) (character 8)) (end (line 306) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 307) (character 8)) (end (line 307) (character 80))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 307) (character 22)) (end (line 307) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind "attribute def") (name "ParticleConcentrationValue") (declared-name "ParticleConcentrationValue") (range (start (line 286) (character 4)) (end (line 286) (character 635))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 286) (character 4)) (end (line 286) (character 635))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 300) (character 8)) (end (line 300) (character 57))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ParticleConcentrationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 300) (character 22)) (end (line 300) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 299) (character 8)) (end (line 299) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 299) (character 22)) (end (line 299) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind "attribute def") (name "RelativeAtomicMassValue") (declared-name "RelativeAtomicMassValue") (range (start (line 47) (character 4)) (end (line 47) (character 1010))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue::_documentation"))) (kind "documentation") (name "") (range (start (line 47) (character 4)) (end (line 47) (character 1010))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind "attribute def") (name "SpecificOpticalRotatoryPowerUnit") (declared-name "SpecificOpticalRotatoryPowerUnit") (range (start (line 1346) (character 4)) (end (line 1346) (character 369))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1347) (character 8)) (end (line 1347) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1348) (character 8)) (end (line 1348) (character 101))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1349) (character 8)) (end (line 1349) (character 90))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1349) (character 22)) (end (line 1349) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind "attribute def") (name "SpecificOpticalRotatoryPowerValue") (declared-name "SpecificOpticalRotatoryPowerValue") (range (start (line 1327) (character 4)) (end (line 1327) (character 816))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1327) (character 4)) (end (line 1327) (character 816))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1341) (character 8)) (end (line 1341) (character 64))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificOpticalRotatoryPowerUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1341) (character 22)) (end (line 1341) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1340) (character 8)) (end (line 1340) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1340) (character 22)) (end (line 1340) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind "attribute def") (name "StandardAbsoluteActivityInMixtureValue") (declared-name "StandardAbsoluteActivityInMixtureValue") (range (start (line 640) (character 4)) (end (line 640) (character 768))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 640) (character 4)) (end (line 640) (character 768))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind "attribute def") (name "StandardAbsoluteActivityInSolutionValue") (declared-name "StandardAbsoluteActivityInSolutionValue") (range (start (line 693) (character 4)) (end (line 693) (character 886))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 693) (character 4)) (end (line 693) (character 886))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind "attribute def") (name "StandardAbsoluteActivityOfSolventValue") (declared-name "StandardAbsoluteActivityOfSolventValue") (range (start (line 748) (character 4)) (end (line 748) (character 693))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue::_documentation"))) (kind "documentation") (name "") (range (start (line 748) (character 4)) (end (line 748) (character 693))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind "attribute def") (name "StandardChemicalPotentialUnit") (declared-name "StandardChemicalPotentialUnit") (range (start (line 614) (character 4)) (end (line 614) (character 619))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 618) (character 8)) (end (line 618) (character 114))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 617) (character 8)) (end (line 617) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 615) (character 8)) (end (line 615) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 616) (character 8)) (end (line 616) (character 100))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 619) (character 8)) (end (line 619) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 619) (character 22)) (end (line 619) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind "attribute def") (name "StandardChemicalPotentialValue") (declared-name "StandardChemicalPotentialValue") (range (start (line 595) (character 4)) (end (line 595) (character 989))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::_documentation"))) (kind "documentation") (name "") (range (start (line 595) (character 4)) (end (line 595) (character 989))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 609) (character 8)) (end (line 609) (character 61))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "StandardChemicalPotentialUnit") (range none)) (redefinition (reference "mRef") (range (start (line 609) (character 22)) (end (line 609) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 608) (character 8)) (end (line 608) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 608) (character 22)) (end (line 608) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind "attribute def") (name "StandardEquilibriumConstantValue") (declared-name "StandardEquilibriumConstantValue") (range (start (line 853) (character 4)) (end (line 853) (character 951))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue::_documentation"))) (kind "documentation") (name "") (range (start (line 853) (character 4)) (end (line 853) (character 951))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind "attribute def") (name "StoichiometricNumberOfSubstanceValue") (declared-name "StoichiometricNumberOfSubstanceValue") (range (start (line 792) (character 4)) (end (line 792) (character 845))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 792) (character 4)) (end (line 792) (character 845))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind "attribute def") (name "ThermalDiffusionCoefficientUnit") (declared-name "ThermalDiffusionCoefficientUnit") (range (start (line 1158) (character 4)) (end (line 1158) (character 376))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1160) (character 8)) (end (line 1160) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1159) (character 8)) (end (line 1159) (character 102))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1161) (character 8)) (end (line 1161) (character 94))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1161) (character 22)) (end (line 1161) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind "attribute def") (name "ThermalDiffusionCoefficientValue") (declared-name "ThermalDiffusionCoefficientValue") (range (start (line 1139) (character 4)) (end (line 1139) (character 661))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1139) (character 4)) (end (line 1139) (character 661))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1153) (character 8)) (end (line 1153) (character 63))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalDiffusionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1153) (character 22)) (end (line 1153) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1152) (character 8)) (end (line 1152) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1152) (character 22)) (end (line 1152) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind "attribute def") (name "ThermalDiffusionFactorValue") (declared-name "ThermalDiffusionFactorValue") (range (start (line 1122) (character 4)) (end (line 1122) (character 651))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1122) (character 4)) (end (line 1122) (character 651))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind "attribute def") (name "ThermalDiffusionRatioValue") (declared-name "ThermalDiffusionRatioValue") (range (start (line 1105) (character 4)) (end (line 1105) (character 796))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1105) (character 4)) (end (line 1105) (character 796))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind "attribute def") (name "TransportNumberOfTheIonBValue") (declared-name "TransportNumberOfTheIonBValue") (range (start (line 1266) (character 4)) (end (line 1266) (character 655))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1266) (character 4)) (end (line 1266) (character 655))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind "attribute def") (name "VolumeFractionUnit") (declared-name "VolumeFractionUnit") (range (start (line 448) (character 4)) (end (line 448) (character 64))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind "attribute def") (name "VolumeFractionValue") (declared-name "VolumeFractionValue") (range (start (line 429) (character 4)) (end (line 429) (character 1052))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 429) (character 4)) (end (line 429) (character 1052))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 443) (character 8)) (end (line 443) (character 50))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeFractionUnit") (range none)) (redefinition (reference "mRef") (range (start (line 443) (character 22)) (end (line 443) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 442) (character 8)) (end (line 442) (character 32))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 442) (character 22)) (end (line 442) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 71442))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind "attribute def") (name "absoluteActivity") (declared-name "absoluteActivity") (range (start (line 538) (character 4)) (end (line 538) (character 74))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsoluteActivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind "attribute def") (name "activityCoefficient") (declared-name "activityCoefficient") (range (start (line 690) (character 4)) (end (line 690) (character 80))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind "attribute def") (name "activityFactor") (declared-name "activityFactor") (range (start (line 637) (character 4)) (end (line 637) (character 70))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind "attribute def") (name "activityOfSolute") (declared-name "activityOfSolute") (range (start (line 671) (character 4)) (end (line 671) (character 74))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityOfSoluteValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind "attribute def") (name "activityOfSolvent") (declared-name "activityOfSolvent") (range (start (line 724) (character 4)) (end (line 724) (character 76))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityOfSolventValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind "attribute def") (name "affinityOfAChemicalReaction") (declared-name "affinityOfAChemicalReaction") (range (start (line 826) (character 4)) (end (line 826) (character 109))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AffinityOfAChemicalReactionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind "attribute def") (name "amountOfSubstanceConcentration") (declared-name "amountOfSubstanceConcentration") (range (start (line 387) (character 4)) (end (line 387) (character 115))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind "attribute def") (name "amountOfSubstanceFractionMoleFraction") (declared-name "amountOfSubstanceFractionMoleFraction") (range (start (line 426) (character 4)) (end (line 426) (character 116))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceFractionMoleFractionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind "attribute def") (name "angleOfOpticalRotation") (declared-name "angleOfOpticalRotation") (range (start (line 1285) (character 4)) (end (line 1285) (character 603))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation::_documentation"))) (kind "documentation") (name "") (range (start (line 1285) (character 4)) (end (line 1285) (character 603))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind "attribute def") (name "canonicalPartitionFunction") (declared-name "canonicalPartitionFunction") (range (start (line 955) (character 4)) (end (line 955) (character 94))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CanonicalPartitionFunctionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind "attribute def") (name "chemicalPotential") (declared-name "chemicalPotential") (range (start (line 513) (character 4)) (end (line 513) (character 89))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ChemicalPotentialValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::currentFractionOfTheIonB"))) (kind "alias") (name "currentFractionOfTheIonB") (declared-name "currentFractionOfTheIonB") (range (start (line 1282) (character 4)) (end (line 1282) (character 64))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind "attribute def") (name "degeneracy") (declared-name "degeneracy") (range (start (line 1026) (character 4)) (end (line 1026) (character 62))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DegeneracyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind "attribute def") (name "degreeOfDissociation") (declared-name "degreeOfDissociation") (range (start (line 1205) (character 4)) (end (line 1205) (character 82))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DegreeOfDissociationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind "attribute def") (name "diffusionCoefficient") (declared-name "diffusionCoefficient") (range (start (line 1096) (character 4)) (end (line 1096) (character 95))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "DiffusionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::dissociationFraction"))) (kind "alias") (name "dissociationFraction") (declared-name "dissociationFraction") (range (start (line 1207) (character 4)) (end (line 1207) (character 56))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind "attribute def") (name "electrolyticConductivity") (declared-name "electrolyticConductivity") (range (start (line 1227) (character 4)) (end (line 1227) (character 103))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectrolyticConductivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::enthalpyOfPhaseTransition"))) (kind "alias") (name "enthalpyOfPhaseTransition") (declared-name "enthalpyOfPhaseTransition") (range (start (line 493) (character 4)) (end (line 493) (character 68))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind "attribute def") (name "equilibriumConstantOnConcentrationBasis") (declared-name "equilibriumConstantOnConcentrationBasis") (range (start (line 916) (character 4)) (end (line 916) (character 133))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "EquilibriumConstantOnConcentrationBasisValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind "attribute def") (name "equilibriumConstantOnPressureBasis") (declared-name "equilibriumConstantOnPressureBasis") (range (start (line 889) (character 4)) (end (line 889) (character 123))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "EquilibriumConstantOnPressureBasisValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind "attribute def") (name "extentOfReaction") (declared-name "extentOfReaction") (range (start (line 837) (character 4)) (end (line 837) (character 705))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction::_documentation"))) (kind "documentation") (name "") (range (start (line 837) (character 4)) (end (line 837) (character 705))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind "attribute def") (name "fugacity") (declared-name "fugacity") (range (start (line 585) (character 4)) (end (line 585) (character 71))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "FugacityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind "attribute def") (name "grandCanonicalPartitionFunction") (declared-name "grandCanonicalPartitionFunction") (range (start (line 972) (character 4)) (end (line 972) (character 104))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "GrandCanonicalPartitionFunctionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::grandPartitionFunction"))) (kind "alias") (name "grandPartitionFunction") (declared-name "grandPartitionFunction") (range (start (line 974) (character 4)) (end (line 974) (character 69))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind "attribute def") (name "ionicStrength") (declared-name "ionicStrength") (range (start (line 1182) (character 4)) (end (line 1182) (character 81))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "IonicStrengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind "attribute def") (name "latentHeatOfPhaseTransition") (declared-name "latentHeatOfPhaseTransition") (range (start (line 478) (character 4)) (end (line 478) (character 939))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition::_documentation"))) (kind "documentation") (name "") (range (start (line 478) (character 4)) (end (line 478) (character 939))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind "attribute def") (name "massConcentration") (declared-name "massConcentration") (range (start (line 344) (character 4)) (end (line 344) (character 89))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassConcentrationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind "attribute def") (name "massFraction") (declared-name "massFraction") (range (start (line 367) (character 4)) (end (line 367) (character 66))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFractionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind "attribute def") (name "meanFreePath") (declared-name "meanFreePath") (range (start (line 1063) (character 4)) (end (line 1063) (character 525))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath::_documentation"))) (kind "documentation") (name "") (range (start (line 1063) (character 4)) (end (line 1063) (character 525))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind "attribute def") (name "microcanonicalPartitionFunction") (declared-name "microcanonicalPartitionFunction") (range (start (line 925) (character 4)) (end (line 925) (character 695))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction::_documentation"))) (kind "documentation") (name "") (range (start (line 925) (character 4)) (end (line 925) (character 695))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (kind "attribute def") (name "molality") (declared-name "molality") (range (start (line 469) (character 4)) (end (line 469) (character 71))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolalityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind "attribute def") (name "molarConductivity") (declared-name "molarConductivity") (range (start (line 1255) (character 4)) (end (line 1255) (character 89))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarConductivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind "attribute def") (name "molarEnthalpy") (declared-name "molarEnthalpy") (range (start (line 161) (character 4)) (end (line 161) (character 81))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarEnthalpyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind "attribute def") (name "molarEntropy") (declared-name "molarEntropy") (range (start (line 274) (character 4)) (end (line 274) (character 79))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarEntropyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind "attribute def") (name "molarGasConstant") (declared-name "molarGasConstant") (range (start (line 1048) (character 4)) (end (line 1048) (character 87))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarGasConstantValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind "attribute def") (name "molarGibbsEnergy") (declared-name "molarGibbsEnergy") (range (start (line 217) (character 4)) (end (line 217) (character 87))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarGibbsEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind "attribute def") (name "molarHeatCapacity") (declared-name "molarHeatCapacity") (range (start (line 245) (character 4)) (end (line 245) (character 89))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHeatCapacityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind "attribute def") (name "molarHelmholtzEnergy") (declared-name "molarHelmholtzEnergy") (range (start (line 189) (character 4)) (end (line 189) (character 95))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHelmholtzEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind "attribute def") (name "molarInternalEnergy") (declared-name "molarInternalEnergy") (range (start (line 133) (character 4)) (end (line 133) (character 93))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarInternalEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind "attribute def") (name "molarMass") (declared-name "molarMass") (range (start (line 81) (character 4)) (end (line 81) (character 73))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarMassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind "attribute def") (name "molarOpticalRotatoryPower") (declared-name "molarOpticalRotatoryPower") (range (start (line 1318) (character 4)) (end (line 1318) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarOpticalRotatoryPowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind "attribute def") (name "molarVolume") (declared-name "molarVolume") (range (start (line 107) (character 4)) (end (line 107) (character 77))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind "attribute def") (name "molecularConcentration") (declared-name "molecularConcentration") (range (start (line 311) (character 4)) (end (line 311) (character 627))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ParticleConcentrationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration::_documentation"))) (kind "documentation") (name "") (range (start (line 311) (character 4)) (end (line 311) (character 627))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind "attribute def") (name "molecularPartitionFunction") (declared-name "molecularPartitionFunction") (range (start (line 991) (character 4)) (end (line 991) (character 94))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolecularPartitionFunctionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::multiplicity"))) (kind "alias") (name "multiplicity") (declared-name "multiplicity") (range (start (line 1028) (character 4)) (end (line 1028) (character 38))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind "attribute def") (name "numberOfEntities") (declared-name "numberOfEntities") (range (start (line 24) (character 4)) (end (line 24) (character 786))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities::_documentation"))) (kind "documentation") (name "") (range (start (line 24) (character 4)) (end (line 24) (character 786))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfMoles"))) (kind "alias") (name "numberOfMoles") (declared-name "numberOfMoles") (range (start (line 44) (character 4)) (end (line 44) (character 46))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticCoefficientOfSolventA"))) (kind "alias") (name "osmoticCoefficientOfSolventA") (declared-name "osmoticCoefficientOfSolventA") (range (start (line 745) (character 4)) (end (line 745) (character 66))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind "attribute def") (name "osmoticFactorOfSolvent") (declared-name "osmoticFactorOfSolvent") (range (start (line 743) (character 4)) (end (line 743) (character 86))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "OsmoticFactorOfSolventValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind "attribute def") (name "osmoticPressure") (declared-name "osmoticPressure") (range (start (line 782) (character 4)) (end (line 782) (character 85))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "OsmoticPressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind "attribute def") (name "partialPressure") (declared-name "partialPressure") (range (start (line 558) (character 4)) (end (line 558) (character 85))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "PartialPressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind "attribute def") (name "particleConcentration") (declared-name "particleConcentration") (range (start (line 303) (character 4)) (end (line 303) (character 97))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ParticleConcentrationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::partitionFunctionOfAMolecule"))) (kind "alias") (name "partitionFunctionOfAMolecule") (declared-name "partitionFunctionOfAMolecule") (range (start (line 993) (character 4)) (end (line 993) (character 70))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolute"))) (kind "alias") (name "relativeActivityOfSolute") (declared-name "relativeActivityOfSolute") (range (start (line 673) (character 4)) (end (line 673) (character 56))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeActivityOfSolvent"))) (kind "alias") (name "relativeActivityOfSolvent") (declared-name "relativeActivityOfSolvent") (range (start (line 726) (character 4)) (end (line 726) (character 58))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind "attribute def") (name "relativeAtomicMass") (declared-name "relativeAtomicMass") (range (start (line 61) (character 4)) (end (line 61) (character 78))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeAtomicMassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind "attribute def") (name "specificOpticalRotatoryPower") (declared-name "specificOpticalRotatoryPower") (range (start (line 1344) (character 4)) (end (line 1344) (character 111))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificOpticalRotatoryPowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind "attribute def") (name "standardAbsoluteActivityInMixture") (declared-name "standardAbsoluteActivityInMixture") (range (start (line 654) (character 4)) (end (line 654) (character 108))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardAbsoluteActivityInMixtureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind "attribute def") (name "standardAbsoluteActivityInSolution") (declared-name "standardAbsoluteActivityInSolution") (range (start (line 707) (character 4)) (end (line 707) (character 110))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardAbsoluteActivityInSolutionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind "attribute def") (name "standardAbsoluteActivityOfSolvent") (declared-name "standardAbsoluteActivityOfSolvent") (range (start (line 762) (character 4)) (end (line 762) (character 108))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardAbsoluteActivityOfSolventValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind "attribute def") (name "standardAmountOfSubstanceConcentration") (declared-name "standardAmountOfSubstanceConcentration") (range (start (line 396) (character 4)) (end (line 396) (character 686))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration::_documentation"))) (kind "documentation") (name "") (range (start (line 396) (character 4)) (end (line 396) (character 686))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind "attribute def") (name "standardChemicalPotential") (declared-name "standardChemicalPotential") (range (start (line 612) (character 4)) (end (line 612) (character 105))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardChemicalPotentialValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind "attribute def") (name "standardEquilibriumConstant") (declared-name "standardEquilibriumConstant") (range (start (line 867) (character 4)) (end (line 867) (character 96))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StandardEquilibriumConstantValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind "attribute def") (name "statisticalWeightOfSubsystem") (declared-name "statisticalWeightOfSubsystem") (range (start (line 996) (character 4)) (end (line 996) (character 501))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem::_documentation"))) (kind "documentation") (name "") (range (start (line 996) (character 4)) (end (line 996) (character 501))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind "attribute def") (name "stoichiometricNumberOfSubstance") (declared-name "stoichiometricNumberOfSubstance") (range (start (line 806) (character 4)) (end (line 806) (character 104))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "StoichiometricNumberOfSubstanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind "attribute def") (name "thermalDiffusionCoefficient") (declared-name "thermalDiffusionCoefficient") (range (start (line 1156) (character 4)) (end (line 1156) (character 109))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind "attribute def") (name "thermalDiffusionFactor") (declared-name "thermalDiffusionFactor") (range (start (line 1136) (character 4)) (end (line 1136) (character 86))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusionFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind "attribute def") (name "thermalDiffusionRatio") (declared-name "thermalDiffusionRatio") (range (start (line 1119) (character 4)) (end (line 1119) (character 84))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusionRatioValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::thermodynamicEquilibriumConstant"))) (kind "alias") (name "thermodynamicEquilibriumConstant") (declared-name "thermodynamicEquilibriumConstant") (range (start (line 869) (character 4)) (end (line 869) (character 75))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind "attribute def") (name "transportNumberOfTheIonB") (declared-name "transportNumberOfTheIonB") (range (start (line 1280) (character 4)) (end (line 1280) (character 90))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransportNumberOfTheIonBValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind "attribute def") (name "volumeFraction") (declared-name "volumeFraction") (range (start (line 446) (character 4)) (end (line 446) (character 83))) (parent (node (document "d0") (qualified-name "ISQChemistryMolecular"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFractionValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 833) (character 22)) (end (line 833) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AffinityOfAChemicalReactionUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 823) (character 22)) (end (line 823) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 822) (character 22)) (end (line 822) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 392) (character 22)) (end (line 392) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 384) (character 22)) (end (line 384) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 383) (character 22)) (end (line 383) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::AngularMeasureValue") (range (start (line 20) (character 19)) (end (line 20) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 520) (character 22)) (end (line 520) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ChemicalPotentialUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 510) (character 22)) (end (line 510) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 509) (character 22)) (end (line 509) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1101) (character 22)) (end (line 1101) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffusionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1093) (character 22)) (end (line 1093) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1092) (character 22)) (end (line 1092) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1234) (character 22)) (end (line 1234) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectrolyticConductivityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1224) (character 22)) (end (line 1224) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1223) (character 22)) (end (line 1223) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQThermodynamics::EnergyValue") (range (start (line 21) (character 19)) (end (line 21) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 921) (character 22)) (end (line 921) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnConcentrationBasisUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 913) (character 22)) (end (line 913) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 912) (character 22)) (end (line 912) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 895) (character 22)) (end (line 895) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnPressureBasisUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 886) (character 22)) (end (line 886) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 885) (character 22)) (end (line 885) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 591) (character 22)) (end (line 591) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "FugacityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 582) (character 22)) (end (line 582) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 581) (character 22)) (end (line 581) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1187) (character 22)) (end (line 1187) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IonicStrengthUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1179) (character 22)) (end (line 1179) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1178) (character 22)) (end (line 1178) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 349) (character 22)) (end (line 349) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 341) (character 22)) (end (line 341) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 340) (character 22)) (end (line 340) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 474) (character 22)) (end (line 474) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolalityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 466) (character 22)) (end (line 466) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 465) (character 22)) (end (line 465) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1262) (character 22)) (end (line 1262) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1252) (character 22)) (end (line 1252) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1251) (character 22)) (end (line 1251) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 168) (character 22)) (end (line 168) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEnthalpyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 158) (character 22)) (end (line 158) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 157) (character 22)) (end (line 157) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 282) (character 22)) (end (line 282) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEntropyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 271) (character 22)) (end (line 271) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 270) (character 22)) (end (line 270) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1056) (character 22)) (end (line 1056) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGasConstantUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1045) (character 22)) (end (line 1045) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1044) (character 22)) (end (line 1044) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 224) (character 22)) (end (line 224) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGibbsEnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 214) (character 22)) (end (line 214) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 213) (character 22)) (end (line 213) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 253) (character 22)) (end (line 253) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 242) (character 22)) (end (line 242) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 241) (character 22)) (end (line 241) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 196) (character 22)) (end (line 196) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHelmholtzEnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 186) (character 22)) (end (line 186) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 185) (character 22)) (end (line 185) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 140) (character 22)) (end (line 140) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 130) (character 22)) (end (line 130) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 129) (character 22)) (end (line 129) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 86) (character 22)) (end (line 86) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 78) (character 22)) (end (line 78) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 77) (character 22)) (end (line 77) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1323) (character 22)) (end (line 1323) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarOpticalRotatoryPowerUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1315) (character 22)) (end (line 1315) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1314) (character 22)) (end (line 1314) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 112) (character 22)) (end (line 112) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 104) (character 22)) (end (line 104) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 103) (character 22)) (end (line 103) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 788) (character 22)) (end (line 788) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "OsmoticPressureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 779) (character 22)) (end (line 779) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 778) (character 22)) (end (line 778) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 564) (character 22)) (end (line 564) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PartialPressureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 555) (character 22)) (end (line 555) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 554) (character 22)) (end (line 554) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 307) (character 22)) (end (line 307) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 300) (character 22)) (end (line 300) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 299) (character 22)) (end (line 299) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1349) (character 22)) (end (line 1349) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificOpticalRotatoryPowerUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1341) (character 22)) (end (line 1341) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1340) (character 22)) (end (line 1340) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 619) (character 22)) (end (line 619) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardChemicalPotentialUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 609) (character 22)) (end (line 609) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 608) (character 22)) (end (line 608) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1161) (character 22)) (end (line 1161) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1153) (character 22)) (end (line 1153) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1152) (character 22)) (end (line 1152) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFractionUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 443) (character 22)) (end (line 443) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 442) (character 22)) (end (line 442) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::absoluteActivity"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsoluteActivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AbsoluteActivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolute"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityOfSoluteValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSoluteValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::activityOfSolvent"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityOfSolventValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ActivityOfSolventValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::affinityOfAChemicalReaction"))) (kind featureTyping) (ordinal 0)) (authored-target "AffinityOfAChemicalReactionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AffinityOfAChemicalReactionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::amountOfSubstanceFractionMoleFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceFractionMoleFractionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceFractionMoleFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::angleOfOpticalRotation"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::canonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "CanonicalPartitionFunctionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::CanonicalPartitionFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::chemicalPotential"))) (kind featureTyping) (ordinal 0)) (authored-target "ChemicalPotentialValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ChemicalPotentialValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degeneracy"))) (kind featureTyping) (ordinal 0)) (authored-target "DegeneracyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DegeneracyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::degreeOfDissociation"))) (kind featureTyping) (ordinal 0)) (authored-target "DegreeOfDissociationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DegreeOfDissociationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::diffusionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffusionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::DiffusionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::electrolyticConductivity"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectrolyticConductivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ElectrolyticConductivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnConcentrationBasis"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnConcentrationBasisValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnConcentrationBasisValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::equilibriumConstantOnPressureBasis"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnPressureBasisValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EquilibriumConstantOnPressureBasisValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::extentOfReaction"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::fugacity"))) (kind featureTyping) (ordinal 0)) (authored-target "FugacityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::FugacityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::grandCanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "GrandCanonicalPartitionFunctionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::GrandCanonicalPartitionFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::ionicStrength"))) (kind featureTyping) (ordinal 0)) (authored-target "IonicStrengthValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::IonicStrengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::latentHeatOfPhaseTransition"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::massFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MassFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::meanFreePath"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::microcanonicalPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molality"))) (kind featureTyping) (ordinal 0)) (authored-target "MolalityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolalityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarConductivity"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarConductivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEnthalpy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEnthalpyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEnthalpyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarEntropy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarEntropyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarEntropyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGasConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGasConstantValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGasConstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarGibbsEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarGibbsEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarGibbsEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHeatCapacity"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHeatCapacityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarHelmholtzEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHelmholtzEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarHelmholtzEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarInternalEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarInternalEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarOpticalRotatoryPowerValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarOpticalRotatoryPowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molarVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolarVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::molecularPartitionFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "MolecularPartitionFunctionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::MolecularPartitionFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::numberOfEntities"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticFactorOfSolvent"))) (kind featureTyping) (ordinal 0)) (authored-target "OsmoticFactorOfSolventValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticFactorOfSolventValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::osmoticPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "OsmoticPressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::OsmoticPressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::partialPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "PartialPressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::PartialPressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::particleConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ParticleConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::relativeAtomicMass"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeAtomicMassValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::RelativeAtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::specificOpticalRotatoryPower"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificOpticalRotatoryPowerValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::SpecificOpticalRotatoryPowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInMixture"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardAbsoluteActivityInMixtureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInMixtureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityInSolution"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardAbsoluteActivityInSolutionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityInSolutionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAbsoluteActivityOfSolvent"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardAbsoluteActivityOfSolventValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardAbsoluteActivityOfSolventValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardAmountOfSubstanceConcentration"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::AmountOfSubstanceConcentrationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardChemicalPotential"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardChemicalPotentialValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardChemicalPotentialValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::standardEquilibriumConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "StandardEquilibriumConstantValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StandardEquilibriumConstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::statisticalWeightOfSubsystem"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::stoichiometricNumberOfSubstance"))) (kind featureTyping) (ordinal 0)) (authored-target "StoichiometricNumberOfSubstanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::StoichiometricNumberOfSubstanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::thermalDiffusionRatio"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusionRatioValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::ThermalDiffusionRatioValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::transportNumberOfTheIonB"))) (kind featureTyping) (ordinal 0)) (authored-target "TransportNumberOfTheIonBValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::TransportNumberOfTheIonBValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQChemistryMolecular::volumeFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFractionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQChemistryMolecular::VolumeFractionValue")))))
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
