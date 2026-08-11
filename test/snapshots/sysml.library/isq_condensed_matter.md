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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_condensed_matter.md"
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
        (range (start 20 19) (end 20 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 19) (end 21 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 19) (end 22 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 19) (end 23 66))
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
        (range (start 25 19) (end 25 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 19) (end 26 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 19) (end 27 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 4) (end 30 688))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 4) (end 50 810))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 4) (end 70 749))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 4) (end 89 269))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 8) (end 90 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 4) (end 94 747))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 4) (end 113 258))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 120 4) (end 120 783))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 4) (end 139 273))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 140 8) (end 140 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 144 4) (end 144 791))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 266))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 170 4) (end 170 578))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 202 4) (end 202 727))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 219 4) (end 219 700))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 236 4) (end 236 747))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 253 4) (end 253 789))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 270 4) (end 270 599))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 290 4) (end 290 701))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 310 4) (end 310 656))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 330 4) (end 330 888))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 350 4) (end 350 656))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 437 4) (end 437 819))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 453 4) (end 453 972))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 472 4) (end 472 375))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 473 8) (end 473 103))
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
        (range (start 479 4) (end 479 756))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 496 4) (end 496 750))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 513 4) (end 513 528))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 529 4) (end 529 536))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 545 4) (end 545 978))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 564 4) (end 564 480))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 565 8) (end 565 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 566 8) (end 566 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 567 8) (end 567 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 588 4) (end 588 885))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 607 4) (end 607 760))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 608 8) (end 608 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 609 8) (end 609 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 610 8) (end 610 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 611 8) (end 611 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 612 8) (end 612 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 617 4) (end 617 818))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 636 4) (end 636 496))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 637 8) (end 637 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 638 8) (end 638 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 639 8) (end 639 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 660 4) (end 660 926))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 679 4) (end 679 779))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 680 8) (end 680 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 681 8) (end 681 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 682 8) (end 682 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 683 8) (end 683 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 684 8) (end 684 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 705 4) (end 705 778))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 724 4) (end 724 761))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 725 8) (end 725 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 726 8) (end 726 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 727 8) (end 727 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 728 8) (end 728 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 729 8) (end 729 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 782 4) (end 782 789))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 801 4) (end 801 534))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 802 8) (end 802 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 803 8) (end 803 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 804 8) (end 804 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 841 4) (end 841 754))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 857 4) (end 857 818))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 876 4) (end 876 245))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 877 8) (end 877 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 882 4) (end 882 795))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 901 4) (end 901 241))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 902 8) (end 902 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 907 4) (end 907 877))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 926 4) (end 926 253))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 927 8) (end 927 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 932 4) (end 932 535))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 951 4) (end 951 242))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 952 8) (end 952 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 957 4) (end 957 550))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 976 4) (end 976 245))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 977 8) (end 977 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 982 4) (end 982 815))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 998 4) (end 998 638))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1015 4) (end 1015 702))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1031 4) (end 1031 668))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1047 4) (end 1047 709))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1079 4) (end 1079 588))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1095 4) (end 1095 533))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1111 4) (end 1111 590))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1191 4) (end 1191 701))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1207 4) (end 1207 565))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f98422a683043b9ad359842a0eec9299843f8eb5e327a93910a18809523524b4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter"))) (kind "package") (name "ISQCondensedMatter") (declared-name "ISQCondensedMatter") (range (start (line 0) (character 0)) (end (line 0) (character 59984))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit"))) (kind "attribute def") (name "AcceptorDensityUnit") (declared-name "AcceptorDensityUnit") (range (start (line 976) (character 4)) (end (line 976) (character 245))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 977) (character 8)) (end (line 977) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 978) (character 8)) (end (line 978) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 978) (character 22)) (end (line 978) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue"))) (kind "attribute def") (name "AcceptorDensityValue") (declared-name "AcceptorDensityValue") (range (start (line 957) (character 4)) (end (line 957) (character 550))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 957) (character 4)) (end (line 957) (character 550))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 971) (character 8)) (end (line 971) (character 51))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptorDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 971) (character 22)) (end (line 971) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 970) (character 8)) (end (line 970) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 970) (character 22)) (end (line 970) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularFrequencyValue"))) (kind "import") (name "AngularFrequencyValue") (declared-name "AngularFrequencyValue") (range (start (line 24) (character 4)) (end (line 24) (character 55))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::AngularFrequencyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 24) (character 19)) (end (line 24) (character 54))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularMeasureValue"))) (kind "import") (name "AngularMeasureValue") (declared-name "AngularMeasureValue") (range (start (line 25) (character 4)) (end (line 25) (character 53))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::AngularMeasureValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 25) (character 19)) (end (line 25) (character 52))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit"))) (kind "attribute def") (name "AngularReciprocalLatticeVectorMagnitudeUnit") (declared-name "AngularReciprocalLatticeVectorMagnitudeUnit") (range (start (line 89) (character 4)) (end (line 89) (character 269))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 90) (character 8)) (end (line 90) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 91) (character 8)) (end (line 91) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 91) (character 22)) (end (line 91) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue"))) (kind "attribute def") (name "AngularReciprocalLatticeVectorMagnitudeValue") (declared-name "AngularReciprocalLatticeVectorMagnitudeValue") (range (start (line 70) (character 4)) (end (line 70) (character 749))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 70) (character 4)) (end (line 70) (character 749))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 84) (character 8)) (end (line 84) (character 75))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularReciprocalLatticeVectorMagnitudeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 84) (character 22)) (end (line 84) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 83) (character 8)) (end (line 83) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 83) (character 22)) (end (line 83) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AtomicScatteringFactorValue"))) (kind "attribute def") (name "AtomicScatteringFactorValue") (declared-name "AtomicScatteringFactorValue") (range (start (line 236) (character 4)) (end (line 236) (character 747))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::AtomicScatteringFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 236) (character 4)) (end (line 236) (character 747))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::AtomicScatteringFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularReciprocalLattice3dCoordinateFrame") (declared-name "CartesianAngularReciprocalLattice3dCoordinateFrame") (range (start (line 113) (character 4)) (end (line 113) (character 258))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 114) (character 8)) (end (line 114) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 114) (character 22)) (end (line 114) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 115) (character 8)) (end (line 115) (character 42))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 115) (character 22)) (end (line 115) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 116) (character 8)) (end (line 116) (character 76))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularReciprocalLatticeVectorMagnitudeUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 116) (character 22)) (end (line 116) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector"))) (kind "attribute def") (name "CartesianAngularReciprocalLattice3dVector") (declared-name "CartesianAngularReciprocalLattice3dVector") (range (start (line 94) (character 4)) (end (line 94) (character 747))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 94) (character 4)) (end (line 94) (character 747))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 107) (character 8)) (end (line 107) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 107) (character 22)) (end (line 107) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 108) (character 8)) (end (line 108) (character 82))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularReciprocalLattice3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 108) (character 22)) (end (line 108) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector"))) (kind "attribute def") (name "CartesianBurgers3dVector") (declared-name "CartesianBurgers3dVector") (range (start (line 270) (character 4)) (end (line 270) (character 599))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 270) (character 4)) (end (line 270) (character 599))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 283) (character 8)) (end (line 283) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 283) (character 22)) (end (line 283) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 284) (character 8)) (end (line 284) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 284) (character 22)) (end (line 284) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector"))) (kind "attribute def") (name "CartesianDisplacement3dVector") (declared-name "CartesianDisplacement3dVector") (range (start (line 330) (character 4)) (end (line 330) (character 888))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 330) (character 4)) (end (line 330) (character 888))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 343) (character 8)) (end (line 343) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 343) (character 22)) (end (line 343) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 344) (character 8)) (end (line 344) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 344) (character 22)) (end (line 344) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector"))) (kind "attribute def") (name "CartesianEquilibriumPosition3dVector") (declared-name "CartesianEquilibriumPosition3dVector") (range (start (line 310) (character 4)) (end (line 310) (character 656))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 310) (character 4)) (end (line 310) (character 656))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 323) (character 8)) (end (line 323) (character 37))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 323) (character 22)) (end (line 323) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 324) (character 8)) (end (line 324) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 324) (character 22)) (end (line 324) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector"))) (kind "attribute def") (name "CartesianFundamentalLattice3dVector") (declared-name "CartesianFundamentalLattice3dVector") (range (start (line 50) (character 4)) (end (line 50) (character 810))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 4)) (end (line 50) (character 810))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 63) (character 8)) (end (line 63) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 63) (character 22)) (end (line 63) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 64) (character 8)) (end (line 64) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 64) (character 22)) (end (line 64) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame"))) (kind "attribute def") (name "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (declared-name "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (range (start (line 163) (character 4)) (end (line 163) (character 266))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 164) (character 8)) (end (line 164) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 164) (character 22)) (end (line 164) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 165) (character 8)) (end (line 165) (character 42))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 165) (character 22)) (end (line 165) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 166) (character 8)) (end (line 166) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "FundamentalReciprocalLatticeVectorMagnitudeUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 166) (character 22)) (end (line 166) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector"))) (kind "attribute def") (name "CartesianFundamentalReciprocalLattice3dVector") (declared-name "CartesianFundamentalReciprocalLattice3dVector") (range (start (line 144) (character 4)) (end (line 144) (character 791))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 144) (character 4)) (end (line 144) (character 791))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 157) (character 8)) (end (line 157) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 157) (character 22)) (end (line 157) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 158) (character 8)) (end (line 158) (character 86))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 158) (character 22)) (end (line 158) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector"))) (kind "attribute def") (name "CartesianLattice3dVector") (declared-name "CartesianLattice3dVector") (range (start (line 30) (character 4)) (end (line 30) (character 688))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 30) (character 4)) (end (line 30) (character 688))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 43) (character 8)) (end (line 43) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 43) (character 22)) (end (line 43) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 44) (character 8)) (end (line 44) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 44) (character 22)) (end (line 44) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector"))) (kind "attribute def") (name "CartesianParticlePosition3dVector") (declared-name "CartesianParticlePosition3dVector") (range (start (line 290) (character 4)) (end (line 290) (character 701))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 290) (character 4)) (end (line 290) (character 701))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 303) (character 8)) (end (line 303) (character 37))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 303) (character 22)) (end (line 303) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 304) (character 8)) (end (line 304) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 304) (character 22)) (end (line 304) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (kind "import") (name "CartesianSpatial3dCoordinateFrame") (declared-name "CartesianSpatial3dCoordinateFrame") (range (start (line 23) (character 4)) (end (line 23) (character 67))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::CartesianSpatial3dCoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 23) (character 19)) (end (line 23) (character 66))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DebyeWallerFactorValue"))) (kind "attribute def") (name "DebyeWallerFactorValue") (declared-name "DebyeWallerFactorValue") (range (start (line 350) (character 4)) (end (line 350) (character 656))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DebyeWallerFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 350) (character 4)) (end (line 350) (character 656))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DebyeWallerFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit"))) (kind "attribute def") (name "DensityOfVibrationalStatesUnit") (declared-name "DensityOfVibrationalStatesUnit") (range (start (line 472) (character 4)) (end (line 472) (character 375))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 474) (character 8)) (end (line 474) (character 104))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 473) (character 8)) (end (line 473) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 475) (character 8)) (end (line 475) (character 94))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 475) (character 22)) (end (line 475) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue"))) (kind "attribute def") (name "DensityOfVibrationalStatesValue") (declared-name "DensityOfVibrationalStatesValue") (range (start (line 453) (character 4)) (end (line 453) (character 972))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::_documentation"))) (kind "documentation") (name "") (range (start (line 453) (character 4)) (end (line 453) (character 972))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 467) (character 8)) (end (line 467) (character 62))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DensityOfVibrationalStatesUnit") (range none)) (redefinition (reference "mRef") (range (start (line 467) (character 22)) (end (line 467) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 466) (character 8)) (end (line 466) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 466) (character 22)) (end (line 466) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit"))) (kind "attribute def") (name "DonorDensityUnit") (declared-name "DonorDensityUnit") (range (start (line 951) (character 4)) (end (line 951) (character 242))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 952) (character 8)) (end (line 952) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 953) (character 8)) (end (line 953) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 953) (character 22)) (end (line 953) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue"))) (kind "attribute def") (name "DonorDensityValue") (declared-name "DonorDensityValue") (range (start (line 932) (character 4)) (end (line 932) (character 535))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 932) (character 4)) (end (line 932) (character 535))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 946) (character 8)) (end (line 946) (character 48))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DonorDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 946) (character 22)) (end (line 946) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 945) (character 8)) (end (line 945) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 945) (character 22)) (end (line 945) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectricPotentialDifferenceValue"))) (kind "import") (name "ElectricPotentialDifferenceValue") (declared-name "ElectricPotentialDifferenceValue") (range (start (line 20) (character 4)) (end (line 20) (character 73))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQElectromagnetism::ElectricPotentialDifferenceValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 19)) (end (line 20) (character 72))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit"))) (kind "attribute def") (name "ElectronDensityUnit") (declared-name "ElectronDensityUnit") (range (start (line 876) (character 4)) (end (line 876) (character 245))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 877) (character 8)) (end (line 877) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 878) (character 8)) (end (line 878) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 878) (character 22)) (end (line 878) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue"))) (kind "attribute def") (name "ElectronDensityValue") (declared-name "ElectronDensityValue") (range (start (line 857) (character 4)) (end (line 857) (character 818))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 857) (character 4)) (end (line 857) (character 818))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 871) (character 8)) (end (line 871) (character 51))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ElectronDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 871) (character 22)) (end (line 871) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 870) (character 8)) (end (line 870) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 870) (character 22)) (end (line 870) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (kind "attribute def") (name "EnergyDensityOfStatesUnit") (declared-name "EnergyDensityOfStatesUnit") (range (start (line 564) (character 4)) (end (line 564) (character 480))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 567) (character 8)) (end (line 567) (character 104))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 565) (character 8)) (end (line 565) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 566) (character 8)) (end (line 566) (character 101))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 568) (character 8)) (end (line 568) (character 102))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 568) (character 22)) (end (line 568) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue"))) (kind "attribute def") (name "EnergyDensityOfStatesValue") (declared-name "EnergyDensityOfStatesValue") (range (start (line 545) (character 4)) (end (line 545) (character 978))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::_documentation"))) (kind "documentation") (name "") (range (start (line 545) (character 4)) (end (line 545) (character 978))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 559) (character 8)) (end (line 559) (character 57))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EnergyDensityOfStatesUnit") (range none)) (redefinition (reference "mRef") (range (start (line 559) (character 22)) (end (line 559) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 558) (character 8)) (end (line 558) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 558) (character 22)) (end (line 558) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (kind "import") (name "EnergyValue") (declared-name "EnergyValue") (range (start (line 27) (character 4)) (end (line 27) (character 50))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQThermodynamics::EnergyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 27) (character 19)) (end (line 27) (character 49))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit"))) (kind "attribute def") (name "FundamentalReciprocalLatticeVectorMagnitudeUnit") (declared-name "FundamentalReciprocalLatticeVectorMagnitudeUnit") (range (start (line 139) (character 4)) (end (line 139) (character 273))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 140) (character 8)) (end (line 140) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 141) (character 8)) (end (line 141) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 141) (character 22)) (end (line 141) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue"))) (kind "attribute def") (name "FundamentalReciprocalLatticeVectorMagnitudeValue") (declared-name "FundamentalReciprocalLatticeVectorMagnitudeValue") (range (start (line 120) (character 4)) (end (line 120) (character 783))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 120) (character 4)) (end (line 120) (character 783))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 134) (character 8)) (end (line 134) (character 79))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "FundamentalReciprocalLatticeVectorMagnitudeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 134) (character 22)) (end (line 134) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 133) (character 8)) (end (line 133) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 133) (character 22)) (end (line 133) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::GrüneisenParameterValue"))) (kind "attribute def") (name "GrüneisenParameterValue") (declared-name "GrüneisenParameterValue") (range (start (line 496) (character 4)) (end (line 496) (character 750))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::GrüneisenParameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 496) (character 4)) (end (line 496) (character 750))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::GrüneisenParameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (kind "attribute def") (name "HallCoefficientUnit") (declared-name "HallCoefficientUnit") (range (start (line 636) (character 4)) (end (line 636) (character 496))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 638) (character 8)) (end (line 638) (character 105))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 639) (character 8)) (end (line 639) (character 112))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 637) (character 8)) (end (line 637) (character 102))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 640) (character 8)) (end (line 640) (character 113))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 640) (character 22)) (end (line 640) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue"))) (kind "attribute def") (name "HallCoefficientValue") (declared-name "HallCoefficientValue") (range (start (line 617) (character 4)) (end (line 617) (character 818))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 617) (character 4)) (end (line 617) (character 818))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 631) (character 8)) (end (line 631) (character 51))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "HallCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 631) (character 22)) (end (line 631) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 630) (character 8)) (end (line 630) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 630) (character 22)) (end (line 630) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit"))) (kind "attribute def") (name "HoleDensityUnit") (declared-name "HoleDensityUnit") (range (start (line 901) (character 4)) (end (line 901) (character 241))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 902) (character 8)) (end (line 902) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 903) (character 8)) (end (line 903) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 903) (character 22)) (end (line 903) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue"))) (kind "attribute def") (name "HoleDensityValue") (declared-name "HoleDensityValue") (range (start (line 882) (character 4)) (end (line 882) (character 795))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 882) (character 4)) (end (line 882) (character 795))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 896) (character 8)) (end (line 896) (character 47))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "HoleDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 896) (character 22)) (end (line 896) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 895) (character 8)) (end (line 895) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 895) (character 22)) (end (line 895) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit"))) (kind "attribute def") (name "IntrinsicCarrierDensityUnit") (declared-name "IntrinsicCarrierDensityUnit") (range (start (line 926) (character 4)) (end (line 926) (character 253))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 927) (character 8)) (end (line 927) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 928) (character 8)) (end (line 928) (character 80))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 928) (character 22)) (end (line 928) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue"))) (kind "attribute def") (name "IntrinsicCarrierDensityValue") (declared-name "IntrinsicCarrierDensityValue") (range (start (line 907) (character 4)) (end (line 907) (character 877))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 907) (character 4)) (end (line 907) (character 877))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 921) (character 8)) (end (line 921) (character 59))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IntrinsicCarrierDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 921) (character 22)) (end (line 921) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 920) (character 8)) (end (line 920) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 920) (character 22)) (end (line 920) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LongRangeOrderParameterValue"))) (kind "attribute def") (name "LongRangeOrderParameterValue") (declared-name "LongRangeOrderParameterValue") (range (start (line 219) (character 4)) (end (line 219) (character 700))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LongRangeOrderParameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 219) (character 4)) (end (line 219) (character 700))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LongRangeOrderParameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (kind "attribute def") (name "LorenzCoefficientUnit") (declared-name "LorenzCoefficientUnit") (range (start (line 607) (character 4)) (end (line 607) (character 760))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 610) (character 8)) (end (line 610) (character 105))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 611) (character 8)) (end (line 611) (character 112))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 608) (character 8)) (end (line 608) (character 102))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 609) (character 8)) (end (line 609) (character 100))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 613) (character 8)) (end (line 613) (character 149))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 613) (character 22)) (end (line 613) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 612) (character 8)) (end (line 612) (character 124))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue"))) (kind "attribute def") (name "LorenzCoefficientValue") (declared-name "LorenzCoefficientValue") (range (start (line 588) (character 4)) (end (line 588) (character 885))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 588) (character 4)) (end (line 588) (character 885))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 602) (character 8)) (end (line 602) (character 53))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LorenzCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 602) (character 22)) (end (line 602) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 601) (character 8)) (end (line 601) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 601) (character 22)) (end (line 601) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue"))) (kind "import") (name "MagneticFluxDensityValue") (declared-name "MagneticFluxDensityValue") (range (start (line 21) (character 4)) (end (line 21) (character 65))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQElectromagnetism::MagneticFluxDensityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 19)) (end (line 21) (character 64))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::MobilityRatioValue"))) (kind "attribute def") (name "MobilityRatioValue") (declared-name "MobilityRatioValue") (range (start (line 998) (character 4)) (end (line 998) (character 638))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::MobilityRatioValue::_documentation"))) (kind "documentation") (name "") (range (start (line 998) (character 4)) (end (line 998) (character 638))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::MobilityRatioValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue"))) (kind "import") (name "RepetencyValue") (declared-name "RepetencyValue") (range (start (line 26) (character 4)) (end (line 26) (character 48))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::RepetencyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 26) (character 19)) (end (line 26) (character 47))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ResistivityValue"))) (kind "import") (name "ResistivityValue") (declared-name "ResistivityValue") (range (start (line 22) (character 4)) (end (line 22) (character 57))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQElectromagnetism::ResistivityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 22) (character 19)) (end (line 22) (character 56))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (kind "attribute def") (name "RichardsonConstantUnit") (declared-name "RichardsonConstantUnit") (range (start (line 801) (character 4)) (end (line 801) (character 534))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 803) (character 8)) (end (line 803) (character 111))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 802) (character 8)) (end (line 802) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 805) (character 8)) (end (line 805) (character 129))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 805) (character 22)) (end (line 805) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 804) (character 8)) (end (line 804) (character 124))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue"))) (kind "attribute def") (name "RichardsonConstantValue") (declared-name "RichardsonConstantValue") (range (start (line 782) (character 4)) (end (line 782) (character 789))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::_documentation"))) (kind "documentation") (name "") (range (start (line 782) (character 4)) (end (line 782) (character 789))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 796) (character 8)) (end (line 796) (character 54))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RichardsonConstantUnit") (range none)) (redefinition (reference "mRef") (range (start (line 796) (character 22)) (end (line 796) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 795) (character 8)) (end (line 795) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 795) (character 22)) (end (line 795) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (kind "attribute def") (name "SeebeckCoefficientForSubstancesAAndBUnit") (declared-name "SeebeckCoefficientForSubstancesAAndBUnit") (range (start (line 679) (character 4)) (end (line 679) (character 779))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 682) (character 8)) (end (line 682) (character 105))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 683) (character 8)) (end (line 683) (character 112))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 680) (character 8)) (end (line 680) (character 102))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 681) (character 8)) (end (line 681) (character 100))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 685) (character 8)) (end (line 685) (character 149))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 685) (character 22)) (end (line 685) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 684) (character 8)) (end (line 684) (character 124))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue"))) (kind "attribute def") (name "SeebeckCoefficientForSubstancesAAndBValue") (declared-name "SeebeckCoefficientForSubstancesAAndBValue") (range (start (line 660) (character 4)) (end (line 660) (character 926))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::_documentation"))) (kind "documentation") (name "") (range (start (line 660) (character 4)) (end (line 660) (character 926))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 674) (character 8)) (end (line 674) (character 72))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SeebeckCoefficientForSubstancesAAndBUnit") (range none)) (redefinition (reference "mRef") (range (start (line 674) (character 22)) (end (line 674) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 673) (character 8)) (end (line 673) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 673) (character 22)) (end (line 673) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ShortRangeOrderParameterValue"))) (kind "attribute def") (name "ShortRangeOrderParameterValue") (declared-name "ShortRangeOrderParameterValue") (range (start (line 202) (character 4)) (end (line 202) (character 727))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ShortRangeOrderParameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 202) (character 4)) (end (line 202) (character 727))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ShortRangeOrderParameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::StructureFactorValue"))) (kind "attribute def") (name "StructureFactorValue") (declared-name "StructureFactorValue") (range (start (line 253) (character 4)) (end (line 253) (character 789))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::StructureFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 253) (character 4)) (end (line 253) (character 789))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::StructureFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThermodynamicGrüneisenParameterValue"))) (kind "attribute def") (name "ThermodynamicGrüneisenParameterValue") (declared-name "ThermodynamicGrüneisenParameterValue") (range (start (line 479) (character 4)) (end (line 479) (character 756))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThermodynamicGrüneisenParameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 479) (character 4)) (end (line 479) (character 756))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThermodynamicGrüneisenParameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (kind "attribute def") (name "ThomsonCoefficientUnit") (declared-name "ThomsonCoefficientUnit") (range (start (line 724) (character 4)) (end (line 724) (character 761))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 727) (character 8)) (end (line 727) (character 105))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (range (start (line 728) (character 8)) (end (line 728) (character 112))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 725) (character 8)) (end (line 725) (character 102))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 726) (character 8)) (end (line 726) (character 100))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 730) (character 8)) (end (line 730) (character 149))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 730) (character 22)) (end (line 730) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 729) (character 8)) (end (line 729) (character 124))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue"))) (kind "attribute def") (name "ThomsonCoefficientValue") (declared-name "ThomsonCoefficientValue") (range (start (line 705) (character 4)) (end (line 705) (character 778))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 705) (character 4)) (end (line 705) (character 778))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 719) (character 8)) (end (line 719) (character 54))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThomsonCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 719) (character 22)) (end (line 719) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 718) (character 8)) (end (line 718) (character 32))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 718) (character 22)) (end (line 718) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 59984))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::acceptorDensity"))) (kind "attribute def") (name "acceptorDensity") (declared-name "acceptorDensity") (range (start (line 974) (character 4)) (end (line 974) (character 85))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "AcceptorDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::angularReciprocalLatticeVectorMagnitude"))) (kind "attribute def") (name "angularReciprocalLatticeVectorMagnitude") (declared-name "angularReciprocalLatticeVectorMagnitude") (range (start (line 87) (character 4)) (end (line 87) (character 133))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularReciprocalLatticeVectorMagnitudeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::angularRepetency"))) (kind "alias") (name "angularRepetency") (declared-name "angularRepetency") (range (start (line 382) (character 4)) (end (line 382) (character 49))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::angularWavenumber"))) (kind "attribute def") (name "angularWavenumber") (declared-name "angularWavenumber") (range (start (line 367) (character 4)) (end (line 367) (character 1293))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "RepetencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::angularWavenumber::_documentation"))) (kind "documentation") (name "") (range (start (line 367) (character 4)) (end (line 367) (character 1293))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::angularWavenumber"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::atomicScatteringFactor"))) (kind "attribute def") (name "atomicScatteringFactor") (declared-name "atomicScatteringFactor") (range (start (line 250) (character 4)) (end (line 250) (character 86))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "AtomicScatteringFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::braggAngle"))) (kind "attribute def") (name "braggAngle") (declared-name "braggAngle") (range (start (line 186) (character 4)) (end (line 186) (character 677))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::braggAngle::_documentation"))) (kind "documentation") (name "") (range (start (line 186) (character 4)) (end (line 186) (character 677))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::braggAngle"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::carrierLifetime"))) (kind "attribute def") (name "carrierLifetime") (declared-name "carrierLifetime") (range (start (line 1031) (character 4)) (end (line 1031) (character 668))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::carrierLifetime::_documentation"))) (kind "documentation") (name "") (range (start (line 1031) (character 4)) (end (line 1031) (character 668))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::carrierLifetime"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianAngularReciprocalLattice3dVector"))) (kind "attribute def") (name "cartesianAngularReciprocalLattice3dVector") (declared-name "cartesianAngularReciprocalLattice3dVector") (range (start (line 111) (character 4)) (end (line 111) (character 119))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularReciprocalLattice3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianBurgers3dVector"))) (kind "attribute def") (name "cartesianBurgers3dVector") (declared-name "cartesianBurgers3dVector") (range (start (line 287) (character 4)) (end (line 287) (character 85))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianBurgers3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianDisplacement3dVector"))) (kind "attribute def") (name "cartesianDisplacement3dVector") (declared-name "cartesianDisplacement3dVector") (range (start (line 347) (character 4)) (end (line 347) (character 95))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianDisplacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianEquilibriumPosition3dVector"))) (kind "attribute def") (name "cartesianEquilibriumPosition3dVector") (declared-name "cartesianEquilibriumPosition3dVector") (range (start (line 327) (character 4)) (end (line 327) (character 109))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianEquilibriumPosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalLattice3dVector"))) (kind "attribute def") (name "cartesianFundamentalLattice3dVector") (declared-name "cartesianFundamentalLattice3dVector") (range (start (line 67) (character 4)) (end (line 67) (character 107))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianFundamentalLattice3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalReciprocalLattice3dVector"))) (kind "attribute def") (name "cartesianFundamentalReciprocalLattice3dVector") (declared-name "cartesianFundamentalReciprocalLattice3dVector") (range (start (line 161) (character 4)) (end (line 161) (character 127))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianFundamentalReciprocalLattice3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianLattice3dVector"))) (kind "attribute def") (name "cartesianLattice3dVector") (declared-name "cartesianLattice3dVector") (range (start (line 47) (character 4)) (end (line 47) (character 85))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianLattice3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianParticlePosition3dVector"))) (kind "attribute def") (name "cartesianParticlePosition3dVector") (declared-name "cartesianParticlePosition3dVector") (range (start (line 307) (character 4)) (end (line 307) (character 103))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianParticlePosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::coherenceLength"))) (kind "attribute def") (name "coherenceLength") (declared-name "coherenceLength") (range (start (line 1207) (character 4)) (end (line 1207) (character 565))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::coherenceLength::_documentation"))) (kind "documentation") (name "") (range (start (line 1207) (character 4)) (end (line 1207) (character 565))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::coherenceLength"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::curieTemperature"))) (kind "attribute def") (name "curieTemperature") (declared-name "curieTemperature") (range (start (line 1079) (character 4)) (end (line 1079) (character 588))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::curieTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 1079) (character 4)) (end (line 1079) (character 588))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::curieTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularFrequency"))) (kind "attribute def") (name "debyeAngularFrequency") (declared-name "debyeAngularFrequency") (range (start (line 421) (character 4)) (end (line 421) (character 595))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularFrequencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularFrequency::_documentation"))) (kind "documentation") (name "") (range (start (line 421) (character 4)) (end (line 421) (character 595))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularFrequency"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularRepetency"))) (kind "alias") (name "debyeAngularRepetency") (declared-name "debyeAngularRepetency") (range (start (line 418) (character 4)) (end (line 418) (character 59))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularWavenumber"))) (kind "attribute def") (name "debyeAngularWavenumber") (declared-name "debyeAngularWavenumber") (range (start (line 403) (character 4)) (end (line 403) (character 686))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "RepetencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularWavenumber::_documentation"))) (kind "documentation") (name "") (range (start (line 403) (character 4)) (end (line 403) (character 686))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularWavenumber"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeTemperature"))) (kind "attribute def") (name "debyeTemperature") (declared-name "debyeTemperature") (range (start (line 437) (character 4)) (end (line 437) (character 819))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 437) (character 4)) (end (line 437) (character 819))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::debyeTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::debyeWallerFactor"))) (kind "attribute def") (name "debyeWallerFactor") (declared-name "debyeWallerFactor") (range (start (line 364) (character 4)) (end (line 364) (character 76))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DebyeWallerFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::densityOfVibrationalStates"))) (kind "attribute def") (name "densityOfVibrationalStates") (declared-name "densityOfVibrationalStates") (range (start (line 470) (character 4)) (end (line 470) (character 107))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfVibrationalStatesValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::diffusionLengthForCondensedMatterPhysics"))) (kind "attribute def") (name "diffusionLengthForCondensedMatterPhysics") (declared-name "diffusionLengthForCondensedMatterPhysics") (range (start (line 1047) (character 4)) (end (line 1047) (character 709))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::diffusionLengthForCondensedMatterPhysics::_documentation"))) (kind "documentation") (name "") (range (start (line 1047) (character 4)) (end (line 1047) (character 709))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::diffusionLengthForCondensedMatterPhysics"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::donorDensity"))) (kind "attribute def") (name "donorDensity") (declared-name "donorDensity") (range (start (line 949) (character 4)) (end (line 949) (character 79))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DonorDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::effectiveMass"))) (kind "attribute def") (name "effectiveMass") (declared-name "effectiveMass") (range (start (line 982) (character 4)) (end (line 982) (character 815))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::effectiveMass::_documentation"))) (kind "documentation") (name "") (range (start (line 982) (character 4)) (end (line 982) (character 815))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::effectiveMass"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::electronAffinity"))) (kind "attribute def") (name "electronAffinity") (declared-name "electronAffinity") (range (start (line 766) (character 4)) (end (line 766) (character 627))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::electronAffinity::_documentation"))) (kind "documentation") (name "") (range (start (line 766) (character 4)) (end (line 766) (character 627))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::electronAffinity"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::electronDensity"))) (kind "attribute def") (name "electronDensity") (declared-name "electronDensity") (range (start (line 874) (character 4)) (end (line 874) (character 85))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectronDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::energyDensityOfStates"))) (kind "attribute def") (name "energyDensityOfStates") (declared-name "energyDensityOfStates") (range (start (line 562) (character 4)) (end (line 562) (character 97))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDensityOfStatesValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::exchangeIntegral"))) (kind "attribute def") (name "exchangeIntegral") (declared-name "exchangeIntegral") (range (start (line 1063) (character 4)) (end (line 1063) (character 605))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::exchangeIntegral::_documentation"))) (kind "documentation") (name "") (range (start (line 1063) (character 4)) (end (line 1063) (character 605))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::exchangeIntegral"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularRepetency"))) (kind "alias") (name "fermiAngularRepetency") (declared-name "fermiAngularRepetency") (range (start (line 400) (character 4)) (end (line 400) (character 59))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularWavenumber"))) (kind "attribute def") (name "fermiAngularWavenumber") (declared-name "fermiAngularWavenumber") (range (start (line 385) (character 4)) (end (line 385) (character 622))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "RepetencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularWavenumber::_documentation"))) (kind "documentation") (name "") (range (start (line 385) (character 4)) (end (line 385) (character 622))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularWavenumber"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiEnergy"))) (kind "attribute def") (name "fermiEnergy") (declared-name "fermiEnergy") (range (start (line 809) (character 4)) (end (line 809) (character 898))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 809) (character 4)) (end (line 809) (character 898))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::fermiEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiTemperature"))) (kind "attribute def") (name "fermiTemperature") (declared-name "fermiTemperature") (range (start (line 841) (character 4)) (end (line 841) (character 754))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fermiTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 841) (character 4)) (end (line 841) (character 754))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::fermiTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::fundamentalReciprocalLatticeVectorMagnitude"))) (kind "attribute def") (name "fundamentalReciprocalLatticeVectorMagnitude") (declared-name "fundamentalReciprocalLatticeVectorMagnitude") (range (start (line 137) (character 4)) (end (line 137) (character 141))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "FundamentalReciprocalLatticeVectorMagnitudeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::gapEnergy"))) (kind "attribute def") (name "gapEnergy") (declared-name "gapEnergy") (range (start (line 825) (character 4)) (end (line 825) (character 593))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::gapEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 825) (character 4)) (end (line 825) (character 593))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::gapEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::grüneisenParameter"))) (kind "attribute def") (name "grüneisenParameter") (declared-name "grüneisenParameter") (range (start (line 510) (character 4)) (end (line 510) (character 84))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "GrüneisenParameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::hallCoefficient"))) (kind "attribute def") (name "hallCoefficient") (declared-name "hallCoefficient") (range (start (line 634) (character 4)) (end (line 634) (character 85))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "HallCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::holeDensity"))) (kind "attribute def") (name "holeDensity") (declared-name "holeDensity") (range (start (line 899) (character 4)) (end (line 899) (character 77))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "HoleDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::intrinsicCarrierDensity"))) (kind "attribute def") (name "intrinsicCarrierDensity") (declared-name "intrinsicCarrierDensity") (range (start (line 924) (character 4)) (end (line 924) (character 101))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "IntrinsicCarrierDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ionizationEnergy"))) (kind "attribute def") (name "ionizationEnergy") (declared-name "ionizationEnergy") (range (start (line 750) (character 4)) (end (line 750) (character 617))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::ionizationEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 750) (character 4)) (end (line 750) (character 617))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::ionizationEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::latticePlaneSpacing"))) (kind "attribute def") (name "latticePlaneSpacing") (declared-name "latticePlaneSpacing") (range (start (line 170) (character 4)) (end (line 170) (character 578))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::latticePlaneSpacing::_documentation"))) (kind "documentation") (name "") (range (start (line 170) (character 4)) (end (line 170) (character 578))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::latticePlaneSpacing"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::londonPenetrationDepth"))) (kind "attribute def") (name "londonPenetrationDepth") (declared-name "londonPenetrationDepth") (range (start (line 1191) (character 4)) (end (line 1191) (character 701))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::londonPenetrationDepth::_documentation"))) (kind "documentation") (name "") (range (start (line 1191) (character 4)) (end (line 1191) (character 701))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::londonPenetrationDepth"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::longRangeOrderParameter"))) (kind "attribute def") (name "longRangeOrderParameter") (declared-name "longRangeOrderParameter") (range (start (line 233) (character 4)) (end (line 233) (character 88))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LongRangeOrderParameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::lorenzCoefficient"))) (kind "attribute def") (name "lorenzCoefficient") (declared-name "lorenzCoefficient") (range (start (line 605) (character 4)) (end (line 605) (character 89))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LorenzCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::lowerCriticalMagneticFluxDensity"))) (kind "attribute def") (name "lowerCriticalMagneticFluxDensity") (declared-name "lowerCriticalMagneticFluxDensity") (range (start (line 1143) (character 4)) (end (line 1143) (character 666))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::lowerCriticalMagneticFluxDensity::_documentation"))) (kind "documentation") (name "") (range (start (line 1143) (character 4)) (end (line 1143) (character 666))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::lowerCriticalMagneticFluxDensity"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfElectrons"))) (kind "attribute def") (name "meanFreePathOfElectrons") (declared-name "meanFreePathOfElectrons") (range (start (line 529) (character 4)) (end (line 529) (character 536))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfElectrons::_documentation"))) (kind "documentation") (name "") (range (start (line 529) (character 4)) (end (line 529) (character 536))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfElectrons"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfPhonons"))) (kind "attribute def") (name "meanFreePathOfPhonons") (declared-name "meanFreePathOfPhonons") (range (start (line 513) (character 4)) (end (line 513) (character 528))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfPhonons::_documentation"))) (kind "documentation") (name "") (range (start (line 513) (character 4)) (end (line 513) (character 528))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfPhonons"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::mobilityRatio"))) (kind "attribute def") (name "mobilityRatio") (declared-name "mobilityRatio") (range (start (line 1012) (character 4)) (end (line 1012) (character 68))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "MobilityRatioValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::néelTemperature"))) (kind "attribute def") (name "néelTemperature") (declared-name "néelTemperature") (range (start (line 1095) (character 4)) (end (line 1095) (character 533))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::néelTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 1095) (character 4)) (end (line 1095) (character 533))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::néelTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::peltierCoefficientForSubstancesAAndB"))) (kind "attribute def") (name "peltierCoefficientForSubstancesAAndB") (declared-name "peltierCoefficientForSubstancesAAndB") (range (start (line 689) (character 4)) (end (line 689) (character 832))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricPotentialDifferenceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::peltierCoefficientForSubstancesAAndB::_documentation"))) (kind "documentation") (name "") (range (start (line 689) (character 4)) (end (line 689) (character 832))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::peltierCoefficientForSubstancesAAndB"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::relaxationTime"))) (kind "attribute def") (name "relaxationTime") (declared-name "relaxationTime") (range (start (line 1015) (character 4)) (end (line 1015) (character 702))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::relaxationTime::_documentation"))) (kind "documentation") (name "") (range (start (line 1015) (character 4)) (end (line 1015) (character 702))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::relaxationTime"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::residualResistivity"))) (kind "attribute def") (name "residualResistivity") (declared-name "residualResistivity") (range (start (line 572) (character 4)) (end (line 572) (character 579))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::residualResistivity::_documentation"))) (kind "documentation") (name "") (range (start (line 572) (character 4)) (end (line 572) (character 579))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::residualResistivity"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::richardsonConstant"))) (kind "attribute def") (name "richardsonConstant") (declared-name "richardsonConstant") (range (start (line 799) (character 4)) (end (line 799) (character 91))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "RichardsonConstantValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::seebeckCoefficientForSubstancesAAndB"))) (kind "attribute def") (name "seebeckCoefficientForSubstancesAAndB") (declared-name "seebeckCoefficientForSubstancesAAndB") (range (start (line 677) (character 4)) (end (line 677) (character 127))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "SeebeckCoefficientForSubstancesAAndBValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::shortRangeOrderParameter"))) (kind "attribute def") (name "shortRangeOrderParameter") (declared-name "shortRangeOrderParameter") (range (start (line 216) (character 4)) (end (line 216) (character 90))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ShortRangeOrderParameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::structureFactor"))) (kind "attribute def") (name "structureFactor") (declared-name "structureFactor") (range (start (line 267) (character 4)) (end (line 267) (character 72))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "StructureFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::superconductionTransitionTemperature"))) (kind "attribute def") (name "superconductionTransitionTemperature") (declared-name "superconductionTransitionTemperature") (range (start (line 1111) (character 4)) (end (line 1111) (character 590))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::superconductionTransitionTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 1111) (character 4)) (end (line 1111) (character 590))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::superconductionTransitionTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::superconductorEnergyGap"))) (kind "attribute def") (name "superconductorEnergyGap") (declared-name "superconductorEnergyGap") (range (start (line 1175) (character 4)) (end (line 1175) (character 538))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::superconductorEnergyGap::_documentation"))) (kind "documentation") (name "") (range (start (line 1175) (character 4)) (end (line 1175) (character 538))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::superconductorEnergyGap"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicCriticalMagneticFluxDensity"))) (kind "attribute def") (name "thermodynamicCriticalMagneticFluxDensity") (declared-name "thermodynamicCriticalMagneticFluxDensity") (range (start (line 1127) (character 4)) (end (line 1127) (character 1062))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicCriticalMagneticFluxDensity::_documentation"))) (kind "documentation") (name "") (range (start (line 1127) (character 4)) (end (line 1127) (character 1062))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicCriticalMagneticFluxDensity"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicGrüneisenParameter"))) (kind "attribute def") (name "thermodynamicGrüneisenParameter") (declared-name "thermodynamicGrüneisenParameter") (range (start (line 493) (character 4)) (end (line 493) (character 110))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicGrüneisenParameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::thermoelectricVoltageBetweenSubstancesAAndB"))) (kind "attribute def") (name "thermoelectricVoltageBetweenSubstancesAAndB") (declared-name "thermoelectricVoltageBetweenSubstancesAAndB") (range (start (line 644) (character 4)) (end (line 644) (character 675))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricPotentialDifferenceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::thermoelectricVoltageBetweenSubstancesAAndB::_documentation"))) (kind "documentation") (name "") (range (start (line 644) (character 4)) (end (line 644) (character 675))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::thermoelectricVoltageBetweenSubstancesAAndB"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::thomsonCoefficient"))) (kind "attribute def") (name "thomsonCoefficient") (declared-name "thomsonCoefficient") (range (start (line 722) (character 4)) (end (line 722) (character 91))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThomsonCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::upperCriticalMagneticFluxDensity"))) (kind "attribute def") (name "upperCriticalMagneticFluxDensity") (declared-name "upperCriticalMagneticFluxDensity") (range (start (line 1159) (character 4)) (end (line 1159) (character 650))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::upperCriticalMagneticFluxDensity::_documentation"))) (kind "documentation") (name "") (range (start (line 1159) (character 4)) (end (line 1159) (character 650))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::upperCriticalMagneticFluxDensity"))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::workFunction"))) (kind "attribute def") (name "workFunction") (declared-name "workFunction") (range (start (line 734) (character 4)) (end (line 734) (character 993))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCondensedMatter::workFunction::_documentation"))) (kind "documentation") (name "") (range (start (line 734) (character 4)) (end (line 734) (character 993))) (parent (node (document "d0") (qualified-name "ISQCondensedMatter::workFunction"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 978) (character 22)) (end (line 978) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptorDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 971) (character 22)) (end (line 971) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 970) (character 22)) (end (line 970) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularFrequencyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::AngularFrequencyValue") (range (start (line 24) (character 19)) (end (line 24) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularMeasureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::AngularMeasureValue") (range (start (line 25) (character 19)) (end (line 25) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 91) (character 22)) (end (line 91) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularReciprocalLatticeVectorMagnitudeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 84) (character 22)) (end (line 84) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 83) (character 22)) (end (line 83) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::AtomicScatteringFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 114) (character 22)) (end (line 114) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 115) (character 22)) (end (line 115) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularReciprocalLatticeVectorMagnitudeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 116) (character 22)) (end (line 116) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 107) (character 22)) (end (line 107) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularReciprocalLattice3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 108) (character 22)) (end (line 108) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 283) (character 22)) (end (line 283) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 284) (character 22)) (end (line 284) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 343) (character 22)) (end (line 343) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 344) (character 22)) (end (line 344) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 323) (character 22)) (end (line 323) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 324) (character 22)) (end (line 324) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 63) (character 22)) (end (line 63) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 64) (character 22)) (end (line 64) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 164) (character 22)) (end (line 164) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 165) (character 22)) (end (line 165) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "FundamentalReciprocalLatticeVectorMagnitudeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 166) (character 22)) (end (line 166) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 157) (character 22)) (end (line 157) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 158) (character 22)) (end (line 158) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 43) (character 22)) (end (line 43) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 44) (character 22)) (end (line 44) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 303) (character 22)) (end (line 303) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 304) (character 22)) (end (line 304) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::CartesianSpatial3dCoordinateFrame") (range (start (line 23) (character 19)) (end (line 23) (character 66))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DebyeWallerFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 475) (character 22)) (end (line 475) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfVibrationalStatesUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 467) (character 22)) (end (line 467) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 466) (character 22)) (end (line 466) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 953) (character 22)) (end (line 953) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DonorDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 946) (character 22)) (end (line 946) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 945) (character 22)) (end (line 945) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectricPotentialDifferenceValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQElectromagnetism::ElectricPotentialDifferenceValue") (range (start (line 20) (character 19)) (end (line 20) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 878) (character 22)) (end (line 878) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectronDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 871) (character 22)) (end (line 871) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 870) (character 22)) (end (line 870) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 568) (character 22)) (end (line 568) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDensityOfStatesUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 559) (character 22)) (end (line 559) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 558) (character 22)) (end (line 558) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQThermodynamics::EnergyValue") (range (start (line 27) (character 19)) (end (line 27) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 141) (character 22)) (end (line 141) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "FundamentalReciprocalLatticeVectorMagnitudeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 134) (character 22)) (end (line 134) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 133) (character 22)) (end (line 133) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::GrüneisenParameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 640) (character 22)) (end (line 640) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "HallCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 631) (character 22)) (end (line 631) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 630) (character 22)) (end (line 630) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 903) (character 22)) (end (line 903) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "HoleDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 896) (character 22)) (end (line 896) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 895) (character 22)) (end (line 895) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 928) (character 22)) (end (line 928) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IntrinsicCarrierDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 921) (character 22)) (end (line 921) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 920) (character 22)) (end (line 920) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LongRangeOrderParameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 613) (character 22)) (end (line 613) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LorenzCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 602) (character 22)) (end (line 602) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 601) (character 22)) (end (line 601) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQElectromagnetism::MagneticFluxDensityValue") (range (start (line 21) (character 19)) (end (line 21) (character 64))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::MobilityRatioValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::RepetencyValue") (range (start (line 26) (character 19)) (end (line 26) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ResistivityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQElectromagnetism::ResistivityValue") (range (start (line 22) (character 19)) (end (line 22) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 805) (character 22)) (end (line 805) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RichardsonConstantUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 796) (character 22)) (end (line 796) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 795) (character 22)) (end (line 795) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 685) (character 22)) (end (line 685) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SeebeckCoefficientForSubstancesAAndBUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 674) (character 22)) (end (line 674) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 673) (character 22)) (end (line 673) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ShortRangeOrderParameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::StructureFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThermodynamicGrüneisenParameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 730) (character 22)) (end (line 730) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThomsonCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 719) (character 22)) (end (line 719) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 718) (character 22)) (end (line 718) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::acceptorDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptorDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::angularReciprocalLatticeVectorMagnitude"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularReciprocalLatticeVectorMagnitudeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::angularWavenumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::atomicScatteringFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "AtomicScatteringFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AtomicScatteringFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::braggAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::carrierLifetime"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianAngularReciprocalLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularReciprocalLattice3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianBurgers3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianBurgers3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianDisplacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianEquilibriumPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianEquilibriumPosition3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianFundamentalLattice3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalReciprocalLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianFundamentalReciprocalLattice3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianLattice3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianLattice3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianParticlePosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianParticlePosition3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::coherenceLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::curieTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularFrequencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularFrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularWavenumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeWallerFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "DebyeWallerFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DebyeWallerFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::densityOfVibrationalStates"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfVibrationalStatesValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::diffusionLengthForCondensedMatterPhysics"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::donorDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "DonorDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::effectiveMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::electronAffinity"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::electronDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectronDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::energyDensityOfStates"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDensityOfStatesValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::exchangeIntegral"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularWavenumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::fundamentalReciprocalLatticeVectorMagnitude"))) (kind featureTyping) (ordinal 0)) (authored-target "FundamentalReciprocalLatticeVectorMagnitudeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::gapEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::grüneisenParameter"))) (kind featureTyping) (ordinal 0)) (authored-target "GrüneisenParameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::GrüneisenParameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::hallCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "HallCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::holeDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "HoleDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::intrinsicCarrierDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "IntrinsicCarrierDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::ionizationEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::latticePlaneSpacing"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::londonPenetrationDepth"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::longRangeOrderParameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LongRangeOrderParameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LongRangeOrderParameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::lorenzCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "LorenzCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::lowerCriticalMagneticFluxDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfElectrons"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::meanFreePathOfPhonons"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::mobilityRatio"))) (kind featureTyping) (ordinal 0)) (authored-target "MobilityRatioValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MobilityRatioValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::néelTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::peltierCoefficientForSubstancesAAndB"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricPotentialDifferenceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectricPotentialDifferenceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::relaxationTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::residualResistivity"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ResistivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::richardsonConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "RichardsonConstantValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::seebeckCoefficientForSubstancesAAndB"))) (kind featureTyping) (ordinal 0)) (authored-target "SeebeckCoefficientForSubstancesAAndBValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::shortRangeOrderParameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ShortRangeOrderParameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ShortRangeOrderParameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::structureFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "StructureFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::StructureFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::superconductionTransitionTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::superconductorEnergyGap"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicCriticalMagneticFluxDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicGrüneisenParameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicGrüneisenParameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThermodynamicGrüneisenParameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermoelectricVoltageBetweenSubstancesAAndB"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricPotentialDifferenceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectricPotentialDifferenceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::thomsonCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "ThomsonCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::upperCriticalMagneticFluxDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCondensedMatter::workFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::acceptorDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AcceptorDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::acceptorDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::angularReciprocalLatticeVectorMagnitude"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularReciprocalLatticeVectorMagnitudeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::angularReciprocalLatticeVectorMagnitude"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::angularWavenumber"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::angularWavenumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::atomicScatteringFactor"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AtomicScatteringFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::atomicScatteringFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::braggAngle"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularMeasureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::braggAngle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianAngularReciprocalLattice3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianAngularReciprocalLattice3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianAngularReciprocalLattice3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianBurgers3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianBurgers3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianBurgers3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianDisplacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianEquilibriumPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianEquilibriumPosition3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianEquilibriumPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalLattice3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalLattice3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalLattice3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalReciprocalLattice3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianFundamentalReciprocalLattice3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianFundamentalReciprocalLattice3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianLattice3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianLattice3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianLattice3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianParticlePosition3dVector"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::CartesianParticlePosition3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::cartesianParticlePosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularFrequency"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::AngularFrequencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularFrequency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularWavenumber"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeAngularWavenumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeWallerFactor"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DebyeWallerFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::debyeWallerFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::densityOfVibrationalStates"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DensityOfVibrationalStatesValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::densityOfVibrationalStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::donorDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::DonorDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::donorDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::electronAffinity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::electronAffinity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::electronDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectronDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::electronDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::energyDensityOfStates"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyDensityOfStatesValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::energyDensityOfStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::exchangeIntegral"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::exchangeIntegral"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularWavenumber"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RepetencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiAngularWavenumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiEnergy"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::fermiEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::fundamentalReciprocalLatticeVectorMagnitude"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::FundamentalReciprocalLatticeVectorMagnitudeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::fundamentalReciprocalLatticeVectorMagnitude"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::gapEnergy"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::gapEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::grüneisenParameter"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::GrüneisenParameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::grüneisenParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::hallCoefficient"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HallCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::hallCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::holeDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::HoleDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::holeDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::intrinsicCarrierDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::IntrinsicCarrierDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::intrinsicCarrierDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::ionizationEnergy"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::ionizationEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::longRangeOrderParameter"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LongRangeOrderParameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::longRangeOrderParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::lorenzCoefficient"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::LorenzCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::lorenzCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::lowerCriticalMagneticFluxDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::lowerCriticalMagneticFluxDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::mobilityRatio"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MobilityRatioValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::mobilityRatio"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::peltierCoefficientForSubstancesAAndB"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectricPotentialDifferenceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::peltierCoefficientForSubstancesAAndB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::residualResistivity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ResistivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::residualResistivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::richardsonConstant"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::RichardsonConstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::richardsonConstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::seebeckCoefficientForSubstancesAAndB"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::SeebeckCoefficientForSubstancesAAndBValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::seebeckCoefficientForSubstancesAAndB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::shortRangeOrderParameter"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ShortRangeOrderParameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::shortRangeOrderParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::structureFactor"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::StructureFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::structureFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::superconductorEnergyGap"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::superconductorEnergyGap"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicCriticalMagneticFluxDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicCriticalMagneticFluxDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicGrüneisenParameter"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThermodynamicGrüneisenParameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermodynamicGrüneisenParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermoelectricVoltageBetweenSubstancesAAndB"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ElectricPotentialDifferenceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::thermoelectricVoltageBetweenSubstancesAAndB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::thomsonCoefficient"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::ThomsonCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::thomsonCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::upperCriticalMagneticFluxDensity"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::MagneticFluxDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::upperCriticalMagneticFluxDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCondensedMatter::workFunction"))) (target (node (document "d0") (qualified-name "ISQCondensedMatter::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCondensedMatter::workFunction"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
