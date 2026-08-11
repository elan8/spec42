# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQAtomicNuclear
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQAtomicNuclear {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-10:2019 "Atomic and nuclear physics"
     * see also https://www.iso.org/standard/64980.html
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
    private import ISQChemistryMolecular::DiffusionCoefficientUnit;
    private import ISQChemistryMolecular::DiffusionCoefficientValue;
    private import ISQChemistryMolecular::diffusionCoefficient;    
    private import ISQElectromagnetism::ElectricChargeValue;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AreaValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-10 item 10-1.1 atomic number, proton number */
    attribute atomicNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.1 atomic number, proton number
         * symbol(s): `Z`
         * application domain: generic
         * name: AtomicNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of protons in an atomic nucleus
         * remarks: A nuclide is a species of atom with specified numbers of protons and neutrons. Nuclides with the same value of `Z` but different values of `N` are called isotopes of an element. The ordinal number of an element in the periodic table is equal to the atomic number. The atomic number equals the quotient of the charge (IEC 80000-6) of the nucleus and the elementary charge (ISO 80000-1).
         */
    }

    alias protonNumber for atomicNumber;

    /* ISO-80000-10 item 10-1.2 neutron number */
    attribute neutronNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.2 neutron number
         * symbol(s): `N`
         * application domain: generic
         * name: NeutronNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of neutrons in an atomic nucleus
         * remarks: Nuclides with the same value of `N` but different values of `Z` are called isotones. `N - Z` is called the neutron excess number.
         */
    }

    /* ISO-80000-10 item 10-1.3 nucleon number, mass number */
    attribute nucleonNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.3 nucleon number, mass number
         * symbol(s): `A`
         * application domain: generic
         * name: NucleonNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of nucleons in an atomic nucleus
         * remarks: `A` = `Z` + `N` Nuclides with the same value of `A` are called isobars.
         */
    }

    alias massNumber for nucleonNumber;

    /* ISO-80000-10 item 10-2 rest mass, proper mass */
    attribute restMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-2 rest mass, proper mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: RestMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: for particle X, mass (ISO 80000-4) of that particle at rest in an inertial frame
         * remarks: EXAMPLE `m(H_2O)` for a water molecule, `m_e` for an electron. Rest mass is often denoted `m_0`. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    alias properMass for restMass;

    /* ISO-80000-10 item 10-3 rest energy */
    attribute restEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-3 rest energy
         * symbol(s): `E_0`
         * application domain: generic
         * name: RestEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy `E_0` (ISO 80000-5) of a particle at rest: `E_0 = m_0 c_0^2` where `m_0` is the rest mass (item 10-2) of that particle, and `c_0` is speed of light in vacuum (ISO 80000-1)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-4.1 atomic mass */
    attribute atomicMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.1 atomic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: AtomicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of an atom X in the ground state
         * remarks: `m(X)/m_u` is called the relative atomic mass. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-4.2 nuclidic mass */
    attribute nuclidicMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.2 nuclidic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: NuclidicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of a nuclide X in the ground state
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-4.3 unified atomic mass constant */
    attribute unifiedAtomicMassConstant: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.3 unified atomic mass constant
         * symbol(s): `m_u`
         * application domain: generic
         * name: UnifiedAtomicMassConstant (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: 1/12 of the mass (ISO 80000-4) of an atom of the nuclide ^(12)C in the ground state at rest
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-5.1 elementary charge */
    attribute elementaryCharge: ElectricChargeValue :> scalarQuantities {
        doc
        /*
         * source: item 10-5.1 elementary charge
         * symbol(s): `e`
         * application domain: generic
         * name: ElementaryCharge (specializes ElectricCharge)
         * quantity dimension: T^1*I^1
         * measurement unit(s): C, s*A
         * tensor order: 0
         * definition: one of the fundamental constants in the SI system (ISO 80000-1), equal to the charge of the proton and opposite to the charge of the electron
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-5.2 charge number, ionization number */
    attribute def ChargeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-5.2 charge number, ionization number
         * symbol(s): `c`
         * application domain: generic
         * name: ChargeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a particle, quotient of the electric charge (IEC 80000-6) and the elementary charge (ISO 80000-1)
         * remarks: A particle is said to be electrically neutral if its charge number is equal to zero. The charge number of a particle can be positive, negative, or zero. The state of charge of a particle may be presented as a superscript to the symbol of that particle, e.g. `H^+, He^(++), Al^(3+), Cl^-, S^(--), N^(3-)`.
         */
    }
    attribute chargeNumber: ChargeNumberValue :> scalarQuantities;

    alias ionizationNumber for chargeNumber;

    /* ISO-80000-10 item 10-6 Bohr radius */
    attribute bohrRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-6 Bohr radius
         * symbol(s): `a_0`
         * application domain: generic
         * name: BohrRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m, Å
         * tensor order: 0
         * definition: radius (ISO 80000-3) of the electron orbital in the hydrogen atom in its ground state in the Bohr model of the atom: `a_0 = (4 π ε_0 ℏ^2)/(m_e e^2)` where `ε_0` is the electric constant (IEC 80000-6), `ℏ` is the reduced Planck constant (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `e` is the elementary charge (ISO 80000-1)
         * remarks: The radius of the electron orbital in the H atom in its ground state is `a_0` in the Bohr model of the atom. ångström (Å), `1 Å := 10^-10 m`.
         */
    }

    /* ISO-80000-10 item 10-7 Rydberg constant */
    attribute def RydbergConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-7 Rydberg constant
         * symbol(s): `R_∞`
         * application domain: generic
         * name: RydbergConstant
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: spectroscopic constant that determines the wave numbers of the lines in the spectrum of hydrogen: `R_(oo) = e^2/(8 π ε_0 a_0 h c_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `a_0` is the Bohr radius (item 10-6), `h` is the Planck constant (ISO 80000-1), and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The quantity `R_y = R_∞ h c_0` is called the Rydberg energy.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RydbergConstantUnit[1];
    }

    attribute rydbergConstant: RydbergConstantValue[*] nonunique :> scalarQuantities;

    attribute def RydbergConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-8 Hartree energy */
    attribute def HartreeEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-8 Hartree energy
         * symbol(s): `E_H`, `E_h`
         * application domain: generic
         * name: HartreeEnergy
         * quantity dimension: L^6*M^3*T^-6
         * measurement unit(s): eV*J*kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) of the electron in a hydrogen atom in its ground state: `E_H = e^2/(4 π ε_0 a_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), and `a_0` is the Bohr radius (item 10-6)
         * remarks: The energy of the electron in an H atom in its ground state is `E_H`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HartreeEnergyUnit[1];
    }

    attribute hartreeEnergy: HartreeEnergyValue[*] nonunique :> scalarQuantities;

    attribute def HartreeEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 6; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -6; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-9.1 magnetic dipole moment */
    attribute def MagneticDipoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (magnitude)
         * symbol(s): `μ`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticDipoleMomentUnit[1];
    }

    attribute magneticDipoleMoment: MagneticDipoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def MagneticDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (vector)
         * symbol(s): `vec(μ)`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 1
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticDipoleMoment3dCoordinateFrame[1];
    }

    attribute cartesianMagneticDipoleMoment3dVector: CartesianMagneticDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticDipoleMomentUnit[3];
    }

    /* ISO-80000-10 item 10-9.2 Bohr magneton */
    attribute bohrMagneton: MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.2 Bohr magneton
         * symbol(s): `μ_B`
         * application domain: generic
         * name: BohrMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: magnitude of the magnetic moment of an electron in a state with orbital angular momentum quantum number `l`=1 (item 10-13.3) due to its orbital motion: `μ_B = (e ℏ)/(2 m_e)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_e` is the rest mass (item 10-2) of electron
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-9.3 nuclear magneton */
    attribute nuclearMagneton: MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.3 nuclear magneton
         * symbol(s): `μ_N`
         * application domain: generic
         * name: NuclearMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: absolute value of the magnetic moment of a nucleus: `μ_N = (e ℏ)/(2 m_p)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_p` is the rest mass (item 10-2) of proton
         * remarks: Subscript N stands for nucleus. For the neutron magnetic moment, subscript n is used. The magnetic moments of protons and neutrons differ from this quantity by their specific `g` factors (item 10-14.2).
         */
    }

    /* ISO-80000-10 item 10-10 spin */
    attribute def SpinValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-10 spin (magnitude)
         * symbol(s): `s`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpinUnit[1];
    }

    attribute spin: SpinValue[*] nonunique :> scalarQuantities;

    attribute def SpinUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianSpin3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-10 spin (vector)
         * symbol(s): `vec(s)`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpin3dCoordinateFrame[1];
    }

    attribute cartesianSpin3dVector: CartesianSpin3dVector :> vectorQuantities;

    attribute def CartesianSpin3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SpinUnit[3];
    }

    /* ISO-80000-10 item 10-11 total angular momentum */
    attribute def TotalAngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-11 total angular momentum (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalAngularMomentumUnit[1];
    }

    attribute totalAngularMomentum: TotalAngularMomentumValue[*] nonunique :> scalarQuantities;

    attribute def TotalAngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianTotalAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-11 total angular momentum (vector)
         * symbol(s): `vec(J)`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianTotalAngularMomentum3dCoordinateFrame[1];
    }

    attribute cartesianTotalAngularMomentum3dVector: CartesianTotalAngularMomentum3dVector :> vectorQuantities;

    attribute def CartesianTotalAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: TotalAngularMomentumUnit[3];
    }

    /* ISO-80000-10 item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient */
    attribute def GyromagneticRatioValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: GyromagneticRatio
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum: `vec(μ)` = `γ` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1 The systematic name is "gyromagnetic coefficient", but "gyromagnetic ratio" is more usual. The gyromagnetic ratio of the proton is denoted by `γ_p`. The gyromagnetic ratio of the neutron is denoted by `γ_n`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: GyromagneticRatioUnit[1];
    }

    attribute gyromagneticRatio: GyromagneticRatioValue[*] nonunique :> scalarQuantities;

    attribute def GyromagneticRatioUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    alias MagnetogyricRatioUnit for GyromagneticRatioUnit;
    alias MagnetogyricRatioValue for GyromagneticRatioValue;
    alias magnetogyricRatio for gyromagneticRatio;

    alias GyromagneticCoefficientUnit for GyromagneticRatioUnit;
    alias GyromagneticCoefficientValue for GyromagneticRatioValue;
    alias gyromagneticCoefficient for gyromagneticRatio;

    /* ISO-80000-10 item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron */
    attribute def GyromagneticRatioOfTheElectronValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron
         * symbol(s): `γ_e`
         * application domain: generic
         * name: GyromagneticRatioOfTheElectron
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum of the electron `vec(μ)` = `γ_e` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1
         */
        attribute :>> num: Real;
        attribute :>> mRef: GyromagneticRatioOfTheElectronUnit[1];
    }

    attribute gyromagneticRatioOfTheElectron: GyromagneticRatioOfTheElectronValue[*] nonunique :> scalarQuantities;

    attribute def GyromagneticRatioOfTheElectronUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    alias MagnetogyricRatioOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias MagnetogyricRatioOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias magnetogyricRatioOfTheElectron for gyromagneticRatioOfTheElectron;

    alias GyromagneticCoefficientOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias GyromagneticCoefficientOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias gyromagneticCoefficientOfTheElectron for gyromagneticRatioOfTheElectron;

    /* ISO-80000-10 item 10-13.1 quantum number */
    attribute def QuantumNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-13.1 quantum number
         * symbol(s): `N`, `L`, `M`, `j`, `s`, `F`
         * application domain: generic
         * name: QuantumNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number describing a particular state of a quantum system
         * remarks: Electron states determine the binding energy `E = E(n,l,m,j,s,f)` in an atom. Upper case letters `N, L, M, J, S, F` are usually used for the whole system. The spatial probability distribution of an electron is given by `|Ψ|^2`, where `Ψ` is its wave function. For an electron in an H atom in a non-relativistic approximation, the wave function can be presented as: `Ψ(r,θ,φ) = R_(nl)(r)*Y_l^m(θ,φ)`, where `r,θ,φ` are spherical coordinates (ISO 80000-2) with respect to the nucleus and to a given (quantization) axis, `R_(nl)(r)` is the radial distribution function, and `Y_l^m(θ,φ)` are spherical harmonics. In the Bohr model of one-electron atoms, `n`, `l`, and `m` define the possible orbits of an electron about the nucleus.
         */
    }
    attribute quantumNumber: QuantumNumberValue :> scalarQuantities;

    /* ISO-80000-10 item 10-13.2 principal quantum number */
    attribute principalQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.2 principal quantum number
         * symbol(s): `n`
         * application domain: generic
         * name: PrincipalQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the number `n`-1 of radial nodes of one-electron wave functions
         * remarks: In the Bohr model, `n = 1,2,…,∞` is related to the binding energy of an electron and the radius of spherical orbits (principal axis of the elliptic orbits). For an electron in an H atom, the semi-classical radius of its orbit is `r_n = a_0 n^2` and its binding energy is `E_n = E_H/n^2`.
         */
    }

    /* ISO-80000-10 item 10-13.3 orbital angular momentum quantum number */
    attribute orbitalAngularMomentumQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.3 orbital angular momentum quantum number
         * symbol(s): `l`, `l_i`, `L`
         * application domain: generic
         * name: OrbitalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the orbital angular momentum `l` of a one-electron state
         * remarks: `abs(l)^2 = ℏ^2 l (l-1)` , `l = 0, 1, …, n-1` where `vec(l)` is the orbital angular momentum and `ℏ` is the reduced Planck constant (ISO 80000-1). If reference is made to a specific particle `i`, the symbol `l_i` is used instead of `l`; if reference is made to the whole system, the symbol `L` is used instead of `l`. An electron in an H atom for `l = 0` appears as a spherical cloud. In the Bohr model, it is related to the form of the orbit.
         */
    }

    /* ISO-80000-10 item 10-13.4 magnetic quantum number */
    attribute magneticQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.4 magnetic quantum number
         * symbol(s): `m`, `m_i`, `M`
         * application domain: generic
         * name: MagneticQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the `z` component `l_z`, `j_z` or `s_z`, of the orbital, total, or spin angular momentum
         * remarks: `l_z = m_l ℏ` , `j_z = m_j ℏ` , and `s_z = m_s ℏ` , with the ranges from `-l` to `l`, from `-j` to `j`, and `±1/2`, respectively. `m_i` refers to a specific particle `i`. `M` is used for the whole system. Subscripts `l`, `s`, `j`, etc., as appropriate, indicate the angular momentum involved. `ℏ` is the reduced Planck constant (ISO 80000-1).
         */
    }

    /* ISO-80000-10 item 10-13.5 spin quantum number */
    attribute spinQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.5 spin quantum number
         * symbol(s): `s`
         * application domain: generic
         * name: SpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characteristic quantum number `s` of a particle, related to its spin (item 10-10), `vec(s)`: `s^2 = ℏ^2 s (s+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Spin quantum numbers of fermions are odd multiples of 1/2, and those of bosons are integers.
         */
    }

    /* ISO-80000-10 item 10-13.6 total angular momentum quantum number */
    attribute totalAngularMomentumQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.6 total angular momentum quantum number
         * symbol(s): `j`, `j_i`, `J`
         * application domain: generic
         * name: TotalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number in an atom describing the magnitude of total angular momentum `vec(J)` (item 10-11)
         * remarks: `j_i` refers to a specific particle `i`; `J` is used for the whole system. The quantum number `J` and the magnitude of total angular momentum `vec(J)` (item 10-11) are different quantities. The two values of `j` are `l`±1/2. (See item 10-13.3.)
         */
    }

    /* ISO-80000-10 item 10-13.7 nuclear spin quantum number */
    attribute nuclearSpinQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.7 nuclear spin quantum number
         * symbol(s): `I`
         * application domain: generic
         * name: NuclearSpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number related to the total angular momentum (item 10-11), `vec(J)`, of a nucleus in any specified state, normally called nuclear spin: `vec(J)^2 = ℏ^2 I (I+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Nuclear spin is composed of spins of the nucleons (protons and neutrons) and their (orbital) motions. In principle there is no upper limit for the nuclear spin quantum number. It has possible values `I` = 0,1,2,… for even `A` and `I = 1/2, 3/2, …` for odd `A`. In nuclear and particle physics, `vec(J)` is often used.
         */
    }

    /* ISO-80000-10 item 10-13.8 hyperfine structure quantum number */
    attribute hyperfineStructureQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.8 hyperfine structure quantum number
         * symbol(s): `F`
         * application domain: generic
         * name: HyperfineStructureQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number of an atom describing the inclination of the nuclear spin with respect to a quantization axis given by the magnetic field produced by the orbital electrons
         * remarks: The interval of `F` is │`I`-`J`│, │`I`-`J`│+1, ..., `I`-`J`. This is related to the hyperfine splitting of the atomic energy levels due to the interaction between the electron and nuclear magnetic moments.
         */
    }

    /* ISO-80000-10 item 10-14.1 Lande factor, g factor of atom */
    attribute def LandeFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.1 Lande factor, g factor of atom
         * symbol(s): `g`
         * application domain: generic
         * name: LandeFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the total angular momentum quantum number and the Bohr magneton: `g = μ/(J*μ_B)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `J` is total angular momentum quantum number (item 10-13.6), and `μ_B` is the Bohr magneton (item 10-9.2)
         * remarks: These quantities are also called `g` values. The Landé factor can be calculated from the expression: `g(L, S, J) = 1 + (g_e -1) xx (J(J+1) + S(S+1) - L(L+1))/(2J(J+1))` where `g_e` is the` g` factor of the electron.
         */
    }
    attribute landeFactor: LandeFactorValue :> scalarQuantities;

    alias gFactorOfAtom for landeFactor;

    /* ISO-80000-10 item 10-14.2 g factor of nucleus or nuclear particle */
    attribute def GFactorOfNucleusOrNuclearParticleValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.2 g factor of nucleus or nuclear particle
         * symbol(s): `g`
         * application domain: generic
         * name: GFactorOfNucleusOrNuclearParticle (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the nuclear spin quantum number and the nuclear magneton: `g = μ/(I*μ_N)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `I` is nuclear spin quantum number (item 10-13.7), and `μ_N` is the nuclear magneton (item 10-9.3)
         * remarks: The `g` factors for nuclei or nucleons are known from measurements.
         */
    }
    attribute gFactorOfNucleusOrNuclearParticle: GFactorOfNucleusOrNuclearParticleValue :> scalarQuantities;

    /* ISO-80000-10 item 10-15.1 Larmor angular frequency */
    attribute larmorAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.1 Larmor angular frequency
         * symbol(s): `ω_L`
         * application domain: generic
         * name: LarmorAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: angular frequency (ISO 80000-3) of the electron angular momentum (ISO 80000-4) vector precession about the axis of an external magnetic field: `ω_L = e/(2 m_e) B` where `e` is the elementary charge (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-15.2 Larmor frequency */
    attribute def LarmorFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-15.2 Larmor frequency
         * symbol(s): `ν_L`
         * application domain: generic
         * name: LarmorFrequency
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of Larmor angular frequency (ISO 80000-3) and 2π
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LarmorFrequencyUnit[1];
    }

    attribute larmorFrequency: LarmorFrequencyValue[*] nonunique :> scalarQuantities;

    attribute def LarmorFrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-15.3 nuclear precession angular frequency */
    attribute nuclearPrecessionAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.3 nuclear precession angular frequency
         * symbol(s): `ω_N`
         * application domain: generic
         * name: NuclearPrecessionAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: frequency (ISO 80000-3) by which the nucleus angular momentum vector (ISO 80000-4) precesses about the axis of an external magnetic field: `ω_N` = `γ` `B` where `γ` is the gyromagnetic ratio (item 10-12.1), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-16 cyclotron angular frequency */
    attribute cyclotronAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-16 cyclotron angular frequency
         * symbol(s): `ω_c`
         * application domain: generic
         * name: CyclotronAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: quotient of the product of the electric charge of a particle and the magnitude of the magnetic flux density of the magnetic field, and the particle mass: `ω_c = abs(q)/m B` where `q` is the electric charge (IEC 80000-6) of the particle, `m` is the mass (ISO 80000-4) of the particle, and `B` is the absolute value of the magnetic flux density (IEC 80000-6)
         * remarks: The quantity `v_c` = `ω_c`/2π is called the cyclotron frequency.
         */
    }

    /* ISO-80000-10 item 10-17 gyroradius, Larmor radius */
    attribute gyroradius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-17 gyroradius, Larmor radius
         * symbol(s): `r_g`, `r_L`
         * application domain: generic
         * name: Gyroradius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (ISO 80000-3) of circular movement of a particle with mass (ISO 80000-4), velocity `vec(v)` (ISO 80000-3), and electric charge `q` (IEC 80000-6), moving in a magnetic field with magnetic flux density `vec(B)` (IEC 80000-6): `r_g = (m abs(vec(v) xx vec(B)))/(q B^2)`
         * remarks: None.
         */
    }

    alias larmorRadius for gyroradius;

    /* ISO-80000-10 item 10-18 nuclear quadrupole moment */
    attribute def NuclearQuadrupoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-18 nuclear quadrupole moment
         * symbol(s): `Q`
         * application domain: generic
         * name: NuclearQuadrupoleMoment
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: `z` component of the diagonalized tensor of nuclear quadrupole moment: `Q = (1/e) int (3z^2 - r^2) ρ(x, y, z) dV` in the quantum state with the nuclear spin in the field direction (`z`), where `e` is the elementary charge (ISO 80000-1), `r^2 = x^2 + y^2 + z^2`, `ρ(x,y,z)` is the nuclear electric charge density (IEC 80000-6), and `dV` is the volume element `dx dy dz`
         * remarks: The electric nuclear quadrupole moment is `eQ`. This value is equal to the `z` component of the diagonalized tensor of quadrupole moment.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NuclearQuadrupoleMomentUnit[1];
    }

    attribute nuclearQuadrupoleMoment: NuclearQuadrupoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def NuclearQuadrupoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-19.1 nuclear radius */
    attribute nuclearRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.1 nuclear radius
         * symbol(s): `R`
         * application domain: generic
         * name: NuclearRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: conventional radius (ISO 80000-3) of sphere in which the nuclear matter is included
         * remarks: This quantity is not exactly defined. It is given approximately for nuclei in their ground state by: `R = r_0 A^(1//3)` where `r_0 ~~ 1.2 * 10^-15` m, and `A` is the nucleon number (item 10-1.3). Nuclear radius is usually expressed in femtometres, 1 fm = 10^(-15) m.
         */
    }

    /* ISO-80000-10 item 10-19.2 electron radius */
    attribute electronRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.2 electron radius
         * symbol(s): `r_e`
         * application domain: generic
         * name: ElectronRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius of a sphere such that the relativistic electron energy is distributed uniformly: `r_e = e^2/(4 π ε_0 m_e c_0^2)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `m_e` is the rest mass (item 10-2) of electron, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: This quantity corresponds to the electrostatic energy `E` of a charge distributed inside a sphere of radius `r_e` as if all the rest energy (item 10-3) of the electron were attributed to the energy of electromagnetic origin, using the relation `E = m_e c_0^2`.
         */
    }

    /* ISO-80000-10 item 10-20 Compton wavelength */
    attribute comptonWavelength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-20 Compton wavelength
         * symbol(s): `λ_C`
         * application domain: generic
         * name: ComptonWavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: quotient of the Planck constant and the product of the mass of the particle and the speed of light in vacuum: `λ_C = h / (m c_0)` where `h` is the Planck constant (ISO 80000-1), `m` is the rest mass (item 10-2) of a particle, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The wavelength of electromagnetic radiation scattered from free electrons (Compton scattering) is larger than that of the incident radiation by a maximum of 2`λ_C`.
         */
    }

    /* ISO-80000-10 item 10-21.1 mass excess */
    attribute massExcess: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.1 mass excess
         * symbol(s): `Δ`
         * application domain: generic
         * name: MassExcess (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: difference between the mass of an atom, and the product of its mass number and the unified mass constant: `Δ = m_a - A*m_u`, where `m_a` is the rest mass (item 10-2) of the atom, `A` is its nucleon number (item 10-1.3), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. See item 10-2.
         */
    }

    /* ISO-80000-10 item 10-21.2 mass defect */
    attribute massDefect: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.2 mass defect
         * symbol(s): `B`
         * application domain: generic
         * name: MassDefect (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: sum of the product of the proton number and the hydrogen atomic mass, and the neutron rest mass, minus the rest mass of the atom: `B = Z*m(⁢^1"H") + N*m_n - m_a` where `Z` is the proton number (item 10-1.1) of the atom, `m(⁢^1"H")` is atomic mass (item 10-4.1) of `⁢^1"H"`, `N` is neutron number (item 10-1.2), `m_n` is the rest mass (item 10-2) of the neutron, and `m_a` is the rest mass (item 10-2) of the atom
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. If the binding energy of the orbital electrons is neglected, `B c_0^2` is equal to the binding energy of the nucleus.
         */
    }

    /* ISO-80000-10 item 10-22.1 relative mass excess */
    attribute def RelativeMassExcessValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.1 relative mass excess
         * symbol(s): `Δ_r`
         * application domain: generic
         * name: RelativeMassExcess (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass excess and the unified atomic mass constant: `Δ_r = Δ/m_u` where `Δ` is mass excess (item 10-21.1), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassExcess: RelativeMassExcessValue :> scalarQuantities;

    /* ISO-80000-10 item 10-22.2 relative mass defect */
    attribute def RelativeMassDefectValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.2 relative mass defect
         * symbol(s): `B_r`
         * application domain: generic
         * name: RelativeMassDefect (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass defect and the unified atomic mass constant: `B_r = B/m_u` where `B` is mass defect (item 10-21.2), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassDefect: RelativeMassDefectValue :> scalarQuantities;

    /* ISO-80000-10 item 10-23.1 packing fraction */
    attribute def PackingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.1 packing fraction
         * symbol(s): `f`
         * application domain: generic
         * name: PackingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass excess and the nucleon number: `f` = Δ_r/A` where `Δ_r` is relative mass excess (item 10-22.1), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute packingFraction: PackingFractionValue :> scalarQuantities;

    /* ISO-80000-10 item 10-23.2 binding fraction */
    attribute def BindingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.2 binding fraction
         * symbol(s): `b`
         * application domain: generic
         * name: BindingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass defect and the nucleon number: `b = B_r/A` where `B_r` is relative mass defect (item 10-22.2), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute bindingFraction: BindingFractionValue :> scalarQuantities;

    /* ISO-80000-10 item 10-24 decay constant, disintegration constant */
    attribute def DecayConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-24 decay constant, disintegration constant
         * symbol(s): `λ`
         * application domain: generic
         * name: DecayConstant
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of `(-dN)/N` and `dt`, where `(dN)/N` is the mean fractional change in the number of nuclei in a particular energy state due to spontaneous transformations in a time interval of duration (ISO 80000-3) `dt`: `λ = -1/N (dN)/(dt)`
         * remarks: For exponential decay, this quantity is constant. For more than one decay channel, `λ = sum λ_a` where `λ_a` denotes the decay constant for a specified final state and the sum is taken over all final states.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DecayConstantUnit[1];
    }

    attribute decayConstant: DecayConstantValue[*] nonunique :> scalarQuantities;

    attribute def DecayConstantUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    alias DisintegrationConstantUnit for DecayConstantUnit;
    alias DisintegrationConstantValue for DecayConstantValue;
    alias disintegrationConstant for decayConstant;

    /* ISO-80000-10 item 10-25 mean duration of life, mean life time */
    attribute meanDurationOfLife: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-25 mean duration of life, mean life time
         * symbol(s): `τ`
         * application domain: atomic and nuclear physics
         * name: MeanDurationOfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: reciprocal of the decay constant `λ` (item 10-24): `τ = 1/λ`
         * remarks: Mean duration of life is the expected value of the duration of life of an unstable particle or an excited state of a particle when the number of decay events in a short time interval follows a Poisson distribution.
         */
    }

    alias meanLifeTime for meanDurationOfLife;

    /* ISO-80000-10 item 10-26 level width */
    attribute levelWidth: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-26 level width
         * symbol(s): `Γ`
         * application domain: generic
         * name: LevelWidth (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the reduced Planck constant and the mean life: `Γ = ℏ/τ` where `ℏ` is the reduced Planck constant (ISO 80000-1), and `τ` is mean duration of life (item 10-25)
         * remarks: Level width is the uncertainty of the energy of an unstable particle or an excited state of a system due to the Heisenberg principle. The term energy level refers to the configuration of the distribution function of the density of states. Energy levels may be considered as discrete, like those in an atom, or may have a finite width, like e.g. this item or like e.g. the valence or conduction band in solid state physics. Energy levels are applicable to both real and virtual particles, e.g. electrons and phonons, respectively.
         */
    }

    /* ISO-80000-10 item 10-27 nuclear activity */
    attribute def NuclearActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-27 nuclear activity
         * symbol(s): `A`
         * application domain: generic
         * name: NuclearActivity
         * quantity dimension: T^-1
         * measurement unit(s): Bq, s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the mean change in the number of nuclei in a particular energy state due to spontaneous nuclear transformations in a time interval of duration (ISO 80000-3) `dt`: `A = -(dN)/(dt)`
         * remarks: For exponential decay, `A = λN`, where `λ` is the decay constant (item 10-24). The becquerel (Bq) is a special name for second to the power minus one, to be used as the coherent SI unit of activity. In report 85a of the ICRU a definition with an equivalent meaning is given as: The activity, `A`, of an amount of a radionuclide in a particular energy state at a given time is the quotient of `-dN` by `dt`, where `dN` is the mean change in the number of nuclei in that energy state due to spontaneous nuclear transformations in the time interval `dt`: `A = -(dN)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NuclearActivityUnit[1];
    }

    attribute nuclearActivity: NuclearActivityValue[*] nonunique :> scalarQuantities;

    attribute def NuclearActivityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-28 specific activity, massic activity */
    attribute def SpecificActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-28 specific activity, massic activity
         * symbol(s): `a`
         * application domain: generic
         * name: SpecificActivity
         * quantity dimension: M^-1*T^-1
         * measurement unit(s): Bq/kg, kg^-1*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificActivityUnit[1];
    }

    attribute specificActivity: SpecificActivityValue[*] nonunique :> scalarQuantities;

    attribute def SpecificActivityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    alias MassicActivityUnit for SpecificActivityUnit;
    alias MassicActivityValue for SpecificActivityValue;
    alias massicActivity for specificActivity;

    /* ISO-80000-10 item 10-29 activity density, volumic activity, activity concentration */
    attribute def ActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-29 activity density, volumic activity, activity concentration
         * symbol(s): `c_A`
         * application domain: generic
         * name: ActivityDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): Bq/m^3, m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ActivityDensityUnit[1];
    }

    attribute activityDensity: ActivityDensityValue[*] nonunique :> scalarQuantities;

    attribute def ActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias VolumicActivityUnit for ActivityDensityUnit;
    alias VolumicActivityValue for ActivityDensityValue;
    alias volumicActivity for activityDensity;

    alias ActivityConcentrationUnit for ActivityDensityUnit;
    alias ActivityConcentrationValue for ActivityDensityValue;
    alias activityConcentration for activityDensity;

    /* ISO-80000-10 item 10-30 surface-activity density */
    attribute def SurfaceActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-30 surface-activity density
         * symbol(s): `a_S`
         * application domain: generic
         * name: SurfaceActivityDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): Bq/m^2, m^-2*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the total area `S` (ISO 80000-3) of the surface of that sample: `a_S` = `A`/`S`
         * remarks: This value is usually defined for flat sources, where `S` corresponds to the total area of surface of one side of the source.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceActivityDensityUnit[1];
    }

    attribute surfaceActivityDensity: SurfaceActivityDensityValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-31 half life */
    attribute halfLife: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-31 half life
         * symbol(s): `T_(1/2)`
         * application domain: generic
         * name: HalfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: mean duration (ISO 80000-3) required for the decay of one half of the atoms or nuclei
         * remarks: For exponential decay, `T_(1/2) = (ln2)/λ`, where `λ` is the decay constant (item 10-24).
         */
    }

    /* ISO-80000-10 item 10-32 alpha disintegration energy */
    attribute alphaDisintegrationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-32 alpha disintegration energy
         * symbol(s): `Q_α`
         * application domain: generic
         * name: AlphaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the kinetic energy (ISO 80000-4) of the α-particle produced in the disintegration process and the recoil energy (ISO 80000-5) of the product atom in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: The ground-state alpha disintegration energy, `Q_(α,0)`, also includes the energy of any nuclear transitions that take place in the daughter produced.
         */
    }

    /* ISO-80000-10 item 10-33 maximum beta-particle energy */
    attribute maximumBetaParticleEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-33 maximum beta-particle energy
         * symbol(s): `E_β`
         * application domain: generic
         * name: MaximumBetaParticleEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: maximum kinetic energy (ISO 80000-4) of the emitted beta particle produced in the nuclear disintegration process
         * remarks: The maximum kinetic energy corresponds to the highest energy of the beta spectrum.
         */
    }

    /* ISO-80000-10 item 10-34 beta disintegration energy */
    attribute betaDisintegrationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-34 beta disintegration energy
         * symbol(s): `Q_β`
         * application domain: generic
         * name: BetaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the maximum beta-particle kinetic energy (item 10-33) and the recoil energy (ISO 80000-5) of the atom produced in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: For positron emitters, the energy for the production of the annihilation radiation created in the combination of an electron with the positron is part of the beta disintegration energy. The ground-state beta disintegration energy, `Q_(β,0)`, also includes the energy of any nuclear transitions that take place in the daughter product.
         */
    }

    /* ISO-80000-10 item 10-35 internal conversion factor */
    attribute def InternalConversionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-35 internal conversion factor
         * symbol(s): `α`
         * application domain: generic
         * name: InternalConversionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the number of internal conversion electrons and the number of gamma quanta emitted by the radioactive atom in a given transition, where a conversion electron represents an orbital electron emitted through the radioactive decay
         * remarks: The quantity `α/(α+1)` is also used and called the internal-conversion fraction. Partial conversion fractions referring to the various electron shells `K, L, ...` are indicated by `α_K`, `α_L`, ... `α_K/α_L` is called the K-to-L internal conversion ratio.
         */
    }
    attribute internalConversionFactor: InternalConversionFactorValue :> scalarQuantities;

    /* ISO-80000-10 item 10-36 particle emission rate */
    attribute def ParticleEmissionRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-36 particle emission rate
         * symbol(s): `dot(N)`
         * application domain: generic
         * name: ParticleEmissionRate
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the number of particles being emitted from an infinitesimally small volume element in the time interval of duration `dt` (ISO 80000-3), and `dt`: `dot(N) = (dN)/(dt)`
         * remarks: Usually the kind of particles is specified, e.g. neutron emission rate or alpha particle emission rate.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleEmissionRateUnit[1];
    }

    attribute particleEmissionRate: ParticleEmissionRateValue[*] nonunique :> scalarQuantities;

    attribute def ParticleEmissionRateUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-37.1 reaction energy */
    attribute reactionEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.1 reaction energy
         * symbol(s): `Q`
         * application domain: generic
         * name: ReactionEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a nuclear reaction, sum of the kinetic energies (ISO 80000-4) and photon energies (ISO 80000-5) of the reaction products minus the sum of the kinetic and photon energies of the reactants
         * remarks: For exothermic nuclear reactions, `Q>0`. For endothermic nuclear reactions, `Q<0`.
         */
    }

    /* ISO-80000-10 item 10-37.2 resonance energy */
    attribute resonanceEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.2 resonance energy
         * symbol(s): `E_r`, `E_"res"`
         * application domain: generic
         * name: ResonanceEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: kinetic energy (ISO 80000-4) of an incident particle, in the reference frame of the target, corresponding to a resonance in a nuclear reaction
         * remarks: The energy of the resonance corresponds to the difference of the energy levels involved of the nucleus.
         */
    }

    /* ISO-80000-10 item 10-38.1 cross section */
    attribute crossSection: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.1 cross section
         * symbol(s): `σ`
         * application domain: atomic physics
         * name: CrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: for a specified target entity and for a specified reaction or process produced by incident charged or uncharged particles of a given type and energy, the quotient of the mean number of such reactions or processes and the incident-particle fluence (item 10-43)
         * remarks: The type of process is indicated by subscripts, e.g. absorption cross section `σ_a`, scattering cross section `σ_s`, fission cross section `σ_f`. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }

    /* ISO-80000-10 item 10-38.2 total cross section */
    attribute totalCrossSection: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.2 total cross section
         * symbol(s): `σ_"tot"`, `σ_"T"`
         * application domain: atomic physics
         * name: TotalCrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: sum of all cross sections (item 10-38.1) corresponding to the various reactions or processes between an incident particle of specified type and energy (ISO 80000-5) and a target entity
         * remarks: In the case of a narrow unidirectional beam of incident particles, this is the effective cross section for the removal of an incident particle from the beam. See the Remarks for item 10-52. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }

    /* ISO-80000-10 item 10-39 direction distribution of cross section */
    attribute def DirectionDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-39 direction distribution of cross section
         * symbol(s): `σ_Ω`
         * application domain: atomic physics
         * name: DirectionDistributionOfCrossSection
         * quantity dimension: L^2
         * measurement unit(s): m^2*sr^-1, m^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to `Ω`, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a specified direction, and `Ω` is the solid angle (ISO 80000-3) around that direction: `σ_Ω = (dσ)/(dΩ)`
         * remarks: Quantities listed under items 10-39, 10-40 and 10-41 are sometimes called differential cross sections. The type of interaction needs to be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DirectionDistributionOfCrossSectionUnit[1];
    }

    attribute directionDistributionOfCrossSection: DirectionDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def DirectionDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-40 energy distribution of cross section */
    attribute def EnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-40 energy distribution of cross section
         * symbol(s): `σ_E`
         * application domain: atomic physics
         * name: EnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/J, kg^-1*s^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to energy, where `σ` is the cross section (item 10-38.1) for a process in which the energy `E` (ISO 80000-5) of the ejected or scattered particle is between `E` and `E + dE`: `σ_E = (dσ)/(dE)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyDistributionOfCrossSectionUnit[1];
    }

    attribute energyDistributionOfCrossSection: EnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def EnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-41 direction and energy distribution of cross section */
    attribute def DirectionAndEnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-41 direction and energy distribution of cross section
         * symbol(s): `σ_(Ω,E)`
         * application domain: atomic physics
         * name: DirectionAndEnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/(J*sr), kg^-1*s^2
         * tensor order: 0
         * definition: partial differential quotient of `σ` with respect to solid angle and energy, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a solid angle `dΩ` around a specified direction and with an energy between `E` and `E+dE`: `σ_(Ω,E) = (del^2 σ) / (del Ω del E)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DirectionAndEnergyDistributionOfCrossSectionUnit[1];
    }

    attribute directionAndEnergyDistributionOfCrossSection: DirectionAndEnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def DirectionAndEnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-42.1 volumic cross section, macroscopic cross section */
    attribute def VolumicCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.1 volumic cross section, macroscopic cross section
         * symbol(s): `Σ`
         * application domain: atomic physics
         * name: VolumicCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and of the cross section (item 10-38.1) `σ_a` for a given type of atoms: `Σ = n_a σ_a`
         * remarks: When the target particles of the medium are at rest, `Σ = 1/l`, where `l` is the mean free path (item 10-71).
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumicCrossSectionUnit[1];
    }

    attribute volumicCrossSection: VolumicCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def VolumicCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias MacroscopicCrossSectionUnit for VolumicCrossSectionUnit;
    alias MacroscopicCrossSectionValue for VolumicCrossSectionValue;
    alias macroscopicCrossSection for volumicCrossSection;

    /* ISO-80000-10 item 10-42.2 volumic total cross section, macroscopic total cross section */
    attribute def VolumicTotalCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.2 volumic total cross section, macroscopic total cross section
         * symbol(s): `Σ_"tot"`, `Σ_"T"`
         * application domain: atomic physics
         * name: VolumicTotalCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and the cross section (item 10-38.1) `σ_"tot"` for a given type of atoms: `Σ_"tot" = n_a*σ_"tot"`
         * remarks: See the Remarks for item 10-49.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumicTotalCrossSectionUnit[1];
    }

    attribute volumicTotalCrossSection: VolumicTotalCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def VolumicTotalCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias MacroscopicTotalCrossSectionUnit for VolumicTotalCrossSectionUnit;
    alias MacroscopicTotalCrossSectionValue for VolumicTotalCrossSectionValue;
    alias macroscopicTotalCrossSection for volumicTotalCrossSection;

    /* ISO-80000-10 item 10-43 particle fluence */
    attribute def ParticleFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-43 particle fluence
         * symbol(s): `Φ`
         * application domain: generic
         * name: ParticleFluence
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: differential quotient of `N` with respect to `a`, where `N` is the number of particles incident on a sphere of cross-sectional area `a` (item 10-38.1): `Φ = (dN)/(da)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` fluence. If a flat area of size `dA` is passed perpendicularly by a number of `dN` particles, the corresponding particle fluence is: `Φ = (dN)/(dA)`. A plane area of size `dA` crossed at an angle `α` with respect to the surface normal by a number of `dN` particles results in the particle fluence: `Φ = (dN)/(cos(α) dA)` In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence, `Φ` , is the quotient of `dN` and `da`, where `dN` is the number of particles incident on a sphere of cross-sectional area `da`: `Φ = (dN)/(dA)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleFluenceUnit[1];
    }

    attribute particleFluence: ParticleFluenceValue[*] nonunique :> scalarQuantities;

    attribute def ParticleFluenceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-44 particle fluence rate */
    attribute def ParticleFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-44 particle fluence rate
         * symbol(s): `dot(Φ)`
         * application domain: generic
         * name: ParticleFluenceRate
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: differential quotient of fluence `Φ` (item 10-43) with respect to time (ISO 80000-3): `dot(Φ) = (dΦ)/(dA)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example proton fluence rate. The distribution function expressed in terms of speed and energy, `dot(Φ)_v` and `dot(Φ)_E` , are related to by: `dot(Φ) = int dot(Φ)_v dv = int dot(Φ)_E dE`. This quantity has also been termed particle flux density. Because the word "density" has several connotations, the term "fluence rate" is preferred. For a radiation field composed of particles of velocity `v`, the fluence rate is equal to `n`·`v` where `n` is the particle number density. See Remarks for item 10-43. In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence rate, `dot(Φ)` , is the quotient of `d Φ` and `dt`, where `d Φ` is the increment of the fluence in the time interval `dt`: `dot(Φ) = (dΦ)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleFluenceRateUnit[1];
    }

    attribute particleFluenceRate: ParticleFluenceRateValue[*] nonunique :> scalarQuantities;

    attribute def ParticleFluenceRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-45 radiant energy */
    attribute radiantEnergyForIonizingRadiation: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-45 radiant energy
         * symbol(s): `R`
         * application domain: ionizing radiation
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: mean energy (ISO 80000-5), excluding rest energy (item 10-3), of the particles that are emitted, transferred, or received
         * remarks: For particles of energy `E` (excluding rest energy), the radiant energy, `R`, is equal to the product `N·E` where `N` is the number of the particles that are emitted, transferred, or received The distributions, `N_E` and `R_E`, of the particle number and the radiant energy with respect to energy are given by `N_E` = `dN`/d`E` and `R_E` = `dR`/d`E`, respectively, where `dN` is the number of particles with energy between `E` and `E`+d`E`, and `dR` is their radiant energy. The two distributions are related by `R_E` = `E`·`N_E`.
         */
    }

    /* ISO-80000-10 item 10-46 energy fluence */
    attribute def EnergyFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-46 energy fluence
         * symbol(s): `Ψ`
         * application domain: generic
         * name: EnergyFluence
         * quantity dimension: M^1*T^-2
         * measurement unit(s): eV/m^2, J/m^2, kg*s^-2
         * tensor order: 0
         * definition: differential quotient of radiant energy `R` (item 10-45) incident on a sphere of cross-sectional area (item 10-38.1) `a` with respect to that area: `Ψ = (dR)/(da)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy fluence, `Ψ` is the quotient of `dR` and `da`, where `dR` is the radiant energy incident on a sphere of cross-sectional area `da`: `Ψ = (dR)/(da)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyFluenceUnit[1];
    }

    attribute energyFluence: EnergyFluenceValue[*] nonunique :> scalarQuantities;

    attribute def EnergyFluenceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-47 energy fluence rate */
    attribute def EnergyFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-47 energy fluence rate
         * symbol(s): `dot(Ψ)`
         * application domain: generic
         * name: EnergyFluenceRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: differential quotient of the energy fluence `Ψ` (item 10-46) with respect to time (ISO 80000-3): `dot(Ψ) = (d Ψ)/(dt)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy-fluence rate, `dot(Ψ)` , is the quotient of `d Ψ` by `dt`, where `d Ψ` is the increment of the energy fluence in the time interval `dt`: `dot(Ψ) = (d Ψ)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyFluenceRateUnit[1];
    }

    attribute energyFluenceRate: EnergyFluenceRateValue[*] nonunique :> scalarQuantities;

    attribute def EnergyFluenceRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-48 particle current density */
    attribute def ParticleCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-48 particle current density (magnitude)
         * symbol(s): `J`, `S`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleCurrentDensityUnit[1];
    }

    attribute particleCurrentDensity: ParticleCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    attribute def CartesianParticleCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-48 particle current density (vector)
         * symbol(s): `vec(J)`, `vec(S)`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 1
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianParticleCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianParticleCurrentDensity3dVector: CartesianParticleCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianParticleCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ParticleCurrentDensityUnit[3];
    }

    /* ISO-80000-10 item 10-49 linear attenuation coefficient */
    attribute def LinearAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-49 linear attenuation coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: ionizing radiation
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: for uncharged particles of a given type and energy the differential quotient `n` with respect to `l,` where `n` is the fraction of `N` incoming particles that experience interactions in traversing a distance (ISO 80000-3) `l` in a given material: `μ = (dn)/(dl) = 1/N (dN)/(dl)` where `dN` is the number of particles that experience interactions in traversing `dl`
         * remarks: `μ` is equal to the macroscopic total cross section `Σ_"tot"` for the removal of particles from the beam. Using the relation `μ_m = μ/ρ` between the linear attenuation coefficient `μ`, the mass attenuation coefficient `μ_m` (item 10-50) and the density `ρ`, the definition given for the mass attenuation coefficient in report 85a of the ICRU can be applied to the linear attenuation coefficient resulting in: The linear attenuation coefficient, `μ`, of a material, for uncharged particles of a given type and energy, is the quotient of `(dN)/N` by `dl`, where `(dN)/N` is the mean fraction of the particles that experience interactions in traversing a distance `dl` in the material: `μ = 1/(dl) (dN)/(N)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAttenuationCoefficientForIonizingRadiationUnit[1];
    }

    attribute linearAttenuationCoefficientForIonizingRadiation: LinearAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def LinearAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-50 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-50 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: ionizing radiation
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the mass density `ρ` (ISO 80000-4) of the medium: `μ_m = μ/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAttenuationCoefficientForIonizingRadiationUnit[1];
    }

    attribute massAttenuationCoefficientForIonizingRadiation: MassAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def MassAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-51 molar attenuation coefficient */
    attribute def MolarAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-51 molar attenuation coefficient
         * symbol(s): `μ_c`
         * application domain: generic
         * name: MolarAttenuationCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: quotient of linear attenuation coefficient `µ` (item 10-49) and the amount c (ISO 80000-9) of the medium: `μ_c = μ/c`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarAttenuationCoefficientUnit[1];
    }

    attribute molarAttenuationCoefficient: MolarAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MolarAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-10 item 10-52 atomic attenuation coefficient */
    attribute def AtomicAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-52 atomic attenuation coefficient
         * symbol(s): `μ_a`
         * application domain: generic
         * name: AtomicAttenuationCoefficient
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the number density (item 10-62.1), `n`, of atoms in the substance: `μ_a = μ/n`
         * remarks: `μ` is equal to the total cross section `σ_"tot"` for the removal of particles from the beam. See also item 10-38.2.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AtomicAttenuationCoefficientUnit[1];
    }

    attribute atomicAttenuationCoefficient: AtomicAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def AtomicAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-53 half-value thickness */
    attribute halfValueThickness: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-53 half-value thickness
         * symbol(s): `d_(1//2)`
         * application domain: generic
         * name: HalfValueThickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: thickness (ISO 80000-3) of the attenuating layer that reduces the quantity of interest of a unidirectional beam of infinitesimal width to half of its initial value
         * remarks: For exponential attenuation, `d_(1/2) = ln(2)/μ`. The quantity of interest is often the air kerma or exposure.
         */
    }

    /* ISO-80000-10 item 10-54 total linear stopping power, linear stopping power */
    attribute def TotalLinearStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-54 total linear stopping power, linear stopping power
         * symbol(s): `S`, `S_l`
         * application domain: generic
         * name: TotalLinearStoppingPower
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: for charged particles of a given type and energy `E_0` the differential quotient of `E` with respect to `x,` where `E` is the mean energy (ISO 80000-4) lost by the charged particles in traversing a distance (ISO 80000-3) `x` in the given material: `S = -(dE)/(dx)`
         * remarks: The total linear stopping power is sometimes also called stopping power. Both electronic losses and radiative losses are included. The quotient of the total linear stopping power of a substance and that of a reference substance is called the relative linear stopping power. See also item 10-85. Using the relation `S_m = S/ρ` between the total mass stopping power `S_m` (item 10-55), the total linear stopping power `S`, and the density `ρ`, the definition given for the mass stopping in report 85a of the ICRU can be applied to that of the total linear stopping power resulting in: The linear stopping power, `S`, of a material, for charged particles of a given type and energy, is the quotient of `dE` by `dl`, where `dE` is the mean energy lost by the charged particles in traversing a distance `dl` in the material: `S = -(dE)/(dx)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalLinearStoppingPowerUnit[1];
    }

    attribute totalLinearStoppingPower: TotalLinearStoppingPowerValue[*] nonunique :> scalarQuantities;

    attribute def TotalLinearStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias LinearStoppingPowerUnit for TotalLinearStoppingPowerUnit;
    alias LinearStoppingPowerValue for TotalLinearStoppingPowerValue;
    alias linearStoppingPower for totalLinearStoppingPower;

    /* ISO-80000-10 item 10-55 total mass stopping power, mass stopping power */
    attribute def TotalMassStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-55 total mass stopping power, mass stopping power
         * symbol(s): `S_m`
         * application domain: generic
         * name: TotalMassStoppingPower
         * quantity dimension: L^4*T^-2
         * measurement unit(s): eV*m^-2/kg, J*m^2/kg, m^4*s^-2
         * tensor order: 0
         * definition: quotient of the total linear stopping power `S` (item 10-54) and the mass density `ρ` (ISO 80000-4) of the material: `S_m = S/ρ`
         * remarks: The quotient of total mass stopping power of a material and that of a reference material is called relative mass stopping power.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalMassStoppingPowerUnit[1];
    }

    attribute totalMassStoppingPower: TotalMassStoppingPowerValue[*] nonunique :> scalarQuantities;

    attribute def TotalMassStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias MassStoppingPowerUnit for TotalMassStoppingPowerUnit;
    alias MassStoppingPowerValue for TotalMassStoppingPowerValue;
    alias massStoppingPower for totalMassStoppingPower;

    /* ISO-80000-10 item 10-56 mean linear range */
    attribute meanLinearRange: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-56 mean linear range
         * symbol(s): `R`, `R_l`
         * application domain: generic
         * name: MeanLinearRange (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: mean total rectified path length (ISO 80000-3) travelled by a particle in the course of slowing down to rest in a given material averaged over a group of particles having the same initial energy (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-57 mean mass range */
    attribute def MeanMassRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-57 mean mass range
         * symbol(s): `R_ρ`, `R_m`
         * application domain: generic
         * name: MeanMassRange
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: product of the mean linear range (item 10-56) `R` and the mass density `ρ` (ISO 80000-4) of the material: `R_ρ = R*ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MeanMassRangeUnit[1];
    }

    attribute meanMassRange: MeanMassRangeValue[*] nonunique :> scalarQuantities;

    attribute def MeanMassRangeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-58 linear ionization */
    attribute def LinearIonizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-58 linear ionization
         * symbol(s): `N_{i_l}`
         * application domain: generic
         * name: LinearIonization
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: differential quotient of `q` with respect to `l`, where `q` is the average total charge (IEC 80000-6) of all positive ions produced by an ionizing charged particle over a path `l` (ISO 80000-3), divided by the elementary charge, `e` (ISO 80000-1): `N_{i_l} = 1/e*(dq)/(dl)`
         * remarks: Ionization due to secondary ionizing particles is included.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearIonizationUnit[1];
    }

    attribute linearIonization: LinearIonizationValue[*] nonunique :> scalarQuantities;

    attribute def LinearIonizationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-59 total ionization */
    attribute def TotalIonizationValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-59 total ionization
         * symbol(s): `N_i`
         * application domain: generic
         * name: TotalIonization (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total mean charge of all positive ions produced by an ionizing charged particle along its entire path and along the paths of any secondary charged particles, and the elementary charge, `e` (ISO 80000-1)
         * remarks: `N_i = int N_(il) dl` See item 10-58.
         */
    }
    attribute totalIonization: TotalIonizationValue :> scalarQuantities;

    /* ISO-80000-10 item 10-60 average energy loss per elementary charge produced */
    attribute def AverageEnergyLossPerElementaryChargeProducedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-60 average energy loss per elementary charge produced
         * symbol(s): `W_i`
         * application domain: generic
         * name: AverageEnergyLossPerElementaryChargeProduced
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the initial kinetic energy `E_k` (ISO 80000-4) of an ionizing charged particle and the total ionization `N_i` (item 10-59) produced by that particle: `W_i = E_k/N_i`
         * remarks: The name "average energy loss per ion pair formed" is usually used, although it is ambiguous. In the practical dosimetry of ionizing radiation the term `W`/`e`, the quotient of `W`, the average energy deposited in dry air per ion pair formed, and `e`, the elementary charge, is used as the factor which, when multiplied with the electric charge of one sign carried by all ion pairs formed in dry air of given mass, gives the energy deposited in this amount of dry air in the form of excitations and ionizations. In ICRU Report 85a, the mean energy expended in a gas per ion pair formed, `W`, is the quotient of `E` by `N,` where `N` is the mean total liberated charge of either sign, divided by the elementary charge when the initial kinetic energy `E` of a charged particle introduced into the gas is completely dissipated in the gas. Thus, `W` = `E`/`N`. It follows from the definition of `W` that the ions produced by bremsstrahlung or other secondary radiation emitted by the initial and secondary charged particles are included in `N`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AverageEnergyLossPerElementaryChargeProducedUnit[1];
    }

    attribute averageEnergyLossPerElementaryChargeProduced: AverageEnergyLossPerElementaryChargeProducedValue[*] nonunique :> scalarQuantities;

    attribute def AverageEnergyLossPerElementaryChargeProducedUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-61 mobility */
    attribute def MobilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-61 mobility
         * symbol(s): `μ`, `μ_m`
         * application domain: generic
         * name: Mobility
         * quantity dimension: M^-1*T^2*I^1
         * measurement unit(s): m^2/(V*s), kg^-1*s^2*A
         * tensor order: 0
         * definition: quotient of average drift speed (ISO 80000-3) imparted to a charged particle in a medium by an electric field, and the electric field strength (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MobilityUnit[1];
    }

    attribute mobility: MobilityValue[*] nonunique :> scalarQuantities;

    attribute def MobilityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-10 item 10-62.1 particle number density */
    attribute def ParticleNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.1 particle number density
         * symbol(s): `n`
         * application domain: generic
         * name: ParticleNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the mean number `N` of particles in the volume (ISO 80000-3) `V` and volume: `n = N/V`
         * remarks: `n` is the general symbol for the number density of particles. The distribution functions expressed in terms of speed and energy, `n_v` and `n_E`, are related to `n` by: `n = int n_v dv = int n_E dE`. The word "particle" is usually replaced by the name of a specific particle, for example `neutron` number density.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleNumberDensityUnit[1];
    }

    attribute particleNumberDensity: ParticleNumberDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-62.2 ion number density, ion density */
    attribute def IonNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.2 ion number density, ion density
         * symbol(s): `n^"+"`, `n^"-"`
         * application domain: generic
         * name: IonNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the number of positive and negative ions, `N^"+"` and `N^"-"`, respectively, in the volume `V` (ISO 80000-3), and that volume: `n^"+" = N^"+" / V`, `n^"-" = N^"-" / V`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IonNumberDensityUnit[1];
    }

    attribute ionNumberDensity: IonNumberDensityValue[*] nonunique :> scalarQuantities;

    attribute def IonNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias IonDensityUnit for IonNumberDensityUnit;
    alias IonDensityValue for IonNumberDensityValue;
    alias ionDensity for ionNumberDensity;

    /* ISO-80000-10 item 10-63 Recombination coefficient */
    attribute def RecombinationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-63 Recombination coefficient
         * symbol(s): `α`
         * application domain: generic
         * name: RecombinationCoefficient
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: coefficient in the law of recombination: `-(dn^"+")/(dt) = -(dn^"-")/(dt) = α*n^"+"*n^"-"`, where `n^"+"` and `n^"-"` are the ion number densities (item 10-62.2) of positive and negative ions, respectively, recombined during a time interval of duration `dt` (ISO 80000-3)
         * remarks: The widely used term "recombination factor" is not correct because "factor" should only be used for quantities with dimension 1. The terms `(dn^"+")/(dt)` , `(dn^"-")/(dt)` are differential quotients.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RecombinationCoefficientUnit[1];
    }

    attribute recombinationCoefficient: RecombinationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def RecombinationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-64 diffusion coefficient, diffusion coefficient for particle number density */
    /* Refer to declaration for DiffusionCoefficient in ISQChemistryMolecular item 9-39 diffusion coefficient */

    alias DiffusionCoefficientForParticleNumberDensityUnit for DiffusionCoefficientUnit;
    alias DiffusionCoefficientForParticleNumberDensityValue for DiffusionCoefficientValue;
    alias diffusionCoefficientForParticleNumberDensity for diffusionCoefficient;

    /* ISO-80000-10 item 10-65 diffusion coefficient for fluence rate */
    attribute diffusionCoefficientForFluenceRate: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-65 diffusion coefficient for fluence rate
         * symbol(s): `D_ϕ`, `D`
         * application domain: generic
         * name: DiffusionCoefficientForFluenceRate (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: proportionality constant between the particle current density `vec(J )`(item 10-48) and the gradient of the particle fluence rate `dot(Φ)` (item 10-44): `vec(J) = -vec(D) * nabla Φ`
         * remarks: For a particle of a given speed `v`: `D_Ψ(v) = -J_{v,x}/(partial Ψ // partial x)` and `vec(v) * vec(D_Ψ)(v) = -vec(D_n)(v)`
         */
    }

    /* ISO-80000-10 item 10-66 particle source density */
    attribute def ParticleSourceDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-66 particle source density
         * symbol(s): `S`
         * application domain: generic
         * name: ParticleSourceDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the mean rate of production of particles in a volume, and that volume (ISO 80000-3)
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` source density. The distribution functions expressed in terms of speed and energy, `S_v` and `S_E`, are related to `S` by: `S = int S_v dv = int S_E dE`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleSourceDensityUnit[1];
    }

    attribute particleSourceDensity: ParticleSourceDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleSourceDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-67 slowing-down density */
    attribute def SlowingDownDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-67 slowing-down density
         * symbol(s): `q`
         * application domain: generic
         * name: SlowingDownDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: differential quotient of `n` with respect to time, where `n` is the number density of particles that are slowed down in a time interval of duration (ISO 80000-3) `t`: `q = -(dn)/(dt)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SlowingDownDensityUnit[1];
    }

    attribute slowingDownDensity: SlowingDownDensityValue[*] nonunique :> scalarQuantities;

    attribute def SlowingDownDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-68 resonance escape probability */
    attribute def ResonanceEscapeProbabilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-68 resonance escape probability
         * symbol(s): `p`
         * application domain: generic
         * name: ResonanceEscapeProbability (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the probability that a neutron slowing down will traverse all or some specified portion of the range of resonance energies (item 10-37.2) without being absorbed
         * remarks: None.
         */
    }
    attribute resonanceEscapeProbability: ResonanceEscapeProbabilityValue :> scalarQuantities;

    /* ISO-80000-10 item 10-69 lethargy */
    attribute def LethargyValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-69 lethargy
         * symbol(s): `u`
         * application domain: generic
         * name: Lethargy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a neutron of kinetic energy `E` (ISO 80000-4) : `u = ln(E_0/E)`, where `E_0` is a reference energy
         * remarks: Lethargy is also referred to as logarithmic energy decrement.
         */
    }
    attribute lethargy: LethargyValue :> scalarQuantities;

    /* ISO-80000-10 item 10-70 average logarithmic energy decrement */
    attribute def AverageLogarithmicEnergyDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-70 average logarithmic energy decrement
         * symbol(s): `ζ`
         * application domain: generic
         * name: AverageLogarithmicEnergyDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average value of the increase in lethargy (item 10-69) in elastic collisions between neutrons and nuclei whose kinetic energy (ISO 80000-4) is negligible compared with that of the neutrons
         * remarks: None.
         */
    }
    attribute averageLogarithmicEnergyDecrement: AverageLogarithmicEnergyDecrementValue :> scalarQuantities;

    /* ISO-80000-10 item 10-71 mean free path */
    attribute meanFreePathForAtomicPhysics: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-71 mean free path
         * symbol(s): `l`, `λ`
         * application domain: atomic physics
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that particles travel between two successive specified reactions or processes
         * remarks: See the Remarks for item 10-42.1.
         */
    }

    /* ISO-80000-10 item 10-72.1 slowing-down area */
    attribute slowingDownArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.1 slowing-down area
         * symbol(s): `L_s^2`, `L_"sl"^2`
         * application domain: generic
         * name: SlowingDownArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square of the distance (ISO 80000-3) between the neutron source and the point where a neutron reaches a given energy (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-72.2 diffusion area */
    attribute diffusionArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.2 diffusion area
         * symbol(s): `L^2`
         * application domain: generic
         * name: DiffusionArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square distance (ISO 80000-3) between the point where a neutron enters a specified class and the point where it leaves this class
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
    }

    /* ISO-80000-10 item 10-72.3 migration area */
    attribute migrationArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.3 migration area
         * symbol(s): `M^2`
         * application domain: generic
         * name: MigrationArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: sum of the slowing-down area (item 10-72.1) from fission energy to thermal energy (ISO 80000-5) and the diffusion area (item 10-72.2) for thermal neutrons
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.1 slowing-down length */
    attribute slowingDownLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.1 slowing-down length
         * symbol(s): `L_s`, `L_"sl"`
         * application domain: generic
         * name: SlowingDownLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the slowing down area `L_s^2` (item 10-72.1): `L_s = sqrt(L_s^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.2 diffusion length */
    attribute diffusionLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.2 diffusion length
         * symbol(s): `L`
         * application domain: atomic physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the diffusion area `L^2` (item 10-72.2): `L = sqrt(L^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.3 migration length */
    attribute migrationLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.3 migration length
         * symbol(s): `M`
         * application domain: generic
         * name: MigrationLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the migration area `M^2` (item 10-72.3): `M = sqrt(M^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-74.1 neutron yield per fission */
    attribute neutronYieldPerFission: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.1 neutron yield per fission
         * symbol(s): `ν`
         * application domain: generic
         * name: NeutronYieldPerFission (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per fission event
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-74.2 neutron yield per absorption */
    attribute neutronYieldPerAbsorption: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.2 neutron yield per absorption
         * symbol(s): `η`
         * application domain: generic
         * name: NeutronYieldPerAbsorption (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per neutron absorbed in a fissionable nuclide or in a nuclear fuel, as specified
         * remarks: `ν/η` is equal to the quotient of the macroscopic cross section for fission and that for absorption, both for neutrons in the fuel material.
         */
    }

    /* ISO-80000-10 item 10-75 fast fission factor */
    attribute def FastFissionFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-75 fast fission factor
         * symbol(s): `φ`
         * application domain: generic
         * name: FastFissionFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the mean number of neutrons produced by fission due to neutrons of all energies (ISO 80000-5) and the mean number of neutrons produced by fissions due to thermal neutrons only
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FastFissionFactorUnit[1];
    }

    attribute fastFissionFactor: FastFissionFactorValue[*] nonunique :> scalarQuantities;

    attribute def FastFissionFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-76 thermal utilization factor */
    attribute def ThermalUtilizationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-76 thermal utilization factor
         * symbol(s): `f`
         * application domain: generic
         * name: ThermalUtilizationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the number of thermal neutrons absorbed in a fissionable nuclide or in a nuclear fuel, as specified, and the total number of thermal neutrons absorbed
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalUtilizationFactorUnit[1];
    }

    attribute thermalUtilizationFactor: ThermalUtilizationFactorValue[*] nonunique :> scalarQuantities;

    attribute def ThermalUtilizationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-77 non-leakage probability */
    attribute def NonLeakageProbabilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-77 non-leakage probability
         * symbol(s): `Λ`
         * application domain: generic
         * name: NonLeakageProbability
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: probability that a neutron will not escape from the reactor during the slowing-down process or while it diffuses as a thermal neutron
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NonLeakageProbabilityUnit[1];
    }

    attribute nonLeakageProbability: NonLeakageProbabilityValue[*] nonunique :> scalarQuantities;

    attribute def NonLeakageProbabilityUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-78.1 multiplication factor */
    attribute def MultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.1 multiplication factor
         * symbol(s): `k`
         * application domain: generic
         * name: MultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total number of fission or fission-dependent neutrons produced in the duration of a time interval and the total number of neutrons lost by absorption and leakage in that duration
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MultiplicationFactorUnit[1];
    }

    attribute multiplicationFactor: MultiplicationFactorValue[*] nonunique :> scalarQuantities;

    attribute def MultiplicationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-78.2 infinite multiplication factor */
    attribute def InfiniteMultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.2 infinite multiplication factor
         * symbol(s): `k_∞`
         * application domain: generic
         * name: InfiniteMultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: multiplication factor (item 10-78.1) for an infinite medium or for an infinite repeating lattice
         * remarks: For a thermal reactor, `k_∞ = η*ε*p*f`
         */
        attribute :>> num: Real;
        attribute :>> mRef: InfiniteMultiplicationFactorUnit[1];
    }

    attribute infiniteMultiplicationFactor: InfiniteMultiplicationFactorValue[*] nonunique :> scalarQuantities;

    attribute def InfiniteMultiplicationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-79 reactor time constant */
    attribute reactorTimeConstant: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-79 reactor time constant
         * symbol(s): `T`
         * application domain: generic
         * name: ReactorTimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (ISO 80000-3) required for the neutron fluence rate (item 10-44) in a reactor to change by the factor e when the fluence rate is rising or falling exponentially
         * remarks: Also called reactor period.
         */
    }

    /* ISO-80000-10 item 10-80.1 energy imparted */
    attribute energyImparted: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.1 energy imparted
         * symbol(s): `ε`
         * application domain: generic
         * name: EnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of all energy deposits in a given volume: `ε = sum_i ε_i` where the summation is performed over all energy (ISO 80000-5) deposits `ε_i` of interaction `i` in that volume
         * remarks: Energy imparted is a stochastic quantity. `ε_i` is given by: `ε_i = ε_(i n) - ε_"out" + Q` where `ε_(i n)` is the energy (ISO 80000-5) of the incident ionizing particle, excluding rest energy (item 10-3), `ε_"out"` is the sum of the energies (ISO 80000-5) of all ionizing particles leaving the interaction, excluding rest energy (item 10-3), and `Q` is the change in the rest energies (item 10-3) of the nucleus and of all particles involved in the interaction. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy. Stochastic quantities such as the energy imparted and the specific energy imparted (item 10-81.2) and their probability distributions have been introduced as they describe the discontinuous nature of the ionizing radiations as a determinant of radiochemical and radiobiological effects. In radiation applications involving large numbers of ionizing particles, e.g. in medicine, radiation protection and materials testing and processing, these fluctuations are adequately represented by the expectation values of the probability distributions. Non-stochastic quantities such as particle fluence (item 10-43), absorbed dose (item 10-81.1) and kerma (item 10-86.1) are based on these expectation values.
         */
    }

    /* ISO-80000-10 item 10-80.2 mean energy imparted */
    attribute meanEnergyImparted: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.2 mean energy imparted
         * symbol(s): `bar(ε)`
         * application domain: generic
         * name: MeanEnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: expectation value of the energy imparted (item 10-80.1): `bar(ε) = R_"in" - R_"out" + sum Q` where `R_"in"` is the radiant energy (item 10-45) of all those charged and uncharged ionizing particles that enter the volume, `R_"out"` is the radiant energy of all those charged and uncharged ionizing particles that leave the volume, and `sum Q` is the sum of all changes of the rest energy (item 10-3) of nuclei and elementary particles that occur in that volume
         * remarks: Sometimes, it has been called the integral absorbed dose. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy.
         */
    }

    /* ISO-80000-10 item 10-81.1 absorbed dose */
    attribute def AbsorbedDoseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-81.1 absorbed dose
         * symbol(s): `D`
         * application domain: generic
         * name: AbsorbedDose
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: differential quotient of `bar(ε)` with respect to `m`, where `bar(ε)` is the mean energy (ISO 80000-5) imparted by ionizing radiation to matter of mass (ISO 80000-4) `m`: `D = (d bar(ε))/(dm)`
         * remarks: The gray is a special name for joule per kilogram, to be used as the coherent SI unit for absorbed dose. `1 "Gy" = 1 "J"/"kg"`. `bar(ε) = int D dm` where `dm` is the element of mass of the irradiated matter. In the limit of a small domain, the mean specific energy `bar(z) = (Δ bar(ε))/(Δ m)` is equal to the absorbed dose `D`. The absorbed dose can also be expressed in terms of the volume of the mass element by: `D = (d bar(ε))/(dm) = (d bar(ε))/(ρ dV)` where `ρ` is the mass density of the mass element. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed dose, `D`, is the quotient of `d bar(ε)` by dm, where `d bar(ε)` is the mean energy imparted by ionizing radiation to matter of mass `dm`: `D = (d bar(ε))/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AbsorbedDoseUnit[1];
    }

    attribute absorbedDose: AbsorbedDoseValue[*] nonunique :> scalarQuantities;

    attribute def AbsorbedDoseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-81.2 specific energy imparted */
    attribute specificEnergyImparted: AbsorbedDoseValue :> scalarQuantities {
        doc
        /*
         * source: item 10-81.2 specific energy imparted
         * symbol(s): `z`
         * application domain: generic
         * name: SpecificEnergyImparted (specializes AbsorbedDose)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of the energy imparted `ε` (item 10-80.1) and the mass `m` (ISO 80000-4) of the matter in a given volume element: `z = ε / m`
         * remarks: `z` is a stochastic quantity. In the limit of a small domain, the mean specific energy `bar(z)` is equal to the absorbed dose `D`. The specific energy imparted can be due to one or more (energy-deposition) events.
         */
    }

    /* ISO-80000-10 item 10-82 quality factor */
    attribute def QualityFactorForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-82 quality factor
         * symbol(s): `Q`
         * application domain: ionizing radiation
         * name: QualityFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor in the calculation and measurement of dose equivalent (item 10-83.1), by which the absorbed dose (item 10-81.1) is to be weighted in order to account for different biological effectiveness of radiations, for radiation protection purposes
         * remarks: `Q` is determined by the linear energy transfer (item 10-85) for `Δ -> ∞` , `L_∞` (often denoted as `L` or LET), of charged particles passing through a small volume element at this point (the value of `L_∞` refers to water, not to tissue; the difference, however, is small). The relationship between `L` and `Q` is given in ICRP Publication 103 (ICRP, 2007).
         */
        attribute :>> num: Real;
        attribute :>> mRef: QualityFactorForIonizingRadiationUnit[1];
    }

    attribute qualityFactorForIonizingRadiation: QualityFactorForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def QualityFactorForIonizingRadiationUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-83.1 dose equivalent */
    attribute def DoseEquivalentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-83.1 dose equivalent
         * symbol(s): `H`
         * application domain: generic
         * name: DoseEquivalent
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Sv, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: product of the absorbed dose `D` (item 10-81.1) to tissue at the point of interest and the quality factor `Q` (item 10-82) at that point: `H = DQ`
         * remarks: The sievert (Sv) is a special name for joule per kilogram, and is the coherent SI unit for dose equivalent. `1 "Sv" = 1 "J/kg"`. The dose equivalent at a point in tissue is given by: `H = int_0^∞ Q(L) D_L dL` where `D_L = (dD)/(dL)` is the distribution of `D` in `L` at the point of interest. See ICRP Publication 103 (ICRP, 2007). The quantities measured with radiation protection dosimeters are based on the definition `H = Q*D`. If various radiation qualities `i` have to be simultaneously accounted for, the definition is: `H = sum_i Q_i*D_i`. In ICRU 51 this quantity is denoted as "dose equivalent". In order to quantify the radiation exposition of the human body and to specify dose limits, use is made of a quantity defined in ICRP 103, the "equivalent dose to a tissue or organ": `H_T = w_T*sum_R w_R*D_{T,R}`. The weighting factors `w_T` for various tissues and organs `T` and `w_R` for various radiation qualities `R` have been numerically laid down in ICRP 103. `D_{T,R}` is the mean absorbed dose to tissue within a tissue or organ `T`, imparted by radiation with radiation quality `R`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DoseEquivalentUnit[1];
    }

    attribute doseEquivalent: DoseEquivalentValue[*] nonunique :> scalarQuantities;

    attribute def DoseEquivalentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-83.2 dose equivalent rate */
    attribute doseEquivalentRate: DoseEquivalentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-83.2 dose equivalent rate
         * symbol(s): `dot(H)`
         * application domain: generic
         * name: DoseEquivalentRate (specializes DoseEquivalent)
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Sv/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of dose equivalent `H` (item 10-83.1) with respect to time (ISO 80000-3): `dot(H) = (dH)/(dt)`
         * remarks: `1 "Sv/s" = 1 "W/kg"`. See the remarks for item 10-83.1.
         */
    }

    /* ISO-80000-10 item 10-84 absorbed-dose rate */
    attribute def AbsorbedDoseRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-84 absorbed-dose rate
         * symbol(s): `dot(D)`
         * application domain: generic
         * name: AbsorbedDoseRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of the absorbed dose `D` (item 10-81.1) with respect to time (ISO 80000-3): `dot(D) = (dD)/(dt)`
         * remarks: `1 "Gy/s"  = 1 "W/kg"` See the remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed-does rate, `dot(D)` , is the quotient of `dD` by `dt`, where `dD` is the increment of absorbed does in the time interval `dt`: `dot(D) = (dD)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AbsorbedDoseRateUnit[1];
    }

    attribute absorbedDoseRate: AbsorbedDoseRateValue[*] nonunique :> scalarQuantities;

    attribute def AbsorbedDoseRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-85 linear energy transfer */
    attribute def LinearEnergyTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-85 linear energy transfer
         * symbol(s): `L_Δ`
         * application domain: generic
         * name: LinearEnergyTransfer
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: quotient of the mean energy (ISO 80000-4) `dE_Δ` lost by the charged particles due to electronic interactions in traversing a distance (ISO 80000-3) `dl`, minus the mean sum of the kinetic energies in excess of `Δ` of all the electrons released by the charged particles and `dl`: `L_Δ = (dE_Δ)/(dl)`
         * remarks: This quantity is not completely defined unless `Δ` is specified, i.e. the maximum kinetic energy of secondary electrons whose energy is considered to be "locally deposited". `Δ` may be expressed in `"eV"`. Note that the abbreviation LET specifically refers to the quantity `L_∞` mentioned in the remark to 10-82.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearEnergyTransferUnit[1];
    }

    attribute linearEnergyTransfer: LinearEnergyTransferValue[*] nonunique :> scalarQuantities;

    attribute def LinearEnergyTransferUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-86.1 kerma */
    attribute def KermaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.1 kerma
         * symbol(s): `K`
         * application domain: generic
         * name: Kerma
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: for uncharged ionizing radiation, differential quotient of `E_(`tr) with respect to `m`, where `E_(`tr) is the mean sum of the initial kinetic energies (ISO 80000-4) of all the charged ionizing particles liberated in a mass (ISO 80000-4) `m` of a material: `K = (dE_tr)/(dm)`
         * remarks: `1 "Gy" = 1 "J/kg"` See the remarks for item 10-81.1. The name "kerma" is derived from Kinetic Energy Released in MAtter (or MAss or MAterial). The quantity `dE_(tr)` includes also the kinetic energy of the charged particles emitted in the decay of excited atoms, molecules, or nuclei. When the mass element `dm` consists of air the term air kerma is used. It can be convenient to refer to a value of air kerma in free space or at a point inside a material different from air, e.g. to the air kerma at a point inside a water phantom. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma, `K`, for ionizing uncharged particles, is the quotient of `dE_(tr)` by `dm`, where `dE_(tr)` is the mean sum of the initial kinetic energies of all the charged particles liberated in a mass `dm` of a material by the uncharged particles incident on `dm`: `K = (dE_(tr))/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KermaUnit[1];
    }

    attribute kerma: KermaValue[*] nonunique :> scalarQuantities;

    attribute def KermaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-86.2 kerma rate */
    attribute def KermaRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.2 kerma rate
         * symbol(s): `dot(K)`
         * application domain: generic
         * name: KermaRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of kerma (item 10-86.1) with respect to time (ISO 80000-3): `dot(K) = (dK)/(dt)`
         * remarks: `1 "Gy/s" = 1 "W/kg"`. See the Remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma rate, `dot(K)` , is the quotient of `dK` by `dt`, where `dK` is the increment of kerma in the time interval `dt`: `dot(K) = (dK)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KermaRateUnit[1];
    }

    attribute kermaRate: KermaRateValue[*] nonunique :> scalarQuantities;

    attribute def KermaRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-87 mass energy-transfer coefficient */
    attribute def MassEnergyTransferCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-87 mass energy-transfer coefficient
         * symbol(s): `μ_"tr"/ρ`
         * application domain: generic
         * name: MassEnergyTransferCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: for ionizing uncharged particles of a given type and energy, the differential quotient of `R_"tr"` with respect to `l`: `m_"tr"/ρ = 1/ρ 1/R (dR_"tr")/(dl)` where `R_"tr"` is the mean energy (ISO 80000-5) that is transferred to kinetic energy (ISO 80000-4) of charged particles by interactions of the uncharged particles of incident radiant energy `R` (item 10-45) in traversing a distance (ISO 80000-3) `l` in the material of density (ISO 80000-4) `ρ`, divided by `ρ` and `R`
         * remarks: `m_(tr)/ρ = (dot(K))/ψ` , where `dot(K)` is kerma rate (item 10-86.2) and `ψ` is energy fluence rate (item 10-47). The quantity: `μ_(en)/ρ = μ_(tr)/ρ(1-g)` where `g` is mean fraction of the kinetic energy of the liberated charged particles that is lost in radiative processes in the material, is called mass energy-absorption coefficient. The mass energy-absorption coefficient of a compound material depends on the stopping power of the material. Thus, its evaluation cannot, in principle, be reduced to a simple summation of the mass energy-absorption coefficient of the atomic constituents. Such a summation can provide an adequate approximation when the value of `g` is sufficiently small. In report 85a of the ICRU a definition with an equivalent meaning is given as: The mass energy-transfer coefficient, `μ_(tr)/ρ` , of a material, for uncharged particles of a given type and energy, is the quotient of `(dR_(tr))/R` by `ρ dl`, where `dR_(tr)` is the mean energy that is transferred to kinetic energy of charged particles by interactions of the uncharged particles of incident radiant energy `R` in traversing a distance `dl` in the material of density `ρ` : `μ_(tr)/ρ = 1/(ρ dl) (d R_(tr))/R`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassEnergyTransferCoefficientUnit[1];
    }

    attribute massEnergyTransferCoefficient: MassEnergyTransferCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassEnergyTransferCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-88 exposure */
    attribute def ExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-88 exposure
         * symbol(s): `X`
         * application domain: ionizing radiation
         * name: Exposure
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): C/kg, kg^-1*s*A
         * tensor order: 0
         * definition: for X- or gamma radiation the differential quotient of `q` with respect to `m`, where `q` is the absolute value of the mean total electric charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on an element of dry air with mass `m` (ISO 80000-4) are completely stopped in dry air: `X = (dq)/(dm)`
         * remarks: The ionization produced by electrons emitted in atomic or molecular relaxation is included in `dq`. The ionization due to photons emitted by radiative processes (i.e. bremsstrahlung and fluorescence photons) is not included in `dq`. This quantity should not be confused with the quantity photon exposure (ISO 80000-7), radiation exposure (ISO 80000-7), or the quantity luminous exposure (ISO 80000-7). It can be convenient to refer to a value of exposure in free space or at a point inside a material different from air, e.g. to the exposure at a point inside a water phantom. The exposure is related to the air kerma, `K_a`, (see item 10-86.1) by: `X = (e (1-g))/W K_a` , where `e` is the elementary charge (ISO 80000-1), `W` the average energy loss per elementary charge produced (item 10-60), and `g` is the fraction of the kinetic energy of liberated charged particles that is lost in radiative processes. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure, `X`, is the quotient of `dq` by `dm`, where `dq` is the absolute value of the mean total charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on a mass `dm` of dry air are completely stopped in dry air: `X = (dq)/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ExposureUnit[1];
    }

    attribute exposure: ExposureValue[*] nonunique :> scalarQuantities;

    attribute def ExposureUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-10 item 10-89 exposure rate */
    attribute def ExposureRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-89 exposure rate
         * symbol(s): `dot(X)`
         * application domain: generic
         * name: ExposureRate
         * quantity dimension: M^-1*I^1
         * measurement unit(s): C/(kg*s), kg^-1*A
         * tensor order: 0
         * definition: differential quotient of the exposure `X` (item 10-88) with respect to time (ISO 80000-3): `dot(X) = (dX)/(dt)`
         * remarks: `1 "C/(kg s)" = 1 "A/kg"`. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure rate, `dot(X)` , is the quotient of `dX` by `dt`, where `dX` is the increment of exposure in the time interval `dt`: `dot(X) = (dX)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ExposureRateUnit[1];
    }

    attribute exposureRate: ExposureRateValue[*] nonunique :> scalarQuantities;

    attribute def ExposureRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, electricCurrentPF); }
    }

}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ElectricChargeValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
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
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularFrequencyValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'AngularFrequencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularFrequencyValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
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
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
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
semantic.unresolved_name 'DimensionOneUnit'
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
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ElectricChargeValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
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
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularFrequencyValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'AngularFrequencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularFrequencyValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MassValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
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
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AreaValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
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
semantic.unresolved_name 'DimensionOneUnit'
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
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
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
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
RegularComment,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQAtomicNuclear'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQChemistryMolecular::DiffusionCoefficientUnit')
    (import_decl private 'ISQChemistryMolecular::DiffusionCoefficientValue')
    (import_decl private 'ISQChemistryMolecular::diffusionCoefficient')
    (import_decl private 'ISQElectromagnetism::ElectricChargeValue')
    (import_decl private 'ISQSpaceTime::AngularFrequencyValue')
    (import_decl private 'ISQSpaceTime::AreaValue')
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (attribute_usage 'atomicNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'protonNumber' for 'atomicNumber')
    (comment)
    (attribute_usage 'neutronNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'nucleonNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'massNumber' for 'nucleonNumber')
    (comment)
    (attribute_usage 'restMass' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'properMass' for 'restMass')
    (comment)
    (attribute_usage 'restEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'atomicMass' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'nuclidicMass' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'unifiedAtomicMassConstant' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'elementaryCharge' : 'ElectricChargeValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ChargeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'chargeNumber' : 'ChargeNumberValue' :> 'scalarQuantities')
    (alias_member 'ionizationNumber' for 'chargeNumber')
    (comment)
    (attribute_usage 'bohrRadius' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'RydbergConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RydbergConstantUnit' multiplicity))
    (attribute_usage 'rydbergConstant' : 'RydbergConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RydbergConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'HartreeEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'HartreeEnergyUnit' multiplicity))
    (attribute_usage 'hartreeEnergy' : 'HartreeEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'HartreeEnergyUnit' :> 'DerivedUnit'
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
    (attribute_def 'MagneticDipoleMomentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticDipoleMomentUnit' multiplicity))
    (attribute_usage 'magneticDipoleMoment' : 'MagneticDipoleMomentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticDipoleMomentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticDipoleMoment3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticDipoleMoment3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticDipoleMoment3dVector' : 'CartesianMagneticDipoleMoment3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticDipoleMoment3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticDipoleMomentUnit' multiplicity))
    (comment)
    (attribute_usage 'bohrMagneton' : 'MagneticDipoleMomentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'nuclearMagneton' : 'MagneticDipoleMomentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'SpinValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpinUnit' multiplicity))
    (attribute_usage 'spin' : 'SpinValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpinUnit' :> 'DerivedUnit'
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
    (attribute_def 'CartesianSpin3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpin3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianSpin3dVector' : 'CartesianSpin3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianSpin3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'SpinUnit' multiplicity))
    (comment)
    (attribute_def 'TotalAngularMomentumValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TotalAngularMomentumUnit' multiplicity))
    (attribute_usage 'totalAngularMomentum' : 'TotalAngularMomentumValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TotalAngularMomentumUnit' :> 'DerivedUnit'
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
    (attribute_def 'CartesianTotalAngularMomentum3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianTotalAngularMomentum3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianTotalAngularMomentum3dVector' : 'CartesianTotalAngularMomentum3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianTotalAngularMomentum3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'TotalAngularMomentumUnit' multiplicity))
    (comment)
    (attribute_def 'GyromagneticRatioValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'GyromagneticRatioUnit' multiplicity))
    (attribute_usage 'gyromagneticRatio' : 'GyromagneticRatioValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'GyromagneticRatioUnit' :> 'DerivedUnit'
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
    (alias_member 'MagnetogyricRatioUnit' for 'GyromagneticRatioUnit')
    (alias_member 'MagnetogyricRatioValue' for 'GyromagneticRatioValue')
    (alias_member 'magnetogyricRatio' for 'gyromagneticRatio')
    (alias_member 'GyromagneticCoefficientUnit' for 'GyromagneticRatioUnit')
    (alias_member 'GyromagneticCoefficientValue' for 'GyromagneticRatioValue')
    (alias_member 'gyromagneticCoefficient' for 'gyromagneticRatio')
    (comment)
    (attribute_def 'GyromagneticRatioOfTheElectronValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'GyromagneticRatioOfTheElectronUnit' multiplicity))
    (attribute_usage 'gyromagneticRatioOfTheElectron' : 'GyromagneticRatioOfTheElectronValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'GyromagneticRatioOfTheElectronUnit' :> 'DerivedUnit'
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
    (alias_member 'MagnetogyricRatioOfTheElectronUnit' for 'GyromagneticRatioOfTheElectronUnit')
    (alias_member 'MagnetogyricRatioOfTheElectronValue' for 'GyromagneticRatioOfTheElectronValue')
    (alias_member 'magnetogyricRatioOfTheElectron' for 'gyromagneticRatioOfTheElectron')
    (alias_member 'GyromagneticCoefficientOfTheElectronUnit' for 'GyromagneticRatioOfTheElectronUnit')
    (alias_member 'GyromagneticCoefficientOfTheElectronValue' for 'GyromagneticRatioOfTheElectronValue')
    (alias_member 'gyromagneticCoefficientOfTheElectron' for 'gyromagneticRatioOfTheElectron')
    (comment)
    (attribute_def 'QuantumNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'quantumNumber' : 'QuantumNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'principalQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'orbitalAngularMomentumQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'magneticQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'spinQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'totalAngularMomentumQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'nuclearSpinQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'hyperfineStructureQuantumNumber' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'LandeFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'landeFactor' : 'LandeFactorValue' :> 'scalarQuantities')
    (alias_member 'gFactorOfAtom' for 'landeFactor')
    (comment)
    (attribute_def 'GFactorOfNucleusOrNuclearParticleValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'gFactorOfNucleusOrNuclearParticle' : 'GFactorOfNucleusOrNuclearParticleValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'larmorAngularFrequency' : 'AngularFrequencyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'LarmorFrequencyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LarmorFrequencyUnit' multiplicity))
    (attribute_usage 'larmorFrequency' : 'LarmorFrequencyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LarmorFrequencyUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'nuclearPrecessionAngularFrequency' : 'AngularFrequencyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'cyclotronAngularFrequency' : 'AngularFrequencyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'gyroradius' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'larmorRadius' for 'gyroradius')
    (comment)
    (attribute_def 'NuclearQuadrupoleMomentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'NuclearQuadrupoleMomentUnit' multiplicity))
    (attribute_usage 'nuclearQuadrupoleMoment' : 'NuclearQuadrupoleMomentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'NuclearQuadrupoleMomentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'nuclearRadius' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'electronRadius' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'comptonWavelength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'massExcess' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'massDefect' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'RelativeMassExcessValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeMassExcess' : 'RelativeMassExcessValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RelativeMassDefectValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeMassDefect' : 'RelativeMassDefectValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PackingFractionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'packingFraction' : 'PackingFractionValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BindingFractionValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bindingFraction' : 'BindingFractionValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'DecayConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DecayConstantUnit' multiplicity))
    (attribute_usage 'decayConstant' : 'DecayConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DecayConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'DisintegrationConstantUnit' for 'DecayConstantUnit')
    (alias_member 'DisintegrationConstantValue' for 'DecayConstantValue')
    (alias_member 'disintegrationConstant' for 'decayConstant')
    (comment)
    (attribute_usage 'meanDurationOfLife' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'meanLifeTime' for 'meanDurationOfLife')
    (comment)
    (attribute_usage 'levelWidth' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'NuclearActivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'NuclearActivityUnit' multiplicity))
    (attribute_usage 'nuclearActivity' : 'NuclearActivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'NuclearActivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificActivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificActivityUnit' multiplicity))
    (attribute_usage 'specificActivity' : 'SpecificActivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificActivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'MassicActivityUnit' for 'SpecificActivityUnit')
    (alias_member 'MassicActivityValue' for 'SpecificActivityValue')
    (alias_member 'massicActivity' for 'specificActivity')
    (comment)
    (attribute_def 'ActivityDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ActivityDensityUnit' multiplicity))
    (attribute_usage 'activityDensity' : 'ActivityDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ActivityDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'VolumicActivityUnit' for 'ActivityDensityUnit')
    (alias_member 'VolumicActivityValue' for 'ActivityDensityValue')
    (alias_member 'volumicActivity' for 'activityDensity')
    (alias_member 'ActivityConcentrationUnit' for 'ActivityDensityUnit')
    (alias_member 'ActivityConcentrationValue' for 'ActivityDensityValue')
    (alias_member 'activityConcentration' for 'activityDensity')
    (comment)
    (attribute_def 'SurfaceActivityDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SurfaceActivityDensityUnit' multiplicity))
    (attribute_usage 'surfaceActivityDensity' : 'SurfaceActivityDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SurfaceActivityDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'halfLife' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'alphaDisintegrationEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'maximumBetaParticleEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'betaDisintegrationEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'InternalConversionFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'internalConversionFactor' : 'InternalConversionFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ParticleEmissionRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleEmissionRateUnit' multiplicity))
    (attribute_usage 'particleEmissionRate' : 'ParticleEmissionRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleEmissionRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'reactionEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'resonanceEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'crossSection' : 'AreaValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'totalCrossSection' : 'AreaValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'DirectionDistributionOfCrossSectionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DirectionDistributionOfCrossSectionUnit' multiplicity))
    (attribute_usage 'directionDistributionOfCrossSection' : 'DirectionDistributionOfCrossSectionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DirectionDistributionOfCrossSectionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'EnergyDistributionOfCrossSectionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EnergyDistributionOfCrossSectionUnit' multiplicity))
    (attribute_usage 'energyDistributionOfCrossSection' : 'EnergyDistributionOfCrossSectionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EnergyDistributionOfCrossSectionUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'DirectionAndEnergyDistributionOfCrossSectionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DirectionAndEnergyDistributionOfCrossSectionUnit' multiplicity))
    (attribute_usage 'directionAndEnergyDistributionOfCrossSection' : 'DirectionAndEnergyDistributionOfCrossSectionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DirectionAndEnergyDistributionOfCrossSectionUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'VolumicCrossSectionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'VolumicCrossSectionUnit' multiplicity))
    (attribute_usage 'volumicCrossSection' : 'VolumicCrossSectionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'VolumicCrossSectionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'MacroscopicCrossSectionUnit' for 'VolumicCrossSectionUnit')
    (alias_member 'MacroscopicCrossSectionValue' for 'VolumicCrossSectionValue')
    (alias_member 'macroscopicCrossSection' for 'volumicCrossSection')
    (comment)
    (attribute_def 'VolumicTotalCrossSectionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'VolumicTotalCrossSectionUnit' multiplicity))
    (attribute_usage 'volumicTotalCrossSection' : 'VolumicTotalCrossSectionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'VolumicTotalCrossSectionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'MacroscopicTotalCrossSectionUnit' for 'VolumicTotalCrossSectionUnit')
    (alias_member 'MacroscopicTotalCrossSectionValue' for 'VolumicTotalCrossSectionValue')
    (alias_member 'macroscopicTotalCrossSection' for 'volumicTotalCrossSection')
    (comment)
    (attribute_def 'ParticleFluenceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleFluenceUnit' multiplicity))
    (attribute_usage 'particleFluence' : 'ParticleFluenceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleFluenceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ParticleFluenceRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleFluenceRateUnit' multiplicity))
    (attribute_usage 'particleFluenceRate' : 'ParticleFluenceRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleFluenceRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'radiantEnergyForIonizingRadiation' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'EnergyFluenceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EnergyFluenceUnit' multiplicity))
    (attribute_usage 'energyFluence' : 'EnergyFluenceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EnergyFluenceUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'EnergyFluenceRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EnergyFluenceRateUnit' multiplicity))
    (attribute_usage 'energyFluenceRate' : 'EnergyFluenceRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EnergyFluenceRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ParticleCurrentDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleCurrentDensityUnit' multiplicity))
    (attribute_usage 'particleCurrentDensity' : 'ParticleCurrentDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleCurrentDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianParticleCurrentDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianParticleCurrentDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianParticleCurrentDensity3dVector' : 'CartesianParticleCurrentDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianParticleCurrentDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ParticleCurrentDensityUnit' multiplicity))
    (comment)
    (attribute_def 'LinearAttenuationCoefficientForIonizingRadiationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearAttenuationCoefficientForIonizingRadiationUnit' multiplicity))
    (attribute_usage 'linearAttenuationCoefficientForIonizingRadiation' : 'LinearAttenuationCoefficientForIonizingRadiationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearAttenuationCoefficientForIonizingRadiationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassAttenuationCoefficientForIonizingRadiationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassAttenuationCoefficientForIonizingRadiationUnit' multiplicity))
    (attribute_usage 'massAttenuationCoefficientForIonizingRadiation' : 'MassAttenuationCoefficientForIonizingRadiationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassAttenuationCoefficientForIonizingRadiationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarAttenuationCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarAttenuationCoefficientUnit' multiplicity))
    (attribute_usage 'molarAttenuationCoefficient' : 'MolarAttenuationCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarAttenuationCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AtomicAttenuationCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AtomicAttenuationCoefficientUnit' multiplicity))
    (attribute_usage 'atomicAttenuationCoefficient' : 'AtomicAttenuationCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AtomicAttenuationCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'halfValueThickness' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'TotalLinearStoppingPowerValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TotalLinearStoppingPowerUnit' multiplicity))
    (attribute_usage 'totalLinearStoppingPower' : 'TotalLinearStoppingPowerValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TotalLinearStoppingPowerUnit' :> 'DerivedUnit'
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
    (alias_member 'LinearStoppingPowerUnit' for 'TotalLinearStoppingPowerUnit')
    (alias_member 'LinearStoppingPowerValue' for 'TotalLinearStoppingPowerValue')
    (alias_member 'linearStoppingPower' for 'totalLinearStoppingPower')
    (comment)
    (attribute_def 'TotalMassStoppingPowerValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TotalMassStoppingPowerUnit' multiplicity))
    (attribute_usage 'totalMassStoppingPower' : 'TotalMassStoppingPowerValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TotalMassStoppingPowerUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'MassStoppingPowerUnit' for 'TotalMassStoppingPowerUnit')
    (alias_member 'MassStoppingPowerValue' for 'TotalMassStoppingPowerValue')
    (alias_member 'massStoppingPower' for 'totalMassStoppingPower')
    (comment)
    (attribute_usage 'meanLinearRange' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'MeanMassRangeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MeanMassRangeUnit' multiplicity))
    (attribute_usage 'meanMassRange' : 'MeanMassRangeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MeanMassRangeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LinearIonizationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearIonizationUnit' multiplicity))
    (attribute_usage 'linearIonization' : 'LinearIonizationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearIonizationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'TotalIonizationValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'totalIonization' : 'TotalIonizationValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AverageEnergyLossPerElementaryChargeProducedValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AverageEnergyLossPerElementaryChargeProducedUnit' multiplicity))
    (attribute_usage 'averageEnergyLossPerElementaryChargeProduced' : 'AverageEnergyLossPerElementaryChargeProducedValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AverageEnergyLossPerElementaryChargeProducedUnit' :> 'DerivedUnit'
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
    (attribute_def 'MobilityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MobilityUnit' multiplicity))
    (attribute_usage 'mobility' : 'MobilityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MobilityUnit' :> 'DerivedUnit'
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
    (attribute_def 'ParticleNumberDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleNumberDensityUnit' multiplicity))
    (attribute_usage 'particleNumberDensity' : 'ParticleNumberDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleNumberDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'IonNumberDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IonNumberDensityUnit' multiplicity))
    (attribute_usage 'ionNumberDensity' : 'IonNumberDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IonNumberDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'IonDensityUnit' for 'IonNumberDensityUnit')
    (alias_member 'IonDensityValue' for 'IonNumberDensityValue')
    (alias_member 'ionDensity' for 'ionNumberDensity')
    (comment)
    (attribute_def 'RecombinationCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RecombinationCoefficientUnit' multiplicity))
    (attribute_usage 'recombinationCoefficient' : 'RecombinationCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RecombinationCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (comment)
    (alias_member 'DiffusionCoefficientForParticleNumberDensityUnit' for 'DiffusionCoefficientUnit')
    (alias_member 'DiffusionCoefficientForParticleNumberDensityValue' for 'DiffusionCoefficientValue')
    (alias_member 'diffusionCoefficientForParticleNumberDensity' for 'diffusionCoefficient')
    (comment)
    (attribute_usage 'diffusionCoefficientForFluenceRate' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ParticleSourceDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ParticleSourceDensityUnit' multiplicity))
    (attribute_usage 'particleSourceDensity' : 'ParticleSourceDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ParticleSourceDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SlowingDownDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SlowingDownDensityUnit' multiplicity))
    (attribute_usage 'slowingDownDensity' : 'SlowingDownDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SlowingDownDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ResonanceEscapeProbabilityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'resonanceEscapeProbability' : 'ResonanceEscapeProbabilityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LethargyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lethargy' : 'LethargyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AverageLogarithmicEnergyDecrementValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'averageLogarithmicEnergyDecrement' : 'AverageLogarithmicEnergyDecrementValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'meanFreePathForAtomicPhysics' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'slowingDownArea' : 'AreaValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'diffusionArea' : 'AreaValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'migrationArea' : 'AreaValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'slowingDownLength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'diffusionLength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'migrationLength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'neutronYieldPerFission' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'neutronYieldPerAbsorption' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'FastFissionFactorValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'FastFissionFactorUnit' multiplicity))
    (attribute_usage 'fastFissionFactor' : 'FastFissionFactorValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'FastFissionFactorUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'ThermalUtilizationFactorValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalUtilizationFactorUnit' multiplicity))
    (attribute_usage 'thermalUtilizationFactor' : 'ThermalUtilizationFactorValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalUtilizationFactorUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'NonLeakageProbabilityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'NonLeakageProbabilityUnit' multiplicity))
    (attribute_usage 'nonLeakageProbability' : 'NonLeakageProbabilityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'NonLeakageProbabilityUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'MultiplicationFactorValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MultiplicationFactorUnit' multiplicity))
    (attribute_usage 'multiplicationFactor' : 'MultiplicationFactorValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MultiplicationFactorUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'InfiniteMultiplicationFactorValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'InfiniteMultiplicationFactorUnit' multiplicity))
    (attribute_usage 'infiniteMultiplicationFactor' : 'InfiniteMultiplicationFactorValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'InfiniteMultiplicationFactorUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_usage 'reactorTimeConstant' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'energyImparted' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'meanEnergyImparted' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'AbsorbedDoseValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AbsorbedDoseUnit' multiplicity))
    (attribute_usage 'absorbedDose' : 'AbsorbedDoseValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AbsorbedDoseUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'specificEnergyImparted' : 'AbsorbedDoseValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'QualityFactorForIonizingRadiationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'QualityFactorForIonizingRadiationUnit' multiplicity))
    (attribute_usage 'qualityFactorForIonizingRadiation' : 'QualityFactorForIonizingRadiationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'QualityFactorForIonizingRadiationUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'DoseEquivalentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DoseEquivalentUnit' multiplicity))
    (attribute_usage 'doseEquivalent' : 'DoseEquivalentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DoseEquivalentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'doseEquivalentRate' : 'DoseEquivalentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'AbsorbedDoseRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AbsorbedDoseRateUnit' multiplicity))
    (attribute_usage 'absorbedDoseRate' : 'AbsorbedDoseRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AbsorbedDoseRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LinearEnergyTransferValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearEnergyTransferUnit' multiplicity))
    (attribute_usage 'linearEnergyTransfer' : 'LinearEnergyTransferValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearEnergyTransferUnit' :> 'DerivedUnit'
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
    (attribute_def 'KermaValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'KermaUnit' multiplicity))
    (attribute_usage 'kerma' : 'KermaValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'KermaUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'KermaRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'KermaRateUnit' multiplicity))
    (attribute_usage 'kermaRate' : 'KermaRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'KermaRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassEnergyTransferCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassEnergyTransferCoefficientUnit' multiplicity))
    (attribute_usage 'massEnergyTransferCoefficient' : 'MassEnergyTransferCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassEnergyTransferCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ExposureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ExposureUnit' multiplicity))
    (attribute_usage 'exposure' : 'ExposureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ExposureUnit' :> 'DerivedUnit'
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
    (attribute_def 'ExposureRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ExposureRateUnit' multiplicity))
    (attribute_usage 'exposureRate' : 'ExposureRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ExposureRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))))
~~~
# FORMAT
~~~sysml
standard library package ISQAtomicNuclear {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-10:2019 "Atomic and nuclear physics"
     * see also https://www.iso.org/standard/64980.html
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
    private import ISQChemistryMolecular::DiffusionCoefficientUnit;
    private import ISQChemistryMolecular::DiffusionCoefficientValue;
    private import ISQChemistryMolecular::diffusionCoefficient;
    private import ISQElectromagnetism::ElectricChargeValue;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AreaValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-10 item 10-1.1 atomic number, proton number */
    attribute atomicNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.1 atomic number, proton number
         * symbol(s): `Z`
         * application domain: generic
         * name: AtomicNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of protons in an atomic nucleus
         * remarks: A nuclide is a species of atom with specified numbers of protons and neutrons. Nuclides with the same value of `Z` but different values of `N` are called isotopes of an element. The ordinal number of an element in the periodic table is equal to the atomic number. The atomic number equals the quotient of the charge (IEC 80000-6) of the nucleus and the elementary charge (ISO 80000-1).
         */
    }

    alias protonNumber for atomicNumber;

    /* ISO-80000-10 item 10-1.2 neutron number */
    attribute neutronNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.2 neutron number
         * symbol(s): `N`
         * application domain: generic
         * name: NeutronNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of neutrons in an atomic nucleus
         * remarks: Nuclides with the same value of `N` but different values of `Z` are called isotones. `N - Z` is called the neutron excess number.
         */
    }

    /* ISO-80000-10 item 10-1.3 nucleon number, mass number */
    attribute nucleonNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.3 nucleon number, mass number
         * symbol(s): `A`
         * application domain: generic
         * name: NucleonNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of nucleons in an atomic nucleus
         * remarks: `A` = `Z` + `N` Nuclides with the same value of `A` are called isobars.
         */
    }

    alias massNumber for nucleonNumber;

    /* ISO-80000-10 item 10-2 rest mass, proper mass */
    attribute restMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-2 rest mass, proper mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: RestMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: for particle X, mass (ISO 80000-4) of that particle at rest in an inertial frame
         * remarks: EXAMPLE `m(H_2O)` for a water molecule, `m_e` for an electron. Rest mass is often denoted `m_0`. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    alias properMass for restMass;

    /* ISO-80000-10 item 10-3 rest energy */
    attribute restEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-3 rest energy
         * symbol(s): `E_0`
         * application domain: generic
         * name: RestEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy `E_0` (ISO 80000-5) of a particle at rest: `E_0 = m_0 c_0^2` where `m_0` is the rest mass (item 10-2) of that particle, and `c_0` is speed of light in vacuum (ISO 80000-1)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-4.1 atomic mass */
    attribute atomicMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.1 atomic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: AtomicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of an atom X in the ground state
         * remarks: `m(X)/m_u` is called the relative atomic mass. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-4.2 nuclidic mass */
    attribute nuclidicMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.2 nuclidic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: NuclidicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of a nuclide X in the ground state
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-4.3 unified atomic mass constant */
    attribute unifiedAtomicMassConstant: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.3 unified atomic mass constant
         * symbol(s): `m_u`
         * application domain: generic
         * name: UnifiedAtomicMassConstant (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: 1/12 of the mass (ISO 80000-4) of an atom of the nuclide ^(12)C in the ground state at rest
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-5.1 elementary charge */
    attribute elementaryCharge: ElectricChargeValue :> scalarQuantities {
        doc
        /*
         * source: item 10-5.1 elementary charge
         * symbol(s): `e`
         * application domain: generic
         * name: ElementaryCharge (specializes ElectricCharge)
         * quantity dimension: T^1*I^1
         * measurement unit(s): C, s*A
         * tensor order: 0
         * definition: one of the fundamental constants in the SI system (ISO 80000-1), equal to the charge of the proton and opposite to the charge of the electron
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-5.2 charge number, ionization number */
    attribute def ChargeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-5.2 charge number, ionization number
         * symbol(s): `c`
         * application domain: generic
         * name: ChargeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a particle, quotient of the electric charge (IEC 80000-6) and the elementary charge (ISO 80000-1)
         * remarks: A particle is said to be electrically neutral if its charge number is equal to zero. The charge number of a particle can be positive, negative, or zero. The state of charge of a particle may be presented as a superscript to the symbol of that particle, e.g. `H^+, He^(++), Al^(3+), Cl^-, S^(--), N^(3-)`.
         */
    }
    attribute chargeNumber: ChargeNumberValue :> scalarQuantities;

    alias ionizationNumber for chargeNumber;

    /* ISO-80000-10 item 10-6 Bohr radius */
    attribute bohrRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-6 Bohr radius
         * symbol(s): `a_0`
         * application domain: generic
         * name: BohrRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m, Å
         * tensor order: 0
         * definition: radius (ISO 80000-3) of the electron orbital in the hydrogen atom in its ground state in the Bohr model of the atom: `a_0 = (4 π ε_0 ℏ^2)/(m_e e^2)` where `ε_0` is the electric constant (IEC 80000-6), `ℏ` is the reduced Planck constant (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `e` is the elementary charge (ISO 80000-1)
         * remarks: The radius of the electron orbital in the H atom in its ground state is `a_0` in the Bohr model of the atom. ångström (Å), `1 Å := 10^-10 m`.
         */
    }

    /* ISO-80000-10 item 10-7 Rydberg constant */
    attribute def RydbergConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-7 Rydberg constant
         * symbol(s): `R_∞`
         * application domain: generic
         * name: RydbergConstant
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: spectroscopic constant that determines the wave numbers of the lines in the spectrum of hydrogen: `R_(oo) = e^2/(8 π ε_0 a_0 h c_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `a_0` is the Bohr radius (item 10-6), `h` is the Planck constant (ISO 80000-1), and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The quantity `R_y = R_∞ h c_0` is called the Rydberg energy.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RydbergConstantUnit[1];
    }

    attribute rydbergConstant: RydbergConstantValue[*] nonunique :> scalarQuantities;

    attribute def RydbergConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-8 Hartree energy */
    attribute def HartreeEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-8 Hartree energy
         * symbol(s): `E_H`, `E_h`
         * application domain: generic
         * name: HartreeEnergy
         * quantity dimension: L^6*M^3*T^-6
         * measurement unit(s): eV*J*kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) of the electron in a hydrogen atom in its ground state: `E_H = e^2/(4 π ε_0 a_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), and `a_0` is the Bohr radius (item 10-6)
         * remarks: The energy of the electron in an H atom in its ground state is `E_H`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HartreeEnergyUnit[1];
    }

    attribute hartreeEnergy: HartreeEnergyValue[*] nonunique :> scalarQuantities;

    attribute def HartreeEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 6; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -6; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-9.1 magnetic dipole moment */
    attribute def MagneticDipoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (magnitude)
         * symbol(s): `μ`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticDipoleMomentUnit[1];
    }

    attribute magneticDipoleMoment: MagneticDipoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def MagneticDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (vector)
         * symbol(s): `vec(μ)`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 1
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticDipoleMoment3dCoordinateFrame[1];
    }

    attribute cartesianMagneticDipoleMoment3dVector: CartesianMagneticDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticDipoleMomentUnit[3];
    }

    /* ISO-80000-10 item 10-9.2 Bohr magneton */
    attribute bohrMagneton: MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.2 Bohr magneton
         * symbol(s): `μ_B`
         * application domain: generic
         * name: BohrMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: magnitude of the magnetic moment of an electron in a state with orbital angular momentum quantum number `l`=1 (item 10-13.3) due to its orbital motion: `μ_B = (e ℏ)/(2 m_e)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_e` is the rest mass (item 10-2) of electron
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-9.3 nuclear magneton */
    attribute nuclearMagneton: MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.3 nuclear magneton
         * symbol(s): `μ_N`
         * application domain: generic
         * name: NuclearMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: absolute value of the magnetic moment of a nucleus: `μ_N = (e ℏ)/(2 m_p)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_p` is the rest mass (item 10-2) of proton
         * remarks: Subscript N stands for nucleus. For the neutron magnetic moment, subscript n is used. The magnetic moments of protons and neutrons differ from this quantity by their specific `g` factors (item 10-14.2).
         */
    }

    /* ISO-80000-10 item 10-10 spin */
    attribute def SpinValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-10 spin (magnitude)
         * symbol(s): `s`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpinUnit[1];
    }

    attribute spin: SpinValue[*] nonunique :> scalarQuantities;

    attribute def SpinUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianSpin3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-10 spin (vector)
         * symbol(s): `vec(s)`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpin3dCoordinateFrame[1];
    }

    attribute cartesianSpin3dVector: CartesianSpin3dVector :> vectorQuantities;

    attribute def CartesianSpin3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SpinUnit[3];
    }

    /* ISO-80000-10 item 10-11 total angular momentum */
    attribute def TotalAngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-11 total angular momentum (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalAngularMomentumUnit[1];
    }

    attribute totalAngularMomentum: TotalAngularMomentumValue[*] nonunique :> scalarQuantities;

    attribute def TotalAngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianTotalAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-11 total angular momentum (vector)
         * symbol(s): `vec(J)`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianTotalAngularMomentum3dCoordinateFrame[1];
    }

    attribute cartesianTotalAngularMomentum3dVector: CartesianTotalAngularMomentum3dVector :> vectorQuantities;

    attribute def CartesianTotalAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: TotalAngularMomentumUnit[3];
    }

    /* ISO-80000-10 item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient */
    attribute def GyromagneticRatioValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: GyromagneticRatio
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum: `vec(μ)` = `γ` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1 The systematic name is "gyromagnetic coefficient", but "gyromagnetic ratio" is more usual. The gyromagnetic ratio of the proton is denoted by `γ_p`. The gyromagnetic ratio of the neutron is denoted by `γ_n`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: GyromagneticRatioUnit[1];
    }

    attribute gyromagneticRatio: GyromagneticRatioValue[*] nonunique :> scalarQuantities;

    attribute def GyromagneticRatioUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    alias MagnetogyricRatioUnit for GyromagneticRatioUnit;
    alias MagnetogyricRatioValue for GyromagneticRatioValue;
    alias magnetogyricRatio for gyromagneticRatio;

    alias GyromagneticCoefficientUnit for GyromagneticRatioUnit;
    alias GyromagneticCoefficientValue for GyromagneticRatioValue;
    alias gyromagneticCoefficient for gyromagneticRatio;

    /* ISO-80000-10 item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron */
    attribute def GyromagneticRatioOfTheElectronValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron
         * symbol(s): `γ_e`
         * application domain: generic
         * name: GyromagneticRatioOfTheElectron
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum of the electron `vec(μ)` = `γ_e` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1
         */
        attribute :>> num: Real;
        attribute :>> mRef: GyromagneticRatioOfTheElectronUnit[1];
    }

    attribute gyromagneticRatioOfTheElectron: GyromagneticRatioOfTheElectronValue[*] nonunique :> scalarQuantities;

    attribute def GyromagneticRatioOfTheElectronUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    alias MagnetogyricRatioOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias MagnetogyricRatioOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias magnetogyricRatioOfTheElectron for gyromagneticRatioOfTheElectron;

    alias GyromagneticCoefficientOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias GyromagneticCoefficientOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias gyromagneticCoefficientOfTheElectron for gyromagneticRatioOfTheElectron;

    /* ISO-80000-10 item 10-13.1 quantum number */
    attribute def QuantumNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-13.1 quantum number
         * symbol(s): `N`, `L`, `M`, `j`, `s`, `F`
         * application domain: generic
         * name: QuantumNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number describing a particular state of a quantum system
         * remarks: Electron states determine the binding energy `E = E(n,l,m,j,s,f)` in an atom. Upper case letters `N, L, M, J, S, F` are usually used for the whole system. The spatial probability distribution of an electron is given by `|Ψ|^2`, where `Ψ` is its wave function. For an electron in an H atom in a non-relativistic approximation, the wave function can be presented as: `Ψ(r,θ,φ) = R_(nl)(r)*Y_l^m(θ,φ)`, where `r,θ,φ` are spherical coordinates (ISO 80000-2) with respect to the nucleus and to a given (quantization) axis, `R_(nl)(r)` is the radial distribution function, and `Y_l^m(θ,φ)` are spherical harmonics. In the Bohr model of one-electron atoms, `n`, `l`, and `m` define the possible orbits of an electron about the nucleus.
         */
    }
    attribute quantumNumber: QuantumNumberValue :> scalarQuantities;

    /* ISO-80000-10 item 10-13.2 principal quantum number */
    attribute principalQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.2 principal quantum number
         * symbol(s): `n`
         * application domain: generic
         * name: PrincipalQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the number `n`-1 of radial nodes of one-electron wave functions
         * remarks: In the Bohr model, `n = 1,2,…,∞` is related to the binding energy of an electron and the radius of spherical orbits (principal axis of the elliptic orbits). For an electron in an H atom, the semi-classical radius of its orbit is `r_n = a_0 n^2` and its binding energy is `E_n = E_H/n^2`.
         */
    }

    /* ISO-80000-10 item 10-13.3 orbital angular momentum quantum number */
    attribute orbitalAngularMomentumQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.3 orbital angular momentum quantum number
         * symbol(s): `l`, `l_i`, `L`
         * application domain: generic
         * name: OrbitalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the orbital angular momentum `l` of a one-electron state
         * remarks: `abs(l)^2 = ℏ^2 l (l-1)` , `l = 0, 1, …, n-1` where `vec(l)` is the orbital angular momentum and `ℏ` is the reduced Planck constant (ISO 80000-1). If reference is made to a specific particle `i`, the symbol `l_i` is used instead of `l`; if reference is made to the whole system, the symbol `L` is used instead of `l`. An electron in an H atom for `l = 0` appears as a spherical cloud. In the Bohr model, it is related to the form of the orbit.
         */
    }

    /* ISO-80000-10 item 10-13.4 magnetic quantum number */
    attribute magneticQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.4 magnetic quantum number
         * symbol(s): `m`, `m_i`, `M`
         * application domain: generic
         * name: MagneticQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the `z` component `l_z`, `j_z` or `s_z`, of the orbital, total, or spin angular momentum
         * remarks: `l_z = m_l ℏ` , `j_z = m_j ℏ` , and `s_z = m_s ℏ` , with the ranges from `-l` to `l`, from `-j` to `j`, and `±1/2`, respectively. `m_i` refers to a specific particle `i`. `M` is used for the whole system. Subscripts `l`, `s`, `j`, etc., as appropriate, indicate the angular momentum involved. `ℏ` is the reduced Planck constant (ISO 80000-1).
         */
    }

    /* ISO-80000-10 item 10-13.5 spin quantum number */
    attribute spinQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.5 spin quantum number
         * symbol(s): `s`
         * application domain: generic
         * name: SpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characteristic quantum number `s` of a particle, related to its spin (item 10-10), `vec(s)`: `s^2 = ℏ^2 s (s+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Spin quantum numbers of fermions are odd multiples of 1/2, and those of bosons are integers.
         */
    }

    /* ISO-80000-10 item 10-13.6 total angular momentum quantum number */
    attribute totalAngularMomentumQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.6 total angular momentum quantum number
         * symbol(s): `j`, `j_i`, `J`
         * application domain: generic
         * name: TotalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number in an atom describing the magnitude of total angular momentum `vec(J)` (item 10-11)
         * remarks: `j_i` refers to a specific particle `i`; `J` is used for the whole system. The quantum number `J` and the magnitude of total angular momentum `vec(J)` (item 10-11) are different quantities. The two values of `j` are `l`±1/2. (See item 10-13.3.)
         */
    }

    /* ISO-80000-10 item 10-13.7 nuclear spin quantum number */
    attribute nuclearSpinQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.7 nuclear spin quantum number
         * symbol(s): `I`
         * application domain: generic
         * name: NuclearSpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number related to the total angular momentum (item 10-11), `vec(J)`, of a nucleus in any specified state, normally called nuclear spin: `vec(J)^2 = ℏ^2 I (I+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Nuclear spin is composed of spins of the nucleons (protons and neutrons) and their (orbital) motions. In principle there is no upper limit for the nuclear spin quantum number. It has possible values `I` = 0,1,2,… for even `A` and `I = 1/2, 3/2, …` for odd `A`. In nuclear and particle physics, `vec(J)` is often used.
         */
    }

    /* ISO-80000-10 item 10-13.8 hyperfine structure quantum number */
    attribute hyperfineStructureQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.8 hyperfine structure quantum number
         * symbol(s): `F`
         * application domain: generic
         * name: HyperfineStructureQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number of an atom describing the inclination of the nuclear spin with respect to a quantization axis given by the magnetic field produced by the orbital electrons
         * remarks: The interval of `F` is │`I`-`J`│, │`I`-`J`│+1, ..., `I`-`J`. This is related to the hyperfine splitting of the atomic energy levels due to the interaction between the electron and nuclear magnetic moments.
         */
    }

    /* ISO-80000-10 item 10-14.1 Lande factor, g factor of atom */
    attribute def LandeFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.1 Lande factor, g factor of atom
         * symbol(s): `g`
         * application domain: generic
         * name: LandeFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the total angular momentum quantum number and the Bohr magneton: `g = μ/(J*μ_B)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `J` is total angular momentum quantum number (item 10-13.6), and `μ_B` is the Bohr magneton (item 10-9.2)
         * remarks: These quantities are also called `g` values. The Landé factor can be calculated from the expression: `g(L, S, J) = 1 + (g_e -1) xx (J(J+1) + S(S+1) - L(L+1))/(2J(J+1))` where `g_e` is the` g` factor of the electron.
         */
    }
    attribute landeFactor: LandeFactorValue :> scalarQuantities;

    alias gFactorOfAtom for landeFactor;

    /* ISO-80000-10 item 10-14.2 g factor of nucleus or nuclear particle */
    attribute def GFactorOfNucleusOrNuclearParticleValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.2 g factor of nucleus or nuclear particle
         * symbol(s): `g`
         * application domain: generic
         * name: GFactorOfNucleusOrNuclearParticle (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the nuclear spin quantum number and the nuclear magneton: `g = μ/(I*μ_N)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `I` is nuclear spin quantum number (item 10-13.7), and `μ_N` is the nuclear magneton (item 10-9.3)
         * remarks: The `g` factors for nuclei or nucleons are known from measurements.
         */
    }
    attribute gFactorOfNucleusOrNuclearParticle: GFactorOfNucleusOrNuclearParticleValue :> scalarQuantities;

    /* ISO-80000-10 item 10-15.1 Larmor angular frequency */
    attribute larmorAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.1 Larmor angular frequency
         * symbol(s): `ω_L`
         * application domain: generic
         * name: LarmorAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: angular frequency (ISO 80000-3) of the electron angular momentum (ISO 80000-4) vector precession about the axis of an external magnetic field: `ω_L = e/(2 m_e) B` where `e` is the elementary charge (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-15.2 Larmor frequency */
    attribute def LarmorFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-15.2 Larmor frequency
         * symbol(s): `ν_L`
         * application domain: generic
         * name: LarmorFrequency
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of Larmor angular frequency (ISO 80000-3) and 2π
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LarmorFrequencyUnit[1];
    }

    attribute larmorFrequency: LarmorFrequencyValue[*] nonunique :> scalarQuantities;

    attribute def LarmorFrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-15.3 nuclear precession angular frequency */
    attribute nuclearPrecessionAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.3 nuclear precession angular frequency
         * symbol(s): `ω_N`
         * application domain: generic
         * name: NuclearPrecessionAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: frequency (ISO 80000-3) by which the nucleus angular momentum vector (ISO 80000-4) precesses about the axis of an external magnetic field: `ω_N` = `γ` `B` where `γ` is the gyromagnetic ratio (item 10-12.1), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-16 cyclotron angular frequency */
    attribute cyclotronAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-16 cyclotron angular frequency
         * symbol(s): `ω_c`
         * application domain: generic
         * name: CyclotronAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: quotient of the product of the electric charge of a particle and the magnitude of the magnetic flux density of the magnetic field, and the particle mass: `ω_c = abs(q)/m B` where `q` is the electric charge (IEC 80000-6) of the particle, `m` is the mass (ISO 80000-4) of the particle, and `B` is the absolute value of the magnetic flux density (IEC 80000-6)
         * remarks: The quantity `v_c` = `ω_c`/2π is called the cyclotron frequency.
         */
    }

    /* ISO-80000-10 item 10-17 gyroradius, Larmor radius */
    attribute gyroradius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-17 gyroradius, Larmor radius
         * symbol(s): `r_g`, `r_L`
         * application domain: generic
         * name: Gyroradius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (ISO 80000-3) of circular movement of a particle with mass (ISO 80000-4), velocity `vec(v)` (ISO 80000-3), and electric charge `q` (IEC 80000-6), moving in a magnetic field with magnetic flux density `vec(B)` (IEC 80000-6): `r_g = (m abs(vec(v) xx vec(B)))/(q B^2)`
         * remarks: None.
         */
    }

    alias larmorRadius for gyroradius;

    /* ISO-80000-10 item 10-18 nuclear quadrupole moment */
    attribute def NuclearQuadrupoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-18 nuclear quadrupole moment
         * symbol(s): `Q`
         * application domain: generic
         * name: NuclearQuadrupoleMoment
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: `z` component of the diagonalized tensor of nuclear quadrupole moment: `Q = (1/e) int (3z^2 - r^2) ρ(x, y, z) dV` in the quantum state with the nuclear spin in the field direction (`z`), where `e` is the elementary charge (ISO 80000-1), `r^2 = x^2 + y^2 + z^2`, `ρ(x,y,z)` is the nuclear electric charge density (IEC 80000-6), and `dV` is the volume element `dx dy dz`
         * remarks: The electric nuclear quadrupole moment is `eQ`. This value is equal to the `z` component of the diagonalized tensor of quadrupole moment.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NuclearQuadrupoleMomentUnit[1];
    }

    attribute nuclearQuadrupoleMoment: NuclearQuadrupoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def NuclearQuadrupoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-19.1 nuclear radius */
    attribute nuclearRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.1 nuclear radius
         * symbol(s): `R`
         * application domain: generic
         * name: NuclearRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: conventional radius (ISO 80000-3) of sphere in which the nuclear matter is included
         * remarks: This quantity is not exactly defined. It is given approximately for nuclei in their ground state by: `R = r_0 A^(1//3)` where `r_0 ~~ 1.2 * 10^-15` m, and `A` is the nucleon number (item 10-1.3). Nuclear radius is usually expressed in femtometres, 1 fm = 10^(-15) m.
         */
    }

    /* ISO-80000-10 item 10-19.2 electron radius */
    attribute electronRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.2 electron radius
         * symbol(s): `r_e`
         * application domain: generic
         * name: ElectronRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius of a sphere such that the relativistic electron energy is distributed uniformly: `r_e = e^2/(4 π ε_0 m_e c_0^2)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `m_e` is the rest mass (item 10-2) of electron, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: This quantity corresponds to the electrostatic energy `E` of a charge distributed inside a sphere of radius `r_e` as if all the rest energy (item 10-3) of the electron were attributed to the energy of electromagnetic origin, using the relation `E = m_e c_0^2`.
         */
    }

    /* ISO-80000-10 item 10-20 Compton wavelength */
    attribute comptonWavelength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-20 Compton wavelength
         * symbol(s): `λ_C`
         * application domain: generic
         * name: ComptonWavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: quotient of the Planck constant and the product of the mass of the particle and the speed of light in vacuum: `λ_C = h / (m c_0)` where `h` is the Planck constant (ISO 80000-1), `m` is the rest mass (item 10-2) of a particle, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The wavelength of electromagnetic radiation scattered from free electrons (Compton scattering) is larger than that of the incident radiation by a maximum of 2`λ_C`.
         */
    }

    /* ISO-80000-10 item 10-21.1 mass excess */
    attribute massExcess: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.1 mass excess
         * symbol(s): `Δ`
         * application domain: generic
         * name: MassExcess (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: difference between the mass of an atom, and the product of its mass number and the unified mass constant: `Δ = m_a - A*m_u`, where `m_a` is the rest mass (item 10-2) of the atom, `A` is its nucleon number (item 10-1.3), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. See item 10-2.
         */
    }

    /* ISO-80000-10 item 10-21.2 mass defect */
    attribute massDefect: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.2 mass defect
         * symbol(s): `B`
         * application domain: generic
         * name: MassDefect (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: sum of the product of the proton number and the hydrogen atomic mass, and the neutron rest mass, minus the rest mass of the atom: `B = Z*m(⁢^1"H") + N*m_n - m_a` where `Z` is the proton number (item 10-1.1) of the atom, `m(⁢^1"H")` is atomic mass (item 10-4.1) of `⁢^1"H"`, `N` is neutron number (item 10-1.2), `m_n` is the rest mass (item 10-2) of the neutron, and `m_a` is the rest mass (item 10-2) of the atom
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. If the binding energy of the orbital electrons is neglected, `B c_0^2` is equal to the binding energy of the nucleus.
         */
    }

    /* ISO-80000-10 item 10-22.1 relative mass excess */
    attribute def RelativeMassExcessValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.1 relative mass excess
         * symbol(s): `Δ_r`
         * application domain: generic
         * name: RelativeMassExcess (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass excess and the unified atomic mass constant: `Δ_r = Δ/m_u` where `Δ` is mass excess (item 10-21.1), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassExcess: RelativeMassExcessValue :> scalarQuantities;

    /* ISO-80000-10 item 10-22.2 relative mass defect */
    attribute def RelativeMassDefectValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.2 relative mass defect
         * symbol(s): `B_r`
         * application domain: generic
         * name: RelativeMassDefect (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass defect and the unified atomic mass constant: `B_r = B/m_u` where `B` is mass defect (item 10-21.2), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassDefect: RelativeMassDefectValue :> scalarQuantities;

    /* ISO-80000-10 item 10-23.1 packing fraction */
    attribute def PackingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.1 packing fraction
         * symbol(s): `f`
         * application domain: generic
         * name: PackingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass excess and the nucleon number: `f` = Δ_r/A` where `Δ_r` is relative mass excess (item 10-22.1), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute packingFraction: PackingFractionValue :> scalarQuantities;

    /* ISO-80000-10 item 10-23.2 binding fraction */
    attribute def BindingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.2 binding fraction
         * symbol(s): `b`
         * application domain: generic
         * name: BindingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass defect and the nucleon number: `b = B_r/A` where `B_r` is relative mass defect (item 10-22.2), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute bindingFraction: BindingFractionValue :> scalarQuantities;

    /* ISO-80000-10 item 10-24 decay constant, disintegration constant */
    attribute def DecayConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-24 decay constant, disintegration constant
         * symbol(s): `λ`
         * application domain: generic
         * name: DecayConstant
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of `(-dN)/N` and `dt`, where `(dN)/N` is the mean fractional change in the number of nuclei in a particular energy state due to spontaneous transformations in a time interval of duration (ISO 80000-3) `dt`: `λ = -1/N (dN)/(dt)`
         * remarks: For exponential decay, this quantity is constant. For more than one decay channel, `λ = sum λ_a` where `λ_a` denotes the decay constant for a specified final state and the sum is taken over all final states.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DecayConstantUnit[1];
    }

    attribute decayConstant: DecayConstantValue[*] nonunique :> scalarQuantities;

    attribute def DecayConstantUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    alias DisintegrationConstantUnit for DecayConstantUnit;
    alias DisintegrationConstantValue for DecayConstantValue;
    alias disintegrationConstant for decayConstant;

    /* ISO-80000-10 item 10-25 mean duration of life, mean life time */
    attribute meanDurationOfLife: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-25 mean duration of life, mean life time
         * symbol(s): `τ`
         * application domain: atomic and nuclear physics
         * name: MeanDurationOfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: reciprocal of the decay constant `λ` (item 10-24): `τ = 1/λ`
         * remarks: Mean duration of life is the expected value of the duration of life of an unstable particle or an excited state of a particle when the number of decay events in a short time interval follows a Poisson distribution.
         */
    }

    alias meanLifeTime for meanDurationOfLife;

    /* ISO-80000-10 item 10-26 level width */
    attribute levelWidth: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-26 level width
         * symbol(s): `Γ`
         * application domain: generic
         * name: LevelWidth (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the reduced Planck constant and the mean life: `Γ = ℏ/τ` where `ℏ` is the reduced Planck constant (ISO 80000-1), and `τ` is mean duration of life (item 10-25)
         * remarks: Level width is the uncertainty of the energy of an unstable particle or an excited state of a system due to the Heisenberg principle. The term energy level refers to the configuration of the distribution function of the density of states. Energy levels may be considered as discrete, like those in an atom, or may have a finite width, like e.g. this item or like e.g. the valence or conduction band in solid state physics. Energy levels are applicable to both real and virtual particles, e.g. electrons and phonons, respectively.
         */
    }

    /* ISO-80000-10 item 10-27 nuclear activity */
    attribute def NuclearActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-27 nuclear activity
         * symbol(s): `A`
         * application domain: generic
         * name: NuclearActivity
         * quantity dimension: T^-1
         * measurement unit(s): Bq, s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the mean change in the number of nuclei in a particular energy state due to spontaneous nuclear transformations in a time interval of duration (ISO 80000-3) `dt`: `A = -(dN)/(dt)`
         * remarks: For exponential decay, `A = λN`, where `λ` is the decay constant (item 10-24). The becquerel (Bq) is a special name for second to the power minus one, to be used as the coherent SI unit of activity. In report 85a of the ICRU a definition with an equivalent meaning is given as: The activity, `A`, of an amount of a radionuclide in a particular energy state at a given time is the quotient of `-dN` by `dt`, where `dN` is the mean change in the number of nuclei in that energy state due to spontaneous nuclear transformations in the time interval `dt`: `A = -(dN)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NuclearActivityUnit[1];
    }

    attribute nuclearActivity: NuclearActivityValue[*] nonunique :> scalarQuantities;

    attribute def NuclearActivityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-28 specific activity, massic activity */
    attribute def SpecificActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-28 specific activity, massic activity
         * symbol(s): `a`
         * application domain: generic
         * name: SpecificActivity
         * quantity dimension: M^-1*T^-1
         * measurement unit(s): Bq/kg, kg^-1*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificActivityUnit[1];
    }

    attribute specificActivity: SpecificActivityValue[*] nonunique :> scalarQuantities;

    attribute def SpecificActivityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    alias MassicActivityUnit for SpecificActivityUnit;
    alias MassicActivityValue for SpecificActivityValue;
    alias massicActivity for specificActivity;

    /* ISO-80000-10 item 10-29 activity density, volumic activity, activity concentration */
    attribute def ActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-29 activity density, volumic activity, activity concentration
         * symbol(s): `c_A`
         * application domain: generic
         * name: ActivityDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): Bq/m^3, m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ActivityDensityUnit[1];
    }

    attribute activityDensity: ActivityDensityValue[*] nonunique :> scalarQuantities;

    attribute def ActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias VolumicActivityUnit for ActivityDensityUnit;
    alias VolumicActivityValue for ActivityDensityValue;
    alias volumicActivity for activityDensity;

    alias ActivityConcentrationUnit for ActivityDensityUnit;
    alias ActivityConcentrationValue for ActivityDensityValue;
    alias activityConcentration for activityDensity;

    /* ISO-80000-10 item 10-30 surface-activity density */
    attribute def SurfaceActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-30 surface-activity density
         * symbol(s): `a_S`
         * application domain: generic
         * name: SurfaceActivityDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): Bq/m^2, m^-2*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the total area `S` (ISO 80000-3) of the surface of that sample: `a_S` = `A`/`S`
         * remarks: This value is usually defined for flat sources, where `S` corresponds to the total area of surface of one side of the source.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceActivityDensityUnit[1];
    }

    attribute surfaceActivityDensity: SurfaceActivityDensityValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-31 half life */
    attribute halfLife: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-31 half life
         * symbol(s): `T_(1/2)`
         * application domain: generic
         * name: HalfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: mean duration (ISO 80000-3) required for the decay of one half of the atoms or nuclei
         * remarks: For exponential decay, `T_(1/2) = (ln2)/λ`, where `λ` is the decay constant (item 10-24).
         */
    }

    /* ISO-80000-10 item 10-32 alpha disintegration energy */
    attribute alphaDisintegrationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-32 alpha disintegration energy
         * symbol(s): `Q_α`
         * application domain: generic
         * name: AlphaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the kinetic energy (ISO 80000-4) of the α-particle produced in the disintegration process and the recoil energy (ISO 80000-5) of the product atom in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: The ground-state alpha disintegration energy, `Q_(α,0)`, also includes the energy of any nuclear transitions that take place in the daughter produced.
         */
    }

    /* ISO-80000-10 item 10-33 maximum beta-particle energy */
    attribute maximumBetaParticleEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-33 maximum beta-particle energy
         * symbol(s): `E_β`
         * application domain: generic
         * name: MaximumBetaParticleEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: maximum kinetic energy (ISO 80000-4) of the emitted beta particle produced in the nuclear disintegration process
         * remarks: The maximum kinetic energy corresponds to the highest energy of the beta spectrum.
         */
    }

    /* ISO-80000-10 item 10-34 beta disintegration energy */
    attribute betaDisintegrationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-34 beta disintegration energy
         * symbol(s): `Q_β`
         * application domain: generic
         * name: BetaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the maximum beta-particle kinetic energy (item 10-33) and the recoil energy (ISO 80000-5) of the atom produced in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: For positron emitters, the energy for the production of the annihilation radiation created in the combination of an electron with the positron is part of the beta disintegration energy. The ground-state beta disintegration energy, `Q_(β,0)`, also includes the energy of any nuclear transitions that take place in the daughter product.
         */
    }

    /* ISO-80000-10 item 10-35 internal conversion factor */
    attribute def InternalConversionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-35 internal conversion factor
         * symbol(s): `α`
         * application domain: generic
         * name: InternalConversionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the number of internal conversion electrons and the number of gamma quanta emitted by the radioactive atom in a given transition, where a conversion electron represents an orbital electron emitted through the radioactive decay
         * remarks: The quantity `α/(α+1)` is also used and called the internal-conversion fraction. Partial conversion fractions referring to the various electron shells `K, L, ...` are indicated by `α_K`, `α_L`, ... `α_K/α_L` is called the K-to-L internal conversion ratio.
         */
    }
    attribute internalConversionFactor: InternalConversionFactorValue :> scalarQuantities;

    /* ISO-80000-10 item 10-36 particle emission rate */
    attribute def ParticleEmissionRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-36 particle emission rate
         * symbol(s): `dot(N)`
         * application domain: generic
         * name: ParticleEmissionRate
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the number of particles being emitted from an infinitesimally small volume element in the time interval of duration `dt` (ISO 80000-3), and `dt`: `dot(N) = (dN)/(dt)`
         * remarks: Usually the kind of particles is specified, e.g. neutron emission rate or alpha particle emission rate.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleEmissionRateUnit[1];
    }

    attribute particleEmissionRate: ParticleEmissionRateValue[*] nonunique :> scalarQuantities;

    attribute def ParticleEmissionRateUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-37.1 reaction energy */
    attribute reactionEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.1 reaction energy
         * symbol(s): `Q`
         * application domain: generic
         * name: ReactionEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a nuclear reaction, sum of the kinetic energies (ISO 80000-4) and photon energies (ISO 80000-5) of the reaction products minus the sum of the kinetic and photon energies of the reactants
         * remarks: For exothermic nuclear reactions, `Q>0`. For endothermic nuclear reactions, `Q<0`.
         */
    }

    /* ISO-80000-10 item 10-37.2 resonance energy */
    attribute resonanceEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.2 resonance energy
         * symbol(s): `E_r`, `E_"res"`
         * application domain: generic
         * name: ResonanceEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: kinetic energy (ISO 80000-4) of an incident particle, in the reference frame of the target, corresponding to a resonance in a nuclear reaction
         * remarks: The energy of the resonance corresponds to the difference of the energy levels involved of the nucleus.
         */
    }

    /* ISO-80000-10 item 10-38.1 cross section */
    attribute crossSection: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.1 cross section
         * symbol(s): `σ`
         * application domain: atomic physics
         * name: CrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: for a specified target entity and for a specified reaction or process produced by incident charged or uncharged particles of a given type and energy, the quotient of the mean number of such reactions or processes and the incident-particle fluence (item 10-43)
         * remarks: The type of process is indicated by subscripts, e.g. absorption cross section `σ_a`, scattering cross section `σ_s`, fission cross section `σ_f`. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }

    /* ISO-80000-10 item 10-38.2 total cross section */
    attribute totalCrossSection: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.2 total cross section
         * symbol(s): `σ_"tot"`, `σ_"T"`
         * application domain: atomic physics
         * name: TotalCrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: sum of all cross sections (item 10-38.1) corresponding to the various reactions or processes between an incident particle of specified type and energy (ISO 80000-5) and a target entity
         * remarks: In the case of a narrow unidirectional beam of incident particles, this is the effective cross section for the removal of an incident particle from the beam. See the Remarks for item 10-52. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }

    /* ISO-80000-10 item 10-39 direction distribution of cross section */
    attribute def DirectionDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-39 direction distribution of cross section
         * symbol(s): `σ_Ω`
         * application domain: atomic physics
         * name: DirectionDistributionOfCrossSection
         * quantity dimension: L^2
         * measurement unit(s): m^2*sr^-1, m^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to `Ω`, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a specified direction, and `Ω` is the solid angle (ISO 80000-3) around that direction: `σ_Ω = (dσ)/(dΩ)`
         * remarks: Quantities listed under items 10-39, 10-40 and 10-41 are sometimes called differential cross sections. The type of interaction needs to be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DirectionDistributionOfCrossSectionUnit[1];
    }

    attribute directionDistributionOfCrossSection: DirectionDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def DirectionDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-40 energy distribution of cross section */
    attribute def EnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-40 energy distribution of cross section
         * symbol(s): `σ_E`
         * application domain: atomic physics
         * name: EnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/J, kg^-1*s^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to energy, where `σ` is the cross section (item 10-38.1) for a process in which the energy `E` (ISO 80000-5) of the ejected or scattered particle is between `E` and `E + dE`: `σ_E = (dσ)/(dE)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyDistributionOfCrossSectionUnit[1];
    }

    attribute energyDistributionOfCrossSection: EnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def EnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-41 direction and energy distribution of cross section */
    attribute def DirectionAndEnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-41 direction and energy distribution of cross section
         * symbol(s): `σ_(Ω,E)`
         * application domain: atomic physics
         * name: DirectionAndEnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/(J*sr), kg^-1*s^2
         * tensor order: 0
         * definition: partial differential quotient of `σ` with respect to solid angle and energy, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a solid angle `dΩ` around a specified direction and with an energy between `E` and `E+dE`: `σ_(Ω,E) = (del^2 σ) / (del Ω del E)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DirectionAndEnergyDistributionOfCrossSectionUnit[1];
    }

    attribute directionAndEnergyDistributionOfCrossSection: DirectionAndEnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def DirectionAndEnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-42.1 volumic cross section, macroscopic cross section */
    attribute def VolumicCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.1 volumic cross section, macroscopic cross section
         * symbol(s): `Σ`
         * application domain: atomic physics
         * name: VolumicCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and of the cross section (item 10-38.1) `σ_a` for a given type of atoms: `Σ = n_a σ_a`
         * remarks: When the target particles of the medium are at rest, `Σ = 1/l`, where `l` is the mean free path (item 10-71).
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumicCrossSectionUnit[1];
    }

    attribute volumicCrossSection: VolumicCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def VolumicCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias MacroscopicCrossSectionUnit for VolumicCrossSectionUnit;
    alias MacroscopicCrossSectionValue for VolumicCrossSectionValue;
    alias macroscopicCrossSection for volumicCrossSection;

    /* ISO-80000-10 item 10-42.2 volumic total cross section, macroscopic total cross section */
    attribute def VolumicTotalCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.2 volumic total cross section, macroscopic total cross section
         * symbol(s): `Σ_"tot"`, `Σ_"T"`
         * application domain: atomic physics
         * name: VolumicTotalCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and the cross section (item 10-38.1) `σ_"tot"` for a given type of atoms: `Σ_"tot" = n_a*σ_"tot"`
         * remarks: See the Remarks for item 10-49.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumicTotalCrossSectionUnit[1];
    }

    attribute volumicTotalCrossSection: VolumicTotalCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def VolumicTotalCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias MacroscopicTotalCrossSectionUnit for VolumicTotalCrossSectionUnit;
    alias MacroscopicTotalCrossSectionValue for VolumicTotalCrossSectionValue;
    alias macroscopicTotalCrossSection for volumicTotalCrossSection;

    /* ISO-80000-10 item 10-43 particle fluence */
    attribute def ParticleFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-43 particle fluence
         * symbol(s): `Φ`
         * application domain: generic
         * name: ParticleFluence
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: differential quotient of `N` with respect to `a`, where `N` is the number of particles incident on a sphere of cross-sectional area `a` (item 10-38.1): `Φ = (dN)/(da)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` fluence. If a flat area of size `dA` is passed perpendicularly by a number of `dN` particles, the corresponding particle fluence is: `Φ = (dN)/(dA)`. A plane area of size `dA` crossed at an angle `α` with respect to the surface normal by a number of `dN` particles results in the particle fluence: `Φ = (dN)/(cos(α) dA)` In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence, `Φ` , is the quotient of `dN` and `da`, where `dN` is the number of particles incident on a sphere of cross-sectional area `da`: `Φ = (dN)/(dA)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleFluenceUnit[1];
    }

    attribute particleFluence: ParticleFluenceValue[*] nonunique :> scalarQuantities;

    attribute def ParticleFluenceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-44 particle fluence rate */
    attribute def ParticleFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-44 particle fluence rate
         * symbol(s): `dot(Φ)`
         * application domain: generic
         * name: ParticleFluenceRate
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: differential quotient of fluence `Φ` (item 10-43) with respect to time (ISO 80000-3): `dot(Φ) = (dΦ)/(dA)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example proton fluence rate. The distribution function expressed in terms of speed and energy, `dot(Φ)_v` and `dot(Φ)_E` , are related to by: `dot(Φ) = int dot(Φ)_v dv = int dot(Φ)_E dE`. This quantity has also been termed particle flux density. Because the word "density" has several connotations, the term "fluence rate" is preferred. For a radiation field composed of particles of velocity `v`, the fluence rate is equal to `n`·`v` where `n` is the particle number density. See Remarks for item 10-43. In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence rate, `dot(Φ)` , is the quotient of `d Φ` and `dt`, where `d Φ` is the increment of the fluence in the time interval `dt`: `dot(Φ) = (dΦ)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleFluenceRateUnit[1];
    }

    attribute particleFluenceRate: ParticleFluenceRateValue[*] nonunique :> scalarQuantities;

    attribute def ParticleFluenceRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-45 radiant energy */
    attribute radiantEnergyForIonizingRadiation: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-45 radiant energy
         * symbol(s): `R`
         * application domain: ionizing radiation
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: mean energy (ISO 80000-5), excluding rest energy (item 10-3), of the particles that are emitted, transferred, or received
         * remarks: For particles of energy `E` (excluding rest energy), the radiant energy, `R`, is equal to the product `N·E` where `N` is the number of the particles that are emitted, transferred, or received The distributions, `N_E` and `R_E`, of the particle number and the radiant energy with respect to energy are given by `N_E` = `dN`/d`E` and `R_E` = `dR`/d`E`, respectively, where `dN` is the number of particles with energy between `E` and `E`+d`E`, and `dR` is their radiant energy. The two distributions are related by `R_E` = `E`·`N_E`.
         */
    }

    /* ISO-80000-10 item 10-46 energy fluence */
    attribute def EnergyFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-46 energy fluence
         * symbol(s): `Ψ`
         * application domain: generic
         * name: EnergyFluence
         * quantity dimension: M^1*T^-2
         * measurement unit(s): eV/m^2, J/m^2, kg*s^-2
         * tensor order: 0
         * definition: differential quotient of radiant energy `R` (item 10-45) incident on a sphere of cross-sectional area (item 10-38.1) `a` with respect to that area: `Ψ = (dR)/(da)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy fluence, `Ψ` is the quotient of `dR` and `da`, where `dR` is the radiant energy incident on a sphere of cross-sectional area `da`: `Ψ = (dR)/(da)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyFluenceUnit[1];
    }

    attribute energyFluence: EnergyFluenceValue[*] nonunique :> scalarQuantities;

    attribute def EnergyFluenceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-47 energy fluence rate */
    attribute def EnergyFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-47 energy fluence rate
         * symbol(s): `dot(Ψ)`
         * application domain: generic
         * name: EnergyFluenceRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: differential quotient of the energy fluence `Ψ` (item 10-46) with respect to time (ISO 80000-3): `dot(Ψ) = (d Ψ)/(dt)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy-fluence rate, `dot(Ψ)` , is the quotient of `d Ψ` by `dt`, where `d Ψ` is the increment of the energy fluence in the time interval `dt`: `dot(Ψ) = (d Ψ)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyFluenceRateUnit[1];
    }

    attribute energyFluenceRate: EnergyFluenceRateValue[*] nonunique :> scalarQuantities;

    attribute def EnergyFluenceRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-48 particle current density */
    attribute def ParticleCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-48 particle current density (magnitude)
         * symbol(s): `J`, `S`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleCurrentDensityUnit[1];
    }

    attribute particleCurrentDensity: ParticleCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    attribute def CartesianParticleCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-48 particle current density (vector)
         * symbol(s): `vec(J)`, `vec(S)`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 1
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianParticleCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianParticleCurrentDensity3dVector: CartesianParticleCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianParticleCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ParticleCurrentDensityUnit[3];
    }

    /* ISO-80000-10 item 10-49 linear attenuation coefficient */
    attribute def LinearAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-49 linear attenuation coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: ionizing radiation
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: for uncharged particles of a given type and energy the differential quotient `n` with respect to `l,` where `n` is the fraction of `N` incoming particles that experience interactions in traversing a distance (ISO 80000-3) `l` in a given material: `μ = (dn)/(dl) = 1/N (dN)/(dl)` where `dN` is the number of particles that experience interactions in traversing `dl`
         * remarks: `μ` is equal to the macroscopic total cross section `Σ_"tot"` for the removal of particles from the beam. Using the relation `μ_m = μ/ρ` between the linear attenuation coefficient `μ`, the mass attenuation coefficient `μ_m` (item 10-50) and the density `ρ`, the definition given for the mass attenuation coefficient in report 85a of the ICRU can be applied to the linear attenuation coefficient resulting in: The linear attenuation coefficient, `μ`, of a material, for uncharged particles of a given type and energy, is the quotient of `(dN)/N` by `dl`, where `(dN)/N` is the mean fraction of the particles that experience interactions in traversing a distance `dl` in the material: `μ = 1/(dl) (dN)/(N)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAttenuationCoefficientForIonizingRadiationUnit[1];
    }

    attribute linearAttenuationCoefficientForIonizingRadiation: LinearAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def LinearAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-50 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-50 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: ionizing radiation
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the mass density `ρ` (ISO 80000-4) of the medium: `μ_m = μ/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAttenuationCoefficientForIonizingRadiationUnit[1];
    }

    attribute massAttenuationCoefficientForIonizingRadiation: MassAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def MassAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-51 molar attenuation coefficient */
    attribute def MolarAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-51 molar attenuation coefficient
         * symbol(s): `μ_c`
         * application domain: generic
         * name: MolarAttenuationCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: quotient of linear attenuation coefficient `µ` (item 10-49) and the amount c (ISO 80000-9) of the medium: `μ_c = μ/c`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarAttenuationCoefficientUnit[1];
    }

    attribute molarAttenuationCoefficient: MolarAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MolarAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-10 item 10-52 atomic attenuation coefficient */
    attribute def AtomicAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-52 atomic attenuation coefficient
         * symbol(s): `μ_a`
         * application domain: generic
         * name: AtomicAttenuationCoefficient
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the number density (item 10-62.1), `n`, of atoms in the substance: `μ_a = μ/n`
         * remarks: `μ` is equal to the total cross section `σ_"tot"` for the removal of particles from the beam. See also item 10-38.2.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AtomicAttenuationCoefficientUnit[1];
    }

    attribute atomicAttenuationCoefficient: AtomicAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def AtomicAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-53 half-value thickness */
    attribute halfValueThickness: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-53 half-value thickness
         * symbol(s): `d_(1//2)`
         * application domain: generic
         * name: HalfValueThickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: thickness (ISO 80000-3) of the attenuating layer that reduces the quantity of interest of a unidirectional beam of infinitesimal width to half of its initial value
         * remarks: For exponential attenuation, `d_(1/2) = ln(2)/μ`. The quantity of interest is often the air kerma or exposure.
         */
    }

    /* ISO-80000-10 item 10-54 total linear stopping power, linear stopping power */
    attribute def TotalLinearStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-54 total linear stopping power, linear stopping power
         * symbol(s): `S`, `S_l`
         * application domain: generic
         * name: TotalLinearStoppingPower
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: for charged particles of a given type and energy `E_0` the differential quotient of `E` with respect to `x,` where `E` is the mean energy (ISO 80000-4) lost by the charged particles in traversing a distance (ISO 80000-3) `x` in the given material: `S = -(dE)/(dx)`
         * remarks: The total linear stopping power is sometimes also called stopping power. Both electronic losses and radiative losses are included. The quotient of the total linear stopping power of a substance and that of a reference substance is called the relative linear stopping power. See also item 10-85. Using the relation `S_m = S/ρ` between the total mass stopping power `S_m` (item 10-55), the total linear stopping power `S`, and the density `ρ`, the definition given for the mass stopping in report 85a of the ICRU can be applied to that of the total linear stopping power resulting in: The linear stopping power, `S`, of a material, for charged particles of a given type and energy, is the quotient of `dE` by `dl`, where `dE` is the mean energy lost by the charged particles in traversing a distance `dl` in the material: `S = -(dE)/(dx)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalLinearStoppingPowerUnit[1];
    }

    attribute totalLinearStoppingPower: TotalLinearStoppingPowerValue[*] nonunique :> scalarQuantities;

    attribute def TotalLinearStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias LinearStoppingPowerUnit for TotalLinearStoppingPowerUnit;
    alias LinearStoppingPowerValue for TotalLinearStoppingPowerValue;
    alias linearStoppingPower for totalLinearStoppingPower;

    /* ISO-80000-10 item 10-55 total mass stopping power, mass stopping power */
    attribute def TotalMassStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-55 total mass stopping power, mass stopping power
         * symbol(s): `S_m`
         * application domain: generic
         * name: TotalMassStoppingPower
         * quantity dimension: L^4*T^-2
         * measurement unit(s): eV*m^-2/kg, J*m^2/kg, m^4*s^-2
         * tensor order: 0
         * definition: quotient of the total linear stopping power `S` (item 10-54) and the mass density `ρ` (ISO 80000-4) of the material: `S_m = S/ρ`
         * remarks: The quotient of total mass stopping power of a material and that of a reference material is called relative mass stopping power.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalMassStoppingPowerUnit[1];
    }

    attribute totalMassStoppingPower: TotalMassStoppingPowerValue[*] nonunique :> scalarQuantities;

    attribute def TotalMassStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias MassStoppingPowerUnit for TotalMassStoppingPowerUnit;
    alias MassStoppingPowerValue for TotalMassStoppingPowerValue;
    alias massStoppingPower for totalMassStoppingPower;

    /* ISO-80000-10 item 10-56 mean linear range */
    attribute meanLinearRange: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-56 mean linear range
         * symbol(s): `R`, `R_l`
         * application domain: generic
         * name: MeanLinearRange (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: mean total rectified path length (ISO 80000-3) travelled by a particle in the course of slowing down to rest in a given material averaged over a group of particles having the same initial energy (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-57 mean mass range */
    attribute def MeanMassRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-57 mean mass range
         * symbol(s): `R_ρ`, `R_m`
         * application domain: generic
         * name: MeanMassRange
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: product of the mean linear range (item 10-56) `R` and the mass density `ρ` (ISO 80000-4) of the material: `R_ρ = R*ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MeanMassRangeUnit[1];
    }

    attribute meanMassRange: MeanMassRangeValue[*] nonunique :> scalarQuantities;

    attribute def MeanMassRangeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-58 linear ionization */
    attribute def LinearIonizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-58 linear ionization
         * symbol(s): `N_{i_l}`
         * application domain: generic
         * name: LinearIonization
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: differential quotient of `q` with respect to `l`, where `q` is the average total charge (IEC 80000-6) of all positive ions produced by an ionizing charged particle over a path `l` (ISO 80000-3), divided by the elementary charge, `e` (ISO 80000-1): `N_{i_l} = 1/e*(dq)/(dl)`
         * remarks: Ionization due to secondary ionizing particles is included.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearIonizationUnit[1];
    }

    attribute linearIonization: LinearIonizationValue[*] nonunique :> scalarQuantities;

    attribute def LinearIonizationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-59 total ionization */
    attribute def TotalIonizationValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-59 total ionization
         * symbol(s): `N_i`
         * application domain: generic
         * name: TotalIonization (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total mean charge of all positive ions produced by an ionizing charged particle along its entire path and along the paths of any secondary charged particles, and the elementary charge, `e` (ISO 80000-1)
         * remarks: `N_i = int N_(il) dl` See item 10-58.
         */
    }
    attribute totalIonization: TotalIonizationValue :> scalarQuantities;

    /* ISO-80000-10 item 10-60 average energy loss per elementary charge produced */
    attribute def AverageEnergyLossPerElementaryChargeProducedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-60 average energy loss per elementary charge produced
         * symbol(s): `W_i`
         * application domain: generic
         * name: AverageEnergyLossPerElementaryChargeProduced
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the initial kinetic energy `E_k` (ISO 80000-4) of an ionizing charged particle and the total ionization `N_i` (item 10-59) produced by that particle: `W_i = E_k/N_i`
         * remarks: The name "average energy loss per ion pair formed" is usually used, although it is ambiguous. In the practical dosimetry of ionizing radiation the term `W`/`e`, the quotient of `W`, the average energy deposited in dry air per ion pair formed, and `e`, the elementary charge, is used as the factor which, when multiplied with the electric charge of one sign carried by all ion pairs formed in dry air of given mass, gives the energy deposited in this amount of dry air in the form of excitations and ionizations. In ICRU Report 85a, the mean energy expended in a gas per ion pair formed, `W`, is the quotient of `E` by `N,` where `N` is the mean total liberated charge of either sign, divided by the elementary charge when the initial kinetic energy `E` of a charged particle introduced into the gas is completely dissipated in the gas. Thus, `W` = `E`/`N`. It follows from the definition of `W` that the ions produced by bremsstrahlung or other secondary radiation emitted by the initial and secondary charged particles are included in `N`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AverageEnergyLossPerElementaryChargeProducedUnit[1];
    }

    attribute averageEnergyLossPerElementaryChargeProduced: AverageEnergyLossPerElementaryChargeProducedValue[*] nonunique :> scalarQuantities;

    attribute def AverageEnergyLossPerElementaryChargeProducedUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-61 mobility */
    attribute def MobilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-61 mobility
         * symbol(s): `μ`, `μ_m`
         * application domain: generic
         * name: Mobility
         * quantity dimension: M^-1*T^2*I^1
         * measurement unit(s): m^2/(V*s), kg^-1*s^2*A
         * tensor order: 0
         * definition: quotient of average drift speed (ISO 80000-3) imparted to a charged particle in a medium by an electric field, and the electric field strength (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MobilityUnit[1];
    }

    attribute mobility: MobilityValue[*] nonunique :> scalarQuantities;

    attribute def MobilityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-10 item 10-62.1 particle number density */
    attribute def ParticleNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.1 particle number density
         * symbol(s): `n`
         * application domain: generic
         * name: ParticleNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the mean number `N` of particles in the volume (ISO 80000-3) `V` and volume: `n = N/V`
         * remarks: `n` is the general symbol for the number density of particles. The distribution functions expressed in terms of speed and energy, `n_v` and `n_E`, are related to `n` by: `n = int n_v dv = int n_E dE`. The word "particle" is usually replaced by the name of a specific particle, for example `neutron` number density.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleNumberDensityUnit[1];
    }

    attribute particleNumberDensity: ParticleNumberDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-62.2 ion number density, ion density */
    attribute def IonNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.2 ion number density, ion density
         * symbol(s): `n^"+"`, `n^"-"`
         * application domain: generic
         * name: IonNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the number of positive and negative ions, `N^"+"` and `N^"-"`, respectively, in the volume `V` (ISO 80000-3), and that volume: `n^"+" = N^"+" / V`, `n^"-" = N^"-" / V`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IonNumberDensityUnit[1];
    }

    attribute ionNumberDensity: IonNumberDensityValue[*] nonunique :> scalarQuantities;

    attribute def IonNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias IonDensityUnit for IonNumberDensityUnit;
    alias IonDensityValue for IonNumberDensityValue;
    alias ionDensity for ionNumberDensity;

    /* ISO-80000-10 item 10-63 Recombination coefficient */
    attribute def RecombinationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-63 Recombination coefficient
         * symbol(s): `α`
         * application domain: generic
         * name: RecombinationCoefficient
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: coefficient in the law of recombination: `-(dn^"+")/(dt) = -(dn^"-")/(dt) = α*n^"+"*n^"-"`, where `n^"+"` and `n^"-"` are the ion number densities (item 10-62.2) of positive and negative ions, respectively, recombined during a time interval of duration `dt` (ISO 80000-3)
         * remarks: The widely used term "recombination factor" is not correct because "factor" should only be used for quantities with dimension 1. The terms `(dn^"+")/(dt)` , `(dn^"-")/(dt)` are differential quotients.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RecombinationCoefficientUnit[1];
    }

    attribute recombinationCoefficient: RecombinationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def RecombinationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-64 diffusion coefficient, diffusion coefficient for particle number density */
    /* Refer to declaration for DiffusionCoefficient in ISQChemistryMolecular item 9-39 diffusion coefficient */

    alias DiffusionCoefficientForParticleNumberDensityUnit for DiffusionCoefficientUnit;
    alias DiffusionCoefficientForParticleNumberDensityValue for DiffusionCoefficientValue;
    alias diffusionCoefficientForParticleNumberDensity for diffusionCoefficient;

    /* ISO-80000-10 item 10-65 diffusion coefficient for fluence rate */
    attribute diffusionCoefficientForFluenceRate: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-65 diffusion coefficient for fluence rate
         * symbol(s): `D_ϕ`, `D`
         * application domain: generic
         * name: DiffusionCoefficientForFluenceRate (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: proportionality constant between the particle current density `vec(J )`(item 10-48) and the gradient of the particle fluence rate `dot(Φ)` (item 10-44): `vec(J) = -vec(D) * nabla Φ`
         * remarks: For a particle of a given speed `v`: `D_Ψ(v) = -J_{v,x}/(partial Ψ // partial x)` and `vec(v) * vec(D_Ψ)(v) = -vec(D_n)(v)`
         */
    }

    /* ISO-80000-10 item 10-66 particle source density */
    attribute def ParticleSourceDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-66 particle source density
         * symbol(s): `S`
         * application domain: generic
         * name: ParticleSourceDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the mean rate of production of particles in a volume, and that volume (ISO 80000-3)
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` source density. The distribution functions expressed in terms of speed and energy, `S_v` and `S_E`, are related to `S` by: `S = int S_v dv = int S_E dE`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleSourceDensityUnit[1];
    }

    attribute particleSourceDensity: ParticleSourceDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleSourceDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-67 slowing-down density */
    attribute def SlowingDownDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-67 slowing-down density
         * symbol(s): `q`
         * application domain: generic
         * name: SlowingDownDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: differential quotient of `n` with respect to time, where `n` is the number density of particles that are slowed down in a time interval of duration (ISO 80000-3) `t`: `q = -(dn)/(dt)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SlowingDownDensityUnit[1];
    }

    attribute slowingDownDensity: SlowingDownDensityValue[*] nonunique :> scalarQuantities;

    attribute def SlowingDownDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-68 resonance escape probability */
    attribute def ResonanceEscapeProbabilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-68 resonance escape probability
         * symbol(s): `p`
         * application domain: generic
         * name: ResonanceEscapeProbability (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the probability that a neutron slowing down will traverse all or some specified portion of the range of resonance energies (item 10-37.2) without being absorbed
         * remarks: None.
         */
    }
    attribute resonanceEscapeProbability: ResonanceEscapeProbabilityValue :> scalarQuantities;

    /* ISO-80000-10 item 10-69 lethargy */
    attribute def LethargyValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-69 lethargy
         * symbol(s): `u`
         * application domain: generic
         * name: Lethargy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a neutron of kinetic energy `E` (ISO 80000-4) : `u = ln(E_0/E)`, where `E_0` is a reference energy
         * remarks: Lethargy is also referred to as logarithmic energy decrement.
         */
    }
    attribute lethargy: LethargyValue :> scalarQuantities;

    /* ISO-80000-10 item 10-70 average logarithmic energy decrement */
    attribute def AverageLogarithmicEnergyDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-70 average logarithmic energy decrement
         * symbol(s): `ζ`
         * application domain: generic
         * name: AverageLogarithmicEnergyDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average value of the increase in lethargy (item 10-69) in elastic collisions between neutrons and nuclei whose kinetic energy (ISO 80000-4) is negligible compared with that of the neutrons
         * remarks: None.
         */
    }
    attribute averageLogarithmicEnergyDecrement: AverageLogarithmicEnergyDecrementValue :> scalarQuantities;

    /* ISO-80000-10 item 10-71 mean free path */
    attribute meanFreePathForAtomicPhysics: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-71 mean free path
         * symbol(s): `l`, `λ`
         * application domain: atomic physics
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that particles travel between two successive specified reactions or processes
         * remarks: See the Remarks for item 10-42.1.
         */
    }

    /* ISO-80000-10 item 10-72.1 slowing-down area */
    attribute slowingDownArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.1 slowing-down area
         * symbol(s): `L_s^2`, `L_"sl"^2`
         * application domain: generic
         * name: SlowingDownArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square of the distance (ISO 80000-3) between the neutron source and the point where a neutron reaches a given energy (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-72.2 diffusion area */
    attribute diffusionArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.2 diffusion area
         * symbol(s): `L^2`
         * application domain: generic
         * name: DiffusionArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square distance (ISO 80000-3) between the point where a neutron enters a specified class and the point where it leaves this class
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
    }

    /* ISO-80000-10 item 10-72.3 migration area */
    attribute migrationArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.3 migration area
         * symbol(s): `M^2`
         * application domain: generic
         * name: MigrationArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: sum of the slowing-down area (item 10-72.1) from fission energy to thermal energy (ISO 80000-5) and the diffusion area (item 10-72.2) for thermal neutrons
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.1 slowing-down length */
    attribute slowingDownLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.1 slowing-down length
         * symbol(s): `L_s`, `L_"sl"`
         * application domain: generic
         * name: SlowingDownLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the slowing down area `L_s^2` (item 10-72.1): `L_s = sqrt(L_s^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.2 diffusion length */
    attribute diffusionLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.2 diffusion length
         * symbol(s): `L`
         * application domain: atomic physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the diffusion area `L^2` (item 10-72.2): `L = sqrt(L^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.3 migration length */
    attribute migrationLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.3 migration length
         * symbol(s): `M`
         * application domain: generic
         * name: MigrationLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the migration area `M^2` (item 10-72.3): `M = sqrt(M^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-74.1 neutron yield per fission */
    attribute neutronYieldPerFission: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.1 neutron yield per fission
         * symbol(s): `ν`
         * application domain: generic
         * name: NeutronYieldPerFission (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per fission event
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-74.2 neutron yield per absorption */
    attribute neutronYieldPerAbsorption: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.2 neutron yield per absorption
         * symbol(s): `η`
         * application domain: generic
         * name: NeutronYieldPerAbsorption (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per neutron absorbed in a fissionable nuclide or in a nuclear fuel, as specified
         * remarks: `ν/η` is equal to the quotient of the macroscopic cross section for fission and that for absorption, both for neutrons in the fuel material.
         */
    }

    /* ISO-80000-10 item 10-75 fast fission factor */
    attribute def FastFissionFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-75 fast fission factor
         * symbol(s): `φ`
         * application domain: generic
         * name: FastFissionFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the mean number of neutrons produced by fission due to neutrons of all energies (ISO 80000-5) and the mean number of neutrons produced by fissions due to thermal neutrons only
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FastFissionFactorUnit[1];
    }

    attribute fastFissionFactor: FastFissionFactorValue[*] nonunique :> scalarQuantities;

    attribute def FastFissionFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-76 thermal utilization factor */
    attribute def ThermalUtilizationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-76 thermal utilization factor
         * symbol(s): `f`
         * application domain: generic
         * name: ThermalUtilizationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the number of thermal neutrons absorbed in a fissionable nuclide or in a nuclear fuel, as specified, and the total number of thermal neutrons absorbed
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalUtilizationFactorUnit[1];
    }

    attribute thermalUtilizationFactor: ThermalUtilizationFactorValue[*] nonunique :> scalarQuantities;

    attribute def ThermalUtilizationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-77 non-leakage probability */
    attribute def NonLeakageProbabilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-77 non-leakage probability
         * symbol(s): `Λ`
         * application domain: generic
         * name: NonLeakageProbability
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: probability that a neutron will not escape from the reactor during the slowing-down process or while it diffuses as a thermal neutron
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NonLeakageProbabilityUnit[1];
    }

    attribute nonLeakageProbability: NonLeakageProbabilityValue[*] nonunique :> scalarQuantities;

    attribute def NonLeakageProbabilityUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-78.1 multiplication factor */
    attribute def MultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.1 multiplication factor
         * symbol(s): `k`
         * application domain: generic
         * name: MultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total number of fission or fission-dependent neutrons produced in the duration of a time interval and the total number of neutrons lost by absorption and leakage in that duration
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MultiplicationFactorUnit[1];
    }

    attribute multiplicationFactor: MultiplicationFactorValue[*] nonunique :> scalarQuantities;

    attribute def MultiplicationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-78.2 infinite multiplication factor */
    attribute def InfiniteMultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.2 infinite multiplication factor
         * symbol(s): `k_∞`
         * application domain: generic
         * name: InfiniteMultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: multiplication factor (item 10-78.1) for an infinite medium or for an infinite repeating lattice
         * remarks: For a thermal reactor, `k_∞ = η*ε*p*f`
         */
        attribute :>> num: Real;
        attribute :>> mRef: InfiniteMultiplicationFactorUnit[1];
    }

    attribute infiniteMultiplicationFactor: InfiniteMultiplicationFactorValue[*] nonunique :> scalarQuantities;

    attribute def InfiniteMultiplicationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-79 reactor time constant */
    attribute reactorTimeConstant: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-79 reactor time constant
         * symbol(s): `T`
         * application domain: generic
         * name: ReactorTimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (ISO 80000-3) required for the neutron fluence rate (item 10-44) in a reactor to change by the factor e when the fluence rate is rising or falling exponentially
         * remarks: Also called reactor period.
         */
    }

    /* ISO-80000-10 item 10-80.1 energy imparted */
    attribute energyImparted: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.1 energy imparted
         * symbol(s): `ε`
         * application domain: generic
         * name: EnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of all energy deposits in a given volume: `ε = sum_i ε_i` where the summation is performed over all energy (ISO 80000-5) deposits `ε_i` of interaction `i` in that volume
         * remarks: Energy imparted is a stochastic quantity. `ε_i` is given by: `ε_i = ε_(i n) - ε_"out" + Q` where `ε_(i n)` is the energy (ISO 80000-5) of the incident ionizing particle, excluding rest energy (item 10-3), `ε_"out"` is the sum of the energies (ISO 80000-5) of all ionizing particles leaving the interaction, excluding rest energy (item 10-3), and `Q` is the change in the rest energies (item 10-3) of the nucleus and of all particles involved in the interaction. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy. Stochastic quantities such as the energy imparted and the specific energy imparted (item 10-81.2) and their probability distributions have been introduced as they describe the discontinuous nature of the ionizing radiations as a determinant of radiochemical and radiobiological effects. In radiation applications involving large numbers of ionizing particles, e.g. in medicine, radiation protection and materials testing and processing, these fluctuations are adequately represented by the expectation values of the probability distributions. Non-stochastic quantities such as particle fluence (item 10-43), absorbed dose (item 10-81.1) and kerma (item 10-86.1) are based on these expectation values.
         */
    }

    /* ISO-80000-10 item 10-80.2 mean energy imparted */
    attribute meanEnergyImparted: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.2 mean energy imparted
         * symbol(s): `bar(ε)`
         * application domain: generic
         * name: MeanEnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: expectation value of the energy imparted (item 10-80.1): `bar(ε) = R_"in" - R_"out" + sum Q` where `R_"in"` is the radiant energy (item 10-45) of all those charged and uncharged ionizing particles that enter the volume, `R_"out"` is the radiant energy of all those charged and uncharged ionizing particles that leave the volume, and `sum Q` is the sum of all changes of the rest energy (item 10-3) of nuclei and elementary particles that occur in that volume
         * remarks: Sometimes, it has been called the integral absorbed dose. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy.
         */
    }

    /* ISO-80000-10 item 10-81.1 absorbed dose */
    attribute def AbsorbedDoseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-81.1 absorbed dose
         * symbol(s): `D`
         * application domain: generic
         * name: AbsorbedDose
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: differential quotient of `bar(ε)` with respect to `m`, where `bar(ε)` is the mean energy (ISO 80000-5) imparted by ionizing radiation to matter of mass (ISO 80000-4) `m`: `D = (d bar(ε))/(dm)`
         * remarks: The gray is a special name for joule per kilogram, to be used as the coherent SI unit for absorbed dose. `1 "Gy" = 1 "J"/"kg"`. `bar(ε) = int D dm` where `dm` is the element of mass of the irradiated matter. In the limit of a small domain, the mean specific energy `bar(z) = (Δ bar(ε))/(Δ m)` is equal to the absorbed dose `D`. The absorbed dose can also be expressed in terms of the volume of the mass element by: `D = (d bar(ε))/(dm) = (d bar(ε))/(ρ dV)` where `ρ` is the mass density of the mass element. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed dose, `D`, is the quotient of `d bar(ε)` by dm, where `d bar(ε)` is the mean energy imparted by ionizing radiation to matter of mass `dm`: `D = (d bar(ε))/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AbsorbedDoseUnit[1];
    }

    attribute absorbedDose: AbsorbedDoseValue[*] nonunique :> scalarQuantities;

    attribute def AbsorbedDoseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-81.2 specific energy imparted */
    attribute specificEnergyImparted: AbsorbedDoseValue :> scalarQuantities {
        doc
        /*
         * source: item 10-81.2 specific energy imparted
         * symbol(s): `z`
         * application domain: generic
         * name: SpecificEnergyImparted (specializes AbsorbedDose)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of the energy imparted `ε` (item 10-80.1) and the mass `m` (ISO 80000-4) of the matter in a given volume element: `z = ε / m`
         * remarks: `z` is a stochastic quantity. In the limit of a small domain, the mean specific energy `bar(z)` is equal to the absorbed dose `D`. The specific energy imparted can be due to one or more (energy-deposition) events.
         */
    }

    /* ISO-80000-10 item 10-82 quality factor */
    attribute def QualityFactorForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-82 quality factor
         * symbol(s): `Q`
         * application domain: ionizing radiation
         * name: QualityFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor in the calculation and measurement of dose equivalent (item 10-83.1), by which the absorbed dose (item 10-81.1) is to be weighted in order to account for different biological effectiveness of radiations, for radiation protection purposes
         * remarks: `Q` is determined by the linear energy transfer (item 10-85) for `Δ -> ∞` , `L_∞` (often denoted as `L` or LET), of charged particles passing through a small volume element at this point (the value of `L_∞` refers to water, not to tissue; the difference, however, is small). The relationship between `L` and `Q` is given in ICRP Publication 103 (ICRP, 2007).
         */
        attribute :>> num: Real;
        attribute :>> mRef: QualityFactorForIonizingRadiationUnit[1];
    }

    attribute qualityFactorForIonizingRadiation: QualityFactorForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def QualityFactorForIonizingRadiationUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-83.1 dose equivalent */
    attribute def DoseEquivalentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-83.1 dose equivalent
         * symbol(s): `H`
         * application domain: generic
         * name: DoseEquivalent
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Sv, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: product of the absorbed dose `D` (item 10-81.1) to tissue at the point of interest and the quality factor `Q` (item 10-82) at that point: `H = DQ`
         * remarks: The sievert (Sv) is a special name for joule per kilogram, and is the coherent SI unit for dose equivalent. `1 "Sv" = 1 "J/kg"`. The dose equivalent at a point in tissue is given by: `H = int_0^∞ Q(L) D_L dL` where `D_L = (dD)/(dL)` is the distribution of `D` in `L` at the point of interest. See ICRP Publication 103 (ICRP, 2007). The quantities measured with radiation protection dosimeters are based on the definition `H = Q*D`. If various radiation qualities `i` have to be simultaneously accounted for, the definition is: `H = sum_i Q_i*D_i`. In ICRU 51 this quantity is denoted as "dose equivalent". In order to quantify the radiation exposition of the human body and to specify dose limits, use is made of a quantity defined in ICRP 103, the "equivalent dose to a tissue or organ": `H_T = w_T*sum_R w_R*D_{T,R}`. The weighting factors `w_T` for various tissues and organs `T` and `w_R` for various radiation qualities `R` have been numerically laid down in ICRP 103. `D_{T,R}` is the mean absorbed dose to tissue within a tissue or organ `T`, imparted by radiation with radiation quality `R`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DoseEquivalentUnit[1];
    }

    attribute doseEquivalent: DoseEquivalentValue[*] nonunique :> scalarQuantities;

    attribute def DoseEquivalentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-83.2 dose equivalent rate */
    attribute doseEquivalentRate: DoseEquivalentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-83.2 dose equivalent rate
         * symbol(s): `dot(H)`
         * application domain: generic
         * name: DoseEquivalentRate (specializes DoseEquivalent)
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Sv/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of dose equivalent `H` (item 10-83.1) with respect to time (ISO 80000-3): `dot(H) = (dH)/(dt)`
         * remarks: `1 "Sv/s" = 1 "W/kg"`. See the remarks for item 10-83.1.
         */
    }

    /* ISO-80000-10 item 10-84 absorbed-dose rate */
    attribute def AbsorbedDoseRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-84 absorbed-dose rate
         * symbol(s): `dot(D)`
         * application domain: generic
         * name: AbsorbedDoseRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of the absorbed dose `D` (item 10-81.1) with respect to time (ISO 80000-3): `dot(D) = (dD)/(dt)`
         * remarks: `1 "Gy/s"  = 1 "W/kg"` See the remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed-does rate, `dot(D)` , is the quotient of `dD` by `dt`, where `dD` is the increment of absorbed does in the time interval `dt`: `dot(D) = (dD)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AbsorbedDoseRateUnit[1];
    }

    attribute absorbedDoseRate: AbsorbedDoseRateValue[*] nonunique :> scalarQuantities;

    attribute def AbsorbedDoseRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-85 linear energy transfer */
    attribute def LinearEnergyTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-85 linear energy transfer
         * symbol(s): `L_Δ`
         * application domain: generic
         * name: LinearEnergyTransfer
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: quotient of the mean energy (ISO 80000-4) `dE_Δ` lost by the charged particles due to electronic interactions in traversing a distance (ISO 80000-3) `dl`, minus the mean sum of the kinetic energies in excess of `Δ` of all the electrons released by the charged particles and `dl`: `L_Δ = (dE_Δ)/(dl)`
         * remarks: This quantity is not completely defined unless `Δ` is specified, i.e. the maximum kinetic energy of secondary electrons whose energy is considered to be "locally deposited". `Δ` may be expressed in `"eV"`. Note that the abbreviation LET specifically refers to the quantity `L_∞` mentioned in the remark to 10-82.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearEnergyTransferUnit[1];
    }

    attribute linearEnergyTransfer: LinearEnergyTransferValue[*] nonunique :> scalarQuantities;

    attribute def LinearEnergyTransferUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-86.1 kerma */
    attribute def KermaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.1 kerma
         * symbol(s): `K`
         * application domain: generic
         * name: Kerma
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: for uncharged ionizing radiation, differential quotient of `E_(`tr) with respect to `m`, where `E_(`tr) is the mean sum of the initial kinetic energies (ISO 80000-4) of all the charged ionizing particles liberated in a mass (ISO 80000-4) `m` of a material: `K = (dE_tr)/(dm)`
         * remarks: `1 "Gy" = 1 "J/kg"` See the remarks for item 10-81.1. The name "kerma" is derived from Kinetic Energy Released in MAtter (or MAss or MAterial). The quantity `dE_(tr)` includes also the kinetic energy of the charged particles emitted in the decay of excited atoms, molecules, or nuclei. When the mass element `dm` consists of air the term air kerma is used. It can be convenient to refer to a value of air kerma in free space or at a point inside a material different from air, e.g. to the air kerma at a point inside a water phantom. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma, `K`, for ionizing uncharged particles, is the quotient of `dE_(tr)` by `dm`, where `dE_(tr)` is the mean sum of the initial kinetic energies of all the charged particles liberated in a mass `dm` of a material by the uncharged particles incident on `dm`: `K = (dE_(tr))/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KermaUnit[1];
    }

    attribute kerma: KermaValue[*] nonunique :> scalarQuantities;

    attribute def KermaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-86.2 kerma rate */
    attribute def KermaRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.2 kerma rate
         * symbol(s): `dot(K)`
         * application domain: generic
         * name: KermaRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of kerma (item 10-86.1) with respect to time (ISO 80000-3): `dot(K) = (dK)/(dt)`
         * remarks: `1 "Gy/s" = 1 "W/kg"`. See the Remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma rate, `dot(K)` , is the quotient of `dK` by `dt`, where `dK` is the increment of kerma in the time interval `dt`: `dot(K) = (dK)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KermaRateUnit[1];
    }

    attribute kermaRate: KermaRateValue[*] nonunique :> scalarQuantities;

    attribute def KermaRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-87 mass energy-transfer coefficient */
    attribute def MassEnergyTransferCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-87 mass energy-transfer coefficient
         * symbol(s): `μ_"tr"/ρ`
         * application domain: generic
         * name: MassEnergyTransferCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: for ionizing uncharged particles of a given type and energy, the differential quotient of `R_"tr"` with respect to `l`: `m_"tr"/ρ = 1/ρ 1/R (dR_"tr")/(dl)` where `R_"tr"` is the mean energy (ISO 80000-5) that is transferred to kinetic energy (ISO 80000-4) of charged particles by interactions of the uncharged particles of incident radiant energy `R` (item 10-45) in traversing a distance (ISO 80000-3) `l` in the material of density (ISO 80000-4) `ρ`, divided by `ρ` and `R`
         * remarks: `m_(tr)/ρ = (dot(K))/ψ` , where `dot(K)` is kerma rate (item 10-86.2) and `ψ` is energy fluence rate (item 10-47). The quantity: `μ_(en)/ρ = μ_(tr)/ρ(1-g)` where `g` is mean fraction of the kinetic energy of the liberated charged particles that is lost in radiative processes in the material, is called mass energy-absorption coefficient. The mass energy-absorption coefficient of a compound material depends on the stopping power of the material. Thus, its evaluation cannot, in principle, be reduced to a simple summation of the mass energy-absorption coefficient of the atomic constituents. Such a summation can provide an adequate approximation when the value of `g` is sufficiently small. In report 85a of the ICRU a definition with an equivalent meaning is given as: The mass energy-transfer coefficient, `μ_(tr)/ρ` , of a material, for uncharged particles of a given type and energy, is the quotient of `(dR_(tr))/R` by `ρ dl`, where `dR_(tr)` is the mean energy that is transferred to kinetic energy of charged particles by interactions of the uncharged particles of incident radiant energy `R` in traversing a distance `dl` in the material of density `ρ` : `μ_(tr)/ρ = 1/(ρ dl) (d R_(tr))/R`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassEnergyTransferCoefficientUnit[1];
    }

    attribute massEnergyTransferCoefficient: MassEnergyTransferCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassEnergyTransferCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-88 exposure */
    attribute def ExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-88 exposure
         * symbol(s): `X`
         * application domain: ionizing radiation
         * name: Exposure
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): C/kg, kg^-1*s*A
         * tensor order: 0
         * definition: for X- or gamma radiation the differential quotient of `q` with respect to `m`, where `q` is the absolute value of the mean total electric charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on an element of dry air with mass `m` (ISO 80000-4) are completely stopped in dry air: `X = (dq)/(dm)`
         * remarks: The ionization produced by electrons emitted in atomic or molecular relaxation is included in `dq`. The ionization due to photons emitted by radiative processes (i.e. bremsstrahlung and fluorescence photons) is not included in `dq`. This quantity should not be confused with the quantity photon exposure (ISO 80000-7), radiation exposure (ISO 80000-7), or the quantity luminous exposure (ISO 80000-7). It can be convenient to refer to a value of exposure in free space or at a point inside a material different from air, e.g. to the exposure at a point inside a water phantom. The exposure is related to the air kerma, `K_a`, (see item 10-86.1) by: `X = (e (1-g))/W K_a` , where `e` is the elementary charge (ISO 80000-1), `W` the average energy loss per elementary charge produced (item 10-60), and `g` is the fraction of the kinetic energy of liberated charged particles that is lost in radiative processes. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure, `X`, is the quotient of `dq` by `dm`, where `dq` is the absolute value of the mean total charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on a mass `dm` of dry air are completely stopped in dry air: `X = (dq)/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ExposureUnit[1];
    }

    attribute exposure: ExposureValue[*] nonunique :> scalarQuantities;

    attribute def ExposureUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-10 item 10-89 exposure rate */
    attribute def ExposureRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-89 exposure rate
         * symbol(s): `dot(X)`
         * application domain: generic
         * name: ExposureRate
         * quantity dimension: M^-1*I^1
         * measurement unit(s): C/(kg*s), kg^-1*A
         * tensor order: 0
         * definition: differential quotient of the exposure `X` (item 10-88) with respect to time (ISO 80000-3): `dot(X) = (dX)/(dt)`
         * remarks: `1 "C/(kg s)" = 1 "A/kg"`. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure rate, `dot(X)` , is the quotient of `dX` by `dt`, where `dX` is the increment of exposure in the time interval `dt`: `dot(X) = (dX)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ExposureRateUnit[1];
    }

    attribute exposureRate: ExposureRateValue[*] nonunique :> scalarQuantities;

    attribute def ExposureRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, electricCurrentPF); }
    }

}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQAtomicNuclear"))) (name "ISQAtomicNuclear") (declared-name "ISQAtomicNuclear")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit"))) (name "AbsorbedDoseRateUnit") (declared-name "AbsorbedDoseRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue"))) (name "AbsorbedDoseRateValue") (declared-name "AbsorbedDoseRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit"))) (name "AbsorbedDoseUnit") (declared-name "AbsorbedDoseUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue"))) (name "AbsorbedDoseValue") (declared-name "AbsorbedDoseValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityConcentrationUnit"))) (name "ActivityConcentrationUnit") (declared-name "ActivityConcentrationUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityConcentrationValue"))) (name "ActivityConcentrationValue") (declared-name "ActivityConcentrationValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit"))) (name "ActivityDensityUnit") (declared-name "ActivityDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue"))) (name "ActivityDensityValue") (declared-name "ActivityDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AngularFrequencyValue"))) (name "AngularFrequencyValue") (declared-name "AngularFrequencyValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AreaValue"))) (name "AreaValue") (declared-name "AreaValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit"))) (name "AtomicAttenuationCoefficientUnit") (declared-name "AtomicAttenuationCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue"))) (name "AtomicAttenuationCoefficientValue") (declared-name "AtomicAttenuationCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit"))) (name "AverageEnergyLossPerElementaryChargeProducedUnit") (declared-name "AverageEnergyLossPerElementaryChargeProducedUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue"))) (name "AverageEnergyLossPerElementaryChargeProducedValue") (declared-name "AverageEnergyLossPerElementaryChargeProducedValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue"))) (name "AverageLogarithmicEnergyDecrementValue") (declared-name "AverageLogarithmicEnergyDecrementValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue"))) (name "BindingFractionValue") (declared-name "BindingFractionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame"))) (name "CartesianMagneticDipoleMoment3dCoordinateFrame") (declared-name "CartesianMagneticDipoleMoment3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector"))) (name "CartesianMagneticDipoleMoment3dVector") (declared-name "CartesianMagneticDipoleMoment3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame"))) (name "CartesianParticleCurrentDensity3dCoordinateFrame") (declared-name "CartesianParticleCurrentDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector"))) (name "CartesianParticleCurrentDensity3dVector") (declared-name "CartesianParticleCurrentDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame"))) (name "CartesianSpin3dCoordinateFrame") (declared-name "CartesianSpin3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector"))) (name "CartesianSpin3dVector") (declared-name "CartesianSpin3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame"))) (name "CartesianTotalAngularMomentum3dCoordinateFrame") (declared-name "CartesianTotalAngularMomentum3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector"))) (name "CartesianTotalAngularMomentum3dVector") (declared-name "CartesianTotalAngularMomentum3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue"))) (name "ChargeNumberValue") (declared-name "ChargeNumberValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit"))) (name "DecayConstantUnit") (declared-name "DecayConstantUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue"))) (name "DecayConstantValue") (declared-name "DecayConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DiffusionCoefficientForParticleNumberDensityUnit"))) (name "DiffusionCoefficientForParticleNumberDensityUnit") (declared-name "DiffusionCoefficientForParticleNumberDensityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DiffusionCoefficientForParticleNumberDensityValue"))) (name "DiffusionCoefficientForParticleNumberDensityValue") (declared-name "DiffusionCoefficientForParticleNumberDensityValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DiffusionCoefficientUnit"))) (name "DiffusionCoefficientUnit") (declared-name "DiffusionCoefficientUnit"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DiffusionCoefficientValue"))) (name "DiffusionCoefficientValue") (declared-name "DiffusionCoefficientValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit"))) (name "DirectionAndEnergyDistributionOfCrossSectionUnit") (declared-name "DirectionAndEnergyDistributionOfCrossSectionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue"))) (name "DirectionAndEnergyDistributionOfCrossSectionValue") (declared-name "DirectionAndEnergyDistributionOfCrossSectionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit"))) (name "DirectionDistributionOfCrossSectionUnit") (declared-name "DirectionDistributionOfCrossSectionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue"))) (name "DirectionDistributionOfCrossSectionValue") (declared-name "DirectionDistributionOfCrossSectionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DisintegrationConstantUnit"))) (name "DisintegrationConstantUnit") (declared-name "DisintegrationConstantUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DisintegrationConstantValue"))) (name "DisintegrationConstantValue") (declared-name "DisintegrationConstantValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit"))) (name "DoseEquivalentUnit") (declared-name "DoseEquivalentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue"))) (name "DoseEquivalentValue") (declared-name "DoseEquivalentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ElectricChargeValue"))) (name "ElectricChargeValue") (declared-name "ElectricChargeValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit"))) (name "EnergyDistributionOfCrossSectionUnit") (declared-name "EnergyDistributionOfCrossSectionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue"))) (name "EnergyDistributionOfCrossSectionValue") (declared-name "EnergyDistributionOfCrossSectionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit"))) (name "EnergyFluenceRateUnit") (declared-name "EnergyFluenceRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue"))) (name "EnergyFluenceRateValue") (declared-name "EnergyFluenceRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit"))) (name "EnergyFluenceUnit") (declared-name "EnergyFluenceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue"))) (name "EnergyFluenceValue") (declared-name "EnergyFluenceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit"))) (name "ExposureRateUnit") (declared-name "ExposureRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue"))) (name "ExposureRateValue") (declared-name "ExposureRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit"))) (name "ExposureUnit") (declared-name "ExposureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue"))) (name "ExposureValue") (declared-name "ExposureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorUnit"))) (name "FastFissionFactorUnit") (declared-name "FastFissionFactorUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue"))) (name "FastFissionFactorValue") (declared-name "FastFissionFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue"))) (name "GFactorOfNucleusOrNuclearParticleValue") (declared-name "GFactorOfNucleusOrNuclearParticleValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticCoefficientOfTheElectronUnit"))) (name "GyromagneticCoefficientOfTheElectronUnit") (declared-name "GyromagneticCoefficientOfTheElectronUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticCoefficientOfTheElectronValue"))) (name "GyromagneticCoefficientOfTheElectronValue") (declared-name "GyromagneticCoefficientOfTheElectronValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticCoefficientUnit"))) (name "GyromagneticCoefficientUnit") (declared-name "GyromagneticCoefficientUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticCoefficientValue"))) (name "GyromagneticCoefficientValue") (declared-name "GyromagneticCoefficientValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit"))) (name "GyromagneticRatioOfTheElectronUnit") (declared-name "GyromagneticRatioOfTheElectronUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue"))) (name "GyromagneticRatioOfTheElectronValue") (declared-name "GyromagneticRatioOfTheElectronValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit"))) (name "GyromagneticRatioUnit") (declared-name "GyromagneticRatioUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue"))) (name "GyromagneticRatioValue") (declared-name "GyromagneticRatioValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit"))) (name "HartreeEnergyUnit") (declared-name "HartreeEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue"))) (name "HartreeEnergyValue") (declared-name "HartreeEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorUnit"))) (name "InfiniteMultiplicationFactorUnit") (declared-name "InfiniteMultiplicationFactorUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue"))) (name "InfiniteMultiplicationFactorValue") (declared-name "InfiniteMultiplicationFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue"))) (name "InternalConversionFactorValue") (declared-name "InternalConversionFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonDensityUnit"))) (name "IonDensityUnit") (declared-name "IonDensityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonDensityValue"))) (name "IonDensityValue") (declared-name "IonDensityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit"))) (name "IonNumberDensityUnit") (declared-name "IonNumberDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue"))) (name "IonNumberDensityValue") (declared-name "IonNumberDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit"))) (name "KermaRateUnit") (declared-name "KermaRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue"))) (name "KermaRateValue") (declared-name "KermaRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit"))) (name "KermaUnit") (declared-name "KermaUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue"))) (name "KermaValue") (declared-name "KermaValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue"))) (name "LandeFactorValue") (declared-name "LandeFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit"))) (name "LarmorFrequencyUnit") (declared-name "LarmorFrequencyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue"))) (name "LarmorFrequencyValue") (declared-name "LarmorFrequencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue"))) (name "LethargyValue") (declared-name "LethargyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit"))) (name "LinearAttenuationCoefficientForIonizingRadiationUnit") (declared-name "LinearAttenuationCoefficientForIonizingRadiationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue"))) (name "LinearAttenuationCoefficientForIonizingRadiationValue") (declared-name "LinearAttenuationCoefficientForIonizingRadiationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit"))) (name "LinearEnergyTransferUnit") (declared-name "LinearEnergyTransferUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue"))) (name "LinearEnergyTransferValue") (declared-name "LinearEnergyTransferValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit"))) (name "LinearIonizationUnit") (declared-name "LinearIonizationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue"))) (name "LinearIonizationValue") (declared-name "LinearIonizationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearStoppingPowerUnit"))) (name "LinearStoppingPowerUnit") (declared-name "LinearStoppingPowerUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearStoppingPowerValue"))) (name "LinearStoppingPowerValue") (declared-name "LinearStoppingPowerValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MacroscopicCrossSectionUnit"))) (name "MacroscopicCrossSectionUnit") (declared-name "MacroscopicCrossSectionUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MacroscopicCrossSectionValue"))) (name "MacroscopicCrossSectionValue") (declared-name "MacroscopicCrossSectionValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MacroscopicTotalCrossSectionUnit"))) (name "MacroscopicTotalCrossSectionUnit") (declared-name "MacroscopicTotalCrossSectionUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MacroscopicTotalCrossSectionValue"))) (name "MacroscopicTotalCrossSectionValue") (declared-name "MacroscopicTotalCrossSectionValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit"))) (name "MagneticDipoleMomentUnit") (declared-name "MagneticDipoleMomentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue"))) (name "MagneticDipoleMomentValue") (declared-name "MagneticDipoleMomentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagnetogyricRatioOfTheElectronUnit"))) (name "MagnetogyricRatioOfTheElectronUnit") (declared-name "MagnetogyricRatioOfTheElectronUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagnetogyricRatioOfTheElectronValue"))) (name "MagnetogyricRatioOfTheElectronValue") (declared-name "MagnetogyricRatioOfTheElectronValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagnetogyricRatioUnit"))) (name "MagnetogyricRatioUnit") (declared-name "MagnetogyricRatioUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MagnetogyricRatioValue"))) (name "MagnetogyricRatioValue") (declared-name "MagnetogyricRatioValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit"))) (name "MassAttenuationCoefficientForIonizingRadiationUnit") (declared-name "MassAttenuationCoefficientForIonizingRadiationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue"))) (name "MassAttenuationCoefficientForIonizingRadiationValue") (declared-name "MassAttenuationCoefficientForIonizingRadiationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit"))) (name "MassEnergyTransferCoefficientUnit") (declared-name "MassEnergyTransferCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue"))) (name "MassEnergyTransferCoefficientValue") (declared-name "MassEnergyTransferCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassStoppingPowerUnit"))) (name "MassStoppingPowerUnit") (declared-name "MassStoppingPowerUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassStoppingPowerValue"))) (name "MassStoppingPowerValue") (declared-name "MassStoppingPowerValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassicActivityUnit"))) (name "MassicActivityUnit") (declared-name "MassicActivityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MassicActivityValue"))) (name "MassicActivityValue") (declared-name "MassicActivityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit"))) (name "MeanMassRangeUnit") (declared-name "MeanMassRangeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue"))) (name "MeanMassRangeValue") (declared-name "MeanMassRangeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit"))) (name "MobilityUnit") (declared-name "MobilityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue"))) (name "MobilityValue") (declared-name "MobilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit"))) (name "MolarAttenuationCoefficientUnit") (declared-name "MolarAttenuationCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue"))) (name "MolarAttenuationCoefficientValue") (declared-name "MolarAttenuationCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorUnit"))) (name "MultiplicationFactorUnit") (declared-name "MultiplicationFactorUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue"))) (name "MultiplicationFactorValue") (declared-name "MultiplicationFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityUnit"))) (name "NonLeakageProbabilityUnit") (declared-name "NonLeakageProbabilityUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue"))) (name "NonLeakageProbabilityValue") (declared-name "NonLeakageProbabilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit"))) (name "NuclearActivityUnit") (declared-name "NuclearActivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue"))) (name "NuclearActivityValue") (declared-name "NuclearActivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit"))) (name "NuclearQuadrupoleMomentUnit") (declared-name "NuclearQuadrupoleMomentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue"))) (name "NuclearQuadrupoleMomentValue") (declared-name "NuclearQuadrupoleMomentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue"))) (name "PackingFractionValue") (declared-name "PackingFractionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit"))) (name "ParticleCurrentDensityUnit") (declared-name "ParticleCurrentDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue"))) (name "ParticleCurrentDensityValue") (declared-name "ParticleCurrentDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit"))) (name "ParticleEmissionRateUnit") (declared-name "ParticleEmissionRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue"))) (name "ParticleEmissionRateValue") (declared-name "ParticleEmissionRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit"))) (name "ParticleFluenceRateUnit") (declared-name "ParticleFluenceRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue"))) (name "ParticleFluenceRateValue") (declared-name "ParticleFluenceRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit"))) (name "ParticleFluenceUnit") (declared-name "ParticleFluenceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue"))) (name "ParticleFluenceValue") (declared-name "ParticleFluenceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit"))) (name "ParticleNumberDensityUnit") (declared-name "ParticleNumberDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue"))) (name "ParticleNumberDensityValue") (declared-name "ParticleNumberDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit"))) (name "ParticleSourceDensityUnit") (declared-name "ParticleSourceDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue"))) (name "ParticleSourceDensityValue") (declared-name "ParticleSourceDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationUnit"))) (name "QualityFactorForIonizingRadiationUnit") (declared-name "QualityFactorForIonizingRadiationUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue"))) (name "QualityFactorForIonizingRadiationValue") (declared-name "QualityFactorForIonizingRadiationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue"))) (name "QuantumNumberValue") (declared-name "QuantumNumberValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit"))) (name "RecombinationCoefficientUnit") (declared-name "RecombinationCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue"))) (name "RecombinationCoefficientValue") (declared-name "RecombinationCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue"))) (name "RelativeMassDefectValue") (declared-name "RelativeMassDefectValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue"))) (name "RelativeMassExcessValue") (declared-name "RelativeMassExcessValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue"))) (name "ResonanceEscapeProbabilityValue") (declared-name "ResonanceEscapeProbabilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit"))) (name "RydbergConstantUnit") (declared-name "RydbergConstantUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue"))) (name "RydbergConstantValue") (declared-name "RydbergConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit"))) (name "SlowingDownDensityUnit") (declared-name "SlowingDownDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue"))) (name "SlowingDownDensityValue") (declared-name "SlowingDownDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit"))) (name "SpecificActivityUnit") (declared-name "SpecificActivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue"))) (name "SpecificActivityValue") (declared-name "SpecificActivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit"))) (name "SpinUnit") (declared-name "SpinUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue"))) (name "SpinValue") (declared-name "SpinValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit"))) (name "SurfaceActivityDensityUnit") (declared-name "SurfaceActivityDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue"))) (name "SurfaceActivityDensityValue") (declared-name "SurfaceActivityDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorUnit"))) (name "ThermalUtilizationFactorUnit") (declared-name "ThermalUtilizationFactorUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue"))) (name "ThermalUtilizationFactorValue") (declared-name "ThermalUtilizationFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit"))) (name "TotalAngularMomentumUnit") (declared-name "TotalAngularMomentumUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue"))) (name "TotalAngularMomentumValue") (declared-name "TotalAngularMomentumValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue"))) (name "TotalIonizationValue") (declared-name "TotalIonizationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit"))) (name "TotalLinearStoppingPowerUnit") (declared-name "TotalLinearStoppingPowerUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue"))) (name "TotalLinearStoppingPowerValue") (declared-name "TotalLinearStoppingPowerValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit"))) (name "TotalMassStoppingPowerUnit") (declared-name "TotalMassStoppingPowerUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue"))) (name "TotalMassStoppingPowerValue") (declared-name "TotalMassStoppingPowerValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicActivityUnit"))) (name "VolumicActivityUnit") (declared-name "VolumicActivityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicActivityValue"))) (name "VolumicActivityValue") (declared-name "VolumicActivityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit"))) (name "VolumicCrossSectionUnit") (declared-name "VolumicCrossSectionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue"))) (name "VolumicCrossSectionValue") (declared-name "VolumicCrossSectionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit"))) (name "VolumicTotalCrossSectionUnit") (declared-name "VolumicTotalCrossSectionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue"))) (name "VolumicTotalCrossSectionValue") (declared-name "VolumicTotalCrossSectionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::absorbedDose"))) (name "absorbedDose") (declared-name "absorbedDose") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::absorbedDoseRate"))) (name "absorbedDoseRate") (declared-name "absorbedDoseRate") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::activityConcentration"))) (name "activityConcentration") (declared-name "activityConcentration"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::activityDensity"))) (name "activityDensity") (declared-name "activityDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::alphaDisintegrationEnergy"))) (name "alphaDisintegrationEnergy") (declared-name "alphaDisintegrationEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::alphaDisintegrationEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::alphaDisintegrationEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicAttenuationCoefficient"))) (name "atomicAttenuationCoefficient") (declared-name "atomicAttenuationCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicMass::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicMass")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicNumber"))) (name "atomicNumber") (declared-name "atomicNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::averageEnergyLossPerElementaryChargeProduced"))) (name "averageEnergyLossPerElementaryChargeProduced") (declared-name "averageEnergyLossPerElementaryChargeProduced") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::averageLogarithmicEnergyDecrement"))) (name "averageLogarithmicEnergyDecrement") (declared-name "averageLogarithmicEnergyDecrement") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::betaDisintegrationEnergy"))) (name "betaDisintegrationEnergy") (declared-name "betaDisintegrationEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::betaDisintegrationEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::betaDisintegrationEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::bindingFraction"))) (name "bindingFraction") (declared-name "bindingFraction") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton"))) (name "bohrMagneton") (declared-name "bohrMagneton") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrRadius"))) (name "bohrRadius") (declared-name "bohrRadius") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrRadius::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrRadius")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianMagneticDipoleMoment3dVector"))) (name "cartesianMagneticDipoleMoment3dVector") (declared-name "cartesianMagneticDipoleMoment3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianParticleCurrentDensity3dVector"))) (name "cartesianParticleCurrentDensity3dVector") (declared-name "cartesianParticleCurrentDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianSpin3dVector"))) (name "cartesianSpin3dVector") (declared-name "cartesianSpin3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianTotalAngularMomentum3dVector"))) (name "cartesianTotalAngularMomentum3dVector") (declared-name "cartesianTotalAngularMomentum3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::chargeNumber"))) (name "chargeNumber") (declared-name "chargeNumber") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::comptonWavelength"))) (name "comptonWavelength") (declared-name "comptonWavelength") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::comptonWavelength::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::comptonWavelength")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::crossSection"))) (name "crossSection") (declared-name "crossSection") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::crossSection::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::crossSection")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::cyclotronAngularFrequency"))) (name "cyclotronAngularFrequency") (declared-name "cyclotronAngularFrequency") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::cyclotronAngularFrequency::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::cyclotronAngularFrequency")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::decayConstant"))) (name "decayConstant") (declared-name "decayConstant") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionArea"))) (name "diffusionArea") (declared-name "diffusionArea") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionArea::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionArea")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficient"))) (name "diffusionCoefficient") (declared-name "diffusionCoefficient"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForFluenceRate"))) (name "diffusionCoefficientForFluenceRate") (declared-name "diffusionCoefficientForFluenceRate") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForFluenceRate::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForFluenceRate")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForParticleNumberDensity"))) (name "diffusionCoefficientForParticleNumberDensity") (declared-name "diffusionCoefficientForParticleNumberDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionLength"))) (name "diffusionLength") (declared-name "diffusionLength") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionLength::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionLength")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::directionAndEnergyDistributionOfCrossSection"))) (name "directionAndEnergyDistributionOfCrossSection") (declared-name "directionAndEnergyDistributionOfCrossSection") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::directionDistributionOfCrossSection"))) (name "directionDistributionOfCrossSection") (declared-name "directionDistributionOfCrossSection") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::disintegrationConstant"))) (name "disintegrationConstant") (declared-name "disintegrationConstant"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalent"))) (name "doseEquivalent") (declared-name "doseEquivalent") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate"))) (name "doseEquivalentRate") (declared-name "doseEquivalentRate") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::electronRadius"))) (name "electronRadius") (declared-name "electronRadius") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::electronRadius::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::electronRadius")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::elementaryCharge"))) (name "elementaryCharge") (declared-name "elementaryCharge") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::elementaryCharge::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::elementaryCharge")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::energyDistributionOfCrossSection"))) (name "energyDistributionOfCrossSection") (declared-name "energyDistributionOfCrossSection") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::energyFluence"))) (name "energyFluence") (declared-name "energyFluence") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::energyFluenceRate"))) (name "energyFluenceRate") (declared-name "energyFluenceRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::energyImparted"))) (name "energyImparted") (declared-name "energyImparted") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::energyImparted::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::energyImparted")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::exposure"))) (name "exposure") (declared-name "exposure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::exposureRate"))) (name "exposureRate") (declared-name "exposureRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::fastFissionFactor"))) (name "fastFissionFactor") (declared-name "fastFissionFactor") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gFactorOfAtom"))) (name "gFactorOfAtom") (declared-name "gFactorOfAtom"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gFactorOfNucleusOrNuclearParticle"))) (name "gFactorOfNucleusOrNuclearParticle") (declared-name "gFactorOfNucleusOrNuclearParticle") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticCoefficient"))) (name "gyromagneticCoefficient") (declared-name "gyromagneticCoefficient"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticCoefficientOfTheElectron"))) (name "gyromagneticCoefficientOfTheElectron") (declared-name "gyromagneticCoefficientOfTheElectron"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticRatio"))) (name "gyromagneticRatio") (declared-name "gyromagneticRatio") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticRatioOfTheElectron"))) (name "gyromagneticRatioOfTheElectron") (declared-name "gyromagneticRatioOfTheElectron") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gyroradius"))) (name "gyroradius") (declared-name "gyroradius") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::gyroradius::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::gyroradius")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::halfLife"))) (name "halfLife") (declared-name "halfLife") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::halfLife::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::halfLife")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::halfValueThickness"))) (name "halfValueThickness") (declared-name "halfValueThickness") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::halfValueThickness::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::halfValueThickness")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::hartreeEnergy"))) (name "hartreeEnergy") (declared-name "hartreeEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::hyperfineStructureQuantumNumber"))) (name "hyperfineStructureQuantumNumber") (declared-name "hyperfineStructureQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::hyperfineStructureQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::hyperfineStructureQuantumNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::infiniteMultiplicationFactor"))) (name "infiniteMultiplicationFactor") (declared-name "infiniteMultiplicationFactor") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::internalConversionFactor"))) (name "internalConversionFactor") (declared-name "internalConversionFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ionDensity"))) (name "ionDensity") (declared-name "ionDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ionNumberDensity"))) (name "ionNumberDensity") (declared-name "ionNumberDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::ionizationNumber"))) (name "ionizationNumber") (declared-name "ionizationNumber"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::kerma"))) (name "kerma") (declared-name "kerma") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::kermaRate"))) (name "kermaRate") (declared-name "kermaRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::landeFactor"))) (name "landeFactor") (declared-name "landeFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorAngularFrequency"))) (name "larmorAngularFrequency") (declared-name "larmorAngularFrequency") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorAngularFrequency::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorAngularFrequency")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorFrequency"))) (name "larmorFrequency") (declared-name "larmorFrequency") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorRadius"))) (name "larmorRadius") (declared-name "larmorRadius"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::lethargy"))) (name "lethargy") (declared-name "lethargy") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::levelWidth"))) (name "levelWidth") (declared-name "levelWidth") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::levelWidth::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::levelWidth")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::linearAttenuationCoefficientForIonizingRadiation"))) (name "linearAttenuationCoefficientForIonizingRadiation") (declared-name "linearAttenuationCoefficientForIonizingRadiation") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::linearEnergyTransfer"))) (name "linearEnergyTransfer") (declared-name "linearEnergyTransfer") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::linearIonization"))) (name "linearIonization") (declared-name "linearIonization") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::linearStoppingPower"))) (name "linearStoppingPower") (declared-name "linearStoppingPower"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::macroscopicCrossSection"))) (name "macroscopicCrossSection") (declared-name "macroscopicCrossSection"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::macroscopicTotalCrossSection"))) (name "macroscopicTotalCrossSection") (declared-name "macroscopicTotalCrossSection"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticDipoleMoment"))) (name "magneticDipoleMoment") (declared-name "magneticDipoleMoment") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticQuantumNumber"))) (name "magneticQuantumNumber") (declared-name "magneticQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticQuantumNumber")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::magnetogyricRatio"))) (name "magnetogyricRatio") (declared-name "magnetogyricRatio"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::magnetogyricRatioOfTheElectron"))) (name "magnetogyricRatioOfTheElectron") (declared-name "magnetogyricRatioOfTheElectron"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massAttenuationCoefficientForIonizingRadiation"))) (name "massAttenuationCoefficientForIonizingRadiation") (declared-name "massAttenuationCoefficientForIonizingRadiation") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massDefect"))) (name "massDefect") (declared-name "massDefect") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massDefect::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::massDefect")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massEnergyTransferCoefficient"))) (name "massEnergyTransferCoefficient") (declared-name "massEnergyTransferCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massExcess"))) (name "massExcess") (declared-name "massExcess") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massExcess::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::massExcess")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massNumber"))) (name "massNumber") (declared-name "massNumber"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massStoppingPower"))) (name "massStoppingPower") (declared-name "massStoppingPower"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::massicActivity"))) (name "massicActivity") (declared-name "massicActivity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::maximumBetaParticleEnergy"))) (name "maximumBetaParticleEnergy") (declared-name "maximumBetaParticleEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::maximumBetaParticleEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::maximumBetaParticleEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanDurationOfLife"))) (name "meanDurationOfLife") (declared-name "meanDurationOfLife") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanDurationOfLife::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::meanDurationOfLife")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanEnergyImparted"))) (name "meanEnergyImparted") (declared-name "meanEnergyImparted") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanEnergyImparted::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::meanEnergyImparted")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanFreePathForAtomicPhysics"))) (name "meanFreePathForAtomicPhysics") (declared-name "meanFreePathForAtomicPhysics") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanFreePathForAtomicPhysics::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::meanFreePathForAtomicPhysics")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLifeTime"))) (name "meanLifeTime") (declared-name "meanLifeTime"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLinearRange"))) (name "meanLinearRange") (declared-name "meanLinearRange") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLinearRange::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLinearRange")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::meanMassRange"))) (name "meanMassRange") (declared-name "meanMassRange") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationArea"))) (name "migrationArea") (declared-name "migrationArea") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationArea::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationArea")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationLength"))) (name "migrationLength") (declared-name "migrationLength") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationLength::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationLength")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::mobility"))) (name "mobility") (declared-name "mobility") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::molarAttenuationCoefficient"))) (name "molarAttenuationCoefficient") (declared-name "molarAttenuationCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::multiplicationFactor"))) (name "multiplicationFactor") (declared-name "multiplicationFactor") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronNumber"))) (name "neutronNumber") (declared-name "neutronNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerAbsorption"))) (name "neutronYieldPerAbsorption") (declared-name "neutronYieldPerAbsorption") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerAbsorption::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerAbsorption")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerFission"))) (name "neutronYieldPerFission") (declared-name "neutronYieldPerFission") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerFission::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerFission")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nonLeakageProbability"))) (name "nonLeakageProbability") (declared-name "nonLeakageProbability") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearActivity"))) (name "nuclearActivity") (declared-name "nuclearActivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton"))) (name "nuclearMagneton") (declared-name "nuclearMagneton") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearPrecessionAngularFrequency"))) (name "nuclearPrecessionAngularFrequency") (declared-name "nuclearPrecessionAngularFrequency") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearPrecessionAngularFrequency::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearPrecessionAngularFrequency")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearQuadrupoleMoment"))) (name "nuclearQuadrupoleMoment") (declared-name "nuclearQuadrupoleMoment") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearRadius"))) (name "nuclearRadius") (declared-name "nuclearRadius") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearRadius::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearRadius")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearSpinQuantumNumber"))) (name "nuclearSpinQuantumNumber") (declared-name "nuclearSpinQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearSpinQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearSpinQuantumNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nucleonNumber"))) (name "nucleonNumber") (declared-name "nucleonNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nucleonNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::nucleonNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclidicMass"))) (name "nuclidicMass") (declared-name "nuclidicMass") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclidicMass::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclidicMass")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::orbitalAngularMomentumQuantumNumber"))) (name "orbitalAngularMomentumQuantumNumber") (declared-name "orbitalAngularMomentumQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::orbitalAngularMomentumQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::orbitalAngularMomentumQuantumNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::packingFraction"))) (name "packingFraction") (declared-name "packingFraction") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::particleCurrentDensity"))) (name "particleCurrentDensity") (declared-name "particleCurrentDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::particleEmissionRate"))) (name "particleEmissionRate") (declared-name "particleEmissionRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::particleFluence"))) (name "particleFluence") (declared-name "particleFluence") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::particleFluenceRate"))) (name "particleFluenceRate") (declared-name "particleFluenceRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::particleNumberDensity"))) (name "particleNumberDensity") (declared-name "particleNumberDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::particleSourceDensity"))) (name "particleSourceDensity") (declared-name "particleSourceDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::principalQuantumNumber"))) (name "principalQuantumNumber") (declared-name "principalQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::principalQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::principalQuantumNumber")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::properMass"))) (name "properMass") (declared-name "properMass"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::protonNumber"))) (name "protonNumber") (declared-name "protonNumber"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::qualityFactorForIonizingRadiation"))) (name "qualityFactorForIonizingRadiation") (declared-name "qualityFactorForIonizingRadiation") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::quantumNumber"))) (name "quantumNumber") (declared-name "quantumNumber") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::radiantEnergyForIonizingRadiation"))) (name "radiantEnergyForIonizingRadiation") (declared-name "radiantEnergyForIonizingRadiation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::radiantEnergyForIonizingRadiation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::radiantEnergyForIonizingRadiation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::reactionEnergy"))) (name "reactionEnergy") (declared-name "reactionEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::reactionEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::reactionEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::reactorTimeConstant"))) (name "reactorTimeConstant") (declared-name "reactorTimeConstant") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::reactorTimeConstant::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::reactorTimeConstant")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::recombinationCoefficient"))) (name "recombinationCoefficient") (declared-name "recombinationCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::relativeMassDefect"))) (name "relativeMassDefect") (declared-name "relativeMassDefect") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::relativeMassExcess"))) (name "relativeMassExcess") (declared-name "relativeMassExcess") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEnergy"))) (name "resonanceEnergy") (declared-name "resonanceEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEscapeProbability"))) (name "resonanceEscapeProbability") (declared-name "resonanceEscapeProbability") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::restEnergy"))) (name "restEnergy") (declared-name "restEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::restEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::restEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::restMass"))) (name "restMass") (declared-name "restMass") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::restMass::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::restMass")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::rydbergConstant"))) (name "rydbergConstant") (declared-name "rydbergConstant") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownArea"))) (name "slowingDownArea") (declared-name "slowingDownArea") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownArea::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownArea")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownDensity"))) (name "slowingDownDensity") (declared-name "slowingDownDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownLength"))) (name "slowingDownLength") (declared-name "slowingDownLength") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownLength::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownLength")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::specificActivity"))) (name "specificActivity") (declared-name "specificActivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted"))) (name "specificEnergyImparted") (declared-name "specificEnergyImparted") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::spin"))) (name "spin") (declared-name "spin") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::spinQuantumNumber"))) (name "spinQuantumNumber") (declared-name "spinQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::spinQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::spinQuantumNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::surfaceActivityDensity"))) (name "surfaceActivityDensity") (declared-name "surfaceActivityDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::thermalUtilizationFactor"))) (name "thermalUtilizationFactor") (declared-name "thermalUtilizationFactor") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentum"))) (name "totalAngularMomentum") (declared-name "totalAngularMomentum") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentumQuantumNumber"))) (name "totalAngularMomentumQuantumNumber") (declared-name "totalAngularMomentumQuantumNumber") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentumQuantumNumber::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentumQuantumNumber")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalCrossSection"))) (name "totalCrossSection") (declared-name "totalCrossSection") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalCrossSection::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::totalCrossSection")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalIonization"))) (name "totalIonization") (declared-name "totalIonization") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalLinearStoppingPower"))) (name "totalLinearStoppingPower") (declared-name "totalLinearStoppingPower") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::totalMassStoppingPower"))) (name "totalMassStoppingPower") (declared-name "totalMassStoppingPower") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::unifiedAtomicMassConstant"))) (name "unifiedAtomicMassConstant") (declared-name "unifiedAtomicMassConstant") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::unifiedAtomicMassConstant::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAtomicNuclear::unifiedAtomicMassConstant")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicActivity"))) (name "volumicActivity") (declared-name "volumicActivity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicCrossSection"))) (name "volumicCrossSection") (declared-name "volumicCrossSection") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicTotalCrossSection"))) (name "volumicTotalCrossSection") (declared-name "volumicTotalCrossSection") (declared (properties (ordered false) (unique false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::alphaDisintegrationEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::alphaDisintegrationEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicMass::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicMass"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::betaDisintegrationEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::betaDisintegrationEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrRadius::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrRadius"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::comptonWavelength::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::comptonWavelength"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::crossSection::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::crossSection"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cyclotronAngularFrequency::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::cyclotronAngularFrequency"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionArea::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionArea"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForFluenceRate::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForFluenceRate"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionLength::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionLength"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::electronRadius::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::electronRadius"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::elementaryCharge::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::elementaryCharge"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyImparted::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::energyImparted"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gyroradius::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::gyroradius"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::halfLife::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::halfLife"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::halfValueThickness::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::halfValueThickness"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::hyperfineStructureQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::hyperfineStructureQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorAngularFrequency::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorAngularFrequency"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::levelWidth::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::levelWidth"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massDefect::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::massDefect"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massExcess::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::massExcess"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::maximumBetaParticleEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::maximumBetaParticleEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanDurationOfLife::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::meanDurationOfLife"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanEnergyImparted::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::meanEnergyImparted"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanFreePathForAtomicPhysics::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::meanFreePathForAtomicPhysics"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLinearRange::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLinearRange"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationArea::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationArea"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationLength::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationLength"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerAbsorption::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerAbsorption"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerFission::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerFission"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearPrecessionAngularFrequency::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearPrecessionAngularFrequency"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearRadius::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearRadius"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearSpinQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearSpinQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nucleonNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::nucleonNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclidicMass::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclidicMass"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::orbitalAngularMomentumQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::orbitalAngularMomentumQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::principalQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::principalQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::radiantEnergyForIonizingRadiation::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::radiantEnergyForIonizingRadiation"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::reactionEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::reactionEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::reactorTimeConstant::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::reactorTimeConstant"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::restEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::restEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::restMass::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::restMass"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownArea::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownArea"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownLength::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownLength"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::spinQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::spinQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentumQuantumNumber::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentumQuantumNumber"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalCrossSection::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::totalCrossSection"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::unifiedAtomicMassConstant::_documentation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::unifiedAtomicMassConstant"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::absorbedDose"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::absorbedDoseRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::activityDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicAttenuationCoefficient"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::averageEnergyLossPerElementaryChargeProduced"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::averageLogarithmicEnergyDecrement"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bindingFraction"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianMagneticDipoleMoment3dVector"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianParticleCurrentDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianSpin3dVector"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianTotalAngularMomentum3dVector"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::chargeNumber"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::decayConstant"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::directionAndEnergyDistributionOfCrossSection"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::directionDistributionOfCrossSection"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalent"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyDistributionOfCrossSection"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyFluence"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyFluenceRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::exposure"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::exposureRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::fastFissionFactor"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gFactorOfNucleusOrNuclearParticle"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticRatio"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticRatioOfTheElectron"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::hartreeEnergy"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::infiniteMultiplicationFactor"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::internalConversionFactor"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ionNumberDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::kerma"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::kermaRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::landeFactor"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorFrequency"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::lethargy"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::linearAttenuationCoefficientForIonizingRadiation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::linearEnergyTransfer"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::linearIonization"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticDipoleMoment"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massAttenuationCoefficientForIonizingRadiation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massEnergyTransferCoefficient"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanMassRange"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::mobility"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::molarAttenuationCoefficient"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::multiplicationFactor"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nonLeakageProbability"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearActivity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearQuadrupoleMoment"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::packingFraction"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleCurrentDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleEmissionRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleFluence"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleFluenceRate"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleNumberDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleSourceDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::qualityFactorForIonizingRadiation"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::quantumNumber"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::recombinationCoefficient"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::relativeMassDefect"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::relativeMassExcess"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEscapeProbability"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::rydbergConstant"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::specificActivity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::spin"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::surfaceActivityDensity"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::thermalUtilizationFactor"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentum"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalIonization"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalLinearStoppingPower"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalMassStoppingPower"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicCrossSection"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicTotalCrossSection"))) (to (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AbsorbedDoseValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ActivityDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AtomicAttenuationCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageEnergyLossPerElementaryChargeProducedValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::AverageLogarithmicEnergyDecrementValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::BindingFractionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianMagneticDipoleMoment3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianParticleCurrentDensity3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianSpin3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::CartesianTotalAngularMomentum3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ChargeNumberValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DecayConstantValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionAndEnergyDistributionOfCrossSectionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DirectionDistributionOfCrossSectionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::DoseEquivalentValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyDistributionOfCrossSectionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::EnergyFluenceValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit::electricCurrentPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::electricCurrentPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ExposureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::FastFissionFactorValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GFactorOfNucleusOrNuclearParticleValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::electricCurrentPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioOfTheElectronValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::electricCurrentPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::GyromagneticRatioValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::HartreeEnergyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InfiniteMultiplicationFactorValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::InternalConversionFactorValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::IonNumberDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::KermaValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LandeFactorValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LarmorFrequencyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LethargyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearAttenuationCoefficientForIonizingRadiationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearEnergyTransferValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::LinearIonizationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit::electricCurrentPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MagneticDipoleMomentValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassAttenuationCoefficientForIonizingRadiationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MassEnergyTransferCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MeanMassRangeValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::electricCurrentPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MobilityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit::amountOfSubstancePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MolarAttenuationCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::MultiplicationFactorValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NonLeakageProbabilityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearActivityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::NuclearQuadrupoleMomentValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::PackingFractionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleCurrentDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleEmissionRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleFluenceValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleNumberDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ParticleSourceDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QualityFactorForIonizingRadiationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::QuantumNumberValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RecombinationCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassDefectValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RelativeMassExcessValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ResonanceEscapeProbabilityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::RydbergConstantValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SlowingDownDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpecificActivityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SpinValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::SurfaceActivityDensityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ThermalUtilizationFactorValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalAngularMomentumValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalIonizationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalLinearStoppingPowerValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::TotalMassStoppingPowerValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicCrossSectionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::VolumicTotalCrossSectionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::absorbedDose"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::absorbedDoseRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::activityDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::alphaDisintegrationEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicAttenuationCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicMass"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::atomicNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::averageEnergyLossPerElementaryChargeProduced"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::averageLogarithmicEnergyDecrement"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::betaDisintegrationEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bindingFraction"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrMagneton"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::bohrRadius"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianMagneticDipoleMoment3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianParticleCurrentDensity3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianSpin3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cartesianTotalAngularMomentum3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::chargeNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::comptonWavelength"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::crossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::cyclotronAngularFrequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::decayConstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionArea"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionCoefficientForFluenceRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::diffusionLength"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::directionAndEnergyDistributionOfCrossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::directionDistributionOfCrossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalent"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::doseEquivalentRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::electronRadius"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::elementaryCharge"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyDistributionOfCrossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyFluence"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyFluenceRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::energyImparted"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::exposure"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::exposureRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::fastFissionFactor"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gFactorOfNucleusOrNuclearParticle"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticRatio"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gyromagneticRatioOfTheElectron"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::gyroradius"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::halfLife"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::halfValueThickness"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::hartreeEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::hyperfineStructureQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::infiniteMultiplicationFactor"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::internalConversionFactor"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::ionNumberDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::kerma"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::kermaRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::landeFactor"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorAngularFrequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::larmorFrequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::lethargy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::levelWidth"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::linearAttenuationCoefficientForIonizingRadiation"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::linearEnergyTransfer"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::linearIonization"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticDipoleMoment"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::magneticQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massAttenuationCoefficientForIonizingRadiation"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massDefect"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massEnergyTransferCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::massExcess"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::maximumBetaParticleEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanDurationOfLife"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanEnergyImparted"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanFreePathForAtomicPhysics"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanLinearRange"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::meanMassRange"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationArea"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::migrationLength"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::mobility"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::molarAttenuationCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::multiplicationFactor"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerAbsorption"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::neutronYieldPerFission"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nonLeakageProbability"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearActivity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearMagneton"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearPrecessionAngularFrequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearQuadrupoleMoment"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearRadius"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclearSpinQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nucleonNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::nuclidicMass"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::orbitalAngularMomentumQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::packingFraction"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleCurrentDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleEmissionRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleFluence"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleFluenceRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleNumberDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::particleSourceDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::principalQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::qualityFactorForIonizingRadiation"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::quantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::radiantEnergyForIonizingRadiation"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::reactionEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::reactorTimeConstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::recombinationCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::relativeMassDefect"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::relativeMassExcess"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::resonanceEscapeProbability"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::restEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::restMass"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::rydbergConstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownArea"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::slowingDownLength"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::specificActivity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::specificEnergyImparted"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::spin"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::spinQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::surfaceActivityDensity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::thermalUtilizationFactor"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentum"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalAngularMomentumQuantumNumber"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalCrossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalIonization"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalLinearStoppingPower"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::totalMassStoppingPower"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::unifiedAtomicMassConstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicCrossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQAtomicNuclear::volumicTotalCrossSection"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/isq_atomic_nuclear.md"
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
        (range (start 20 19) (end 20 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 19) (end 21 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 19) (end 22 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 19) (end 23 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 19) (end 24 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 19) (end 25 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 19) (end 26 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 4) (end 29 838))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 4) (end 47 571))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 63 4) (end 63 526))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 4) (end 81 699))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 4) (end 99 608))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 4) (end 115 618))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 4) (end 131 579))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 4) (end 147 647))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 587))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 179 4) (end 179 837))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 4) (end 198 899))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 214 4) (end 214 911))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 227 8) (end 227 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 227 8) (end 227 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 228 8) (end 228 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 233 4) (end 233 245))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 234 8) (end 234 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 235 8) (end 235 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 239 4) (end 239 810))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 252 8) (end 252 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 252 8) (end 252 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 253 8) (end 253 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 258 4) (end 258 471))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 259 8) (end 259 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 260 8) (end 260 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 261 8) (end 261 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 262 8) (end 262 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 266 4) (end 266 1095))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 279 8) (end 279 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 279 8) (end 279 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 280 8) (end 280 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 285 4) (end 285 382))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 8) (end 286 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 287 8) (end 287 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 288 8) (end 288 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 291 4) (end 291 1141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 304 8) (end 304 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 305 8) (end 305 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 310 4) (end 310 235))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 311 8) (end 311 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 312 8) (end 312 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 313 8) (end 313 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 349 4) (end 349 616))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 362 8) (end 362 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 362 8) (end 362 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 363 8) (end 363 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 368 4) (end 368 462))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 369 8) (end 369 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 370 8) (end 370 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 371 8) (end 371 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 372 8) (end 372 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 375 4) (end 375 662))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 388 8) (end 388 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 389 8) (end 389 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 394 4) (end 394 203))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 395 8) (end 395 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 396 8) (end 396 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 397 8) (end 397 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 401 4) (end 401 1134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 414 8) (end 414 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 414 8) (end 414 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 415 8) (end 415 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 420 4) (end 420 478))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 421 8) (end 421 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 422 8) (end 422 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 423 8) (end 423 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 424 8) (end 424 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 427 4) (end 427 1180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 440 8) (end 440 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 441 8) (end 441 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 446 4) (end 446 235))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 447 8) (end 447 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 448 8) (end 448 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 449 8) (end 449 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 453 4) (end 453 1054))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 466 8) (end 466 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 466 8) (end 466 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 467 8) (end 467 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 472 4) (end 472 493))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 473 8) (end 473 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 474 8) (end 474 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 475 8) (end 475 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 476 8) (end 476 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 488 4) (end 488 950))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 501 8) (end 501 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 501 8) (end 501 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 502 8) (end 502 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 507 4) (end 507 506))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 508 8) (end 508 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 509 8) (end 509 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 510 8) (end 510 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 511 8) (end 511 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 523 4) (end 523 1237))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 540 4) (end 540 819))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 556 4) (end 556 1023))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 572 4) (end 572 913))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 588 4) (end 588 680))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 604 4) (end 604 822))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 620 4) (end 620 987))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 636 4) (end 636 843))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 652 4) (end 652 969))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 671 4) (end 671 859))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 688 4) (end 688 786))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 704 4) (end 704 547))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 717 8) (end 717 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 717 8) (end 717 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 718 8) (end 718 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 723 4) (end 723 249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 724 8) (end 724 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 725 8) (end 725 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 729 4) (end 729 768))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 745 4) (end 745 903))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 761 4) (end 761 697))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 779 4) (end 779 1017))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 792 8) (end 792 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 792 8) (end 792 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 793 8) (end 793 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 798 4) (end 798 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 799 8) (end 799 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 800 8) (end 800 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 804 4) (end 804 757))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 820 4) (end 820 995))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 836 4) (end 836 868))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 852 4) (end 852 759))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 868 4) (end 868 999))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 884 4) (end 884 620))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 901 4) (end 901 616))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 918 4) (end 918 595))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 935 4) (end 935 592))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 952 4) (end 952 944))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 965 8) (end 965 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 965 8) (end 965 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 966 8) (end 966 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 971 4) (end 971 247))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 972 8) (end 972 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 973 8) (end 973 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 981 4) (end 981 740))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 999 4) (end 999 1130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1015 4) (end 1015 1319))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1028 8) (end 1028 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1028 8) (end 1028 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1029 8) (end 1029 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1034 4) (end 1034 249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1035 8) (end 1035 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1036 8) (end 1036 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1040 4) (end 1040 633))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1053 8) (end 1053 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1053 8) (end 1053 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1054 8) (end 1054 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1059 4) (end 1059 362))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1060 8) (end 1060 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1061 8) (end 1061 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1062 8) (end 1062 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1070 4) (end 1070 656))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1083 8) (end 1083 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1083 8) (end 1083 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1084 8) (end 1084 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1089 4) (end 1089 365))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1090 8) (end 1090 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1091 8) (end 1091 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1092 8) (end 1092 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1104 4) (end 1104 790))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1117 8) (end 1117 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1117 8) (end 1117 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1118 8) (end 1118 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1123 4) (end 1123 372))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1124 8) (end 1124 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1125 8) (end 1125 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1126 8) (end 1126 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1130 4) (end 1130 577))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1146 4) (end 1146 864))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1162 4) (end 1162 667))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1178 4) (end 1178 1009))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1194 4) (end 1194 949))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1211 4) (end 1211 838))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1224 8) (end 1224 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1224 8) (end 1224 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1225 8) (end 1225 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1230 4) (end 1230 254))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1231 8) (end 1231 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1232 8) (end 1232 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1236 4) (end 1236 708))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1252 4) (end 1252 698))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1268 4) (end 1268 856))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1284 4) (end 1284 854))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1300 4) (end 1300 982))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1313 8) (end 1313 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1313 8) (end 1313 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1314 8) (end 1314 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1319 4) (end 1319 264))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1320 8) (end 1320 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1321 8) (end 1321 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1325 4) (end 1325 819))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1338 8) (end 1338 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1338 8) (end 1338 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1339 8) (end 1339 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1344 4) (end 1344 377))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1345 8) (end 1345 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1346 8) (end 1346 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1347 8) (end 1347 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1351 4) (end 1351 942))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1364 8) (end 1364 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1364 8) (end 1364 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1365 8) (end 1365 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1370 4) (end 1370 389))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1371 8) (end 1371 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1372 8) (end 1372 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1373 8) (end 1373 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1377 4) (end 1377 781))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1390 8) (end 1390 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1390 8) (end 1390 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1391 8) (end 1391 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1396 4) (end 1396 249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1397 8) (end 1397 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1398 8) (end 1398 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1406 4) (end 1406 756))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1419 8) (end 1419 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1419 8) (end 1419 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1420 8) (end 1420 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1425 4) (end 1425 254))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1426 8) (end 1426 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1427 8) (end 1427 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1435 4) (end 1435 1331))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1448 8) (end 1448 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1448 8) (end 1448 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1449 8) (end 1449 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1454 4) (end 1454 245))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1455 8) (end 1455 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1456 8) (end 1456 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1460 4) (end 1460 1474))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1473 8) (end 1473 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1473 8) (end 1473 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1474 8) (end 1474 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1479 4) (end 1479 369))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1480 8) (end 1480 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1481 8) (end 1481 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1482 8) (end 1482 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1486 4) (end 1486 1116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1502 4) (end 1502 919))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1515 8) (end 1515 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1515 8) (end 1515 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1516 8) (end 1516 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1521 4) (end 1521 358))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1522 8) (end 1522 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1523 8) (end 1523 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1524 8) (end 1524 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1528 4) (end 1528 905))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1541 8) (end 1541 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1541 8) (end 1541 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1542 8) (end 1542 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1547 4) (end 1547 362))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1548 8) (end 1548 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1549 8) (end 1549 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1550 8) (end 1550 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1554 4) (end 1554 1359))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1567 8) (end 1567 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1567 8) (end 1567 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1568 8) (end 1568 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1573 4) (end 1573 372))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1574 8) (end 1574 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1575 8) (end 1575 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1576 8) (end 1576 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1579 4) (end 1579 1410))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1592 8) (end 1592 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1593 8) (end 1593 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1598 4) (end 1598 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1599 8) (end 1599 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1600 8) (end 1600 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1601 8) (end 1601 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1605 4) (end 1605 1779))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1618 8) (end 1618 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1618 8) (end 1618 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1619 8) (end 1619 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1624 4) (end 1624 278))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1625 8) (end 1625 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1626 8) (end 1626 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1630 4) (end 1630 726))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1643 8) (end 1643 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1643 8) (end 1643 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1644 8) (end 1644 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1649 4) (end 1649 387))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1650 8) (end 1650 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1651 8) (end 1651 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1652 8) (end 1652 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1656 4) (end 1656 666))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1669 8) (end 1669 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1669 8) (end 1669 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1670 8) (end 1670 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1675 4) (end 1675 394))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1676 8) (end 1676 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1677 8) (end 1677 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1678 8) (end 1678 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1682 4) (end 1682 800))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1695 8) (end 1695 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1695 8) (end 1695 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1696 8) (end 1696 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1701 4) (end 1701 257))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1702 8) (end 1702 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1703 8) (end 1703 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1707 4) (end 1707 703))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1723 4) (end 1723 1781))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1736 8) (end 1736 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1736 8) (end 1736 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1737 8) (end 1737 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1742 4) (end 1742 482))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1743 8) (end 1743 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1744 8) (end 1744 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1745 8) (end 1745 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1746 8) (end 1746 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1754 4) (end 1754 820))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1767 8) (end 1767 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1767 8) (end 1767 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1768 8) (end 1768 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1773 4) (end 1773 371))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1774 8) (end 1774 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1775 8) (end 1775 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1776 8) (end 1776 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1784 4) (end 1784 633))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1800 4) (end 1800 614))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1813 8) (end 1813 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1813 8) (end 1813 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1814 8) (end 1814 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1819 4) (end 1819 354))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1820 8) (end 1820 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1821 8) (end 1821 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1822 8) (end 1822 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1826 4) (end 1826 821))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1839 8) (end 1839 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1839 8) (end 1839 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1840 8) (end 1840 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1845 4) (end 1845 246))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1846 8) (end 1846 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1847 8) (end 1847 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1851 4) (end 1851 678))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1868 4) (end 1868 1841))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1881 8) (end 1881 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1881 8) (end 1881 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1882 8) (end 1882 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1887 4) (end 1887 502))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1888 8) (end 1888 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1889 8) (end 1889 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1890 8) (end 1890 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1891 8) (end 1891 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1895 4) (end 1895 646))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1908 8) (end 1908 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1908 8) (end 1908 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1909 8) (end 1909 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1914 4) (end 1914 484))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1915 8) (end 1915 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1916 8) (end 1916 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1917 8) (end 1917 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1918 8) (end 1918 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1922 4) (end 1922 918))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1935 8) (end 1935 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1935 8) (end 1935 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1936 8) (end 1936 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1941 4) (end 1941 251))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1942 8) (end 1942 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1943 8) (end 1943 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1947 4) (end 1947 696))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1960 8) (end 1960 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1960 8) (end 1960 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1961 8) (end 1961 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1966 4) (end 1966 246))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1967 8) (end 1967 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1968 8) (end 1968 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1976 4) (end 1976 996))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1989 8) (end 1989 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1989 8) (end 1989 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1990 8) (end 1990 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1995 4) (end 1995 373))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1996 8) (end 1996 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1997 8) (end 1997 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1998 8) (end 1998 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2009 4) (end 2009 789))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2025 4) (end 2025 859))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2038 8) (end 2038 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2038 8) (end 2038 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2039 8) (end 2039 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2044 4) (end 2044 371))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2045 8) (end 2045 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2046 8) (end 2046 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2047 8) (end 2047 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2051 4) (end 2051 690))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2064 8) (end 2064 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2064 8) (end 2064 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2065 8) (end 2065 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2070 4) (end 2070 368))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2071 8) (end 2071 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2072 8) (end 2072 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2073 8) (end 2073 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2077 4) (end 2077 643))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2094 4) (end 2094 562))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2111 4) (end 2111 671))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2128 4) (end 2128 574))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2144 4) (end 2144 620))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2160 4) (end 2160 647))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2176 4) (end 2176 567))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2192 4) (end 2192 518))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2208 4) (end 2208 497))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2224 4) (end 2224 490))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2240 4) (end 2240 525))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2256 4) (end 2256 734))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2272 4) (end 2272 751))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2285 8) (end 2285 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2285 8) (end 2285 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2286 8) (end 2286 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2291 4) (end 2291 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2295 4) (end 2295 704))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2308 8) (end 2308 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2308 8) (end 2308 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2309 8) (end 2309 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2314 4) (end 2314 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2318 4) (end 2318 637))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2331 8) (end 2331 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2331 8) (end 2331 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2332 8) (end 2332 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2337 4) (end 2337 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2341 4) (end 2341 694))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2354 8) (end 2354 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2354 8) (end 2354 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2355 8) (end 2355 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2360 4) (end 2360 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2364 4) (end 2364 670))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2377 8) (end 2377 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2377 8) (end 2377 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2378 8) (end 2378 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2383 4) (end 2383 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2387 4) (end 2387 625))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2403 4) (end 2403 1860))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2419 4) (end 2419 1050))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2435 4) (end 2435 1454))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2448 8) (end 2448 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2448 8) (end 2448 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2449 8) (end 2449 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2454 4) (end 2454 361))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2455 8) (end 2455 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2456 8) (end 2456 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2457 8) (end 2457 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2477 4) (end 2477 1125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2490 8) (end 2490 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2490 8) (end 2490 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2491 8) (end 2491 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2496 4) (end 2496 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2500 4) (end 2500 1742))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2513 8) (end 2513 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2513 8) (end 2513 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2514 8) (end 2514 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2519 4) (end 2519 363))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2520 8) (end 2520 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2521 8) (end 2521 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2522 8) (end 2522 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2542 4) (end 2542 924))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2555 8) (end 2555 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2555 8) (end 2555 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2556 8) (end 2556 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2561 4) (end 2561 365))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2562 8) (end 2562 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2563 8) (end 2563 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2564 8) (end 2564 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2568 4) (end 2568 1146))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2581 8) (end 2581 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2581 8) (end 2581 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2582 8) (end 2582 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2587 4) (end 2587 478))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2588 8) (end 2588 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2589 8) (end 2589 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2590 8) (end 2590 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2591 8) (end 2591 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2595 4) (end 2595 1632))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2608 8) (end 2608 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2608 8) (end 2608 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2609 8) (end 2609 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2614 4) (end 2614 354))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2615 8) (end 2615 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2616 8) (end 2616 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2617 8) (end 2617 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2621 4) (end 2621 865))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2634 8) (end 2634 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2634 8) (end 2634 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2635 8) (end 2635 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2640 4) (end 2640 358))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2641 8) (end 2641 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2642 8) (end 2642 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2643 8) (end 2643 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2647 4) (end 2647 2249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2660 8) (end 2660 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2660 8) (end 2660 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2661 8) (end 2661 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2666 4) (end 2666 370))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2667 8) (end 2667 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2668 8) (end 2668 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2669 8) (end 2669 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2673 4) (end 2673 2127))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2686 8) (end 2686 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2686 8) (end 2686 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2687 8) (end 2687 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2692 4) (end 2692 484))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2693 8) (end 2693 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2694 8) (end 2694 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2695 8) (end 2695 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2696 8) (end 2696 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2700 4) (end 2700 857))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2713 8) (end 2713 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2713 8) (end 2713 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2714 8) (end 2714 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2719 4) (end 2719 371))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2720 8) (end 2720 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2721 8) (end 2721 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 2722 8) (end 2722 99))
      )
    )
  )
)
~~~
