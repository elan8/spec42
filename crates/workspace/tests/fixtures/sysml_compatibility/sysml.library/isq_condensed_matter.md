# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQCondensedMatter
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQCondensedMatter {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-12:2019 "Condensed matter physics"
     * see also https://www.iso.org/standard/63480.html
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
    private import ISQElectromagnetism::ElectricPotentialDifferenceValue;
    private import ISQElectromagnetism::MagneticFluxDensityValue;
    private import ISQElectromagnetism::ResistivityValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQSpaceTime::RepetencyValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-12 item 12-1.1 lattice vector */
    attribute def CartesianLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-1.1 lattice vector
         * symbol(s): `vec(R)`
         * application domain: generic
         * name: LatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: translation vector that maps the crystal lattice on itself
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianLattice3dVector: CartesianLattice3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-1.2 fundamental lattice vector */
    attribute def CartesianFundamentalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-1.2 fundamental lattice vector
         * symbol(s): `vec(a_1),vec(a_2),vec(a_3)`, `vec(a),vec(b),vec(c)`
         * application domain: generic
         * name: FundamentalLatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: fundamental translation vectors for the crystal lattice
         * remarks: The lattice vector (item 12-1.1) can be given as `vec(R) = n_1 vec(a_1) + n_2 vec(a_2) + n_3 vec(a_3)` where `n_1`, `n_2` and `n_3` are integers.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianFundamentalLattice3dVector: CartesianFundamentalLattice3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-2.1 angular reciprocal lattice vector */
    attribute def AngularReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-2.1 angular reciprocal lattice vector (magnitude)
         * symbol(s): `G`
         * application domain: generic
         * name: AngularReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularReciprocalLatticeVectorMagnitudeUnit[1];
    }

    attribute angularReciprocalLatticeVectorMagnitude: AngularReciprocalLatticeVectorMagnitudeValue[*] nonunique :> scalarQuantities;

    attribute def AngularReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    attribute def CartesianAngularReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-2.1 angular reciprocal lattice vector
         * symbol(s): `vec(G)`
         * application domain: generic
         * name: AngularReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularReciprocalLattice3dCoordinateFrame[1];
    }

    attribute cartesianAngularReciprocalLattice3dVector: CartesianAngularReciprocalLattice3dVector :> vectorQuantities;

    attribute def CartesianAngularReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularReciprocalLatticeVectorMagnitudeUnit[3];
    }

    /* ISO-80000-12 item 12-2.2 fundamental reciprocal lattice vector */
    attribute def FundamentalReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-2.2 fundamental reciprocal lattice vector (magnitude)
         * symbol(s): `b_1,b_2,b_3`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FundamentalReciprocalLatticeVectorMagnitudeUnit[1];
    }

    attribute fundamentalReciprocalLatticeVectorMagnitude: FundamentalReciprocalLatticeVectorMagnitudeValue[*] nonunique :> scalarQuantities;

    attribute def FundamentalReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    attribute def CartesianFundamentalReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-2.2 fundamental reciprocal lattice vector
         * symbol(s): `vec(b_1),vec(b_2),vec(b_3)`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianFundamentalReciprocalLattice3dCoordinateFrame[1];
    }

    attribute cartesianFundamentalReciprocalLattice3dVector: CartesianFundamentalReciprocalLattice3dVector :> vectorQuantities;

    attribute def CartesianFundamentalReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: FundamentalReciprocalLatticeVectorMagnitudeUnit[3];
    }

    /* ISO-80000-12 item 12-3 lattice plane spacing */
    attribute latticePlaneSpacing: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-3 lattice plane spacing
         * symbol(s): `d`
         * application domain: generic
         * name: LatticePlaneSpacing (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) between successive lattice planes
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
    }

    /* ISO-80000-12 item 12-4 Bragg angle */
    attribute braggAngle: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-4 Bragg angle
         * symbol(s): `ϑ`
         * application domain: generic
         * name: BraggAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): °, 1
         * tensor order: 0
         * definition: angle between the scattered ray and the lattice plane
         * remarks: Bragg angle `ϑ` is given by `2d sin ϑ = nλ`, where `d` is the lattice plane spacing (item 12-3), `λ` is the wavelength (ISO 80000-7) of the radiation, and `n` is the order of reflexion which is an integer.
         */
    }

    /* ISO-80000-12 item 12-5.1 short-range order parameter */
    attribute def ShortRangeOrderParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.1 short-range order parameter
         * symbol(s): `r`, `σ`
         * application domain: generic
         * name: ShortRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of nearest-neighbour atom pairs in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute shortRangeOrderParameter: ShortRangeOrderParameterValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.2 long-range order parameter */
    attribute def LongRangeOrderParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.2 long-range order parameter
         * symbol(s): `R`, `s`
         * application domain: generic
         * name: LongRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of atoms in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute longRangeOrderParameter: LongRangeOrderParameterValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.3 atomic scattering factor */
    attribute def AtomicScatteringFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.3 atomic scattering factor
         * symbol(s): `f`
         * application domain: generic
         * name: AtomicScatteringFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiation amplitude scattered by the atom and radiation amplitude scattered by a single electron
         * remarks: The atomic scattering factor can be expressed by: `f = E_a/(E_e`, where `E_a` is the radiation amplitude scattered by the atom and `E_e` is the radiation amplitude scattered by a single electron.
         */
    }
    attribute atomicScatteringFactor: AtomicScatteringFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.4 structure factor */
    attribute def StructureFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.4 structure factor
         * symbol(s): `F(h,k,l)`
         * application domain: generic
         * name: StructureFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `F(h,k,l) = sum_(n=1)^N f_n exp[2π i (h x_n + k y_n + l z_n)]`, where `f_n` is the atomic scattering factor (item 12-5.3) for atom `n`, `x_n`, `y_n`, `z_n` are fractional coordinates of its position, `N` is the total number of atoms in the unit cell and `h`, `k`, `l` are the Miller indices
         * remarks: For the Miller indices `h`, `k`, `l`, see Annex A.
         */
    }
    attribute structureFactor: StructureFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-6 Burgers vector */
    attribute def CartesianBurgers3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-6 Burgers vector
         * symbol(s): `vec(b)`
         * application domain: generic
         * name: BurgersVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: closing vector in a sequence of vectors encircling a dislocation
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianBurgers3dVector: CartesianBurgers3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.1 particle position vector */
    attribute def CartesianParticlePosition3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.1 particle position vector
         * symbol(s): `vec(r)`, `vec(R)`
         * application domain: generic
         * name: ParticlePositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of a particle
         * remarks: Often, `r` is used for electrons and `R` is used for atoms and other heavier particles.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianParticlePosition3dVector: CartesianParticlePosition3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.2 equilibrium position vector */
    attribute def CartesianEquilibriumPosition3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.2 equilibrium position vector
         * symbol(s): `vec(R_0)`
         * application domain: condensed matter physics
         * name: EquilibriumPositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of an ion or atom in equilibrium
         * remarks: None.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianEquilibriumPosition3dVector: CartesianEquilibriumPosition3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.3 displacement vector */
    attribute def CartesianDisplacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.3 displacement vector
         * symbol(s): `vec(u)`
         * application domain: condensed matter physics
         * name: DisplacementVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: difference between the position vector (ISO 80000-3) of an ion or atom and its position vector in equilibrium
         * remarks: The displacement vector can be expressed by: `vec(u) = vec(R) − vec(R_0)`, where `vec(R)` is particle position vector (item 12-7.1) and `vec(R_0)` is position vector of an ion or atom in equilibrium (item 12-7.2).
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianDisplacement3dVector: CartesianDisplacement3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-8 Debye-Waller factor */
    attribute def DebyeWallerFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-8 Debye-Waller factor
         * symbol(s): `D`, `B`
         * application domain: generic
         * name: DebyeWallerFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor by which the intensity of a diffraction line is reduced because of the lattice vibrations
         * remarks: `D` is sometimes expressed as `D = exp(−2W)`; in Mössbauer spectroscopy, it is also called the `f` factor and denoted by `f`.
         */
    }
    attribute debyeWallerFactor: DebyeWallerFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-9.1 angular wavenumber, angular repetency */
    attribute angularWavenumber: RepetencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-9.1 angular wavenumber, angular repetency
         * symbol(s): `k`, `q`
         * application domain: condensed matter physics
         * name: AngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: quotient of momentum (ISO 80000-4) and the reduced Planck constant (ISO 80000-1)
         * remarks: The corresponding vector (ISO 80000-2) quantity is called wave vector (ISO 80000-3), expressed by: `vec(k) = vec(p)/ħ`, where `vec(p)` is the momentum (ISO 80000-4) of quasi free electrons in an electron gas, and `ħ` is the reduced Planck constant (ISO 80000-1); for phonons, its magnitude is `k = 2π/λ`, where `λ` is the wavelength (ISO 80000-3) of the lattice vibrations. When a distinction is needed between `k` and the symbol for the Boltzmann constant (ISO 80000-1), `k_B` can be used for the latter. When a distinction is needed, `q` should be used for phonons, and `k` for particles such as electrons and neutrons. The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias angularRepetency for angularWavenumber;

    /* ISO-80000-12 item 12-9.2 Fermi angular wavenumber, Fermi angular repetency */
    attribute fermiAngularWavenumber: RepetencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-9.2 Fermi angular wavenumber, Fermi angular repetency
         * symbol(s): `k_F`
         * application domain: generic
         * name: FermiAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: angular wavenumber (item 12-9.1) of electrons in states on the Fermi sphere
         * remarks: In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias fermiAngularRepetency for fermiAngularWavenumber;

    /* ISO-80000-12 item 12-9.3 Debye angular wavenumber, Debye angular repetency */
    attribute debyeAngularWavenumber: RepetencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-9.3 Debye angular wavenumber, Debye angular repetency
         * symbol(s): `q_D`
         * application domain: generic
         * name: DebyeAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: cut-off angular wavenumber (item 12-9.1) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias debyeAngularRepetency for debyeAngularWavenumber;

    /* ISO-80000-12 item 12-10 Debye angular frequency */
    attribute debyeAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-10 Debye angular frequency
         * symbol(s): `ω_D`
         * application domain: generic
         * name: DebyeAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: cut-off angular frequency (ISO 80000-3) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified.
         */
    }

    /* ISO-80000-12 item 12-11 Debye temperature */
    attribute debyeTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-11 Debye temperature
         * symbol(s): `Θ_D`
         * application domain: generic
         * name: DebyeTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the Debye model, quantity given by: `Θ_D = ħ*ω_D/k`, where `k` is the Boltzmann constant, (ISO 80000-1), `ħ` is the reduced Planck constant (ISO 80000-1), and `ω_D` is Debye angular frequency (item 12-10)
         * remarks: A Debye temperature can also be defined by fitting a Debye model result to a certain quantity, for instance, the heat capacity at a certain temperature.
         */
    }

    /* ISO-80000-12 item 12-12 density of vibrational states */
    attribute def DensityOfVibrationalStatesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-12 density of vibrational states
         * symbol(s): `g`
         * application domain: angular frequency
         * name: DensityOfVibrationalStates
         * quantity dimension: L^-3*T^1
         * measurement unit(s): m^-3*s
         * tensor order: 0
         * definition: quotient of the number of vibrational modes in an infinitesimal interval of angular frequency (ISO 80000-3), and the product of the width of that interval and volume (ISO 80000-3)
         * remarks: `g(ω) = n_ω = (dn(ω))/(dω)`, where `n(ω)` is the total number of vibrational modes per volume with angular frequency less than `ω`. The density of states may also be normalized in other ways instead of with respect to volume. See also item 12-16.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DensityOfVibrationalStatesUnit[1];
    }

    attribute densityOfVibrationalStates: DensityOfVibrationalStatesValue[*] nonunique :> scalarQuantities;

    attribute def DensityOfVibrationalStatesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-12 item 12-13 thermodynamic Grüneisen parameter */
    attribute def 'ThermodynamicGrüneisenParameterValue' :> DimensionOneValue {
        doc
        /*
         * source: item 12-13 thermodynamic Grüneisen parameter
         * symbol(s): `γ_G`, `Γ_G`
         * application domain: generic
         * name: ThermodynamicGrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `γ_G = (α_V)/(κ_T c_V ρ)`, where `α_V` is cubic expansion coefficient (ISO 80000-5), `κ_T` is isothermal compressibility (ISO 80000-5), `c_V` is specific heat capacity at constant volume (ISO 80000-5), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'thermodynamicGrüneisenParameter': 'ThermodynamicGrüneisenParameterValue' :> scalarQuantities;

    /* ISO-80000-12 item 12-14 Grüneisen parameter */
    attribute def 'GrüneisenParameterValue' :> DimensionOneValue {
        doc
        /*
         * source: item 12-14 Grüneisen parameter
         * symbol(s): `γ`
         * application domain: generic
         * name: GrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by minus the partial differential quotient: `γ = -(del ln ω)/(del ln V)`, where `ω` is a lattice vibration frequency (ISO 80000-3), and `V` is volume (ISO 80000-3)
         * remarks: `ω` can also refer to an average of the vibrational spectrum, for instance as represented by a Debye angular frequency (item 12-10).
         */
    }
    attribute 'grüneisenParameter': 'GrüneisenParameterValue' :> scalarQuantities;

    /* ISO-80000-12 item 12-15.1 mean free path of phonons */
    attribute meanFreePathOfPhonons: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-15.1 mean free path of phonons
         * symbol(s): `l_p`
         * application domain: generic
         * name: MeanFreePathOfPhonons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that phonons travel between two successive interactions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-15.2 mean free path of electrons */
    attribute meanFreePathOfElectrons: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-15.2 mean free path of electrons
         * symbol(s): `l_e`
         * application domain: generic
         * name: MeanFreePathOfElectrons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that electrons travel between two successive interactions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-16 energy density of states */
    attribute def EnergyDensityOfStatesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-16 energy density of states
         * symbol(s): `n_E(E)`, `ρ(E)`
         * application domain: generic
         * name: EnergyDensityOfStates
         * quantity dimension: L^-5*M^-1*T^2
         * measurement unit(s): J^-1*m^-3*eV^-1*m^-3, kg^-1*m^-5*s^2
         * tensor order: 0
         * definition: quantity given by the differential quotient with respect to energy: `n_E(E) = (dn(E))/(dE)`, where `n_E(E)` is the total number of one-electron states per volume (ISO 80000-3) with energy less than `E` (ISO 80000-5)
         * remarks: Density of states refers to electrons or other entities, e.g. phonons. It may be normalized in other ways instead of with respect to volume, e.g. with respect to amount of substance. See also item 12-12.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyDensityOfStatesUnit[1];
    }

    attribute energyDensityOfStates: EnergyDensityOfStatesValue[*] nonunique :> scalarQuantities;

    attribute def EnergyDensityOfStatesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -5; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-12 item 12-17 residual resistivity */
    attribute residualResistivity: ResistivityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-17 residual resistivity
         * symbol(s): `ρ_0`
         * application domain: generic
         * name: ResidualResistivity (specializes Resistivity)
         * quantity dimension: L^3*M^1*T^-3*I^-2
         * measurement unit(s): Ω*m, kg*m^3*s^-3*A^-2
         * tensor order: 0
         * definition: for metals, the resistivity (IEC 80000-6) extrapolated to zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-18 Lorenz coefficient */
    attribute def LorenzCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-18 Lorenz coefficient
         * symbol(s): `L`
         * application domain: generic
         * name: LorenzCoefficient
         * quantity dimension: L^4*M^2*T^-6*I^-2*Θ^-2
         * measurement unit(s): V^2/K^2, kg^2*m^4*s^-6*A^-2*K^-2
         * tensor order: 0
         * definition: quotient of thermal conductivity (ISO 80000-5), and the product of electric conductivity (IEC 80000-6) and thermodynamic temperature (ISO 80000-3)
         * remarks: The Lorenz coefficient can be expressed by `L = λ/(σT)`, where `λ` is thermal conductivity (ISO 80000-5), `σ` is electric conductivity (IEC 80000-6), and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LorenzCoefficientUnit[1];
    }

    attribute lorenzCoefficient: LorenzCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LorenzCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -6; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-19 Hall coefficient */
    attribute def HallCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-19 Hall coefficient
         * symbol(s): `R_H`, `A_H`
         * application domain: generic
         * name: HallCoefficient
         * quantity dimension: L^3*T^-1*I^-1
         * measurement unit(s): m^3/C, m^3*s^-1*A^-1
         * tensor order: 0
         * definition: in an isotropic conductor, relation between electric field strength, `vec(E)`, (IEC 80000-6) and electric current density, `vec(J)`, (IEC 80000-6) expressed as: `vec(E) = ρ vec(J) + R_H (vec(B) xx vec(J))`, where `ρ` is resistivity (IEC 80000-6), and `vec(B)` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HallCoefficientUnit[1];
    }

    attribute hallCoefficient: HallCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def HallCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-12 item 12-20 thermoelectric voltage (between substances a and b) */
    attribute thermoelectricVoltageBetweenSubstancesAAndB: ElectricPotentialDifferenceValue :> scalarQuantities {
        doc
        /*
         * source: item 12-20 thermoelectric voltage (between substances a and b)
         * symbol(s): `E_(ab)`
         * application domain: generic
         * name: ThermoelectricVoltageBetweenSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: voltage (IEC 80000-6) between substances `a` and `b` caused by the thermoelectric effect
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-21 Seebeck coefficient (for substances a and b) */
    attribute def SeebeckCoefficientForSubstancesAAndBValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-21 Seebeck coefficient (for substances a and b)
         * symbol(s): `S_(ab)`
         * application domain: generic
         * name: SeebeckCoefficientForSubstancesAAndB
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: differential quotient of thermoelectric voltage with respect to thermodynamic temperature: `S_(ab) =      (dE_(ab))/(dT)`, where `E_(ab)` is the thermoelectric voltage between substances `a` and `b` (item 12-20) and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: This term is also called "thermoelectric power".
         */
        attribute :>> num: Real;
        attribute :>> mRef: SeebeckCoefficientForSubstancesAAndBUnit[1];
    }

    attribute seebeckCoefficientForSubstancesAAndB: SeebeckCoefficientForSubstancesAAndBValue[*] nonunique :> scalarQuantities;

    attribute def SeebeckCoefficientForSubstancesAAndBUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-22 Peltier coefficient (for substances a and b) */
    attribute peltierCoefficientForSubstancesAAndB: ElectricPotentialDifferenceValue :> scalarQuantities {
        doc
        /*
         * source: item 12-22 Peltier coefficient (for substances a and b)
         * symbol(s): `Π_(ab)`
         * application domain: generic
         * name: PeltierCoefficientForSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: quotient of Peltier heat power (ISO 80000-5) developed at a junction, and the electric current (IEC 80000-6) flowing from substance `a` to substance `b`
         * remarks: `Π_(ab) = Π_a - Π_b`, where `Π_a` and `Π_b` are the Peltier coefficients of substances `a` and `b`, respectively.
         */
    }

    /* ISO-80000-12 item 12-23 Thomson coefficient */
    attribute def ThomsonCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-23 Thomson coefficient
         * symbol(s): `μ`
         * application domain: generic
         * name: ThomsonCoefficient
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: quotient of Thomson heat power (ISO 80000-5) developed, and the electric current (IEC 80000-6) and temperature (ISO 80000-5) difference
         * remarks: `μ` is positive if heat is developed when the temperature decreases in the direction of the electric current.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThomsonCoefficientUnit[1];
    }

    attribute thomsonCoefficient: ThomsonCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def ThomsonCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-24.1 work function */
    attribute workFunction: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-24.1 work function
         * symbol(s): `ϕ`
         * application domain: generic
         * name: WorkFunction (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and the Fermi energy (item 12-27.1)
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. The contact potential difference between substances `a` and `b` is given by `V_a - V_b = (ϕ_a - ϕ_b)/e`, where `e` is the elementary charge (ISO 80000-1). A set of energy levels, the energies of which occupy an interval practically continuously, is called an energy band. In semi-conductors `E_d` and `E_a` are used for donors and acceptors, respectively.
         */
    }

    /* ISO-80000-12 item 12-24.2 ionization energy */
    attribute ionizationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-24.2 ionization energy
         * symbol(s): `E_i`
         * application domain: generic
         * name: IonizationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and a certain energy level which is the energy of an electron in the interior of a substance
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-25 electron affinity */
    attribute electronAffinity: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-25 electron affinity
         * symbol(s): `χ`
         * application domain: condensed matter physics
         * name: ElectronAffinity (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) difference between an electron at rest at infinity and an electron at the lowest level of the conduction band in an insulator or semiconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-26 Richardson constant */
    attribute def RichardsonConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-26 Richardson constant
         * symbol(s): `A`
         * application domain: generic
         * name: RichardsonConstant
         * quantity dimension: L^-2*I^1*Θ^-2
         * measurement unit(s): A*m^-2*K^-2
         * tensor order: 0
         * definition: parameter in the expression for the thermionic emission current density `J` (IEC 80000-6) for a metal in terms of the thermodynamic temperature `T` (ISO 80000-5) and work function `ϕ`, (item 12-24.1): `J = AT^2 exp(ϕ/(kT))`, where `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RichardsonConstantUnit[1];
    }

    attribute richardsonConstant: RichardsonConstantValue[*] nonunique :> scalarQuantities;

    attribute def RichardsonConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-27.1 Fermi energy */
    attribute fermiEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-27.1 Fermi energy
         * symbol(s): `E_F`
         * application domain: generic
         * name: FermiEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a metal, highest occupied energy level at zero thermodynamic temperature (ISO 80000-5), where energy level means the energy (ISO 80000-5) of an electron in the interior of a substance
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. At `T = 0 [K]`, `E_F` is equal to the chemical potential per electron. In condensed matter physics, the reference level for the energy is sometimes chosen so that, for instance, `E_F = 0`.
         */
    }

    /* ISO-80000-12 item 12-27.2 gap energy */
    attribute gapEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-27.2 gap energy
         * symbol(s): `E_g`
         * application domain: generic
         * name: GapEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference in energy (ISO 80000-5) between the lowest level of conduction band and the highest level of valence band at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-28 Fermi temperature */
    attribute fermiTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-28 Fermi temperature
         * symbol(s): `T_F`
         * application domain: generic
         * name: FermiTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the free electron model, the Fermi energy (item 12-27.1) divided by the Boltzmann constant (ISO 80000-1)
         * remarks: The Fermi temperature is expressed by: `T_F = E_F/k`, where `E_F` is Fermi energy (item 12-27.1) and `k` is the Boltzmann constant (ISO 80000-1). `E_F` is relative to the lowest occupied state.
         */
    }

    /* ISO-80000-12 item 12-29.1 electron density */
    attribute def ElectronDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.1 electron density
         * symbol(s): `n`
         * application domain: generic
         * name: ElectronDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of electrons in conduction band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectronDensityUnit[1];
    }

    attribute electronDensity: ElectronDensityValue[*] nonunique :> scalarQuantities;

    attribute def ElectronDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.2 hole density */
    attribute def HoleDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.2 hole density
         * symbol(s): `p`
         * application domain: generic
         * name: HoleDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of holes in valence band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HoleDensityUnit[1];
    }

    attribute holeDensity: HoleDensityValue[*] nonunique :> scalarQuantities;

    attribute def HoleDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.3 intrinsic carrier density */
    attribute def IntrinsicCarrierDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.3 intrinsic carrier density
         * symbol(s): `n_i`
         * application domain: generic
         * name: IntrinsicCarrierDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quantity given by: `n_i = sqrt(n p)`, where `n` is electron density (item 12-29.1), and `p` is hole
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IntrinsicCarrierDensityUnit[1];
    }

    attribute intrinsicCarrierDensity: IntrinsicCarrierDensityValue[*] nonunique :> scalarQuantities;

    attribute def IntrinsicCarrierDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.4 donor density */
    attribute def DonorDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.4 donor density
         * symbol(s): `n_d`
         * application domain: generic
         * name: DonorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of donor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DonorDensityUnit[1];
    }

    attribute donorDensity: DonorDensityValue[*] nonunique :> scalarQuantities;

    attribute def DonorDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.5 acceptor density */
    attribute def AcceptorDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.5 acceptor density
         * symbol(s): `n_a`
         * application domain: generic
         * name: AcceptorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of acceptor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AcceptorDensityUnit[1];
    }

    attribute acceptorDensity: AcceptorDensityValue[*] nonunique :> scalarQuantities;

    attribute def AcceptorDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-30 effective mass */
    attribute effectiveMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 12-30 effective mass
         * symbol(s): `m"*"`
         * application domain: generic
         * name: EffectiveMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: quantity given by: `m^"*" = (ħ^2 k) / ((dε)/(dk))`, where `k` is wavenumber (ISO 80000-3), `ε` is the energy (ISO 80000-5) of an electron in the interior of a substance, and `ħ` is the reduced Planck constant (ISO 80000-1)
         * remarks: When `k` refers to a state where `ε` has an extremum, `m"*" = (ħ^2 k) / ((d^2ε)/(dk^2))`. The effective mass can be generalized to refer to an anisotropic system with `ε = ε(k)`.
         */
    }

    /* ISO-80000-12 item 12-31 mobility ratio */
    attribute def MobilityRatioValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-31 mobility ratio
         * symbol(s): `b`
         * application domain: generic
         * name: MobilityRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mobilities (ISO 80000-10) of electrons and holes, respectively
         * remarks: The mobility ratio can be expressed by: `b = μ_n/μ_p`, where `μ_n` and `μ_p` are mobilities (ISO 80000-10) for electrons and holes, respectively.
         */
    }
    attribute mobilityRatio: MobilityRatioValue :> scalarQuantities;

    /* ISO-80000-12 item 12-32.1 relaxation time */
    attribute relaxationTime: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 12-32.1 relaxation time
         * symbol(s): `τ`
         * application domain: condensed matter physics
         * name: RelaxationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for scattering, trapping or annihilation of charge carriers, phonons or other quasiparticles
         * remarks: For electrons in metals, `τ = l/v_F`, where `l` is mean free path (item 12-15.2) and `v_F` is speed (ISO 80000-3) of electrons on the Fermi surface.
         */
    }

    /* ISO-80000-12 item 12-32.2 carrier lifetime */
    attribute carrierLifetime: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 12-32.2 carrier lifetime
         * symbol(s): `τ`, `τ_n`, `τ_p`
         * application domain: semiconductors
         * name: CarrierLifetime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for recombination or trapping of minority charge carriers in semiconductors
         * remarks: Indices "n" and "p" denote negative and positive charge carriers, respectively. Positive charge carriers can also be holes.
         */
    }

    /* ISO-80000-12 item 12-33 diffusion length */
    attribute diffusionLengthForCondensedMatterPhysics: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-33 diffusion length
         * symbol(s): `L`, `L_n`, `L_p`
         * application domain: condensed matter physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the product of diffusion coefficient (ISO 80000-10) and lifetime (ISO 80000-10)
         * remarks: The diffusion length can be expressed by: `L = sqrt(Dτ)`, where `D` is the diffusion coefficient (ISO 80000-9) and `τ` is lifetime (ISO 80000-3).
         */
    }

    /* ISO-80000-12 item 12-34 exchange integral */
    attribute exchangeIntegral: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-34 exchange integral
         * symbol(s): `K`, `J`
         * application domain: generic
         * name: ExchangeIntegral (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: constituent of the interaction energy (ISO 80000-5) between the spins of adjacent electrons in matter arising from the overlap of electron state functions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-35.1 Curie temperature */
    attribute curieTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-35.1 Curie temperature
         * symbol(s): `T_C`
         * application domain: generic
         * name: CurieTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a ferromagnet
         * remarks: `T_(cr)` is used for critical thermodynamic temperature in general.
         */
    }

    /* ISO-80000-12 item 12-35.2 Néel temperature */
    attribute 'néelTemperature': ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-35.2 Néel temperature
         * symbol(s): `T_N`
         * application domain: generic
         * name: NéelTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of an antiferromagnet
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-35.3 superconduction transition temperature */
    attribute superconductionTransitionTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-35.3 superconduction transition temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: SuperconductionTransitionTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-36.1 thermodynamic critical magnetic flux density */
    attribute thermodynamicCriticalMagneticFluxDensity: MagneticFluxDensityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-36.1 thermodynamic critical magnetic flux density
         * symbol(s): `B_c`
         * application domain: generic
         * name: ThermodynamicCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: quantity given by: `B_c = sqrt((2μ_0 (G_n - G_s))/V)`, where `G_n` and `G_s` are the Gibbs energies (ISO 80000-5) at zero magnetic flux density (IEC 80000-6) in a normal conductor and superconductor, respectively, `μ_0` is the magnetic constant (IEC 80000-6), and `V` is volume (ISO 80000-3)
         * remarks: In type I superconductors, `B_c` is the critical magnetic flux density for disappearance of superconductivity. The symbol `B_(c3)` is used for the critical magnetic flux density for disappearance of surface superconductivity.
         */
    }

    /* ISO-80000-12 item 12-36.2 lower critical magnetic flux density */
    attribute lowerCriticalMagneticFluxDensity: MagneticFluxDensityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-36.2 lower critical magnetic flux density
         * symbol(s): `B_(c1)`
         * application domain: generic
         * name: LowerCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for magnetic flux (IEC 80000-6) entering the superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-36.3 upper critical magnetic flux density */
    attribute upperCriticalMagneticFluxDensity: MagneticFluxDensityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-36.3 upper critical magnetic flux density
         * symbol(s): `B_(c2)`
         * application domain: generic
         * name: UpperCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for disappearance of bulk superconductivity
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-37 superconductor energy gap */
    attribute superconductorEnergyGap: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-37 superconductor energy gap
         * symbol(s): `Δ`
         * application domain: generic
         * name: SuperconductorEnergyGap (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: width of the forbidden energy band (item 12-24.2) in a superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-38.1 London penetration depth */
    attribute londonPenetrationDepth: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-38.1 London penetration depth
         * symbol(s): `λ_L`
         * application domain: generic
         * name: LondonPenetrationDepth (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) a magnetic field penetrates the plane surface of a semi-finite superconductor according to the expression: `B(x) = B(0) exp(-x/λ_L)`, where `B` is magnetic flux density (IEC 80000-6) and `x` is distance (ISO 80000-3) from the surface
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-38.2 coherence length */
    attribute coherenceLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-38.2 coherence length
         * symbol(s): `ξ`
         * application domain: generic
         * name: CoherenceLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) in a superconductor over which the effect of a perturbation is appreciable at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

}
~~~
# EXPECTED
~~~
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'RepetencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'RepetencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'RepetencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularFrequencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
semantic.unresolved_name 'LengthValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ResistivityValue'
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
semantic.unresolved_name 'ElectricPotentialDifferenceValue'
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
semantic.unresolved_name 'ElectricPotentialDifferenceValue'
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
semantic.unresolved_name 'EnergyValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MagneticFluxDensityValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MagneticFluxDensityValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MagneticFluxDensityValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
~~~
# PROBLEMS
~~~
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'RepetencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'RepetencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'RepetencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularFrequencyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
semantic.unresolved_name 'LengthValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ResistivityValue'
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
semantic.unresolved_name 'ElectricPotentialDifferenceValue'
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
semantic.unresolved_name 'ElectricPotentialDifferenceValue'
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
semantic.unresolved_name 'EnergyValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MagneticFluxDensityValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MagneticFluxDensityValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'MagneticFluxDensityValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,UnrestrictedName,Colon,Ident,ColonGt,Ident,OpenCurly,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQCondensedMatter'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQElectromagnetism::ElectricPotentialDifferenceValue')
    (import_decl private 'ISQElectromagnetism::MagneticFluxDensityValue')
    (import_decl private 'ISQElectromagnetism::ResistivityValue')
    (import_decl private 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame')
    (import_decl private 'ISQSpaceTime::AngularFrequencyValue')
    (import_decl private 'ISQSpaceTime::AngularMeasureValue')
    (import_decl private 'ISQSpaceTime::RepetencyValue')
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (attribute_def 'CartesianLattice3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianLattice3dVector' : 'CartesianLattice3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianFundamentalLattice3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianFundamentalLattice3dVector' : 'CartesianFundamentalLattice3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'AngularReciprocalLatticeVectorMagnitudeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularReciprocalLatticeVectorMagnitudeUnit' multiplicity))
    (attribute_usage 'angularReciprocalLatticeVectorMagnitude' : 'AngularReciprocalLatticeVectorMagnitudeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularReciprocalLatticeVectorMagnitudeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianAngularReciprocalLattice3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAngularReciprocalLattice3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianAngularReciprocalLattice3dVector' : 'CartesianAngularReciprocalLattice3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianAngularReciprocalLattice3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'AngularReciprocalLatticeVectorMagnitudeUnit' multiplicity))
    (comment)
    (attribute_def 'FundamentalReciprocalLatticeVectorMagnitudeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'FundamentalReciprocalLatticeVectorMagnitudeUnit' multiplicity))
    (attribute_usage 'fundamentalReciprocalLatticeVectorMagnitude' : 'FundamentalReciprocalLatticeVectorMagnitudeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'FundamentalReciprocalLatticeVectorMagnitudeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianFundamentalReciprocalLattice3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianFundamentalReciprocalLattice3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianFundamentalReciprocalLattice3dVector' : 'CartesianFundamentalReciprocalLattice3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianFundamentalReciprocalLattice3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'FundamentalReciprocalLatticeVectorMagnitudeUnit' multiplicity))
    (comment)
    (attribute_usage 'latticePlaneSpacing' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'braggAngle' : 'AngularMeasureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ShortRangeOrderParameterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'shortRangeOrderParameter' : 'ShortRangeOrderParameterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LongRangeOrderParameterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'longRangeOrderParameter' : 'LongRangeOrderParameterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AtomicScatteringFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'atomicScatteringFactor' : 'AtomicScatteringFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StructureFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'structureFactor' : 'StructureFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CartesianBurgers3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianBurgers3dVector' : 'CartesianBurgers3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianParticlePosition3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianParticlePosition3dVector' : 'CartesianParticlePosition3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianEquilibriumPosition3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianEquilibriumPosition3dVector' : 'CartesianEquilibriumPosition3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianDisplacement3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianDisplacement3dVector' : 'CartesianDisplacement3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'DebyeWallerFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'debyeWallerFactor' : 'DebyeWallerFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'angularWavenumber' : 'RepetencyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'angularRepetency' for 'angularWavenumber')
    (comment)
    (attribute_usage 'fermiAngularWavenumber' : 'RepetencyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'fermiAngularRepetency' for 'fermiAngularWavenumber')
    (comment)
    (attribute_usage 'debyeAngularWavenumber' : 'RepetencyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'debyeAngularRepetency' for 'debyeAngularWavenumber')
    (comment)
    (attribute_usage 'debyeAngularFrequency' : 'AngularFrequencyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'debyeTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'DensityOfVibrationalStatesValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DensityOfVibrationalStatesUnit' multiplicity))
    (attribute_usage 'densityOfVibrationalStates' : 'DensityOfVibrationalStatesValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DensityOfVibrationalStatesUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def ''ThermodynamicGrüneisenParameterValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''thermodynamicGrüneisenParameter'' : ''ThermodynamicGrüneisenParameterValue'' :> 'scalarQuantities')
    (comment)
    (attribute_def ''GrüneisenParameterValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''grüneisenParameter'' : ''GrüneisenParameterValue'' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'meanFreePathOfPhonons' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'meanFreePathOfElectrons' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'EnergyDensityOfStatesValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EnergyDensityOfStatesUnit' multiplicity))
    (attribute_usage 'energyDensityOfStates' : 'EnergyDensityOfStatesValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EnergyDensityOfStatesUnit' :> 'DerivedUnit'
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
    (attribute_usage 'residualResistivity' : 'ResistivityValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'LorenzCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LorenzCoefficientUnit' multiplicity))
    (attribute_usage 'lorenzCoefficient' : 'LorenzCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LorenzCoefficientUnit' :> 'DerivedUnit'
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
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'HallCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'HallCoefficientUnit' multiplicity))
    (attribute_usage 'hallCoefficient' : 'HallCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'HallCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
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
    (attribute_usage 'thermoelectricVoltageBetweenSubstancesAAndB' : 'ElectricPotentialDifferenceValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'SeebeckCoefficientForSubstancesAAndBValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SeebeckCoefficientForSubstancesAAndBUnit' multiplicity))
    (attribute_usage 'seebeckCoefficientForSubstancesAAndB' : 'SeebeckCoefficientForSubstancesAAndBValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SeebeckCoefficientForSubstancesAAndBUnit' :> 'DerivedUnit'
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
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'peltierCoefficientForSubstancesAAndB' : 'ElectricPotentialDifferenceValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ThomsonCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThomsonCoefficientUnit' multiplicity))
    (attribute_usage 'thomsonCoefficient' : 'ThomsonCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThomsonCoefficientUnit' :> 'DerivedUnit'
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
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'workFunction' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'ionizationEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'electronAffinity' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'RichardsonConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RichardsonConstantUnit' multiplicity))
    (attribute_usage 'richardsonConstant' : 'RichardsonConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RichardsonConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'fermiEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'gapEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'fermiTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ElectronDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectronDensityUnit' multiplicity))
    (attribute_usage 'electronDensity' : 'ElectronDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectronDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'HoleDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'HoleDensityUnit' multiplicity))
    (attribute_usage 'holeDensity' : 'HoleDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'HoleDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'IntrinsicCarrierDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IntrinsicCarrierDensityUnit' multiplicity))
    (attribute_usage 'intrinsicCarrierDensity' : 'IntrinsicCarrierDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IntrinsicCarrierDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'DonorDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DonorDensityUnit' multiplicity))
    (attribute_usage 'donorDensity' : 'DonorDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DonorDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AcceptorDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AcceptorDensityUnit' multiplicity))
    (attribute_usage 'acceptorDensity' : 'AcceptorDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AcceptorDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'effectiveMass' : 'MassValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'MobilityRatioValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'mobilityRatio' : 'MobilityRatioValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'relaxationTime' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'carrierLifetime' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'diffusionLengthForCondensedMatterPhysics' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'exchangeIntegral' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'curieTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage ''néelTemperature'' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'superconductionTransitionTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'thermodynamicCriticalMagneticFluxDensity' : 'MagneticFluxDensityValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'lowerCriticalMagneticFluxDensity' : 'MagneticFluxDensityValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'upperCriticalMagneticFluxDensity' : 'MagneticFluxDensityValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'superconductorEnergyGap' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'londonPenetrationDepth' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'coherenceLength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package ISQCondensedMatter {
    doc /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-12:2019 "Condensed matter physics"
     * see also https://www.iso.org/standard/63480.html
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
    private import ISQElectromagnetism::ElectricPotentialDifferenceValue;
    private import ISQElectromagnetism::MagneticFluxDensityValue;
    private import ISQElectromagnetism::ResistivityValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQSpaceTime::RepetencyValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-12 item 12-1.1 lattice vector */
    attribute def CartesianLattice3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-1.1 lattice vector
         * symbol(s): `vec(R)`
         * application domain: generic
         * name: LatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: translation vector that maps the crystal lattice on itself
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianLattice3dVector : CartesianLattice3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-1.2 fundamental lattice vector */
    attribute def CartesianFundamentalLattice3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-1.2 fundamental lattice vector
         * symbol(s): `vec(a_1),vec(a_2),vec(a_3)`, `vec(a),vec(b),vec(c)`
         * application domain: generic
         * name: FundamentalLatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: fundamental translation vectors for the crystal lattice
         * remarks: The lattice vector (item 12-1.1) can be given as `vec(R) = n_1 vec(a_1) + n_2 vec(a_2) + n_3 vec(a_3)` where `n_1`, `n_2` and `n_3` are integers.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianFundamentalLattice3dVector : CartesianFundamentalLattice3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-2.1 angular reciprocal lattice vector */
    attribute def AngularReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-2.1 angular reciprocal lattice vector (magnitude)
         * symbol(s): `G`
         * application domain: generic
         * name: AngularReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularReciprocalLatticeVectorMagnitudeUnit [1];
    }

    attribute angularReciprocalLatticeVectorMagnitude : AngularReciprocalLatticeVectorMagnitudeValue :> scalarQuantities [*] nonunique;

    attribute def AngularReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    attribute def CartesianAngularReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-2.1 angular reciprocal lattice vector
         * symbol(s): `vec(G)`
         * application domain: generic
         * name: AngularReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularReciprocalLattice3dCoordinateFrame [1];
    }

    attribute cartesianAngularReciprocalLattice3dVector : CartesianAngularReciprocalLattice3dVector :> vectorQuantities;

    attribute def CartesianAngularReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularReciprocalLatticeVectorMagnitudeUnit [3];
    }

    /* ISO-80000-12 item 12-2.2 fundamental reciprocal lattice vector */
    attribute def FundamentalReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-2.2 fundamental reciprocal lattice vector (magnitude)
         * symbol(s): `b_1,b_2,b_3`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : FundamentalReciprocalLatticeVectorMagnitudeUnit [1];
    }

    attribute fundamentalReciprocalLatticeVectorMagnitude : FundamentalReciprocalLatticeVectorMagnitudeValue :> scalarQuantities [*] nonunique;

    attribute def FundamentalReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    attribute def CartesianFundamentalReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-2.2 fundamental reciprocal lattice vector
         * symbol(s): `vec(b_1),vec(b_2),vec(b_3)`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianFundamentalReciprocalLattice3dCoordinateFrame [1];
    }

    attribute cartesianFundamentalReciprocalLattice3dVector : CartesianFundamentalReciprocalLattice3dVector :> vectorQuantities;

    attribute def CartesianFundamentalReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : FundamentalReciprocalLatticeVectorMagnitudeUnit [3];
    }

    /* ISO-80000-12 item 12-3 lattice plane spacing */
    attribute latticePlaneSpacing : LengthValue :> scalarQuantities {
        doc /*
         * source: item 12-3 lattice plane spacing
         * symbol(s): `d`
         * application domain: generic
         * name: LatticePlaneSpacing (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) between successive lattice planes
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
    }

    /* ISO-80000-12 item 12-4 Bragg angle */
    attribute braggAngle : AngularMeasureValue :> scalarQuantities {
        doc /*
         * source: item 12-4 Bragg angle
         * symbol(s): `ϑ`
         * application domain: generic
         * name: BraggAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): °, 1
         * tensor order: 0
         * definition: angle between the scattered ray and the lattice plane
         * remarks: Bragg angle `ϑ` is given by `2d sin ϑ = nλ`, where `d` is the lattice plane spacing (item 12-3), `λ` is the wavelength (ISO 80000-7) of the radiation, and `n` is the order of reflexion which is an integer.
         */
    }

    /* ISO-80000-12 item 12-5.1 short-range order parameter */
    attribute def ShortRangeOrderParameterValue :> DimensionOneValue {
        doc /*
         * source: item 12-5.1 short-range order parameter
         * symbol(s): `r`, `σ`
         * application domain: generic
         * name: ShortRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of nearest-neighbour atom pairs in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute shortRangeOrderParameter : ShortRangeOrderParameterValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.2 long-range order parameter */
    attribute def LongRangeOrderParameterValue :> DimensionOneValue {
        doc /*
         * source: item 12-5.2 long-range order parameter
         * symbol(s): `R`, `s`
         * application domain: generic
         * name: LongRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of atoms in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute longRangeOrderParameter : LongRangeOrderParameterValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.3 atomic scattering factor */
    attribute def AtomicScatteringFactorValue :> DimensionOneValue {
        doc /*
         * source: item 12-5.3 atomic scattering factor
         * symbol(s): `f`
         * application domain: generic
         * name: AtomicScatteringFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiation amplitude scattered by the atom and radiation amplitude scattered by a single electron
         * remarks: The atomic scattering factor can be expressed by: `f = E_a/(E_e`, where `E_a` is the radiation amplitude scattered by the atom and `E_e` is the radiation amplitude scattered by a single electron.
         */
    }
    attribute atomicScatteringFactor : AtomicScatteringFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.4 structure factor */
    attribute def StructureFactorValue :> DimensionOneValue {
        doc /*
         * source: item 12-5.4 structure factor
         * symbol(s): `F(h,k,l)`
         * application domain: generic
         * name: StructureFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `F(h,k,l) = sum_(n=1)^N f_n exp[2π i (h x_n + k y_n + l z_n)]`, where `f_n` is the atomic scattering factor (item 12-5.3) for atom `n`, `x_n`, `y_n`, `z_n` are fractional coordinates of its position, `N` is the total number of atoms in the unit cell and `h`, `k`, `l` are the Miller indices
         * remarks: For the Miller indices `h`, `k`, `l`, see Annex A.
         */
    }
    attribute structureFactor : StructureFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-6 Burgers vector */
    attribute def CartesianBurgers3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-6 Burgers vector
         * symbol(s): `vec(b)`
         * application domain: generic
         * name: BurgersVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: closing vector in a sequence of vectors encircling a dislocation
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianBurgers3dVector : CartesianBurgers3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.1 particle position vector */
    attribute def CartesianParticlePosition3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-7.1 particle position vector
         * symbol(s): `vec(r)`, `vec(R)`
         * application domain: generic
         * name: ParticlePositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of a particle
         * remarks: Often, `r` is used for electrons and `R` is used for atoms and other heavier particles.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianParticlePosition3dVector : CartesianParticlePosition3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.2 equilibrium position vector */
    attribute def CartesianEquilibriumPosition3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-7.2 equilibrium position vector
         * symbol(s): `vec(R_0)`
         * application domain: condensed matter physics
         * name: EquilibriumPositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of an ion or atom in equilibrium
         * remarks: None.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianEquilibriumPosition3dVector : CartesianEquilibriumPosition3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.3 displacement vector */
    attribute def CartesianDisplacement3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 12-7.3 displacement vector
         * symbol(s): `vec(u)`
         * application domain: condensed matter physics
         * name: DisplacementVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: difference between the position vector (ISO 80000-3) of an ion or atom and its position vector in equilibrium
         * remarks: The displacement vector can be expressed by: `vec(u) = vec(R) − vec(R_0)`, where `vec(R)` is particle position vector (item 12-7.1) and `vec(R_0)` is position vector of an ion or atom in equilibrium (item 12-7.2).
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianDisplacement3dVector : CartesianDisplacement3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-8 Debye-Waller factor */
    attribute def DebyeWallerFactorValue :> DimensionOneValue {
        doc /*
         * source: item 12-8 Debye-Waller factor
         * symbol(s): `D`, `B`
         * application domain: generic
         * name: DebyeWallerFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor by which the intensity of a diffraction line is reduced because of the lattice vibrations
         * remarks: `D` is sometimes expressed as `D = exp(−2W)`; in Mössbauer spectroscopy, it is also called the `f` factor and denoted by `f`.
         */
    }
    attribute debyeWallerFactor : DebyeWallerFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-9.1 angular wavenumber, angular repetency */
    attribute angularWavenumber : RepetencyValue :> scalarQuantities {
        doc /*
         * source: item 12-9.1 angular wavenumber, angular repetency
         * symbol(s): `k`, `q`
         * application domain: condensed matter physics
         * name: AngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: quotient of momentum (ISO 80000-4) and the reduced Planck constant (ISO 80000-1)
         * remarks: The corresponding vector (ISO 80000-2) quantity is called wave vector (ISO 80000-3), expressed by: `vec(k) = vec(p)/ħ`, where `vec(p)` is the momentum (ISO 80000-4) of quasi free electrons in an electron gas, and `ħ` is the reduced Planck constant (ISO 80000-1); for phonons, its magnitude is `k = 2π/λ`, where `λ` is the wavelength (ISO 80000-3) of the lattice vibrations. When a distinction is needed between `k` and the symbol for the Boltzmann constant (ISO 80000-1), `k_B` can be used for the latter. When a distinction is needed, `q` should be used for phonons, and `k` for particles such as electrons and neutrons. The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias angularRepetency for angularWavenumber;

    /* ISO-80000-12 item 12-9.2 Fermi angular wavenumber, Fermi angular repetency */
    attribute fermiAngularWavenumber : RepetencyValue :> scalarQuantities {
        doc /*
         * source: item 12-9.2 Fermi angular wavenumber, Fermi angular repetency
         * symbol(s): `k_F`
         * application domain: generic
         * name: FermiAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: angular wavenumber (item 12-9.1) of electrons in states on the Fermi sphere
         * remarks: In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias fermiAngularRepetency for fermiAngularWavenumber;

    /* ISO-80000-12 item 12-9.3 Debye angular wavenumber, Debye angular repetency */
    attribute debyeAngularWavenumber : RepetencyValue :> scalarQuantities {
        doc /*
         * source: item 12-9.3 Debye angular wavenumber, Debye angular repetency
         * symbol(s): `q_D`
         * application domain: generic
         * name: DebyeAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: cut-off angular wavenumber (item 12-9.1) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias debyeAngularRepetency for debyeAngularWavenumber;

    /* ISO-80000-12 item 12-10 Debye angular frequency */
    attribute debyeAngularFrequency : AngularFrequencyValue :> scalarQuantities {
        doc /*
         * source: item 12-10 Debye angular frequency
         * symbol(s): `ω_D`
         * application domain: generic
         * name: DebyeAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: cut-off angular frequency (ISO 80000-3) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified.
         */
    }

    /* ISO-80000-12 item 12-11 Debye temperature */
    attribute debyeTemperature : ThermodynamicTemperatureValue :> scalarQuantities {
        doc /*
         * source: item 12-11 Debye temperature
         * symbol(s): `Θ_D`
         * application domain: generic
         * name: DebyeTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the Debye model, quantity given by: `Θ_D = ħ*ω_D/k`, where `k` is the Boltzmann constant, (ISO 80000-1), `ħ` is the reduced Planck constant (ISO 80000-1), and `ω_D` is Debye angular frequency (item 12-10)
         * remarks: A Debye temperature can also be defined by fitting a Debye model result to a certain quantity, for instance, the heat capacity at a certain temperature.
         */
    }

    /* ISO-80000-12 item 12-12 density of vibrational states */
    attribute def DensityOfVibrationalStatesValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-12 density of vibrational states
         * symbol(s): `g`
         * application domain: angular frequency
         * name: DensityOfVibrationalStates
         * quantity dimension: L^-3*T^1
         * measurement unit(s): m^-3*s
         * tensor order: 0
         * definition: quotient of the number of vibrational modes in an infinitesimal interval of angular frequency (ISO 80000-3), and the product of the width of that interval and volume (ISO 80000-3)
         * remarks: `g(ω) = n_ω = (dn(ω))/(dω)`, where `n(ω)` is the total number of vibrational modes per volume with angular frequency less than `ω`. The density of states may also be normalized in other ways instead of with respect to volume. See also item 12-16.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DensityOfVibrationalStatesUnit [1];
    }

    attribute densityOfVibrationalStates : DensityOfVibrationalStatesValue :> scalarQuantities [*] nonunique;

    attribute def DensityOfVibrationalStatesUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-12 item 12-13 thermodynamic Grüneisen parameter */
    attribute def 'ThermodynamicGrüneisenParameterValue' :> DimensionOneValue {
        doc /*
         * source: item 12-13 thermodynamic Grüneisen parameter
         * symbol(s): `γ_G`, `Γ_G`
         * application domain: generic
         * name: ThermodynamicGrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `γ_G = (α_V)/(κ_T c_V ρ)`, where `α_V` is cubic expansion coefficient (ISO 80000-5), `κ_T` is isothermal compressibility (ISO 80000-5), `c_V` is specific heat capacity at constant volume (ISO 80000-5), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'thermodynamicGrüneisenParameter' : 'ThermodynamicGrüneisenParameterValue' :> scalarQuantities;

    /* ISO-80000-12 item 12-14 Grüneisen parameter */
    attribute def 'GrüneisenParameterValue' :> DimensionOneValue {
        doc /*
         * source: item 12-14 Grüneisen parameter
         * symbol(s): `γ`
         * application domain: generic
         * name: GrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by minus the partial differential quotient: `γ = -(del ln ω)/(del ln V)`, where `ω` is a lattice vibration frequency (ISO 80000-3), and `V` is volume (ISO 80000-3)
         * remarks: `ω` can also refer to an average of the vibrational spectrum, for instance as represented by a Debye angular frequency (item 12-10).
         */
    }
    attribute 'grüneisenParameter' : 'GrüneisenParameterValue' :> scalarQuantities;

    /* ISO-80000-12 item 12-15.1 mean free path of phonons */
    attribute meanFreePathOfPhonons : LengthValue :> scalarQuantities {
        doc /*
         * source: item 12-15.1 mean free path of phonons
         * symbol(s): `l_p`
         * application domain: generic
         * name: MeanFreePathOfPhonons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that phonons travel between two successive interactions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-15.2 mean free path of electrons */
    attribute meanFreePathOfElectrons : LengthValue :> scalarQuantities {
        doc /*
         * source: item 12-15.2 mean free path of electrons
         * symbol(s): `l_e`
         * application domain: generic
         * name: MeanFreePathOfElectrons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that electrons travel between two successive interactions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-16 energy density of states */
    attribute def EnergyDensityOfStatesValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-16 energy density of states
         * symbol(s): `n_E(E)`, `ρ(E)`
         * application domain: generic
         * name: EnergyDensityOfStates
         * quantity dimension: L^-5*M^-1*T^2
         * measurement unit(s): J^-1*m^-3*eV^-1*m^-3, kg^-1*m^-5*s^2
         * tensor order: 0
         * definition: quantity given by the differential quotient with respect to energy: `n_E(E) = (dn(E))/(dE)`, where `n_E(E)` is the total number of one-electron states per volume (ISO 80000-3) with energy less than `E` (ISO 80000-5)
         * remarks: Density of states refers to electrons or other entities, e.g. phonons. It may be normalized in other ways instead of with respect to volume, e.g. with respect to amount of substance. See also item 12-12.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyDensityOfStatesUnit [1];
    }

    attribute energyDensityOfStates : EnergyDensityOfStatesValue :> scalarQuantities [*] nonunique;

    attribute def EnergyDensityOfStatesUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -5;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-12 item 12-17 residual resistivity */
    attribute residualResistivity : ResistivityValue :> scalarQuantities {
        doc /*
         * source: item 12-17 residual resistivity
         * symbol(s): `ρ_0`
         * application domain: generic
         * name: ResidualResistivity (specializes Resistivity)
         * quantity dimension: L^3*M^1*T^-3*I^-2
         * measurement unit(s): Ω*m, kg*m^3*s^-3*A^-2
         * tensor order: 0
         * definition: for metals, the resistivity (IEC 80000-6) extrapolated to zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-18 Lorenz coefficient */
    attribute def LorenzCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-18 Lorenz coefficient
         * symbol(s): `L`
         * application domain: generic
         * name: LorenzCoefficient
         * quantity dimension: L^4*M^2*T^-6*I^-2*Θ^-2
         * measurement unit(s): V^2/K^2, kg^2*m^4*s^-6*A^-2*K^-2
         * tensor order: 0
         * definition: quotient of thermal conductivity (ISO 80000-5), and the product of electric conductivity (IEC 80000-6) and thermodynamic temperature (ISO 80000-3)
         * remarks: The Lorenz coefficient can be expressed by `L = λ/(σT)`, where `λ` is thermal conductivity (ISO 80000-5), `σ` is electric conductivity (IEC 80000-6), and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LorenzCoefficientUnit [1];
    }

    attribute lorenzCoefficient : LorenzCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def LorenzCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 4;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -6;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-12 item 12-19 Hall coefficient */
    attribute def HallCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-19 Hall coefficient
         * symbol(s): `R_H`, `A_H`
         * application domain: generic
         * name: HallCoefficient
         * quantity dimension: L^3*T^-1*I^-1
         * measurement unit(s): m^3/C, m^3*s^-1*A^-1
         * tensor order: 0
         * definition: in an isotropic conductor, relation between electric field strength, `vec(E)`, (IEC 80000-6) and electric current density, `vec(J)`, (IEC 80000-6) expressed as: `vec(E) = ρ vec(J) + R_H (vec(B) xx vec(J))`, where `ρ` is resistivity (IEC 80000-6), and `vec(B)` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HallCoefficientUnit [1];
    }

    attribute hallCoefficient : HallCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def HallCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    /* ISO-80000-12 item 12-20 thermoelectric voltage (between substances a and b) */
    attribute thermoelectricVoltageBetweenSubstancesAAndB : ElectricPotentialDifferenceValue :> scalarQuantities {
        doc /*
         * source: item 12-20 thermoelectric voltage (between substances a and b)
         * symbol(s): `E_(ab)`
         * application domain: generic
         * name: ThermoelectricVoltageBetweenSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: voltage (IEC 80000-6) between substances `a` and `b` caused by the thermoelectric effect
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-21 Seebeck coefficient (for substances a and b) */
    attribute def SeebeckCoefficientForSubstancesAAndBValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-21 Seebeck coefficient (for substances a and b)
         * symbol(s): `S_(ab)`
         * application domain: generic
         * name: SeebeckCoefficientForSubstancesAAndB
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: differential quotient of thermoelectric voltage with respect to thermodynamic temperature: `S_(ab) =      (dE_(ab))/(dT)`, where `E_(ab)` is the thermoelectric voltage between substances `a` and `b` (item 12-20) and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: This term is also called "thermoelectric power".
         */
        attribute :>> num : Real;
        attribute :>> mRef : SeebeckCoefficientForSubstancesAAndBUnit [1];
    }

    attribute seebeckCoefficientForSubstancesAAndB : SeebeckCoefficientForSubstancesAAndBValue :> scalarQuantities [*] nonunique;

    attribute def SeebeckCoefficientForSubstancesAAndBUnit :> DerivedUnit {
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
            :>> exponent = -3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-12 item 12-22 Peltier coefficient (for substances a and b) */
    attribute peltierCoefficientForSubstancesAAndB : ElectricPotentialDifferenceValue :> scalarQuantities {
        doc /*
         * source: item 12-22 Peltier coefficient (for substances a and b)
         * symbol(s): `Π_(ab)`
         * application domain: generic
         * name: PeltierCoefficientForSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: quotient of Peltier heat power (ISO 80000-5) developed at a junction, and the electric current (IEC 80000-6) flowing from substance `a` to substance `b`
         * remarks: `Π_(ab) = Π_a - Π_b`, where `Π_a` and `Π_b` are the Peltier coefficients of substances `a` and `b`, respectively.
         */
    }

    /* ISO-80000-12 item 12-23 Thomson coefficient */
    attribute def ThomsonCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-23 Thomson coefficient
         * symbol(s): `μ`
         * application domain: generic
         * name: ThomsonCoefficient
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: quotient of Thomson heat power (ISO 80000-5) developed, and the electric current (IEC 80000-6) and temperature (ISO 80000-5) difference
         * remarks: `μ` is positive if heat is developed when the temperature decreases in the direction of the electric current.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThomsonCoefficientUnit [1];
    }

    attribute thomsonCoefficient : ThomsonCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def ThomsonCoefficientUnit :> DerivedUnit {
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
            :>> exponent = -3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-12 item 12-24.1 work function */
    attribute workFunction : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-24.1 work function
         * symbol(s): `ϕ`
         * application domain: generic
         * name: WorkFunction (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and the Fermi energy (item 12-27.1)
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. The contact potential difference between substances `a` and `b` is given by `V_a - V_b = (ϕ_a - ϕ_b)/e`, where `e` is the elementary charge (ISO 80000-1). A set of energy levels, the energies of which occupy an interval practically continuously, is called an energy band. In semi-conductors `E_d` and `E_a` are used for donors and acceptors, respectively.
         */
    }

    /* ISO-80000-12 item 12-24.2 ionization energy */
    attribute ionizationEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-24.2 ionization energy
         * symbol(s): `E_i`
         * application domain: generic
         * name: IonizationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and a certain energy level which is the energy of an electron in the interior of a substance
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-25 electron affinity */
    attribute electronAffinity : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-25 electron affinity
         * symbol(s): `χ`
         * application domain: condensed matter physics
         * name: ElectronAffinity (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) difference between an electron at rest at infinity and an electron at the lowest level of the conduction band in an insulator or semiconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-26 Richardson constant */
    attribute def RichardsonConstantValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-26 Richardson constant
         * symbol(s): `A`
         * application domain: generic
         * name: RichardsonConstant
         * quantity dimension: L^-2*I^1*Θ^-2
         * measurement unit(s): A*m^-2*K^-2
         * tensor order: 0
         * definition: parameter in the expression for the thermionic emission current density `J` (IEC 80000-6) for a metal in terms of the thermodynamic temperature `T` (ISO 80000-5) and work function `ϕ`, (item 12-24.1): `J = AT^2 exp(ϕ/(kT))`, where `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : RichardsonConstantUnit [1];
    }

    attribute richardsonConstant : RichardsonConstantValue :> scalarQuantities [*] nonunique;

    attribute def RichardsonConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-12 item 12-27.1 Fermi energy */
    attribute fermiEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-27.1 Fermi energy
         * symbol(s): `E_F`
         * application domain: generic
         * name: FermiEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a metal, highest occupied energy level at zero thermodynamic temperature (ISO 80000-5), where energy level means the energy (ISO 80000-5) of an electron in the interior of a substance
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. At `T = 0 [K]`, `E_F` is equal to the chemical potential per electron. In condensed matter physics, the reference level for the energy is sometimes chosen so that, for instance, `E_F = 0`.
         */
    }

    /* ISO-80000-12 item 12-27.2 gap energy */
    attribute gapEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-27.2 gap energy
         * symbol(s): `E_g`
         * application domain: generic
         * name: GapEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference in energy (ISO 80000-5) between the lowest level of conduction band and the highest level of valence band at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-28 Fermi temperature */
    attribute fermiTemperature : ThermodynamicTemperatureValue :> scalarQuantities {
        doc /*
         * source: item 12-28 Fermi temperature
         * symbol(s): `T_F`
         * application domain: generic
         * name: FermiTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the free electron model, the Fermi energy (item 12-27.1) divided by the Boltzmann constant (ISO 80000-1)
         * remarks: The Fermi temperature is expressed by: `T_F = E_F/k`, where `E_F` is Fermi energy (item 12-27.1) and `k` is the Boltzmann constant (ISO 80000-1). `E_F` is relative to the lowest occupied state.
         */
    }

    /* ISO-80000-12 item 12-29.1 electron density */
    attribute def ElectronDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-29.1 electron density
         * symbol(s): `n`
         * application domain: generic
         * name: ElectronDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of electrons in conduction band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectronDensityUnit [1];
    }

    attribute electronDensity : ElectronDensityValue :> scalarQuantities [*] nonunique;

    attribute def ElectronDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-12 item 12-29.2 hole density */
    attribute def HoleDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-29.2 hole density
         * symbol(s): `p`
         * application domain: generic
         * name: HoleDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of holes in valence band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HoleDensityUnit [1];
    }

    attribute holeDensity : HoleDensityValue :> scalarQuantities [*] nonunique;

    attribute def HoleDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-12 item 12-29.3 intrinsic carrier density */
    attribute def IntrinsicCarrierDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-29.3 intrinsic carrier density
         * symbol(s): `n_i`
         * application domain: generic
         * name: IntrinsicCarrierDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quantity given by: `n_i = sqrt(n p)`, where `n` is electron density (item 12-29.1), and `p` is hole
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IntrinsicCarrierDensityUnit [1];
    }

    attribute intrinsicCarrierDensity : IntrinsicCarrierDensityValue :> scalarQuantities [*] nonunique;

    attribute def IntrinsicCarrierDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-12 item 12-29.4 donor density */
    attribute def DonorDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-29.4 donor density
         * symbol(s): `n_d`
         * application domain: generic
         * name: DonorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of donor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DonorDensityUnit [1];
    }

    attribute donorDensity : DonorDensityValue :> scalarQuantities [*] nonunique;

    attribute def DonorDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-12 item 12-29.5 acceptor density */
    attribute def AcceptorDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 12-29.5 acceptor density
         * symbol(s): `n_a`
         * application domain: generic
         * name: AcceptorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of acceptor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AcceptorDensityUnit [1];
    }

    attribute acceptorDensity : AcceptorDensityValue :> scalarQuantities [*] nonunique;

    attribute def AcceptorDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-12 item 12-30 effective mass */
    attribute effectiveMass : MassValue :> scalarQuantities {
        doc /*
         * source: item 12-30 effective mass
         * symbol(s): `m"*"`
         * application domain: generic
         * name: EffectiveMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: quantity given by: `m^"*" = (ħ^2 k) / ((dε)/(dk))`, where `k` is wavenumber (ISO 80000-3), `ε` is the energy (ISO 80000-5) of an electron in the interior of a substance, and `ħ` is the reduced Planck constant (ISO 80000-1)
         * remarks: When `k` refers to a state where `ε` has an extremum, `m"*" = (ħ^2 k) / ((d^2ε)/(dk^2))`. The effective mass can be generalized to refer to an anisotropic system with `ε = ε(k)`.
         */
    }

    /* ISO-80000-12 item 12-31 mobility ratio */
    attribute def MobilityRatioValue :> DimensionOneValue {
        doc /*
         * source: item 12-31 mobility ratio
         * symbol(s): `b`
         * application domain: generic
         * name: MobilityRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mobilities (ISO 80000-10) of electrons and holes, respectively
         * remarks: The mobility ratio can be expressed by: `b = μ_n/μ_p`, where `μ_n` and `μ_p` are mobilities (ISO 80000-10) for electrons and holes, respectively.
         */
    }
    attribute mobilityRatio : MobilityRatioValue :> scalarQuantities;

    /* ISO-80000-12 item 12-32.1 relaxation time */
    attribute relaxationTime : DurationValue :> scalarQuantities {
        doc /*
         * source: item 12-32.1 relaxation time
         * symbol(s): `τ`
         * application domain: condensed matter physics
         * name: RelaxationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for scattering, trapping or annihilation of charge carriers, phonons or other quasiparticles
         * remarks: For electrons in metals, `τ = l/v_F`, where `l` is mean free path (item 12-15.2) and `v_F` is speed (ISO 80000-3) of electrons on the Fermi surface.
         */
    }

    /* ISO-80000-12 item 12-32.2 carrier lifetime */
    attribute carrierLifetime : DurationValue :> scalarQuantities {
        doc /*
         * source: item 12-32.2 carrier lifetime
         * symbol(s): `τ`, `τ_n`, `τ_p`
         * application domain: semiconductors
         * name: CarrierLifetime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for recombination or trapping of minority charge carriers in semiconductors
         * remarks: Indices "n" and "p" denote negative and positive charge carriers, respectively. Positive charge carriers can also be holes.
         */
    }

    /* ISO-80000-12 item 12-33 diffusion length */
    attribute diffusionLengthForCondensedMatterPhysics : LengthValue :> scalarQuantities {
        doc /*
         * source: item 12-33 diffusion length
         * symbol(s): `L`, `L_n`, `L_p`
         * application domain: condensed matter physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the product of diffusion coefficient (ISO 80000-10) and lifetime (ISO 80000-10)
         * remarks: The diffusion length can be expressed by: `L = sqrt(Dτ)`, where `D` is the diffusion coefficient (ISO 80000-9) and `τ` is lifetime (ISO 80000-3).
         */
    }

    /* ISO-80000-12 item 12-34 exchange integral */
    attribute exchangeIntegral : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-34 exchange integral
         * symbol(s): `K`, `J`
         * application domain: generic
         * name: ExchangeIntegral (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: constituent of the interaction energy (ISO 80000-5) between the spins of adjacent electrons in matter arising from the overlap of electron state functions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-35.1 Curie temperature */
    attribute curieTemperature : ThermodynamicTemperatureValue :> scalarQuantities {
        doc /*
         * source: item 12-35.1 Curie temperature
         * symbol(s): `T_C`
         * application domain: generic
         * name: CurieTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a ferromagnet
         * remarks: `T_(cr)` is used for critical thermodynamic temperature in general.
         */
    }

    /* ISO-80000-12 item 12-35.2 Néel temperature */
    attribute 'néelTemperature' : ThermodynamicTemperatureValue :> scalarQuantities {
        doc /*
         * source: item 12-35.2 Néel temperature
         * symbol(s): `T_N`
         * application domain: generic
         * name: NéelTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of an antiferromagnet
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-35.3 superconduction transition temperature */
    attribute superconductionTransitionTemperature : ThermodynamicTemperatureValue :> scalarQuantities {
        doc /*
         * source: item 12-35.3 superconduction transition temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: SuperconductionTransitionTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-36.1 thermodynamic critical magnetic flux density */
    attribute thermodynamicCriticalMagneticFluxDensity : MagneticFluxDensityValue :> scalarQuantities {
        doc /*
         * source: item 12-36.1 thermodynamic critical magnetic flux density
         * symbol(s): `B_c`
         * application domain: generic
         * name: ThermodynamicCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: quantity given by: `B_c = sqrt((2μ_0 (G_n - G_s))/V)`, where `G_n` and `G_s` are the Gibbs energies (ISO 80000-5) at zero magnetic flux density (IEC 80000-6) in a normal conductor and superconductor, respectively, `μ_0` is the magnetic constant (IEC 80000-6), and `V` is volume (ISO 80000-3)
         * remarks: In type I superconductors, `B_c` is the critical magnetic flux density for disappearance of superconductivity. The symbol `B_(c3)` is used for the critical magnetic flux density for disappearance of surface superconductivity.
         */
    }

    /* ISO-80000-12 item 12-36.2 lower critical magnetic flux density */
    attribute lowerCriticalMagneticFluxDensity : MagneticFluxDensityValue :> scalarQuantities {
        doc /*
         * source: item 12-36.2 lower critical magnetic flux density
         * symbol(s): `B_(c1)`
         * application domain: generic
         * name: LowerCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for magnetic flux (IEC 80000-6) entering the superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-36.3 upper critical magnetic flux density */
    attribute upperCriticalMagneticFluxDensity : MagneticFluxDensityValue :> scalarQuantities {
        doc /*
         * source: item 12-36.3 upper critical magnetic flux density
         * symbol(s): `B_(c2)`
         * application domain: generic
         * name: UpperCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for disappearance of bulk superconductivity
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-37 superconductor energy gap */
    attribute superconductorEnergyGap : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 12-37 superconductor energy gap
         * symbol(s): `Δ`
         * application domain: generic
         * name: SuperconductorEnergyGap (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: width of the forbidden energy band (item 12-24.2) in a superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-38.1 London penetration depth */
    attribute londonPenetrationDepth : LengthValue :> scalarQuantities {
        doc /*
         * source: item 12-38.1 London penetration depth
         * symbol(s): `λ_L`
         * application domain: generic
         * name: LondonPenetrationDepth (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) a magnetic field penetrates the plane surface of a semi-finite superconductor according to the expression: `B(x) = B(0) exp(-x/λ_L)`, where `B` is magnetic flux density (IEC 80000-6) and `x` is distance (ISO 80000-3) from the surface
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-38.2 coherence length */
    attribute coherenceLength : LengthValue :> scalarQuantities {
        doc /*
         * source: item 12-38.2 coherence length
         * symbol(s): `ξ`
         * application domain: generic
         * name: CoherenceLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) in a superconductor over which the effect of a perturbation is appreciable at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ISQCondensedMatter'
      (documentation)
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'ISQBase'[unresolved])
      (membership_import private -> 'ISQElectromagnetism::ElectricPotentialDifferenceValue'[unresolved])
      (membership_import private -> 'ISQElectromagnetism::MagneticFluxDensityValue'[unresolved])
      (membership_import private -> 'ISQElectromagnetism::ResistivityValue'[unresolved])
      (membership_import private -> 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame'[unresolved])
      (membership_import private -> 'ISQSpaceTime::AngularFrequencyValue'[unresolved])
      (membership_import private -> 'ISQSpaceTime::AngularMeasureValue'[unresolved])
      (membership_import private -> 'ISQSpaceTime::RepetencyValue'[unresolved])
      (membership_import private -> 'ISQThermodynamics::EnergyValue'[unresolved])
      (attribute_def 'CartesianLattice3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianLattice3dVector' : 'ISQCondensedMatter::CartesianLattice3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianFundamentalLattice3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianFundamentalLattice3dVector' : 'ISQCondensedMatter::CartesianFundamentalLattice3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'AngularReciprocalLatticeVectorMagnitudeValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'angularReciprocalLatticeVectorMagnitude' : 'ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AngularReciprocalLatticeVectorMagnitudeUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CartesianAngularReciprocalLattice3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianAngularReciprocalLattice3dVector' : 'ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianAngularReciprocalLattice3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_def 'FundamentalReciprocalLatticeVectorMagnitudeValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'fundamentalReciprocalLatticeVectorMagnitude' : 'ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'FundamentalReciprocalLatticeVectorMagnitudeUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CartesianFundamentalReciprocalLattice3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianFundamentalReciprocalLattice3dVector' : 'ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianFundamentalReciprocalLattice3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_usage 'latticePlaneSpacing' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'braggAngle' : 'AngularMeasureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'ShortRangeOrderParameterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'shortRangeOrderParameter' : 'ISQCondensedMatter::ShortRangeOrderParameterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'LongRangeOrderParameterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'longRangeOrderParameter' : 'ISQCondensedMatter::LongRangeOrderParameterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'AtomicScatteringFactorValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'atomicScatteringFactor' : 'ISQCondensedMatter::AtomicScatteringFactorValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'StructureFactorValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'structureFactor' : 'ISQCondensedMatter::StructureFactorValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'CartesianBurgers3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianBurgers3dVector' : 'ISQCondensedMatter::CartesianBurgers3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianParticlePosition3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianParticlePosition3dVector' : 'ISQCondensedMatter::CartesianParticlePosition3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianEquilibriumPosition3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianEquilibriumPosition3dVector' : 'ISQCondensedMatter::CartesianEquilibriumPosition3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianDisplacement3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianDisplacement3dVector' : 'ISQCondensedMatter::CartesianDisplacement3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'DebyeWallerFactorValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'debyeWallerFactor' : 'ISQCondensedMatter::DebyeWallerFactorValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_usage 'angularWavenumber' : 'RepetencyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'angularRepetency' -> 'ISQCondensedMatter::angularWavenumber'[attribute_usage])
      (attribute_usage 'fermiAngularWavenumber' : 'RepetencyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'fermiAngularRepetency' -> 'ISQCondensedMatter::fermiAngularWavenumber'[attribute_usage])
      (attribute_usage 'debyeAngularWavenumber' : 'RepetencyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'debyeAngularRepetency' -> 'ISQCondensedMatter::debyeAngularWavenumber'[attribute_usage])
      (attribute_usage 'debyeAngularFrequency' : 'AngularFrequencyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'debyeTemperature' : 'ThermodynamicTemperatureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'DensityOfVibrationalStatesValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::DensityOfVibrationalStatesUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'densityOfVibrationalStates' : 'ISQCondensedMatter::DensityOfVibrationalStatesValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'DensityOfVibrationalStatesUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'ThermodynamicGrüneisenParameterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'thermodynamicGrüneisenParameter' : 'ISQCondensedMatter::ThermodynamicGrüneisenParameterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'GrüneisenParameterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'grüneisenParameter' : 'ISQCondensedMatter::GrüneisenParameterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_usage 'meanFreePathOfPhonons' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'meanFreePathOfElectrons' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'EnergyDensityOfStatesValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::EnergyDensityOfStatesUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'energyDensityOfStates' : 'ISQCondensedMatter::EnergyDensityOfStatesValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'EnergyDensityOfStatesUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'residualResistivity' : 'ResistivityValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'LorenzCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::LorenzCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'lorenzCoefficient' : 'ISQCondensedMatter::LorenzCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'LorenzCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'electricCurrentPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'HallCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::HallCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'hallCoefficient' : 'ISQCondensedMatter::HallCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'HallCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'electricCurrentPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'thermoelectricVoltageBetweenSubstancesAAndB' : 'ElectricPotentialDifferenceValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'SeebeckCoefficientForSubstancesAAndBValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'seebeckCoefficientForSubstancesAAndB' : 'ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SeebeckCoefficientForSubstancesAAndBUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'electricCurrentPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'peltierCoefficientForSubstancesAAndB' : 'ElectricPotentialDifferenceValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'ThomsonCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::ThomsonCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'thomsonCoefficient' : 'ISQCondensedMatter::ThomsonCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ThomsonCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'electricCurrentPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'workFunction' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'ionizationEnergy' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'electronAffinity' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'RichardsonConstantValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::RichardsonConstantUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'richardsonConstant' : 'ISQCondensedMatter::RichardsonConstantValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'RichardsonConstantUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'electricCurrentPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'fermiEnergy' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'gapEnergy' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'fermiTemperature' : 'ThermodynamicTemperatureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'ElectronDensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::ElectronDensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'electronDensity' : 'ISQCondensedMatter::ElectronDensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ElectronDensityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'HoleDensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::HoleDensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'holeDensity' : 'ISQCondensedMatter::HoleDensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'HoleDensityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'IntrinsicCarrierDensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::IntrinsicCarrierDensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'intrinsicCarrierDensity' : 'ISQCondensedMatter::IntrinsicCarrierDensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'IntrinsicCarrierDensityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'DonorDensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::DonorDensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'donorDensity' : 'ISQCondensedMatter::DonorDensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'DonorDensityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'AcceptorDensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQCondensedMatter::AcceptorDensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'acceptorDensity' : 'ISQCondensedMatter::AcceptorDensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AcceptorDensityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'effectiveMass' : 'MassValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'MobilityRatioValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'mobilityRatio' : 'ISQCondensedMatter::MobilityRatioValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_usage 'relaxationTime' : 'DurationValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'carrierLifetime' : 'DurationValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'diffusionLengthForCondensedMatterPhysics' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'exchangeIntegral' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'curieTemperature' : 'ThermodynamicTemperatureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'néelTemperature' : 'ThermodynamicTemperatureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'superconductionTransitionTemperature' : 'ThermodynamicTemperatureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'thermodynamicCriticalMagneticFluxDensity' : 'MagneticFluxDensityValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'lowerCriticalMagneticFluxDensity' : 'MagneticFluxDensityValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'upperCriticalMagneticFluxDensity' : 'MagneticFluxDensityValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'superconductorEnergyGap' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'londonPenetrationDepth' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'coherenceLength' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation)))))
~~~
