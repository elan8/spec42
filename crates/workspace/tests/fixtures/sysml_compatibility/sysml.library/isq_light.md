# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQLight
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQLight {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-7:2019 "Light and radiation"
     * see also https://www.iso.org/standard/64977.html
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
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-7 item 7-1.1 speed of light in a medium */
    attribute def SpeedOfLightInAMediumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-1.1 speed of light in a medium
         * symbol(s): `c`
         * application domain: generic
         * name: SpeedOfLightInAMedium
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: phase speed of an electromagnetic wave at a given point in a medium
         * remarks: See also ISO 80000-3. The value of the speed of light in a medium can depend on the frequency, polarization, and direction. For the definition of the speed of electromagnetic waves in vacuum, `c_0`, see ISO 80000-1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpeedOfLightInAMediumUnit[1];
    }

    attribute speedOfLightInAMedium: SpeedOfLightInAMediumValue[*] nonunique :> scalarQuantities;

    attribute def SpeedOfLightInAMediumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-1.2 refractive index */
    attribute def RefractiveIndexValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-1.2 refractive index
         * symbol(s): `n`
         * application domain: generic
         * name: RefractiveIndex (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of light in vacuum (ISO 80000-1) and speed of light in a medium (item 7-1.1)
         * remarks: The value of the refractive index can depend on the frequency, polarization, and direction. The refractive index is expressed by n = c_0/c, where c_()_0 is the speed of light in vacuum and c is the speed of light in the medium. For a medium with absorption, the complex refractive index n is defined by n = n + ik where k is spectral absorption index (IEC 60050-845) and i is imaginary unit. The refractivity is expressed by n -1, where n is refractive index.
         */
    }
    attribute refractiveIndex: RefractiveIndexValue :> scalarQuantities;

    /* ISO-80000-7 item 7-2.1 radiant energy */
    attribute radiantEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 7-2.1 radiant energy
         * symbol(s): `Q_e`, `W`, `U`, `(Q)`
         * application domain: electromagnetism
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) emitted, transferred or received in form of electromagnetic waves
         * remarks: Radiant energy can be expressed by the time integral of radiant flux (item 7-4.1), `Φ_e`, over a given duration (ISO 80000-3), `Δt`: `Q_e = int_(Δ t) Φ_e dt`. Radiant energy is expressed either as a function of wavelength (ISO 80000-3), `λ`, as a function of frequency (ISO 80000-3), `ν`, or as a function of wavenumber, `σ`. (See also 0.1.) The corresponding photometric quantity is "luminous energy" (item 7-12). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
    }

    /* ISO-80000-7 item 7-2.2 spectral radiant energy */
    attribute def SpectralRadiantEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-2.2 spectral radiant energy
         * symbol(s): `Q_(e,λ)`, `W_λ`, `U_λ`, `(Q_λ)`
         * application domain: generic
         * name: SpectralRadiantEnergy
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): J/nm, kg*m*s^-2
         * tensor order: 0
         * definition: spectral density of radiant energy, expressed by `Q_(e,λ) = (dQ_e) / (dλ)`, where `Q_e` is radiant energy (item 7-2.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant energy is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Q_e = int_(λ_1)^(λ_2) Q_(e,λ) dλ`
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyUnit[1];
    }

    attribute spectralRadiantEnergy: SpectralRadiantEnergyValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.1 radiant energy density */
    attribute def RadiantEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.1 radiant energy density
         * symbol(s): `w`, `(ρ_e)`
         * application domain: generic
         * name: RadiantEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: volumetric density of radiant energy, expressed by `w = (dQ_e)/(dV)`, where `Q_e` is radiant energy (item 7-2.1) in an elementary three-dimensional domain and `V` is the volume (ISO 80000-3) of that domain
         * remarks: Radiant energy density within a Planckian radiator is given by `w = (4 σ)/(c_0) T^4` where `σ` is the Stefan-Boltzmann constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1) and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantEnergyDensityUnit[1];
    }

    attribute radiantEnergyDensity: RadiantEnergyDensityValue[*] nonunique :> scalarQuantities;

    attribute def RadiantEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.2 spectral radiant energy density in terms of wavelength */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.2 spectral radiant energy density in terms of wavelength
         * symbol(s): `w_λ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavelength
         * quantity dimension: L^-2*M^1*T^-2
         * measurement unit(s): J/(m^3*nm), kg*m^-2*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavelength, expressed by `w_λ = (dw)/(dλ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavelength `λ` (ISO 80000-3)
         * remarks: Spectral radiant energy density within a Planckian radiator is given by `w_λ = 8πhc_0*f(λ, T)`, where `h` is the Planck constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1), `T` is thermodynamic temperature (ISO 80000-5) and `f(λ,T) = (λ^-5)/(exp(c_2 λ^-1 T^-1) - 1)`. For the radiation constant `c_2` in `f(λ,T)`, see ISO 80000-1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyDensityInTermsOfWavelengthUnit[1];
    }

    attribute spectralRadiantEnergyDensityInTermsOfWavelength: SpectralRadiantEnergyDensityInTermsOfWavelengthValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.3 spectral radiant energy density in terms of wavenumber */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.3 spectral radiant energy density in terms of wavenumber
         * symbol(s): `w_ṽ`, `ρ_ṽ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavenumber
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavenumber, expressed by `w_ṽ = (dw)/(dṽ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavenumber `ṽ` (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyDensityInTermsOfWavenumberUnit[1];
    }

    attribute spectralRadiantEnergyDensityInTermsOfWavenumber: SpectralRadiantEnergyDensityInTermsOfWavenumberValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-4.1 radiant flux, radiant power */
    attribute def RadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.1 radiant flux, radiant power
         * symbol(s): `Φ_e`, `P_e`, `Φ`, `P`
         * application domain: generic
         * name: RadiantFlux
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: change in radiant energy with time, expressed by `Φ_e = (dQ_e)/(dt)`, where `Q_e` is the radiant energy (item 7-2.1) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous flux" (item 7-13). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantFluxUnit[1];
    }

    attribute radiantFlux: RadiantFluxValue[*] nonunique :> scalarQuantities;

    attribute def RadiantFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias RadiantPowerUnit for RadiantFluxUnit;
    alias RadiantPowerValue for RadiantFluxValue;
    alias radiantPower for radiantFlux;

    /* ISO-80000-7 item 7-4.2 spectral radiant flux, spectral radiant power */
    attribute def SpectralRadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.2 spectral radiant flux, spectral radiant power
         * symbol(s): `Φ_(e,λ)`, `P_(e,λ)`, `(Φ_λ)`, `(P_λ)`
         * application domain: generic
         * name: SpectralRadiantFlux
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/nm, kg*m*s^-3
         * tensor order: 0
         * definition: spectral density of radiant flux, expressed by `Φ_(e,λ) = (dQ_e)/(dλ)`, where `Φ_e` is radiant flux (item 7-4.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant flux is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Φ_e = int_(λ_1)^(λ_2) Φ_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantFluxUnit[1];
    }

    attribute spectralRadiantFlux: SpectralRadiantFluxValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias SpectralRadiantPowerUnit for SpectralRadiantFluxUnit;
    alias SpectralRadiantPowerValue for SpectralRadiantFluxValue;
    alias spectralRadiantPower for spectralRadiantFlux;

    /* ISO-80000-7 item 7-5.1 radiant intensity */
    attribute def RadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.1 radiant intensity
         * symbol(s): `I_e`, `(I)`
         * application domain: generic
         * name: RadiantIntensity
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W/sr, kg*m^2*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant flux with respect to solid angle in a specified direction, expressed by `I_e = (dΦ_e)/(dΩ)`, where `Φ_e` is the radiant flux (item 7-4.1) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the radiant intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,φ)`, is used to determine the radiant flux (item 7-4.1) within a certain solid angle (ISO 80000-3), `Ω`, of a source: `Φ_e = int int_Ω I_e(θ, φ) sin(θ) dφ dθ`. The corresponding photometric quantity is "luminous intensity" (item 7-14). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantIntensityUnit[1];
    }

    attribute radiantIntensity: RadiantIntensityValue[*] nonunique :> scalarQuantities;

    attribute def RadiantIntensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-5.2 spectral radiant intensity */
    attribute def SpectralRadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.2 spectral radiant intensity
         * symbol(s): `I_(e,λ)`, `(I_λ)`
         * application domain: generic
         * name: SpectralRadiantIntensity
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/(sr*nm), kg*m*s^-3*sr^-1
         * tensor order: 0
         * definition: spectral density of radiant intensity, expressed by `I_(e, λ) = (d I_e)/(dλ)`, where `I_e` is radiant intensity (item 7-5.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant intensity is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `I_e = int_(λ_1)^(λ_2) I_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantIntensityUnit[1];
    }

    attribute spectralRadiantIntensity: SpectralRadiantIntensityValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantIntensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-6.1 radiance */
    attribute def RadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.1 radiance
         * symbol(s): `L_e`, `(L)`
         * application domain: generic
         * name: Radiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/(sr*m^2), kg*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_e = (d I_e)/(dA) * 1/cos(α)`, where `I_e` is radiant intensity (item 7-5.1), `A` is area (ISO 80000-3), and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: See also 0.1. For Planckian radiation, `L_e = σ/π T^4` where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminance" (item 7-15). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadianceUnit[1];
    }

    attribute radiance: RadianceValue[*] nonunique :> scalarQuantities;

    attribute def RadianceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-6.2 spectral radiance */
    attribute def SpectralRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.2 spectral radiance
         * symbol(s): `L_(e,λ)`, `(L_λ)`
         * application domain: generic
         * name: SpectralRadiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(sr*m^2*nm), kg*m^-1*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiance with respect to wavelength, expressed by `L_(e, λ) = (d L_e)/(d λ)` where `L_e` is radiance (item 7-6.1) in terms of wavelength λ(ISO 80000-3)
         * remarks: For Planckian radiation, `L_(e, λ)(λ) = (c(λ))/(4 π) ω_λ(λ) = h c_0^2 * f(λ,T)`, where `c(λ)` is phase speed (ISO 80000-3) of electromagnetic radiation of a wavelength (ISO 80000-3) `λ` in a given medium, `ω_λ(λ)` is spectral radiant energy density in terms of wavelength, `c_0` is speed of light in vacuum (ISO 80000-1), `h` is the Planck constant (ISO 80000-1), and `f(λ,T) = λ^-5/(exp(c_2 λ^-1 T^-1) - 1)`, where the radiation constant `c_2 = (hc)/k`. The integral of (total) radiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `L_e = int_(λ_1)^(λ_2) L_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadianceUnit[1];
    }

    attribute spectralRadiance: SpectralRadianceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-7.1 irradiance */
    attribute def IrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.1 irradiance
         * symbol(s): `E_e`, `(E)`
         * application domain: generic
         * name: Irradiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of incident radiant flux with respect to area at a point on a real or imaginary surface, expressed by `E_e = (d Φ_e)/(d A)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) on which the radiant flux is incident
         * remarks: The corresponding photometric quantity is "illuminance" (item 7-16). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical irradiance" is defined by the mean value of irradiance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(e,0) = int_(4 π) L_e d Ω` where `Ω` is solid angle (ISO 80000-3) and `L_e` is radiance (item 7-6.1). (See CIE DIS 017/E:2016, term 17-21-054.) It can be expressed by the quotient of the radiant flux (item 7-4.1) of all the radiation incident on the outer surface of an infinitely small sphere centred at the specified point and the area (ISO 80000-3) of the diametrical cross-section of that sphere. Spherical irradiance is also called "fluence rate" or "radiant fluence rate". The corresponding photometric quantity to spherical irradiance is called "spherical illuminance".
         */
        attribute :>> num: Real;
        attribute :>> mRef: IrradianceUnit[1];
    }

    attribute irradiance: IrradianceValue[*] nonunique :> scalarQuantities;

    attribute def IrradianceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-7.2 spectral irradiance */
    attribute def SpectralIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.2 spectral irradiance
         * symbol(s): `E_(e,λ)`, `(E_λ)`
         * application domain: generic
         * name: SpectralIrradiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of irradiance with respect to wavelength, expressed by `E_(e,λ) = (d E_e)/(dλ)`, where `E_e` is irradiance (item 7-7.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) irradiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `E_e = int_(λ_1)^(λ_2) E_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralIrradianceUnit[1];
    }

    attribute spectralIrradiance: SpectralIrradianceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralIrradianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-8.1 radiant exitance , radiant emittance */
    attribute def RadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.1 radiant exitance , radiant emittance
         * symbol(s): `M_e`, `(M)`
         * application domain: generic
         * name: RadiantExitance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of exiting radiant flux with respect to area at a point on a real or imaginary surface, expressed by `M_e = (d Φ_e)/(dA)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) from which the radiant flux leaves
         * remarks: For Planckian radiation, `M_e = σT^4`, where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminous exitance" (item 7-17). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantExitanceUnit[1];
    }

    attribute radiantExitance: RadiantExitanceValue[*] nonunique :> scalarQuantities;

    attribute def RadiantExitanceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    alias RadiantEmittanceUnit for RadiantExitanceUnit;
    alias RadiantEmittanceValue for RadiantExitanceValue;
    alias radiantEmittance for radiantExitance;

    /* ISO-80000-7 item 7-8.2 spectral radiant exitance */
    attribute def SpectralRadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.2 spectral radiant exitance
         * symbol(s): `M_(e,λ)`, `(M_λ)`
         * application domain: generic
         * name: SpectralRadiantExitance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of radiant exitance with respect to wavelength, expressed by `M_(e,λ) = (d M_e)/(dλ)`, where `M_e` is radiant exitance (item 7-8.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exitance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `M_e = int_(λ_1)^(λ_2) M_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantExitanceUnit[1];
    }

    attribute spectralRadiantExitance: SpectralRadiantExitanceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-9.1 radiant exposure */
    attribute def RadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.1 radiant exposure
         * symbol(s): `H_e`, `(H)`
         * application domain: generic
         * name: RadiantExposure
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: density of incident radiant energy with respect to area at a point on a real or imaginary surface, expressed by `H_e = (d Q_e)/(dA)`, where `Q_e` is radiant energy (item 7-2.1) and `A` is the area on which the radiant energy is incident (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous exposure" (item 7-18). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantExposureUnit[1];
    }

    attribute radiantExposure: RadiantExposureValue[*] nonunique :> scalarQuantities;

    attribute def RadiantExposureUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-9.2 spectral radiant exposure */
    attribute def SpectralRadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.2 spectral radiant exposure
         * symbol(s): `H_(e,λ)`, `(H_λ)`
         * application domain: generic
         * name: SpectralRadiantExposure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/(m^2*nm), kg*m^-1*s^-2
         * tensor order: 0
         * definition: density of radiant exposure with respect to wavelength, expressed by `H_(e,λ) = (d H_e)/(dλ)`, where `H_e` is radiant exposure (item 7-9.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exposure is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `H_e = int_(λ_1)^(λ_2) H_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantExposureUnit[1];
    }

    attribute spectralRadiantExposure: SpectralRadiantExposureValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-10.1 luminous efficiency */
    attribute def LuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.1 luminous efficiency
         * symbol(s): `V`
         * application domain: specified photometric condition
         * name: LuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant flux (item 7-4.1) weighted by the spectral luminous efficiency (item 7-10.2) and the corresponding radiant flux for a specified photometric condition
         * remarks: Luminous efficiency for photopic vision is expressed by `V = (int_0^∞ Φ_(e,λ)(λ) V(λ) d λ)/(int_0^∞ Φ_(e,λ)(λ) d λ) = K/K_m`, where `Φ_(e,λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency, `λ` is wavelength, `K` is luminous efficacy of radiation (item 7-11.1), and `K_m` is maximum luminous efficacy (item 7-11.3). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V` for photopic vision; `V'` for scotopic vision; `V_(mes;m)` for mesopic vision; `V_10` for the CIE 10° photopic photometric observer; `V_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute luminousEfficiency: LuminousEfficiencyValue :> scalarQuantities;

    /* ISO-80000-7 item 7-10.2 spectral luminous efficiency */
    attribute def SpectralLuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.2 spectral luminous efficiency
         * symbol(s): `V(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant flux (item 7-4.1) at wavelength `λ_m` and that at wavelength `λ`, such that both produce equally intense luminous sensations for a specified photometric condition and `λ_m` is chosen so that the maximum value of this quotient is equal to 1
         * remarks: The spectral luminous efficiency of the human eye depends on a number of factors, particularly the state of visual adaptation and the size and position of the source in the visual field. The photometric condition should be specified (e.g. photopic, scotopic, mesopic). If it is not specified, photopic vision is assumed and the symbol `V(λ)` is used. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V(λ)` for photopic vision; `V'(λ)` for scotopic vision; `V_(mes;m)(λ)` for mesopic vision; `V_10(λ)` for the CIE 10° photopic photometric observer; `V_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute spectralLuminousEfficiency: SpectralLuminousEfficiencyValue :> scalarQuantities;

    /* ISO-80000-7 item 7-11.1 luminous efficacy of radiation */
    attribute def LuminousEfficacyOfRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.1 luminous efficacy of radiation
         * symbol(s): `K`
         * application domain: specified photometric condition
         * name: LuminousEfficacyOfRadiation
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of luminous flux (item 7-13) and the corresponding radiant flux (item 7-4.1) for a specified photometric condition
         * remarks: Luminous efficacy of radiation for photopic vision is expressed by `K = Φ_V/Φ_e`, where `Φ_v` is luminous flux (item 7-13) and `Φ_e` is radiant flux (item 7-4.1). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K` for photopic vision; `K'` for scotopic vision; `K_(mes;m)` for mesopic vision; `K_10` for the CIE 10° photopic photometric observer; `K_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEfficacyOfRadiationUnit[1];
    }

    attribute luminousEfficacyOfRadiation: LuminousEfficacyOfRadiationValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEfficacyOfRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.2 spectral luminous efficacy */
    attribute def SpectralLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.2 spectral luminous efficacy
         * symbol(s): `K(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: product of spectral luminous efficiency (item 7-10.2) and maximum luminous efficacy (item 7-11.3) for a specified photometric condition
         * remarks: Spectral luminous efficacy for photopic vision is expressed by `K(λ) = K_m V(λ)`, where `K_m` is maximum luminous efficacy (item 7-11.3), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K(λ)` for photopic vision>; `K'(λ)` for scotopic vision; `K_(mes;m)(λ)` for mesopic vision; `K_10(λ)` for the CIE 10° photopic photometric observer; `K_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralLuminousEfficacyUnit[1];
    }

    attribute spectralLuminousEfficacy: SpectralLuminousEfficacyValue[*] nonunique :> scalarQuantities;

    attribute def SpectralLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.3 maximum luminous efficacy */
    attribute def MaximumLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.3 maximum luminous efficacy
         * symbol(s): `K_m`
         * application domain: specified photometric condition
         * name: MaximumLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: maximum value of spectral luminous efficacy for a specified photometric condition
         * remarks: See also 0.4 and 0.5. The value of maximum luminous efficacy for photopic vision is calculated by `K_m = 683 / (V(λ_(cd))) ["cd"*"sr"*"W"^-1] = 683 ["lm"*"W"^-1]` where `V(λ)` is the spectral luminous efficiency for photopic vision and `λ_(cd)` is the wavelength in air corresponding to the frequency `540*10^12 ["Hz"]` specified in the definition of the SI unit candela. Symbols for different photometric conditions: `K_m` for photopic vision; `K'_m` for scotopic vision; `K_(m,mes;m)` for mesopic vision; `K_(m,10)` for the CIE 10° photopic photometric observer; `K_(m,M)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MaximumLuminousEfficacyUnit[1];
    }

    attribute maximumLuminousEfficacy: MaximumLuminousEfficacyValue[*] nonunique :> scalarQuantities;

    attribute def MaximumLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.4 luminous efficacy of a source */
    attribute def LuminousEfficacyOfASourceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.4 luminous efficacy of a source
         * symbol(s): `η_v`, `(η)`
         * application domain: generic
         * name: LuminousEfficacyOfASource
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of the luminous flux emitted and the power consumed by the source, expressed by `η_v = Φ_v/P`, where `Φ_v` is luminous flux (item 7-13) and `P` is the power (ISO 80000-4) consumed by the source
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEfficacyOfASourceUnit[1];
    }

    attribute luminousEfficacyOfASource: LuminousEfficacyOfASourceValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEfficacyOfASourceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-12 luminous energy, quantity of light */
    attribute def LuminousEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-12 luminous energy, quantity of light
         * symbol(s): `Q_v`, `(Q)`
         * application domain: generic
         * name: LuminousEnergy
         * quantity dimension: T^1*J^1
         * measurement unit(s): lm*s, cd*sr*s
         * tensor order: 0
         * definition: energy of electromagnetic waves weighted by the spectral luminous efficiency (item 7-10.2) multiplied by maximum luminous efficacy (item 7-11.3) of a specified photometric condition
         * remarks: Luminous energy for photopic vision is expressed by `Q_v = K_m int_0^∞ Q_(e,λ)(λ) V(λ) dλ`, where `Q_(e,λ)(λ)` is the spectral radiant energy (item 7-2.2) at wavelength `λ` (ISO 80000-3), `V(λ)` is spectral luminous efficiency (item 7-10.2), and `K_m` is maximum luminous efficacy (7-11.3). Luminous energy can be emitted, transferred or received. Luminous energy can be expressed by the time integral of the luminous flux (item 7-13), `Φ_v`, over a given duration (ISO 80000-3), `Δt`: `Q_v = int_(Δt) Φ_v dt` . The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEnergyUnit[1];
    }

    attribute luminousEnergy: LuminousEnergyValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEnergyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (durationPF, luminousIntensityPF); }
    }

    alias QuantityOfLightUnit for LuminousEnergyUnit;
    alias QuantityOfLightValue for LuminousEnergyValue;
    alias quantityOfLight for luminousEnergy;

    /* ISO-80000-7 item 7-13 luminous flux */
    attribute def LuminousFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-13 luminous flux
         * symbol(s): `Φ_v`, `(Φ)`
         * application domain: generic
         * name: LuminousFlux
         * quantity dimension: J^1
         * measurement unit(s): lm, cd*sr
         * tensor order: 0
         * definition: change in luminous energy with time, expressed by `Φ_v = (d Q_v)/(dt)`, where `Q_v` is the luminous energy (item 7-12) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: Luminous flux is a quantity derived from the radiant flux (item 7-4.1), `Φ_e`, by evaluating the radiation according to its action upon the CIE standard photometric observer. (See CIE S 017/E:2011, term 17-738.) Luminous flux can be derived from the spectral radiant flux distribution by `Φ_v = K_m int_0^oo Φ_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `Φ_(e,λ)(λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength (ISO 80000-3). The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousFluxUnit[1];
    }

    attribute luminousFlux: LuminousFluxValue[*] nonunique :> scalarQuantities;

    attribute def LuminousFluxUnit :> DerivedUnit {
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = luminousIntensityPF; }
    }

    /* ISO-80000-7 item 7-14 luminous intensity */
    /* See package ISQBase for the declarations of LuminousIntensityValue and LuminousIntensityUnit */

    /* ISO-80000-7 item 7-15 luminance */
    attribute def LuminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-15 luminance
         * symbol(s): `L_v`, `(L)`
         * application domain: generic
         * name: Luminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: density of luminous intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_v = (dI_v)/(dA) 1/cos(α)`, where `I_v` is luminous intensity (item 7-14), `A` is area (ISO 80000-3) and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: Luminance can be derived from the spectral radiance distribution by `L_v = K_m int_0^∞ L_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `L_(e,λ)(λ)` is the spectral radiance (item 7-6.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also 0.1. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminanceUnit[1];
    }

    attribute luminance: LuminanceValue[*] nonunique :> scalarQuantities;

    attribute def LuminanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-16 illuminance */
    attribute def IlluminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-16 illuminance
         * symbol(s): `E_v`, `(E)`
         * application domain: generic
         * name: Illuminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lx, cd*sr*m^-2
         * tensor order: 0
         * definition: density of incident luminous flux with respect to area at a point on a real or imaginary surface, expressed by `E_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) on which the luminous flux is incident
         * remarks: Illuminance can be derived from the spectral irradiance distribution by `E_v = K_m int_0^∞ E_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `E_(e,λ)(λ)` is the spectral irradiance (item 7-7.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical illuminance" is defined by the mean value of illuminance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(v,0) = int_(4π) L_v dΩ`, where `Ω` is solid angle (ISO 80000-3) and `L_v` is luminance (item 7-15). It can be expressed by the quotient of the luminous flux (item 7-13) of all the light incident on the outer surface of an infinitely small sphere centred at the given point, and the area (ISO 80000-3) of the diametrical cross-section of that sphere.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IlluminanceUnit[1];
    }

    attribute illuminance: IlluminanceValue[*] nonunique :> scalarQuantities;

    attribute def IlluminanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-17 luminous exitance */
    attribute def LuminousExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-17 luminous exitance
         * symbol(s): `M_v`, `(M)`
         * application domain: generic
         * name: LuminousExitance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lm/m^2, cd*sr*m^-2
         * tensor order: 0
         * definition: density of exiting luminous flux with respect to area at a point on a real or imaginary surface, expressed by `M_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) from which the luminous flux leaves
         * remarks: Luminous exitance can be derived from the spectral radiant exitance distribution by `M_v = K_m int_0^∞ M_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `M_(e_λ)(λ)` is the spectral radiant exitance (item 7-8.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousExitanceUnit[1];
    }

    attribute luminousExitance: LuminousExitanceValue[*] nonunique :> scalarQuantities;

    attribute def LuminousExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-18 luminous exposure, quantity of illumination, light exposure */
    attribute def LuminousExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-18 luminous exposure, quantity of illumination, light exposure
         * symbol(s): `H_v`, `(H)`
         * application domain: generic
         * name: LuminousExposure
         * quantity dimension: L^-2*T^1*J^1
         * measurement unit(s): lx*s, cd*sr*m^-2*s
         * tensor order: 0
         * definition: density of incident luminous energy with respect to area at a point on a real or imaginary surface, expressed by `H_v = (dQ_v)/(dA)`, where `Q_v` is luminous energy (item 7-12) and `A` is the area on which the luminous energy is incident (ISO 80000-3)
         * remarks: Luminous exposure can be derived from the spectral radiant exposure distribution by `H_v = K_m int_0^∞ H_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `H_(e_λ)(λ)` is the spectral radiant exposure (item 7-9.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousExposureUnit[1];
    }

    attribute luminousExposure: LuminousExposureValue[*] nonunique :> scalarQuantities;

    attribute def LuminousExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, luminousIntensityPF); }
    }

    alias QuantityOfIlluminationUnit for LuminousExposureUnit;
    alias QuantityOfIlluminationValue for LuminousExposureValue;
    alias quantityOfIllumination for luminousExposure;

    alias LightExposureUnit for LuminousExposureUnit;
    alias LightExposureValue for LuminousExposureValue;
    alias lightExposure for luminousExposure;

    /* ISO-80000-7 item 7-19.1 photon number, number of photons */
    attribute def PhotonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-19.1 photon number, number of photons
         * symbol(s): `N_p`
         * application domain: generic
         * name: PhotonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant energy and photon energy, expressed by `N_p = Q_e/(h ν)`, where `Q_e` is radiant energy (item 7-2.1), `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon number can also be expressed by the time integral of the photon flux (item 7-20), `Φ_p`, over a given duration, `Δt`, `N_p = int_(Δt) Φ_p dt`
         */
    }
    attribute photonNumber: PhotonNumberValue :> scalarQuantities;

    alias numberOfPhotons for photonNumber;

    /* ISO-80000-7 item 7-19.2 photon energy */
    attribute photonEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 7-19.2 photon energy
         * symbol(s): `Q_p`, `(Q)`
         * application domain: generic
         * name: PhotonEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: product of the Planck constant and frequency, expressed by `Q_p = h ν` where `h` is the Planck constant (ISO 80000-1) and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon energy can be emitted, transferred or received. For monochromatic radiation, photon energy may be expressed by photon number (item 7-19.1). The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding photometric quantity is "luminous energy" (item 7-12).
         */
    }

    /* ISO-80000-7 item 7-20 photon flux */
    attribute def PhotonFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-20 photon flux
         * symbol(s): `Φ_p`, `(Φ)`
         * application domain: generic
         * name: PhotonFlux
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: rate of photon number per time interval, expressed by `Φ_p = (d N_p)/(dt)`, where `N_p` is photon number (e.g. given by item 7-19.1), transmitted or received, and `t` is time (ISO 80000-3)
         * remarks: Photon flux `Φ_p` is related to radiant flux (item 7-4.1), `Φ_e`, of monochromatic radiation, by `Φ_p = Φ_e/(h ν)` where `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave. The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding photometric quantity is "luminous flux" (item 7-13).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonFluxUnit[1];
    }

    attribute photonFlux: PhotonFluxValue[*] nonunique :> scalarQuantities;

    attribute def PhotonFluxUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-7 item 7-21 photon intensity */
    attribute def PhotonIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-21 photon intensity
         * symbol(s): `I_p`, `(I)`
         * application domain: generic
         * name: PhotonIntensity
         * quantity dimension: T^-1
         * measurement unit(s): s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon flux with respect to solid angle in a specified direction, expressed by `I_p = (dΦ_p)/(dΩ)`, where `Φ_p` is the photon flux (item 7-20) emitted in the given direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The distribution of the photon intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)` , is used to determine the photon flux (item 7-20) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_p = int int_Ω I_v(θ,ϕ) sin(θ) dϕ dθ`. The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding photometric quantity is "luminous intensity" (item 7-14).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonIntensityUnit[1];
    }

    attribute photonIntensity: PhotonIntensityValue[*] nonunique :> scalarQuantities;

    attribute def PhotonIntensityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-7 item 7-22 photon radiance */
    attribute def PhotonRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-22 photon radiance
         * symbol(s): `L_p`, `(L)`
         * application domain: generic
         * name: PhotonRadiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_p = (dI_p)/(dA) 1/cos(α)`, where `I_p` is photon intensity (item 7-21), `A` is area (ISO 80000-3) and `α` the angle between the normal to the surface at the specified point and the specified direction
         * remarks: The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding photometric quantity is "luminance" (item 7-15).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonRadianceUnit[1];
    }

    attribute photonRadiance: PhotonRadianceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonRadianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-23 photon irradiance */
    attribute def PhotonIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-23 photon irradiance
         * symbol(s): `E_p`, `(E)`
         * application domain: generic
         * name: PhotonIrradiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of incident photon flux with respect to area at a point on a real or imaginary surface, expressed by `E_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) on which the photon flux is incident
         * remarks: The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding photometric quantity is "illuminance" (item 7-16).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonIrradianceUnit[1];
    }

    attribute photonIrradiance: PhotonIrradianceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonIrradianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-24 photon exitance */
    attribute def PhotonExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-24 photon exitance
         * symbol(s): `M_p`, `(M)`
         * application domain: generic
         * name: PhotonExitance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of exiting photon flux with respect to area at a point on a real or imaginary surface, expressed by `M_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) from which the photon flux leaves
         * remarks: The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding photometric quantity is "luminous exitance" (item 7-17).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonExitanceUnit[1];
    }

    attribute photonExitance: PhotonExitanceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-25 photon exposure */
    attribute def PhotonExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-25 photon exposure
         * symbol(s): `H_p`, `(H)`
         * application domain: generic
         * name: PhotonExposure
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: density of incident photon number with respect to area at a point on a real or imaginary surface, expressed by `H_p = (dN_p)/(dA)`, where `N_p` is photon number (item 7-19.1) and `A` is the area (ISO 80000-3) on which the photons are incident
         * remarks: The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding photometric quantity is "luminous exposure" (item 7-18).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonExposureUnit[1];
    }

    attribute photonExposure: PhotonExposureValue[*] nonunique :> scalarQuantities;

    attribute def PhotonExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-7 item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer
         * symbol(s): `X,Y,Z`
         * application domain: generic
         * name: TristimulusValuesForTheCie1931StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1931 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `[cd*m^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 // int_0^∞ S_λ(λ) overline y(λ) dλ`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TristimulusValuesForTheCie1931StandardColorimetricObserverUnit[1];
    }

    attribute tristimulusValuesForTheCie1931StandardColorimetricObserver: TristimulusValuesForTheCie1931StandardColorimetricObserverValue[*] nonunique :> scalarQuantities;

    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer
         * symbol(s): `X_10,Y_10,Z_10`
         * application domain: generic
         * name: TristimulusValuesForTheCie1964StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1964 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `["cd"*"m"^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 /( int_0^∞ S_λ(λ) overline y(λ) dλ)`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TristimulusValuesForTheCie1964StandardColorimetricObserverUnit[1];
    }

    attribute tristimulusValuesForTheCie1964StandardColorimetricObserver: TristimulusValuesForTheCie1964StandardColorimetricObserverValue[*] nonunique :> scalarQuantities;

    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer
         * symbol(s): `overline x(λ)`, `overline y(λ)`, `overline z(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x(λ)` , `overline y(λ)` , `overline z(λ)` in the CIE 1931 standard colorimetric system
         * remarks: Values of `overline x(λ)` , `overline y(λ)` and `overline z(λ)` are defined in the CIE 1931 standard colorimetric system (2° observer) — applicable to fields of observation of angular opening from 1° to 4°.
         */
    }
    attribute cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> scalarQuantities;

    /* ISO-80000-7 item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer
         * symbol(s): `overline x_10(λ)`, `overline y_10(λ)`, `overline z_10(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x_10(λ)` , `overline y_10(λ)` , `overline z_10(λ)` in the CIE 1964 standard colorimetric system
         * remarks: Values of `overline x_10(λ)` , `overline y_10(λ)` and `overline z_10(λ)` are defined in the CIE 1964 standard colorimetric system (10° observer) — applicable to fields of observation with angles greater than 4°.
         */
    }
    attribute cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> scalarQuantities;

    /* ISO-80000-7 item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system
         * symbol(s): `x,y,z`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1931 standard colorimetric observer (item 7-26.1) and their sum, expressed by `x = X / (X+Y+Z)` , `y = Y / (X+Y+Z)` , `z = Z / (X+Y+Z)`
         * remarks: Since `x + y + z = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute chromaticityCoordinatesInTheCie1931StandardColorimetricSystem: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> scalarQuantities;

    /* ISO-80000-7 item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system
         * symbol(s): `x_10,y_10,z_10`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1964 standard colorimetric observer (item 7-26.2) and their sum, expressed by `x_10 = X_10 / (X_10+Y_10+Z_10)`, `y_10 = Y_10 / (X_10+Y_10+Z_10)`, `z_10 = Z_10 / (X_10+Y_10+Z_10)`
         * remarks: Since `x_10 + y_10 + z_10 = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute chromaticityCoordinatesInTheCie1964StandardColorimetricSystem: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> scalarQuantities;

    /* ISO-80000-7 item 7-29.1 colour temperature */
    attribute colourTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 7-29.1 colour temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: ColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator whose radiation has the same chromaticity as that of a given stimulus
         * remarks: None.
         */
    }

    /* ISO-80000-7 item 7-29.2 correlated colour temperature */
    attribute correlatedColourTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 7-29.2 correlated colour temperature
         * symbol(s): `T_"cp"`
         * application domain: generic
         * name: CorrelatedColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator having the chromaticity nearest the chromaticity associated with the given spectral distribution on a modified 1976 CIE Uniform Chromaticity Scale (UCS) diagram where `u',2/3 v'` are the coordinates of the Planckian locus and the test stimulus
         * remarks: None.
         */
    }

    /* ISO-80000-7 item 7-30.1 emissivity */
    attribute def EmissivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.1 emissivity
         * symbol(s): `ε`, `ε_T`
         * application domain: generic
         * name: Emissivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator and the radiant exitance of a Planckian radiator at the same temperature, expressed by `ε = M/M_b`, where `M` is the radiant exitance (item 7-8.1) of a thermal radiator and `M_b` is the radiant exitance of a Planckian radiator at the same temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute emissivity: EmissivityValue :> scalarQuantities;

    /* ISO-80000-7 item 7-30.2 emissivity at a specified wavelength */
    attribute def EmissivityAtASpecifiedWavelengthValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.2 emissivity at a specified wavelength
         * symbol(s): `ε(λ)`
         * application domain: generic
         * name: EmissivityAtASpecifiedWavelength (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator at a specified wavelength and the radiant exitance of a Planckian radiator at the same temperature and at the same wavelength, expressed by `ε(λ) = M(λ) / M_b(λ)`, where `M(λ)` is the radiant exitance (item 7-8.1) of a thermal radiator at a specified wavelength and `M_b(λ)` is the radiant exitance of a Planckian radiator at the same temperature at a specified wavelength (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute emissivityAtASpecifiedWavelength: EmissivityAtASpecifiedWavelengthValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.1 absorptance */
    attribute def AbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.1 absorptance
         * symbol(s): `α`, `a`
         * application domain: generic
         * name: Absorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed radiant flux and incident radiant flux, expressed by `α = Φ_a/Φ_m`, where `Φ_a` is absorbed radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `ρ` is reflectance (item 7-31.3) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute absorptance: AbsorptanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.2 luminous absorptance */
    attribute def LuminousAbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.2 luminous absorptance
         * symbol(s): `α_v`
         * application domain: generic
         * name: LuminousAbsorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed luminous flux and incident luminous flux, expressed by `α_v = Φ_(v,a)/Φ_(v,m)`, where `Φ_(v,a)` is absorbed luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral absorptance, `α(λ)`, luminous absorptance can be calculated by `α_v = (int_0^∞ α(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.1.
         */
    }
    attribute luminousAbsorptance: LuminousAbsorptanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.3 reflectance */
    attribute def ReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.3 reflectance
         * symbol(s): `ρ`
         * application domain: generic
         * name: Reflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected radiant flux and incident radiant flux, expressed by `ρ = Φ_r/Φ_m`, where `Φ_r` is reflected radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute reflectance: ReflectanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.4 luminous reflectance */
    attribute def LuminousReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.4 luminous reflectance
         * symbol(s): `ρ_v`
         * application domain: generic
         * name: LuminousReflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected luminous flux and incident luminous flux, is expressed by `ρ_v = Φ_(v,r)/Φ_(v,m)`, where `Φ_(v,r)` is reflected luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral reflectance, `ρ(λ)`, luminous reflectance can be calculated by `ρ_v = (int_0^∞ ρ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.3.
         */
    }
    attribute luminousReflectance: LuminousReflectanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.5 transmittance */
    attribute def TransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.5 transmittance
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: Transmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted radiant flux and incident radiant flux, expressed by `τ = Φ_t/Φ_m`, where `Φ_t` is transmitted radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `ρ` is reflectance (item 7-31.3).
         */
    }
    attribute transmittance: TransmittanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.6 luminous transmittance */
    attribute def LuminousTransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.6 luminous transmittance
         * symbol(s): `τ_v`
         * application domain: generic
         * name: LuminousTransmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted luminous flux and incident luminous flux, expressed by `τ_v = Φ_(v,t)/Φ_(v,m)`, where `Φ_(v,t)` is transmitted luminous flux (item 7-13) and `Φ_(v,m)` is luminous flux of the incident radiation
         * remarks: From the spectral transmittance `τ(λ)`, luminous transmittance can be calculated by `τ_v = (int_0^∞ τ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is the spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is the spectral luminous efficiency (item 7-10.2). See also item 7-31.5.
         */
    }
    attribute luminousTransmittance: LuminousTransmittanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance */
    attribute def TransmittanceOpticalDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance
         * symbol(s): `D`, `A_10`, `D_τ`
         * application domain: generic
         * name: TransmittanceOpticalDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: logarithm to base 10 of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the optical density can be expressed by `A_10(λ) = -log(τ(λ))`, where `τ(λ)` is the transmittance (item 7-31.5) in terms of wavelength. In spectroscopy, the name "absorbance" `A_10` is generally used.
         */
    }
    attribute transmittanceOpticalDensity: TransmittanceOpticalDensityValue :> scalarQuantities;

    alias opticalDensity for transmittanceOpticalDensity;

    alias transmittanceDensity for transmittanceOpticalDensity;

    alias decadicAbsorbance for transmittanceOpticalDensity;

    /* ISO-80000-7 item 7-32.2 Napierian absorbance */
    attribute def NapierianAbsorbanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.2 Napierian absorbance
         * symbol(s): `A_n`, `B`
         * application domain: generic
         * name: NapierianAbsorbance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: natural (Napierian) logarithm of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the Napierian absorbance can be expressed by `A_n(λ) = B(λ) = -log(τ(λ))`. It can also be expressed as `A_n(λ) = l*α(λ)`, where `α` is linear absorption coefficient (item 7-35.2) and `l` is length (ISO 80000-3) traversed.
         */
    }
    attribute napierianAbsorbance: NapierianAbsorbanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-33.1 radiance factor */
    attribute def RadianceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.1 radiance factor
         * symbol(s): `β_e`, `(β)`
         * application domain: generic
         * name: RadianceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiance of a surface element in a specified direction and the radiance of the perfect reflecting diffuser or perfect transmitting diffuser identically irradiated and viewed, expressed by `β_e = L_(e,n)/L_(e,d)`, where `L_(e,n)` is the radiance (item 7-6.1) of a surface element in a given direction and `L_(e,d)` is the radiance of the perfect reflecting or transmitting diffuser identically irradiated and viewed
         * remarks: The definition holds for a surface element of a non-self-radiating medium, in a given direction and under specified conditions of irradiation. Radiance factor is equivalent to reflectance factor (item 7-34) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is `2π ["sr"]`. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called "perfect diffuser".
         */
    }
    attribute radianceFactor: RadianceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-33.2 luminance factor */
    attribute def LuminanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.2 luminance factor
         * symbol(s): `β_v`, `(β)`
         * application domain: generic
         * name: LuminanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the luminance of a surface element in a specified direction and the luminance of the perfect reflecting diffuser or perfect transmitting diffuser identically illuminated and viewed, expressed by `β_v = L_(v,n)/L_(v,d)`, where `L_(v,n)` is the luminance (item 7-15) of a surface element in a given direction and `L_(v,d)` is the luminance of the perfect reflecting or transmitting diffuser identically illuminated and viewed
         * remarks: The definition holds for a surface element of a non-luminous medium, in a given direction and under specified conditions of irradiation. This quantity is also defined spectrally and is called "spectral luminance factor". For the analogous radiant quantity "radiance factor", see item 7-33.1.
         */
    }
    attribute luminanceFactor: LuminanceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-34 reflectance factor */
    attribute def ReflectanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-34 reflectance factor
         * symbol(s): `R`
         * application domain: generic
         * name: ReflectanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the flux reflected in the directions delimited by a given cone with apex at a surface element and the flux reflected in the same directions by a perfect reflecting diffuser identically irradiated or illuminated, expressed by `R = Φ_n/Φ_d`, where `Φ_n` is the flux reflected in the directions delimited by a given cone and `Φ_d` is the flux reflected in the same directions by an identically irradiated diffuser of reflectance (item 7-31.3) equal to 1
         * remarks: The flux can be a radiant flux (item 7‐4.1) or a luminous flux (item 7‐13). The definition holds for a surface element, for the part of the reflected radiation contained in a given cone with apex at the surface element, and for incident radiation of given spectral composition, polarization and geometric distribution. Reflectance factor is equivalent to radiance factor (item 7-33.1) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is 2π sr. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called a perfect diffuser.
         */
    }
    attribute reflectanceFactor: ReflectanceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-35.1 linear attenuation coefficient, linear extinction coefficient */
    attribute def LinearAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.1 linear attenuation coefficient, linear extinction coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: radiometry
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux caused by absorption and scattering
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear attenuation coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing and scattering medium `μ(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAttenuationCoefficientUnit[1];
    }

    attribute linearAttenuationCoefficient: LinearAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias LinearExtinctionCoefficientUnit for LinearAttenuationCoefficientUnit;
    alias LinearExtinctionCoefficientValue for LinearAttenuationCoefficientValue;
    alias linearExtinctionCoefficient for linearAttenuationCoefficient;

    /* ISO-80000-7 item 7-35.2 linear absorption coefficient */
    attribute def LinearAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.2 linear absorption coefficient
         * symbol(s): `α_l`, `a_l`, `α`
         * application domain: radiometry
         * name: LinearAbsorptionCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux (item 7-4.1) caused by absorption
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear absorption coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing medium `α_l(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. It can also be expressed as a function of transmittance (item 7-31.5). `α_l = -ln(τ)/l = A_n/l`. The linear absorption coefficient is that part of the linear attenuation coefficient (item 7-35.1) that is due to absorption. Scattering might also contribute. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAbsorptionCoefficientUnit[1];
    }

    attribute linearAbsorptionCoefficient: LinearAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-7 item 7-36.1 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.1 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: radiometry
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient (item 7-35.1), `μ`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `μ_m(λ) = (μ(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAttenuationCoefficientUnit[1];
    }

    attribute massAttenuationCoefficient: MassAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-7 item 7-36.2 mass absorption coefficient */
    attribute def MassAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.2 mass absorption coefficient
         * symbol(s): `α_m`
         * application domain: radiometry
         * name: MassAbsorptionCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear absorption coefficient (item 7-35.2), `α`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `α_m(λ) = (α(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAbsorptionCoefficientUnit[1];
    }

    attribute massAbsorptionCoefficient: MassAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-7 item 7-37 molar absorption coefficient */
    attribute def MolarAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-37 molar absorption coefficient
         * symbol(s): `χ`
         * application domain: radiometry
         * name: MolarAbsorptionCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: product of linear absorption coefficient and molar volume, expressed by `χ = α V_m`, where `α` is linear absorption coefficient (item 7-35.2) and `V_m` is molar volume (ISO 80000-9)
         * remarks: The molar absorption coefficient can also be expressed by `χ = α c` where `c` is amount-of-substance concentration (ISO 80000-9). Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarAbsorptionCoefficientUnit[1];
    }

    attribute molarAbsorptionCoefficient: MolarAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MolarAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

}
~~~
# EXPECTED
~~~
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
semantic.unresolved_name 'DimensionOneValue'
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
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
semantic.unresolved_name 'DimensionOneValue'
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
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
RegularComment,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQLight'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (attribute_def 'SpeedOfLightInAMediumValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpeedOfLightInAMediumUnit' multiplicity))
    (attribute_usage 'speedOfLightInAMedium' : 'SpeedOfLightInAMediumValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpeedOfLightInAMediumUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RefractiveIndexValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'refractiveIndex' : 'RefractiveIndexValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'radiantEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'SpectralRadiantEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantEnergyUnit' multiplicity))
    (attribute_usage 'spectralRadiantEnergy' : 'SpectralRadiantEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantEnergyUnit' :> 'DerivedUnit'
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
    (attribute_def 'RadiantEnergyDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RadiantEnergyDensityUnit' multiplicity))
    (attribute_usage 'radiantEnergyDensity' : 'RadiantEnergyDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RadiantEnergyDensityUnit' :> 'DerivedUnit'
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
    (attribute_def 'SpectralRadiantEnergyDensityInTermsOfWavelengthValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit' multiplicity))
    (attribute_usage 'spectralRadiantEnergyDensityInTermsOfWavelength' : 'SpectralRadiantEnergyDensityInTermsOfWavelengthValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit' :> 'DerivedUnit'
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
    (attribute_def 'SpectralRadiantEnergyDensityInTermsOfWavenumberValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantEnergyDensityInTermsOfWavenumberUnit' multiplicity))
    (attribute_usage 'spectralRadiantEnergyDensityInTermsOfWavenumber' : 'SpectralRadiantEnergyDensityInTermsOfWavenumberValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantEnergyDensityInTermsOfWavenumberUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RadiantFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RadiantFluxUnit' multiplicity))
    (attribute_usage 'radiantFlux' : 'RadiantFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RadiantFluxUnit' :> 'DerivedUnit'
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
    (alias_member 'RadiantPowerUnit' for 'RadiantFluxUnit')
    (alias_member 'RadiantPowerValue' for 'RadiantFluxValue')
    (alias_member 'radiantPower' for 'radiantFlux')
    (comment)
    (attribute_def 'SpectralRadiantFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantFluxUnit' multiplicity))
    (attribute_usage 'spectralRadiantFlux' : 'SpectralRadiantFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantFluxUnit' :> 'DerivedUnit'
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
    (alias_member 'SpectralRadiantPowerUnit' for 'SpectralRadiantFluxUnit')
    (alias_member 'SpectralRadiantPowerValue' for 'SpectralRadiantFluxValue')
    (alias_member 'spectralRadiantPower' for 'spectralRadiantFlux')
    (comment)
    (attribute_def 'RadiantIntensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RadiantIntensityUnit' multiplicity))
    (attribute_usage 'radiantIntensity' : 'RadiantIntensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RadiantIntensityUnit' :> 'DerivedUnit'
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
    (attribute_def 'SpectralRadiantIntensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantIntensityUnit' multiplicity))
    (attribute_usage 'spectralRadiantIntensity' : 'SpectralRadiantIntensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantIntensityUnit' :> 'DerivedUnit'
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
    (attribute_def 'RadianceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RadianceUnit' multiplicity))
    (attribute_usage 'radiance' : 'RadianceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RadianceUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpectralRadianceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadianceUnit' multiplicity))
    (attribute_usage 'spectralRadiance' : 'SpectralRadianceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadianceUnit' :> 'DerivedUnit'
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
    (attribute_def 'IrradianceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IrradianceUnit' multiplicity))
    (attribute_usage 'irradiance' : 'IrradianceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IrradianceUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpectralIrradianceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralIrradianceUnit' multiplicity))
    (attribute_usage 'spectralIrradiance' : 'SpectralIrradianceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralIrradianceUnit' :> 'DerivedUnit'
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
    (attribute_def 'RadiantExitanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RadiantExitanceUnit' multiplicity))
    (attribute_usage 'radiantExitance' : 'RadiantExitanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RadiantExitanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'RadiantEmittanceUnit' for 'RadiantExitanceUnit')
    (alias_member 'RadiantEmittanceValue' for 'RadiantExitanceValue')
    (alias_member 'radiantEmittance' for 'radiantExitance')
    (comment)
    (attribute_def 'SpectralRadiantExitanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantExitanceUnit' multiplicity))
    (attribute_usage 'spectralRadiantExitance' : 'SpectralRadiantExitanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantExitanceUnit' :> 'DerivedUnit'
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
    (attribute_def 'RadiantExposureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RadiantExposureUnit' multiplicity))
    (attribute_usage 'radiantExposure' : 'RadiantExposureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RadiantExposureUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpectralRadiantExposureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralRadiantExposureUnit' multiplicity))
    (attribute_usage 'spectralRadiantExposure' : 'SpectralRadiantExposureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralRadiantExposureUnit' :> 'DerivedUnit'
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
    (attribute_def 'LuminousEfficiencyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'luminousEfficiency' : 'LuminousEfficiencyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'SpectralLuminousEfficiencyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'spectralLuminousEfficiency' : 'SpectralLuminousEfficiencyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LuminousEfficacyOfRadiationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousEfficacyOfRadiationUnit' multiplicity))
    (attribute_usage 'luminousEfficacyOfRadiation' : 'LuminousEfficacyOfRadiationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousEfficacyOfRadiationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpectralLuminousEfficacyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpectralLuminousEfficacyUnit' multiplicity))
    (attribute_usage 'spectralLuminousEfficacy' : 'SpectralLuminousEfficacyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpectralLuminousEfficacyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MaximumLuminousEfficacyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MaximumLuminousEfficacyUnit' multiplicity))
    (attribute_usage 'maximumLuminousEfficacy' : 'MaximumLuminousEfficacyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MaximumLuminousEfficacyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LuminousEfficacyOfASourceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousEfficacyOfASourceUnit' multiplicity))
    (attribute_usage 'luminousEfficacyOfASource' : 'LuminousEfficacyOfASourceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousEfficacyOfASourceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LuminousEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousEnergyUnit' multiplicity))
    (attribute_usage 'luminousEnergy' : 'LuminousEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousEnergyUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'QuantityOfLightUnit' for 'LuminousEnergyUnit')
    (alias_member 'QuantityOfLightValue' for 'LuminousEnergyValue')
    (alias_member 'quantityOfLight' for 'luminousEnergy')
    (comment)
    (attribute_def 'LuminousFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousFluxUnit' multiplicity))
    (attribute_usage 'luminousFlux' : 'LuminousFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousFluxUnit' :> 'DerivedUnit'
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (comment)
    (comment)
    (attribute_def 'LuminanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminanceUnit' multiplicity))
    (attribute_usage 'luminance' : 'LuminanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'IlluminanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IlluminanceUnit' multiplicity))
    (attribute_usage 'illuminance' : 'IlluminanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IlluminanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LuminousExitanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousExitanceUnit' multiplicity))
    (attribute_usage 'luminousExitance' : 'LuminousExitanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousExitanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LuminousExposureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousExposureUnit' multiplicity))
    (attribute_usage 'luminousExposure' : 'LuminousExposureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousExposureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'QuantityOfIlluminationUnit' for 'LuminousExposureUnit')
    (alias_member 'QuantityOfIlluminationValue' for 'LuminousExposureValue')
    (alias_member 'quantityOfIllumination' for 'luminousExposure')
    (alias_member 'LightExposureUnit' for 'LuminousExposureUnit')
    (alias_member 'LightExposureValue' for 'LuminousExposureValue')
    (alias_member 'lightExposure' for 'luminousExposure')
    (comment)
    (attribute_def 'PhotonNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'photonNumber' : 'PhotonNumberValue' :> 'scalarQuantities')
    (alias_member 'numberOfPhotons' for 'photonNumber')
    (comment)
    (attribute_usage 'photonEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'PhotonFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhotonFluxUnit' multiplicity))
    (attribute_usage 'photonFlux' : 'PhotonFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhotonFluxUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PhotonIntensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhotonIntensityUnit' multiplicity))
    (attribute_usage 'photonIntensity' : 'PhotonIntensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhotonIntensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PhotonRadianceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhotonRadianceUnit' multiplicity))
    (attribute_usage 'photonRadiance' : 'PhotonRadianceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhotonRadianceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PhotonIrradianceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhotonIrradianceUnit' multiplicity))
    (attribute_usage 'photonIrradiance' : 'PhotonIrradianceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhotonIrradianceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PhotonExitanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhotonExitanceUnit' multiplicity))
    (attribute_usage 'photonExitance' : 'PhotonExitanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhotonExitanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PhotonExposureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhotonExposureUnit' multiplicity))
    (attribute_usage 'photonExposure' : 'PhotonExposureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhotonExposureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'TristimulusValuesForTheCie1931StandardColorimetricObserverValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TristimulusValuesForTheCie1931StandardColorimetricObserverUnit' multiplicity))
    (attribute_usage 'tristimulusValuesForTheCie1931StandardColorimetricObserver' : 'TristimulusValuesForTheCie1931StandardColorimetricObserverValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TristimulusValuesForTheCie1931StandardColorimetricObserverUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'TristimulusValuesForTheCie1964StandardColorimetricObserverValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TristimulusValuesForTheCie1964StandardColorimetricObserverUnit' multiplicity))
    (attribute_usage 'tristimulusValuesForTheCie1964StandardColorimetricObserver' : 'TristimulusValuesForTheCie1964StandardColorimetricObserverValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TristimulusValuesForTheCie1964StandardColorimetricObserverUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver' : 'CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver' : 'CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'chromaticityCoordinatesInTheCie1931StandardColorimetricSystem' : 'ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'chromaticityCoordinatesInTheCie1964StandardColorimetricSystem' : 'ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'colourTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'correlatedColourTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'EmissivityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'emissivity' : 'EmissivityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'EmissivityAtASpecifiedWavelengthValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'emissivityAtASpecifiedWavelength' : 'EmissivityAtASpecifiedWavelengthValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AbsorptanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'absorptance' : 'AbsorptanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LuminousAbsorptanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'luminousAbsorptance' : 'LuminousAbsorptanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ReflectanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'reflectance' : 'ReflectanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LuminousReflectanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'luminousReflectance' : 'LuminousReflectanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'TransmittanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'transmittance' : 'TransmittanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LuminousTransmittanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'luminousTransmittance' : 'LuminousTransmittanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'TransmittanceOpticalDensityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'transmittanceOpticalDensity' : 'TransmittanceOpticalDensityValue' :> 'scalarQuantities')
    (alias_member 'opticalDensity' for 'transmittanceOpticalDensity')
    (alias_member 'transmittanceDensity' for 'transmittanceOpticalDensity')
    (alias_member 'decadicAbsorbance' for 'transmittanceOpticalDensity')
    (comment)
    (attribute_def 'NapierianAbsorbanceValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'napierianAbsorbance' : 'NapierianAbsorbanceValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RadianceFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'radianceFactor' : 'RadianceFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LuminanceFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'luminanceFactor' : 'LuminanceFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ReflectanceFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'reflectanceFactor' : 'ReflectanceFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LinearAttenuationCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearAttenuationCoefficientUnit' multiplicity))
    (attribute_usage 'linearAttenuationCoefficient' : 'LinearAttenuationCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearAttenuationCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'LinearExtinctionCoefficientUnit' for 'LinearAttenuationCoefficientUnit')
    (alias_member 'LinearExtinctionCoefficientValue' for 'LinearAttenuationCoefficientValue')
    (alias_member 'linearExtinctionCoefficient' for 'linearAttenuationCoefficient')
    (comment)
    (attribute_def 'LinearAbsorptionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearAbsorptionCoefficientUnit' multiplicity))
    (attribute_usage 'linearAbsorptionCoefficient' : 'LinearAbsorptionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearAbsorptionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassAttenuationCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassAttenuationCoefficientUnit' multiplicity))
    (attribute_usage 'massAttenuationCoefficient' : 'MassAttenuationCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassAttenuationCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassAbsorptionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassAbsorptionCoefficientUnit' multiplicity))
    (attribute_usage 'massAbsorptionCoefficient' : 'MassAbsorptionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassAbsorptionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MolarAbsorptionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MolarAbsorptionCoefficientUnit' multiplicity))
    (attribute_usage 'molarAbsorptionCoefficient' : 'MolarAbsorptionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MolarAbsorptionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))))
~~~
# FORMAT
~~~sysml
standard library package ISQLight {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-7:2019 "Light and radiation"
     * see also https://www.iso.org/standard/64977.html
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
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-7 item 7-1.1 speed of light in a medium */
    attribute def SpeedOfLightInAMediumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-1.1 speed of light in a medium
         * symbol(s): `c`
         * application domain: generic
         * name: SpeedOfLightInAMedium
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: phase speed of an electromagnetic wave at a given point in a medium
         * remarks: See also ISO 80000-3. The value of the speed of light in a medium can depend on the frequency, polarization, and direction. For the definition of the speed of electromagnetic waves in vacuum, `c_0`, see ISO 80000-1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpeedOfLightInAMediumUnit[1];
    }

    attribute speedOfLightInAMedium: SpeedOfLightInAMediumValue[*] nonunique :> scalarQuantities;

    attribute def SpeedOfLightInAMediumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-1.2 refractive index */
    attribute def RefractiveIndexValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-1.2 refractive index
         * symbol(s): `n`
         * application domain: generic
         * name: RefractiveIndex (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of light in vacuum (ISO 80000-1) and speed of light in a medium (item 7-1.1)
         * remarks: The value of the refractive index can depend on the frequency, polarization, and direction. The refractive index is expressed by n = c_0/c, where c_()_0 is the speed of light in vacuum and c is the speed of light in the medium. For a medium with absorption, the complex refractive index n is defined by n = n + ik where k is spectral absorption index (IEC 60050-845) and i is imaginary unit. The refractivity is expressed by n -1, where n is refractive index.
         */
    }
    attribute refractiveIndex: RefractiveIndexValue :> scalarQuantities;

    /* ISO-80000-7 item 7-2.1 radiant energy */
    attribute radiantEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 7-2.1 radiant energy
         * symbol(s): `Q_e`, `W`, `U`, `(Q)`
         * application domain: electromagnetism
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) emitted, transferred or received in form of electromagnetic waves
         * remarks: Radiant energy can be expressed by the time integral of radiant flux (item 7-4.1), `Φ_e`, over a given duration (ISO 80000-3), `Δt`: `Q_e = int_(Δ t) Φ_e dt`. Radiant energy is expressed either as a function of wavelength (ISO 80000-3), `λ`, as a function of frequency (ISO 80000-3), `ν`, or as a function of wavenumber, `σ`. (See also 0.1.) The corresponding photometric quantity is "luminous energy" (item 7-12). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
    }

    /* ISO-80000-7 item 7-2.2 spectral radiant energy */
    attribute def SpectralRadiantEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-2.2 spectral radiant energy
         * symbol(s): `Q_(e,λ)`, `W_λ`, `U_λ`, `(Q_λ)`
         * application domain: generic
         * name: SpectralRadiantEnergy
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): J/nm, kg*m*s^-2
         * tensor order: 0
         * definition: spectral density of radiant energy, expressed by `Q_(e,λ) = (dQ_e) / (dλ)`, where `Q_e` is radiant energy (item 7-2.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant energy is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Q_e = int_(λ_1)^(λ_2) Q_(e,λ) dλ`
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyUnit[1];
    }

    attribute spectralRadiantEnergy: SpectralRadiantEnergyValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.1 radiant energy density */
    attribute def RadiantEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.1 radiant energy density
         * symbol(s): `w`, `(ρ_e)`
         * application domain: generic
         * name: RadiantEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: volumetric density of radiant energy, expressed by `w = (dQ_e)/(dV)`, where `Q_e` is radiant energy (item 7-2.1) in an elementary three-dimensional domain and `V` is the volume (ISO 80000-3) of that domain
         * remarks: Radiant energy density within a Planckian radiator is given by `w = (4 σ)/(c_0) T^4` where `σ` is the Stefan-Boltzmann constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1) and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantEnergyDensityUnit[1];
    }

    attribute radiantEnergyDensity: RadiantEnergyDensityValue[*] nonunique :> scalarQuantities;

    attribute def RadiantEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.2 spectral radiant energy density in terms of wavelength */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.2 spectral radiant energy density in terms of wavelength
         * symbol(s): `w_λ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavelength
         * quantity dimension: L^-2*M^1*T^-2
         * measurement unit(s): J/(m^3*nm), kg*m^-2*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavelength, expressed by `w_λ = (dw)/(dλ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavelength `λ` (ISO 80000-3)
         * remarks: Spectral radiant energy density within a Planckian radiator is given by `w_λ = 8πhc_0*f(λ, T)`, where `h` is the Planck constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1), `T` is thermodynamic temperature (ISO 80000-5) and `f(λ,T) = (λ^-5)/(exp(c_2 λ^-1 T^-1) - 1)`. For the radiation constant `c_2` in `f(λ,T)`, see ISO 80000-1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyDensityInTermsOfWavelengthUnit[1];
    }

    attribute spectralRadiantEnergyDensityInTermsOfWavelength: SpectralRadiantEnergyDensityInTermsOfWavelengthValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.3 spectral radiant energy density in terms of wavenumber */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.3 spectral radiant energy density in terms of wavenumber
         * symbol(s): `w_ṽ`, `ρ_ṽ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavenumber
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavenumber, expressed by `w_ṽ = (dw)/(dṽ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavenumber `ṽ` (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyDensityInTermsOfWavenumberUnit[1];
    }

    attribute spectralRadiantEnergyDensityInTermsOfWavenumber: SpectralRadiantEnergyDensityInTermsOfWavenumberValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-4.1 radiant flux, radiant power */
    attribute def RadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.1 radiant flux, radiant power
         * symbol(s): `Φ_e`, `P_e`, `Φ`, `P`
         * application domain: generic
         * name: RadiantFlux
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: change in radiant energy with time, expressed by `Φ_e = (dQ_e)/(dt)`, where `Q_e` is the radiant energy (item 7-2.1) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous flux" (item 7-13). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantFluxUnit[1];
    }

    attribute radiantFlux: RadiantFluxValue[*] nonunique :> scalarQuantities;

    attribute def RadiantFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias RadiantPowerUnit for RadiantFluxUnit;
    alias RadiantPowerValue for RadiantFluxValue;
    alias radiantPower for radiantFlux;

    /* ISO-80000-7 item 7-4.2 spectral radiant flux, spectral radiant power */
    attribute def SpectralRadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.2 spectral radiant flux, spectral radiant power
         * symbol(s): `Φ_(e,λ)`, `P_(e,λ)`, `(Φ_λ)`, `(P_λ)`
         * application domain: generic
         * name: SpectralRadiantFlux
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/nm, kg*m*s^-3
         * tensor order: 0
         * definition: spectral density of radiant flux, expressed by `Φ_(e,λ) = (dQ_e)/(dλ)`, where `Φ_e` is radiant flux (item 7-4.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant flux is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Φ_e = int_(λ_1)^(λ_2) Φ_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantFluxUnit[1];
    }

    attribute spectralRadiantFlux: SpectralRadiantFluxValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias SpectralRadiantPowerUnit for SpectralRadiantFluxUnit;
    alias SpectralRadiantPowerValue for SpectralRadiantFluxValue;
    alias spectralRadiantPower for spectralRadiantFlux;

    /* ISO-80000-7 item 7-5.1 radiant intensity */
    attribute def RadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.1 radiant intensity
         * symbol(s): `I_e`, `(I)`
         * application domain: generic
         * name: RadiantIntensity
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W/sr, kg*m^2*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant flux with respect to solid angle in a specified direction, expressed by `I_e = (dΦ_e)/(dΩ)`, where `Φ_e` is the radiant flux (item 7-4.1) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the radiant intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,φ)`, is used to determine the radiant flux (item 7-4.1) within a certain solid angle (ISO 80000-3), `Ω`, of a source: `Φ_e = int int_Ω I_e(θ, φ) sin(θ) dφ dθ`. The corresponding photometric quantity is "luminous intensity" (item 7-14). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantIntensityUnit[1];
    }

    attribute radiantIntensity: RadiantIntensityValue[*] nonunique :> scalarQuantities;

    attribute def RadiantIntensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-5.2 spectral radiant intensity */
    attribute def SpectralRadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.2 spectral radiant intensity
         * symbol(s): `I_(e,λ)`, `(I_λ)`
         * application domain: generic
         * name: SpectralRadiantIntensity
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/(sr*nm), kg*m*s^-3*sr^-1
         * tensor order: 0
         * definition: spectral density of radiant intensity, expressed by `I_(e, λ) = (d I_e)/(dλ)`, where `I_e` is radiant intensity (item 7-5.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant intensity is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `I_e = int_(λ_1)^(λ_2) I_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantIntensityUnit[1];
    }

    attribute spectralRadiantIntensity: SpectralRadiantIntensityValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantIntensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-6.1 radiance */
    attribute def RadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.1 radiance
         * symbol(s): `L_e`, `(L)`
         * application domain: generic
         * name: Radiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/(sr*m^2), kg*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_e = (d I_e)/(dA) * 1/cos(α)`, where `I_e` is radiant intensity (item 7-5.1), `A` is area (ISO 80000-3), and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: See also 0.1. For Planckian radiation, `L_e = σ/π T^4` where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminance" (item 7-15). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadianceUnit[1];
    }

    attribute radiance: RadianceValue[*] nonunique :> scalarQuantities;

    attribute def RadianceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-6.2 spectral radiance */
    attribute def SpectralRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.2 spectral radiance
         * symbol(s): `L_(e,λ)`, `(L_λ)`
         * application domain: generic
         * name: SpectralRadiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(sr*m^2*nm), kg*m^-1*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiance with respect to wavelength, expressed by `L_(e, λ) = (d L_e)/(d λ)` where `L_e` is radiance (item 7-6.1) in terms of wavelength λ(ISO 80000-3)
         * remarks: For Planckian radiation, `L_(e, λ)(λ) = (c(λ))/(4 π) ω_λ(λ) = h c_0^2 * f(λ,T)`, where `c(λ)` is phase speed (ISO 80000-3) of electromagnetic radiation of a wavelength (ISO 80000-3) `λ` in a given medium, `ω_λ(λ)` is spectral radiant energy density in terms of wavelength, `c_0` is speed of light in vacuum (ISO 80000-1), `h` is the Planck constant (ISO 80000-1), and `f(λ,T) = λ^-5/(exp(c_2 λ^-1 T^-1) - 1)`, where the radiation constant `c_2 = (hc)/k`. The integral of (total) radiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `L_e = int_(λ_1)^(λ_2) L_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadianceUnit[1];
    }

    attribute spectralRadiance: SpectralRadianceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-7.1 irradiance */
    attribute def IrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.1 irradiance
         * symbol(s): `E_e`, `(E)`
         * application domain: generic
         * name: Irradiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of incident radiant flux with respect to area at a point on a real or imaginary surface, expressed by `E_e = (d Φ_e)/(d A)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) on which the radiant flux is incident
         * remarks: The corresponding photometric quantity is "illuminance" (item 7-16). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical irradiance" is defined by the mean value of irradiance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(e,0) = int_(4 π) L_e d Ω` where `Ω` is solid angle (ISO 80000-3) and `L_e` is radiance (item 7-6.1). (See CIE DIS 017/E:2016, term 17-21-054.) It can be expressed by the quotient of the radiant flux (item 7-4.1) of all the radiation incident on the outer surface of an infinitely small sphere centred at the specified point and the area (ISO 80000-3) of the diametrical cross-section of that sphere. Spherical irradiance is also called "fluence rate" or "radiant fluence rate". The corresponding photometric quantity to spherical irradiance is called "spherical illuminance".
         */
        attribute :>> num: Real;
        attribute :>> mRef: IrradianceUnit[1];
    }

    attribute irradiance: IrradianceValue[*] nonunique :> scalarQuantities;

    attribute def IrradianceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-7.2 spectral irradiance */
    attribute def SpectralIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.2 spectral irradiance
         * symbol(s): `E_(e,λ)`, `(E_λ)`
         * application domain: generic
         * name: SpectralIrradiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of irradiance with respect to wavelength, expressed by `E_(e,λ) = (d E_e)/(dλ)`, where `E_e` is irradiance (item 7-7.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) irradiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `E_e = int_(λ_1)^(λ_2) E_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralIrradianceUnit[1];
    }

    attribute spectralIrradiance: SpectralIrradianceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralIrradianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-8.1 radiant exitance , radiant emittance */
    attribute def RadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.1 radiant exitance , radiant emittance
         * symbol(s): `M_e`, `(M)`
         * application domain: generic
         * name: RadiantExitance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of exiting radiant flux with respect to area at a point on a real or imaginary surface, expressed by `M_e = (d Φ_e)/(dA)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) from which the radiant flux leaves
         * remarks: For Planckian radiation, `M_e = σT^4`, where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminous exitance" (item 7-17). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantExitanceUnit[1];
    }

    attribute radiantExitance: RadiantExitanceValue[*] nonunique :> scalarQuantities;

    attribute def RadiantExitanceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    alias RadiantEmittanceUnit for RadiantExitanceUnit;
    alias RadiantEmittanceValue for RadiantExitanceValue;
    alias radiantEmittance for radiantExitance;

    /* ISO-80000-7 item 7-8.2 spectral radiant exitance */
    attribute def SpectralRadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.2 spectral radiant exitance
         * symbol(s): `M_(e,λ)`, `(M_λ)`
         * application domain: generic
         * name: SpectralRadiantExitance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of radiant exitance with respect to wavelength, expressed by `M_(e,λ) = (d M_e)/(dλ)`, where `M_e` is radiant exitance (item 7-8.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exitance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `M_e = int_(λ_1)^(λ_2) M_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantExitanceUnit[1];
    }

    attribute spectralRadiantExitance: SpectralRadiantExitanceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-9.1 radiant exposure */
    attribute def RadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.1 radiant exposure
         * symbol(s): `H_e`, `(H)`
         * application domain: generic
         * name: RadiantExposure
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: density of incident radiant energy with respect to area at a point on a real or imaginary surface, expressed by `H_e = (d Q_e)/(dA)`, where `Q_e` is radiant energy (item 7-2.1) and `A` is the area on which the radiant energy is incident (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous exposure" (item 7-18). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantExposureUnit[1];
    }

    attribute radiantExposure: RadiantExposureValue[*] nonunique :> scalarQuantities;

    attribute def RadiantExposureUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-9.2 spectral radiant exposure */
    attribute def SpectralRadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.2 spectral radiant exposure
         * symbol(s): `H_(e,λ)`, `(H_λ)`
         * application domain: generic
         * name: SpectralRadiantExposure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/(m^2*nm), kg*m^-1*s^-2
         * tensor order: 0
         * definition: density of radiant exposure with respect to wavelength, expressed by `H_(e,λ) = (d H_e)/(dλ)`, where `H_e` is radiant exposure (item 7-9.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exposure is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `H_e = int_(λ_1)^(λ_2) H_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantExposureUnit[1];
    }

    attribute spectralRadiantExposure: SpectralRadiantExposureValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-10.1 luminous efficiency */
    attribute def LuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.1 luminous efficiency
         * symbol(s): `V`
         * application domain: specified photometric condition
         * name: LuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant flux (item 7-4.1) weighted by the spectral luminous efficiency (item 7-10.2) and the corresponding radiant flux for a specified photometric condition
         * remarks: Luminous efficiency for photopic vision is expressed by `V = (int_0^∞ Φ_(e,λ)(λ) V(λ) d λ)/(int_0^∞ Φ_(e,λ)(λ) d λ) = K/K_m`, where `Φ_(e,λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency, `λ` is wavelength, `K` is luminous efficacy of radiation (item 7-11.1), and `K_m` is maximum luminous efficacy (item 7-11.3). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V` for photopic vision; `V'` for scotopic vision; `V_(mes;m)` for mesopic vision; `V_10` for the CIE 10° photopic photometric observer; `V_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute luminousEfficiency: LuminousEfficiencyValue :> scalarQuantities;

    /* ISO-80000-7 item 7-10.2 spectral luminous efficiency */
    attribute def SpectralLuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.2 spectral luminous efficiency
         * symbol(s): `V(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant flux (item 7-4.1) at wavelength `λ_m` and that at wavelength `λ`, such that both produce equally intense luminous sensations for a specified photometric condition and `λ_m` is chosen so that the maximum value of this quotient is equal to 1
         * remarks: The spectral luminous efficiency of the human eye depends on a number of factors, particularly the state of visual adaptation and the size and position of the source in the visual field. The photometric condition should be specified (e.g. photopic, scotopic, mesopic). If it is not specified, photopic vision is assumed and the symbol `V(λ)` is used. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V(λ)` for photopic vision; `V'(λ)` for scotopic vision; `V_(mes;m)(λ)` for mesopic vision; `V_10(λ)` for the CIE 10° photopic photometric observer; `V_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute spectralLuminousEfficiency: SpectralLuminousEfficiencyValue :> scalarQuantities;

    /* ISO-80000-7 item 7-11.1 luminous efficacy of radiation */
    attribute def LuminousEfficacyOfRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.1 luminous efficacy of radiation
         * symbol(s): `K`
         * application domain: specified photometric condition
         * name: LuminousEfficacyOfRadiation
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of luminous flux (item 7-13) and the corresponding radiant flux (item 7-4.1) for a specified photometric condition
         * remarks: Luminous efficacy of radiation for photopic vision is expressed by `K = Φ_V/Φ_e`, where `Φ_v` is luminous flux (item 7-13) and `Φ_e` is radiant flux (item 7-4.1). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K` for photopic vision; `K'` for scotopic vision; `K_(mes;m)` for mesopic vision; `K_10` for the CIE 10° photopic photometric observer; `K_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEfficacyOfRadiationUnit[1];
    }

    attribute luminousEfficacyOfRadiation: LuminousEfficacyOfRadiationValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEfficacyOfRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.2 spectral luminous efficacy */
    attribute def SpectralLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.2 spectral luminous efficacy
         * symbol(s): `K(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: product of spectral luminous efficiency (item 7-10.2) and maximum luminous efficacy (item 7-11.3) for a specified photometric condition
         * remarks: Spectral luminous efficacy for photopic vision is expressed by `K(λ) = K_m V(λ)`, where `K_m` is maximum luminous efficacy (item 7-11.3), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K(λ)` for photopic vision>; `K'(λ)` for scotopic vision; `K_(mes;m)(λ)` for mesopic vision; `K_10(λ)` for the CIE 10° photopic photometric observer; `K_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralLuminousEfficacyUnit[1];
    }

    attribute spectralLuminousEfficacy: SpectralLuminousEfficacyValue[*] nonunique :> scalarQuantities;

    attribute def SpectralLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.3 maximum luminous efficacy */
    attribute def MaximumLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.3 maximum luminous efficacy
         * symbol(s): `K_m`
         * application domain: specified photometric condition
         * name: MaximumLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: maximum value of spectral luminous efficacy for a specified photometric condition
         * remarks: See also 0.4 and 0.5. The value of maximum luminous efficacy for photopic vision is calculated by `K_m = 683 / (V(λ_(cd))) ["cd"*"sr"*"W"^-1] = 683 ["lm"*"W"^-1]` where `V(λ)` is the spectral luminous efficiency for photopic vision and `λ_(cd)` is the wavelength in air corresponding to the frequency `540*10^12 ["Hz"]` specified in the definition of the SI unit candela. Symbols for different photometric conditions: `K_m` for photopic vision; `K'_m` for scotopic vision; `K_(m,mes;m)` for mesopic vision; `K_(m,10)` for the CIE 10° photopic photometric observer; `K_(m,M)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MaximumLuminousEfficacyUnit[1];
    }

    attribute maximumLuminousEfficacy: MaximumLuminousEfficacyValue[*] nonunique :> scalarQuantities;

    attribute def MaximumLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.4 luminous efficacy of a source */
    attribute def LuminousEfficacyOfASourceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.4 luminous efficacy of a source
         * symbol(s): `η_v`, `(η)`
         * application domain: generic
         * name: LuminousEfficacyOfASource
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of the luminous flux emitted and the power consumed by the source, expressed by `η_v = Φ_v/P`, where `Φ_v` is luminous flux (item 7-13) and `P` is the power (ISO 80000-4) consumed by the source
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEfficacyOfASourceUnit[1];
    }

    attribute luminousEfficacyOfASource: LuminousEfficacyOfASourceValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEfficacyOfASourceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-12 luminous energy, quantity of light */
    attribute def LuminousEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-12 luminous energy, quantity of light
         * symbol(s): `Q_v`, `(Q)`
         * application domain: generic
         * name: LuminousEnergy
         * quantity dimension: T^1*J^1
         * measurement unit(s): lm*s, cd*sr*s
         * tensor order: 0
         * definition: energy of electromagnetic waves weighted by the spectral luminous efficiency (item 7-10.2) multiplied by maximum luminous efficacy (item 7-11.3) of a specified photometric condition
         * remarks: Luminous energy for photopic vision is expressed by `Q_v = K_m int_0^∞ Q_(e,λ)(λ) V(λ) dλ`, where `Q_(e,λ)(λ)` is the spectral radiant energy (item 7-2.2) at wavelength `λ` (ISO 80000-3), `V(λ)` is spectral luminous efficiency (item 7-10.2), and `K_m` is maximum luminous efficacy (7-11.3). Luminous energy can be emitted, transferred or received. Luminous energy can be expressed by the time integral of the luminous flux (item 7-13), `Φ_v`, over a given duration (ISO 80000-3), `Δt`: `Q_v = int_(Δt) Φ_v dt` . The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEnergyUnit[1];
    }

    attribute luminousEnergy: LuminousEnergyValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEnergyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (durationPF, luminousIntensityPF); }
    }

    alias QuantityOfLightUnit for LuminousEnergyUnit;
    alias QuantityOfLightValue for LuminousEnergyValue;
    alias quantityOfLight for luminousEnergy;

    /* ISO-80000-7 item 7-13 luminous flux */
    attribute def LuminousFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-13 luminous flux
         * symbol(s): `Φ_v`, `(Φ)`
         * application domain: generic
         * name: LuminousFlux
         * quantity dimension: J^1
         * measurement unit(s): lm, cd*sr
         * tensor order: 0
         * definition: change in luminous energy with time, expressed by `Φ_v = (d Q_v)/(dt)`, where `Q_v` is the luminous energy (item 7-12) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: Luminous flux is a quantity derived from the radiant flux (item 7-4.1), `Φ_e`, by evaluating the radiation according to its action upon the CIE standard photometric observer. (See CIE S 017/E:2011, term 17-738.) Luminous flux can be derived from the spectral radiant flux distribution by `Φ_v = K_m int_0^oo Φ_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `Φ_(e,λ)(λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength (ISO 80000-3). The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousFluxUnit[1];
    }

    attribute luminousFlux: LuminousFluxValue[*] nonunique :> scalarQuantities;

    attribute def LuminousFluxUnit :> DerivedUnit {
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = luminousIntensityPF; }
    }

    /* ISO-80000-7 item 7-14 luminous intensity */
    /* See package ISQBase for the declarations of LuminousIntensityValue and LuminousIntensityUnit */

    /* ISO-80000-7 item 7-15 luminance */
    attribute def LuminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-15 luminance
         * symbol(s): `L_v`, `(L)`
         * application domain: generic
         * name: Luminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: density of luminous intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_v = (dI_v)/(dA) 1/cos(α)`, where `I_v` is luminous intensity (item 7-14), `A` is area (ISO 80000-3) and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: Luminance can be derived from the spectral radiance distribution by `L_v = K_m int_0^∞ L_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `L_(e,λ)(λ)` is the spectral radiance (item 7-6.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also 0.1. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminanceUnit[1];
    }

    attribute luminance: LuminanceValue[*] nonunique :> scalarQuantities;

    attribute def LuminanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-16 illuminance */
    attribute def IlluminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-16 illuminance
         * symbol(s): `E_v`, `(E)`
         * application domain: generic
         * name: Illuminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lx, cd*sr*m^-2
         * tensor order: 0
         * definition: density of incident luminous flux with respect to area at a point on a real or imaginary surface, expressed by `E_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) on which the luminous flux is incident
         * remarks: Illuminance can be derived from the spectral irradiance distribution by `E_v = K_m int_0^∞ E_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `E_(e,λ)(λ)` is the spectral irradiance (item 7-7.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical illuminance" is defined by the mean value of illuminance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(v,0) = int_(4π) L_v dΩ`, where `Ω` is solid angle (ISO 80000-3) and `L_v` is luminance (item 7-15). It can be expressed by the quotient of the luminous flux (item 7-13) of all the light incident on the outer surface of an infinitely small sphere centred at the given point, and the area (ISO 80000-3) of the diametrical cross-section of that sphere.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IlluminanceUnit[1];
    }

    attribute illuminance: IlluminanceValue[*] nonunique :> scalarQuantities;

    attribute def IlluminanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-17 luminous exitance */
    attribute def LuminousExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-17 luminous exitance
         * symbol(s): `M_v`, `(M)`
         * application domain: generic
         * name: LuminousExitance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lm/m^2, cd*sr*m^-2
         * tensor order: 0
         * definition: density of exiting luminous flux with respect to area at a point on a real or imaginary surface, expressed by `M_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) from which the luminous flux leaves
         * remarks: Luminous exitance can be derived from the spectral radiant exitance distribution by `M_v = K_m int_0^∞ M_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `M_(e_λ)(λ)` is the spectral radiant exitance (item 7-8.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousExitanceUnit[1];
    }

    attribute luminousExitance: LuminousExitanceValue[*] nonunique :> scalarQuantities;

    attribute def LuminousExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-18 luminous exposure, quantity of illumination, light exposure */
    attribute def LuminousExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-18 luminous exposure, quantity of illumination, light exposure
         * symbol(s): `H_v`, `(H)`
         * application domain: generic
         * name: LuminousExposure
         * quantity dimension: L^-2*T^1*J^1
         * measurement unit(s): lx*s, cd*sr*m^-2*s
         * tensor order: 0
         * definition: density of incident luminous energy with respect to area at a point on a real or imaginary surface, expressed by `H_v = (dQ_v)/(dA)`, where `Q_v` is luminous energy (item 7-12) and `A` is the area on which the luminous energy is incident (ISO 80000-3)
         * remarks: Luminous exposure can be derived from the spectral radiant exposure distribution by `H_v = K_m int_0^∞ H_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `H_(e_λ)(λ)` is the spectral radiant exposure (item 7-9.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousExposureUnit[1];
    }

    attribute luminousExposure: LuminousExposureValue[*] nonunique :> scalarQuantities;

    attribute def LuminousExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, luminousIntensityPF); }
    }

    alias QuantityOfIlluminationUnit for LuminousExposureUnit;
    alias QuantityOfIlluminationValue for LuminousExposureValue;
    alias quantityOfIllumination for luminousExposure;

    alias LightExposureUnit for LuminousExposureUnit;
    alias LightExposureValue for LuminousExposureValue;
    alias lightExposure for luminousExposure;

    /* ISO-80000-7 item 7-19.1 photon number, number of photons */
    attribute def PhotonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-19.1 photon number, number of photons
         * symbol(s): `N_p`
         * application domain: generic
         * name: PhotonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant energy and photon energy, expressed by `N_p = Q_e/(h ν)`, where `Q_e` is radiant energy (item 7-2.1), `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon number can also be expressed by the time integral of the photon flux (item 7-20), `Φ_p`, over a given duration, `Δt`, `N_p = int_(Δt) Φ_p dt`
         */
    }
    attribute photonNumber: PhotonNumberValue :> scalarQuantities;

    alias numberOfPhotons for photonNumber;

    /* ISO-80000-7 item 7-19.2 photon energy */
    attribute photonEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 7-19.2 photon energy
         * symbol(s): `Q_p`, `(Q)`
         * application domain: generic
         * name: PhotonEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: product of the Planck constant and frequency, expressed by `Q_p = h ν` where `h` is the Planck constant (ISO 80000-1) and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon energy can be emitted, transferred or received. For monochromatic radiation, photon energy may be expressed by photon number (item 7-19.1). The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding photometric quantity is "luminous energy" (item 7-12).
         */
    }

    /* ISO-80000-7 item 7-20 photon flux */
    attribute def PhotonFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-20 photon flux
         * symbol(s): `Φ_p`, `(Φ)`
         * application domain: generic
         * name: PhotonFlux
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: rate of photon number per time interval, expressed by `Φ_p = (d N_p)/(dt)`, where `N_p` is photon number (e.g. given by item 7-19.1), transmitted or received, and `t` is time (ISO 80000-3)
         * remarks: Photon flux `Φ_p` is related to radiant flux (item 7-4.1), `Φ_e`, of monochromatic radiation, by `Φ_p = Φ_e/(h ν)` where `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave. The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding photometric quantity is "luminous flux" (item 7-13).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonFluxUnit[1];
    }

    attribute photonFlux: PhotonFluxValue[*] nonunique :> scalarQuantities;

    attribute def PhotonFluxUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-7 item 7-21 photon intensity */
    attribute def PhotonIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-21 photon intensity
         * symbol(s): `I_p`, `(I)`
         * application domain: generic
         * name: PhotonIntensity
         * quantity dimension: T^-1
         * measurement unit(s): s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon flux with respect to solid angle in a specified direction, expressed by `I_p = (dΦ_p)/(dΩ)`, where `Φ_p` is the photon flux (item 7-20) emitted in the given direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The distribution of the photon intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)` , is used to determine the photon flux (item 7-20) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_p = int int_Ω I_v(θ,ϕ) sin(θ) dϕ dθ`. The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding photometric quantity is "luminous intensity" (item 7-14).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonIntensityUnit[1];
    }

    attribute photonIntensity: PhotonIntensityValue[*] nonunique :> scalarQuantities;

    attribute def PhotonIntensityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-7 item 7-22 photon radiance */
    attribute def PhotonRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-22 photon radiance
         * symbol(s): `L_p`, `(L)`
         * application domain: generic
         * name: PhotonRadiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_p = (dI_p)/(dA) 1/cos(α)`, where `I_p` is photon intensity (item 7-21), `A` is area (ISO 80000-3) and `α` the angle between the normal to the surface at the specified point and the specified direction
         * remarks: The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding photometric quantity is "luminance" (item 7-15).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonRadianceUnit[1];
    }

    attribute photonRadiance: PhotonRadianceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonRadianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-23 photon irradiance */
    attribute def PhotonIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-23 photon irradiance
         * symbol(s): `E_p`, `(E)`
         * application domain: generic
         * name: PhotonIrradiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of incident photon flux with respect to area at a point on a real or imaginary surface, expressed by `E_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) on which the photon flux is incident
         * remarks: The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding photometric quantity is "illuminance" (item 7-16).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonIrradianceUnit[1];
    }

    attribute photonIrradiance: PhotonIrradianceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonIrradianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-24 photon exitance */
    attribute def PhotonExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-24 photon exitance
         * symbol(s): `M_p`, `(M)`
         * application domain: generic
         * name: PhotonExitance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of exiting photon flux with respect to area at a point on a real or imaginary surface, expressed by `M_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) from which the photon flux leaves
         * remarks: The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding photometric quantity is "luminous exitance" (item 7-17).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonExitanceUnit[1];
    }

    attribute photonExitance: PhotonExitanceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-25 photon exposure */
    attribute def PhotonExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-25 photon exposure
         * symbol(s): `H_p`, `(H)`
         * application domain: generic
         * name: PhotonExposure
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: density of incident photon number with respect to area at a point on a real or imaginary surface, expressed by `H_p = (dN_p)/(dA)`, where `N_p` is photon number (item 7-19.1) and `A` is the area (ISO 80000-3) on which the photons are incident
         * remarks: The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding photometric quantity is "luminous exposure" (item 7-18).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonExposureUnit[1];
    }

    attribute photonExposure: PhotonExposureValue[*] nonunique :> scalarQuantities;

    attribute def PhotonExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-7 item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer
         * symbol(s): `X,Y,Z`
         * application domain: generic
         * name: TristimulusValuesForTheCie1931StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1931 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `[cd*m^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 // int_0^∞ S_λ(λ) overline y(λ) dλ`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TristimulusValuesForTheCie1931StandardColorimetricObserverUnit[1];
    }

    attribute tristimulusValuesForTheCie1931StandardColorimetricObserver: TristimulusValuesForTheCie1931StandardColorimetricObserverValue[*] nonunique :> scalarQuantities;

    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer
         * symbol(s): `X_10,Y_10,Z_10`
         * application domain: generic
         * name: TristimulusValuesForTheCie1964StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1964 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `["cd"*"m"^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 /( int_0^∞ S_λ(λ) overline y(λ) dλ)`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TristimulusValuesForTheCie1964StandardColorimetricObserverUnit[1];
    }

    attribute tristimulusValuesForTheCie1964StandardColorimetricObserver: TristimulusValuesForTheCie1964StandardColorimetricObserverValue[*] nonunique :> scalarQuantities;

    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer
         * symbol(s): `overline x(λ)`, `overline y(λ)`, `overline z(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x(λ)` , `overline y(λ)` , `overline z(λ)` in the CIE 1931 standard colorimetric system
         * remarks: Values of `overline x(λ)` , `overline y(λ)` and `overline z(λ)` are defined in the CIE 1931 standard colorimetric system (2° observer) — applicable to fields of observation of angular opening from 1° to 4°.
         */
    }
    attribute cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> scalarQuantities;

    /* ISO-80000-7 item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer
         * symbol(s): `overline x_10(λ)`, `overline y_10(λ)`, `overline z_10(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x_10(λ)` , `overline y_10(λ)` , `overline z_10(λ)` in the CIE 1964 standard colorimetric system
         * remarks: Values of `overline x_10(λ)` , `overline y_10(λ)` and `overline z_10(λ)` are defined in the CIE 1964 standard colorimetric system (10° observer) — applicable to fields of observation with angles greater than 4°.
         */
    }
    attribute cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> scalarQuantities;

    /* ISO-80000-7 item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system
         * symbol(s): `x,y,z`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1931 standard colorimetric observer (item 7-26.1) and their sum, expressed by `x = X / (X+Y+Z)` , `y = Y / (X+Y+Z)` , `z = Z / (X+Y+Z)`
         * remarks: Since `x + y + z = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute chromaticityCoordinatesInTheCie1931StandardColorimetricSystem: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> scalarQuantities;

    /* ISO-80000-7 item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system
         * symbol(s): `x_10,y_10,z_10`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1964 standard colorimetric observer (item 7-26.2) and their sum, expressed by `x_10 = X_10 / (X_10+Y_10+Z_10)`, `y_10 = Y_10 / (X_10+Y_10+Z_10)`, `z_10 = Z_10 / (X_10+Y_10+Z_10)`
         * remarks: Since `x_10 + y_10 + z_10 = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute chromaticityCoordinatesInTheCie1964StandardColorimetricSystem: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> scalarQuantities;

    /* ISO-80000-7 item 7-29.1 colour temperature */
    attribute colourTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 7-29.1 colour temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: ColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator whose radiation has the same chromaticity as that of a given stimulus
         * remarks: None.
         */
    }

    /* ISO-80000-7 item 7-29.2 correlated colour temperature */
    attribute correlatedColourTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 7-29.2 correlated colour temperature
         * symbol(s): `T_"cp"`
         * application domain: generic
         * name: CorrelatedColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator having the chromaticity nearest the chromaticity associated with the given spectral distribution on a modified 1976 CIE Uniform Chromaticity Scale (UCS) diagram where `u',2/3 v'` are the coordinates of the Planckian locus and the test stimulus
         * remarks: None.
         */
    }

    /* ISO-80000-7 item 7-30.1 emissivity */
    attribute def EmissivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.1 emissivity
         * symbol(s): `ε`, `ε_T`
         * application domain: generic
         * name: Emissivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator and the radiant exitance of a Planckian radiator at the same temperature, expressed by `ε = M/M_b`, where `M` is the radiant exitance (item 7-8.1) of a thermal radiator and `M_b` is the radiant exitance of a Planckian radiator at the same temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute emissivity: EmissivityValue :> scalarQuantities;

    /* ISO-80000-7 item 7-30.2 emissivity at a specified wavelength */
    attribute def EmissivityAtASpecifiedWavelengthValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.2 emissivity at a specified wavelength
         * symbol(s): `ε(λ)`
         * application domain: generic
         * name: EmissivityAtASpecifiedWavelength (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator at a specified wavelength and the radiant exitance of a Planckian radiator at the same temperature and at the same wavelength, expressed by `ε(λ) = M(λ) / M_b(λ)`, where `M(λ)` is the radiant exitance (item 7-8.1) of a thermal radiator at a specified wavelength and `M_b(λ)` is the radiant exitance of a Planckian radiator at the same temperature at a specified wavelength (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute emissivityAtASpecifiedWavelength: EmissivityAtASpecifiedWavelengthValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.1 absorptance */
    attribute def AbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.1 absorptance
         * symbol(s): `α`, `a`
         * application domain: generic
         * name: Absorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed radiant flux and incident radiant flux, expressed by `α = Φ_a/Φ_m`, where `Φ_a` is absorbed radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `ρ` is reflectance (item 7-31.3) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute absorptance: AbsorptanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.2 luminous absorptance */
    attribute def LuminousAbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.2 luminous absorptance
         * symbol(s): `α_v`
         * application domain: generic
         * name: LuminousAbsorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed luminous flux and incident luminous flux, expressed by `α_v = Φ_(v,a)/Φ_(v,m)`, where `Φ_(v,a)` is absorbed luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral absorptance, `α(λ)`, luminous absorptance can be calculated by `α_v = (int_0^∞ α(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.1.
         */
    }
    attribute luminousAbsorptance: LuminousAbsorptanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.3 reflectance */
    attribute def ReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.3 reflectance
         * symbol(s): `ρ`
         * application domain: generic
         * name: Reflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected radiant flux and incident radiant flux, expressed by `ρ = Φ_r/Φ_m`, where `Φ_r` is reflected radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute reflectance: ReflectanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.4 luminous reflectance */
    attribute def LuminousReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.4 luminous reflectance
         * symbol(s): `ρ_v`
         * application domain: generic
         * name: LuminousReflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected luminous flux and incident luminous flux, is expressed by `ρ_v = Φ_(v,r)/Φ_(v,m)`, where `Φ_(v,r)` is reflected luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral reflectance, `ρ(λ)`, luminous reflectance can be calculated by `ρ_v = (int_0^∞ ρ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.3.
         */
    }
    attribute luminousReflectance: LuminousReflectanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.5 transmittance */
    attribute def TransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.5 transmittance
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: Transmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted radiant flux and incident radiant flux, expressed by `τ = Φ_t/Φ_m`, where `Φ_t` is transmitted radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `ρ` is reflectance (item 7-31.3).
         */
    }
    attribute transmittance: TransmittanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.6 luminous transmittance */
    attribute def LuminousTransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.6 luminous transmittance
         * symbol(s): `τ_v`
         * application domain: generic
         * name: LuminousTransmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted luminous flux and incident luminous flux, expressed by `τ_v = Φ_(v,t)/Φ_(v,m)`, where `Φ_(v,t)` is transmitted luminous flux (item 7-13) and `Φ_(v,m)` is luminous flux of the incident radiation
         * remarks: From the spectral transmittance `τ(λ)`, luminous transmittance can be calculated by `τ_v = (int_0^∞ τ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is the spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is the spectral luminous efficiency (item 7-10.2). See also item 7-31.5.
         */
    }
    attribute luminousTransmittance: LuminousTransmittanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance */
    attribute def TransmittanceOpticalDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance
         * symbol(s): `D`, `A_10`, `D_τ`
         * application domain: generic
         * name: TransmittanceOpticalDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: logarithm to base 10 of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the optical density can be expressed by `A_10(λ) = -log(τ(λ))`, where `τ(λ)` is the transmittance (item 7-31.5) in terms of wavelength. In spectroscopy, the name "absorbance" `A_10` is generally used.
         */
    }
    attribute transmittanceOpticalDensity: TransmittanceOpticalDensityValue :> scalarQuantities;

    alias opticalDensity for transmittanceOpticalDensity;

    alias transmittanceDensity for transmittanceOpticalDensity;

    alias decadicAbsorbance for transmittanceOpticalDensity;

    /* ISO-80000-7 item 7-32.2 Napierian absorbance */
    attribute def NapierianAbsorbanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.2 Napierian absorbance
         * symbol(s): `A_n`, `B`
         * application domain: generic
         * name: NapierianAbsorbance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: natural (Napierian) logarithm of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the Napierian absorbance can be expressed by `A_n(λ) = B(λ) = -log(τ(λ))`. It can also be expressed as `A_n(λ) = l*α(λ)`, where `α` is linear absorption coefficient (item 7-35.2) and `l` is length (ISO 80000-3) traversed.
         */
    }
    attribute napierianAbsorbance: NapierianAbsorbanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-33.1 radiance factor */
    attribute def RadianceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.1 radiance factor
         * symbol(s): `β_e`, `(β)`
         * application domain: generic
         * name: RadianceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiance of a surface element in a specified direction and the radiance of the perfect reflecting diffuser or perfect transmitting diffuser identically irradiated and viewed, expressed by `β_e = L_(e,n)/L_(e,d)`, where `L_(e,n)` is the radiance (item 7-6.1) of a surface element in a given direction and `L_(e,d)` is the radiance of the perfect reflecting or transmitting diffuser identically irradiated and viewed
         * remarks: The definition holds for a surface element of a non-self-radiating medium, in a given direction and under specified conditions of irradiation. Radiance factor is equivalent to reflectance factor (item 7-34) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is `2π ["sr"]`. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called "perfect diffuser".
         */
    }
    attribute radianceFactor: RadianceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-33.2 luminance factor */
    attribute def LuminanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.2 luminance factor
         * symbol(s): `β_v`, `(β)`
         * application domain: generic
         * name: LuminanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the luminance of a surface element in a specified direction and the luminance of the perfect reflecting diffuser or perfect transmitting diffuser identically illuminated and viewed, expressed by `β_v = L_(v,n)/L_(v,d)`, where `L_(v,n)` is the luminance (item 7-15) of a surface element in a given direction and `L_(v,d)` is the luminance of the perfect reflecting or transmitting diffuser identically illuminated and viewed
         * remarks: The definition holds for a surface element of a non-luminous medium, in a given direction and under specified conditions of irradiation. This quantity is also defined spectrally and is called "spectral luminance factor". For the analogous radiant quantity "radiance factor", see item 7-33.1.
         */
    }
    attribute luminanceFactor: LuminanceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-34 reflectance factor */
    attribute def ReflectanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-34 reflectance factor
         * symbol(s): `R`
         * application domain: generic
         * name: ReflectanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the flux reflected in the directions delimited by a given cone with apex at a surface element and the flux reflected in the same directions by a perfect reflecting diffuser identically irradiated or illuminated, expressed by `R = Φ_n/Φ_d`, where `Φ_n` is the flux reflected in the directions delimited by a given cone and `Φ_d` is the flux reflected in the same directions by an identically irradiated diffuser of reflectance (item 7-31.3) equal to 1
         * remarks: The flux can be a radiant flux (item 7‐4.1) or a luminous flux (item 7‐13). The definition holds for a surface element, for the part of the reflected radiation contained in a given cone with apex at the surface element, and for incident radiation of given spectral composition, polarization and geometric distribution. Reflectance factor is equivalent to radiance factor (item 7-33.1) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is 2π sr. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called a perfect diffuser.
         */
    }
    attribute reflectanceFactor: ReflectanceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-35.1 linear attenuation coefficient, linear extinction coefficient */
    attribute def LinearAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.1 linear attenuation coefficient, linear extinction coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: radiometry
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux caused by absorption and scattering
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear attenuation coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing and scattering medium `μ(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAttenuationCoefficientUnit[1];
    }

    attribute linearAttenuationCoefficient: LinearAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias LinearExtinctionCoefficientUnit for LinearAttenuationCoefficientUnit;
    alias LinearExtinctionCoefficientValue for LinearAttenuationCoefficientValue;
    alias linearExtinctionCoefficient for linearAttenuationCoefficient;

    /* ISO-80000-7 item 7-35.2 linear absorption coefficient */
    attribute def LinearAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.2 linear absorption coefficient
         * symbol(s): `α_l`, `a_l`, `α`
         * application domain: radiometry
         * name: LinearAbsorptionCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux (item 7-4.1) caused by absorption
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear absorption coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing medium `α_l(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. It can also be expressed as a function of transmittance (item 7-31.5). `α_l = -ln(τ)/l = A_n/l`. The linear absorption coefficient is that part of the linear attenuation coefficient (item 7-35.1) that is due to absorption. Scattering might also contribute. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAbsorptionCoefficientUnit[1];
    }

    attribute linearAbsorptionCoefficient: LinearAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-7 item 7-36.1 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.1 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: radiometry
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient (item 7-35.1), `μ`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `μ_m(λ) = (μ(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAttenuationCoefficientUnit[1];
    }

    attribute massAttenuationCoefficient: MassAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-7 item 7-36.2 mass absorption coefficient */
    attribute def MassAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.2 mass absorption coefficient
         * symbol(s): `α_m`
         * application domain: radiometry
         * name: MassAbsorptionCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear absorption coefficient (item 7-35.2), `α`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `α_m(λ) = (α(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAbsorptionCoefficientUnit[1];
    }

    attribute massAbsorptionCoefficient: MassAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-7 item 7-37 molar absorption coefficient */
    attribute def MolarAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-37 molar absorption coefficient
         * symbol(s): `χ`
         * application domain: radiometry
         * name: MolarAbsorptionCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: product of linear absorption coefficient and molar volume, expressed by `χ = α V_m`, where `α` is linear absorption coefficient (item 7-35.2) and `V_m` is molar volume (ISO 80000-9)
         * remarks: The molar absorption coefficient can also be expressed by `χ = α c` where `c` is amount-of-substance concentration (ISO 80000-9). Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarAbsorptionCoefficientUnit[1];
    }

    attribute molarAbsorptionCoefficient: MolarAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MolarAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQLight"))) (name "ISQLight") (declared-name "ISQLight")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQLight::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQLight::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQLight::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))) (name "AbsorptanceValue") (declared-name "AbsorptanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))) (name "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (declared-name "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))) (name "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (declared-name "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))) (name "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (declared-name "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))) (name "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (declared-name "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))) (name "EmissivityAtASpecifiedWavelengthValue") (declared-name "EmissivityAtASpecifiedWavelengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))) (name "EmissivityValue") (declared-name "EmissivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::EmissivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::EmissivityValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQLight::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (name "IlluminanceUnit") (declared-name "IlluminanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))) (name "IlluminanceValue") (declared-name "IlluminanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::IlluminanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IlluminanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IlluminanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (name "IrradianceUnit") (declared-name "IrradianceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IrradianceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))) (name "IrradianceValue") (declared-name "IrradianceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::IrradianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IrradianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::IrradianceValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::LightExposureUnit"))) (name "LightExposureUnit") (declared-name "LightExposureUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::LightExposureValue"))) (name "LightExposureValue") (declared-name "LightExposureValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))) (name "LinearAbsorptionCoefficientUnit") (declared-name "LinearAbsorptionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))) (name "LinearAbsorptionCoefficientValue") (declared-name "LinearAbsorptionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))) (name "LinearAttenuationCoefficientUnit") (declared-name "LinearAttenuationCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))) (name "LinearAttenuationCoefficientValue") (declared-name "LinearAttenuationCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::LinearExtinctionCoefficientUnit"))) (name "LinearExtinctionCoefficientUnit") (declared-name "LinearExtinctionCoefficientUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::LinearExtinctionCoefficientValue"))) (name "LinearExtinctionCoefficientValue") (declared-name "LinearExtinctionCoefficientValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))) (name "LuminanceFactorValue") (declared-name "LuminanceFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (name "LuminanceUnit") (declared-name "LuminanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))) (name "LuminanceValue") (declared-name "LuminanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))) (name "LuminousAbsorptanceValue") (declared-name "LuminousAbsorptanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (name "LuminousEfficacyOfASourceUnit") (declared-name "LuminousEfficacyOfASourceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))) (name "LuminousEfficacyOfASourceValue") (declared-name "LuminousEfficacyOfASourceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (name "LuminousEfficacyOfRadiationUnit") (declared-name "LuminousEfficacyOfRadiationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))) (name "LuminousEfficacyOfRadiationValue") (declared-name "LuminousEfficacyOfRadiationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))) (name "LuminousEfficiencyValue") (declared-name "LuminousEfficiencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (name "LuminousEnergyUnit") (declared-name "LuminousEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))) (name "LuminousEnergyValue") (declared-name "LuminousEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (name "LuminousExitanceUnit") (declared-name "LuminousExitanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))) (name "LuminousExitanceValue") (declared-name "LuminousExitanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (name "LuminousExposureUnit") (declared-name "LuminousExposureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))) (name "LuminousExposureValue") (declared-name "LuminousExposureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))) (name "LuminousFluxUnit") (declared-name "LuminousFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))) (name "LuminousFluxValue") (declared-name "LuminousFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))) (name "LuminousReflectanceValue") (declared-name "LuminousReflectanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))) (name "LuminousTransmittanceValue") (declared-name "LuminousTransmittanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (name "MassAbsorptionCoefficientUnit") (declared-name "MassAbsorptionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))) (name "MassAbsorptionCoefficientValue") (declared-name "MassAbsorptionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (name "MassAttenuationCoefficientUnit") (declared-name "MassAttenuationCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))) (name "MassAttenuationCoefficientValue") (declared-name "MassAttenuationCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (name "MaximumLuminousEfficacyUnit") (declared-name "MaximumLuminousEfficacyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))) (name "MaximumLuminousEfficacyValue") (declared-name "MaximumLuminousEfficacyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (name "MolarAbsorptionCoefficientUnit") (declared-name "MolarAbsorptionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))) (name "MolarAbsorptionCoefficientValue") (declared-name "MolarAbsorptionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))) (name "NapierianAbsorbanceValue") (declared-name "NapierianAbsorbanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (name "PhotonExitanceUnit") (declared-name "PhotonExitanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))) (name "PhotonExitanceValue") (declared-name "PhotonExitanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))) (name "PhotonExposureUnit") (declared-name "PhotonExposureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))) (name "PhotonExposureValue") (declared-name "PhotonExposureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))) (name "PhotonFluxUnit") (declared-name "PhotonFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))) (name "PhotonFluxValue") (declared-name "PhotonFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))) (name "PhotonIntensityUnit") (declared-name "PhotonIntensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))) (name "PhotonIntensityValue") (declared-name "PhotonIntensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (name "PhotonIrradianceUnit") (declared-name "PhotonIrradianceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))) (name "PhotonIrradianceValue") (declared-name "PhotonIrradianceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))) (name "PhotonNumberValue") (declared-name "PhotonNumberValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (name "PhotonRadianceUnit") (declared-name "PhotonRadianceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))) (name "PhotonRadianceValue") (declared-name "PhotonRadianceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::QuantityOfIlluminationUnit"))) (name "QuantityOfIlluminationUnit") (declared-name "QuantityOfIlluminationUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::QuantityOfIlluminationValue"))) (name "QuantityOfIlluminationValue") (declared-name "QuantityOfIlluminationValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::QuantityOfLightUnit"))) (name "QuantityOfLightUnit") (declared-name "QuantityOfLightUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::QuantityOfLightValue"))) (name "QuantityOfLightValue") (declared-name "QuantityOfLightValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))) (name "RadianceFactorValue") (declared-name "RadianceFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (name "RadianceUnit") (declared-name "RadianceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadianceValue"))) (name "RadianceValue") (declared-name "RadianceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadianceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadianceValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::RadiantEmittanceUnit"))) (name "RadiantEmittanceUnit") (declared-name "RadiantEmittanceUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::RadiantEmittanceValue"))) (name "RadiantEmittanceValue") (declared-name "RadiantEmittanceValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (name "RadiantEnergyDensityUnit") (declared-name "RadiantEnergyDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))) (name "RadiantEnergyDensityValue") (declared-name "RadiantEnergyDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (name "RadiantExitanceUnit") (declared-name "RadiantExitanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))) (name "RadiantExitanceValue") (declared-name "RadiantExitanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (name "RadiantExposureUnit") (declared-name "RadiantExposureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))) (name "RadiantExposureValue") (declared-name "RadiantExposureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (name "RadiantFluxUnit") (declared-name "RadiantFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))) (name "RadiantFluxValue") (declared-name "RadiantFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (name "RadiantIntensityUnit") (declared-name "RadiantIntensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))) (name "RadiantIntensityValue") (declared-name "RadiantIntensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::RadiantPowerUnit"))) (name "RadiantPowerUnit") (declared-name "RadiantPowerUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::RadiantPowerValue"))) (name "RadiantPowerValue") (declared-name "RadiantPowerValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQLight::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))) (name "ReflectanceFactorValue") (declared-name "ReflectanceFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))) (name "ReflectanceValue") (declared-name "ReflectanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::ReflectanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::ReflectanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))) (name "RefractiveIndexValue") (declared-name "RefractiveIndexValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (name "SpectralIrradianceUnit") (declared-name "SpectralIrradianceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))) (name "SpectralIrradianceValue") (declared-name "SpectralIrradianceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (name "SpectralLuminousEfficacyUnit") (declared-name "SpectralLuminousEfficacyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))) (name "SpectralLuminousEfficacyValue") (declared-name "SpectralLuminousEfficacyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))) (name "SpectralLuminousEfficiencyValue") (declared-name "SpectralLuminousEfficiencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (name "SpectralRadianceUnit") (declared-name "SpectralRadianceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))) (name "SpectralRadianceValue") (declared-name "SpectralRadianceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (name "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))) (name "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (name "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))) (name "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (name "SpectralRadiantEnergyUnit") (declared-name "SpectralRadiantEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))) (name "SpectralRadiantEnergyValue") (declared-name "SpectralRadiantEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (name "SpectralRadiantExitanceUnit") (declared-name "SpectralRadiantExitanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))) (name "SpectralRadiantExitanceValue") (declared-name "SpectralRadiantExitanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (name "SpectralRadiantExposureUnit") (declared-name "SpectralRadiantExposureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))) (name "SpectralRadiantExposureValue") (declared-name "SpectralRadiantExposureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (name "SpectralRadiantFluxUnit") (declared-name "SpectralRadiantFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))) (name "SpectralRadiantFluxValue") (declared-name "SpectralRadiantFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (name "SpectralRadiantIntensityUnit") (declared-name "SpectralRadiantIntensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))) (name "SpectralRadiantIntensityValue") (declared-name "SpectralRadiantIntensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantPowerUnit"))) (name "SpectralRadiantPowerUnit") (declared-name "SpectralRadiantPowerUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantPowerValue"))) (name "SpectralRadiantPowerValue") (declared-name "SpectralRadiantPowerValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (name "SpeedOfLightInAMediumUnit") (declared-name "SpeedOfLightInAMediumUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))) (name "SpeedOfLightInAMediumValue") (declared-name "SpeedOfLightInAMediumValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))) (name "TransmittanceOpticalDensityValue") (declared-name "TransmittanceOpticalDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))) (name "TransmittanceValue") (declared-name "TransmittanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::TransmittanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::TransmittanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (name "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (declared-name "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))) (name "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (declared-name "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (name "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (declared-name "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))) (name "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (declared-name "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::absorptance"))) (name "absorptance") (declared-name "absorptance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem"))) (name "chromaticityCoordinatesInTheCie1931StandardColorimetricSystem") (declared-name "chromaticityCoordinatesInTheCie1931StandardColorimetricSystem") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem"))) (name "chromaticityCoordinatesInTheCie1964StandardColorimetricSystem") (declared-name "chromaticityCoordinatesInTheCie1964StandardColorimetricSystem") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver"))) (name "cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver") (declared-name "cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver"))) (name "cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver") (declared-name "cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::colourTemperature"))) (name "colourTemperature") (declared-name "colourTemperature") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::colourTemperature::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::colourTemperature")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature"))) (name "correlatedColourTemperature") (declared-name "correlatedColourTemperature") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::decadicAbsorbance"))) (name "decadicAbsorbance") (declared-name "decadicAbsorbance"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::emissivity"))) (name "emissivity") (declared-name "emissivity") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::emissivityAtASpecifiedWavelength"))) (name "emissivityAtASpecifiedWavelength") (declared-name "emissivityAtASpecifiedWavelength") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::illuminance"))) (name "illuminance") (declared-name "illuminance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::irradiance"))) (name "irradiance") (declared-name "irradiance") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::lightExposure"))) (name "lightExposure") (declared-name "lightExposure"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::linearAbsorptionCoefficient"))) (name "linearAbsorptionCoefficient") (declared-name "linearAbsorptionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::linearAttenuationCoefficient"))) (name "linearAttenuationCoefficient") (declared-name "linearAttenuationCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::linearExtinctionCoefficient"))) (name "linearExtinctionCoefficient") (declared-name "linearExtinctionCoefficient"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminance"))) (name "luminance") (declared-name "luminance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminanceFactor"))) (name "luminanceFactor") (declared-name "luminanceFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousAbsorptance"))) (name "luminousAbsorptance") (declared-name "luminousAbsorptance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfASource"))) (name "luminousEfficacyOfASource") (declared-name "luminousEfficacyOfASource") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfRadiation"))) (name "luminousEfficacyOfRadiation") (declared-name "luminousEfficacyOfRadiation") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousEfficiency"))) (name "luminousEfficiency") (declared-name "luminousEfficiency") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousEnergy"))) (name "luminousEnergy") (declared-name "luminousEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousExitance"))) (name "luminousExitance") (declared-name "luminousExitance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousExposure"))) (name "luminousExposure") (declared-name "luminousExposure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousFlux"))) (name "luminousFlux") (declared-name "luminousFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousReflectance"))) (name "luminousReflectance") (declared-name "luminousReflectance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::luminousTransmittance"))) (name "luminousTransmittance") (declared-name "luminousTransmittance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::massAbsorptionCoefficient"))) (name "massAbsorptionCoefficient") (declared-name "massAbsorptionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::massAttenuationCoefficient"))) (name "massAttenuationCoefficient") (declared-name "massAttenuationCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::maximumLuminousEfficacy"))) (name "maximumLuminousEfficacy") (declared-name "maximumLuminousEfficacy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::molarAbsorptionCoefficient"))) (name "molarAbsorptionCoefficient") (declared-name "molarAbsorptionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::napierianAbsorbance"))) (name "napierianAbsorbance") (declared-name "napierianAbsorbance") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::numberOfPhotons"))) (name "numberOfPhotons") (declared-name "numberOfPhotons"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::opticalDensity"))) (name "opticalDensity") (declared-name "opticalDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonEnergy"))) (name "photonEnergy") (declared-name "photonEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::photonEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::photonEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonExitance"))) (name "photonExitance") (declared-name "photonExitance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonExposure"))) (name "photonExposure") (declared-name "photonExposure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonFlux"))) (name "photonFlux") (declared-name "photonFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonIntensity"))) (name "photonIntensity") (declared-name "photonIntensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonIrradiance"))) (name "photonIrradiance") (declared-name "photonIrradiance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonNumber"))) (name "photonNumber") (declared-name "photonNumber") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::photonRadiance"))) (name "photonRadiance") (declared-name "photonRadiance") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::quantityOfIllumination"))) (name "quantityOfIllumination") (declared-name "quantityOfIllumination"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::quantityOfLight"))) (name "quantityOfLight") (declared-name "quantityOfLight"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiance"))) (name "radiance") (declared-name "radiance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radianceFactor"))) (name "radianceFactor") (declared-name "radianceFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::radiantEmittance"))) (name "radiantEmittance") (declared-name "radiantEmittance"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))) (name "radiantEnergy") (declared-name "radiantEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQLight::radiantEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQLight::radiantEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiantEnergyDensity"))) (name "radiantEnergyDensity") (declared-name "radiantEnergyDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiantExitance"))) (name "radiantExitance") (declared-name "radiantExitance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiantExposure"))) (name "radiantExposure") (declared-name "radiantExposure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiantFlux"))) (name "radiantFlux") (declared-name "radiantFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::radiantIntensity"))) (name "radiantIntensity") (declared-name "radiantIntensity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::radiantPower"))) (name "radiantPower") (declared-name "radiantPower"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::reflectance"))) (name "reflectance") (declared-name "reflectance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::reflectanceFactor"))) (name "reflectanceFactor") (declared-name "reflectanceFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::refractiveIndex"))) (name "refractiveIndex") (declared-name "refractiveIndex") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralIrradiance"))) (name "spectralIrradiance") (declared-name "spectralIrradiance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficacy"))) (name "spectralLuminousEfficacy") (declared-name "spectralLuminousEfficacy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficiency"))) (name "spectralLuminousEfficiency") (declared-name "spectralLuminousEfficiency") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiance"))) (name "spectralRadiance") (declared-name "spectralRadiance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergy"))) (name "spectralRadiantEnergy") (declared-name "spectralRadiantEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavelength"))) (name "spectralRadiantEnergyDensityInTermsOfWavelength") (declared-name "spectralRadiantEnergyDensityInTermsOfWavelength") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavenumber"))) (name "spectralRadiantEnergyDensityInTermsOfWavenumber") (declared-name "spectralRadiantEnergyDensityInTermsOfWavenumber") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantExitance"))) (name "spectralRadiantExitance") (declared-name "spectralRadiantExitance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantExposure"))) (name "spectralRadiantExposure") (declared-name "spectralRadiantExposure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantFlux"))) (name "spectralRadiantFlux") (declared-name "spectralRadiantFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantIntensity"))) (name "spectralRadiantIntensity") (declared-name "spectralRadiantIntensity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantPower"))) (name "spectralRadiantPower") (declared-name "spectralRadiantPower"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::speedOfLightInAMedium"))) (name "speedOfLightInAMedium") (declared-name "speedOfLightInAMedium") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::transmittance"))) (name "transmittance") (declared-name "transmittance") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQLight::transmittanceDensity"))) (name "transmittanceDensity") (declared-name "transmittanceDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::transmittanceOpticalDensity"))) (name "transmittanceOpticalDensity") (declared-name "transmittanceOpticalDensity") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1931StandardColorimetricObserver"))) (name "tristimulusValuesForTheCie1931StandardColorimetricObserver") (declared-name "tristimulusValuesForTheCie1931StandardColorimetricObserver") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1964StandardColorimetricObserver"))) (name "tristimulusValuesForTheCie1964StandardColorimetricObserver") (declared-name "tristimulusValuesForTheCie1964StandardColorimetricObserver") (declared (properties (ordered false) (unique false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::EmissivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::IrradianceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadianceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadianceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::ReflectanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::TransmittanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::colourTemperature::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::colourTemperature"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::photonEnergy"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiantEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (to (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::absorptance"))) (to (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem"))) (to (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem"))) (to (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver"))) (to (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver"))) (to (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::emissivity"))) (to (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::emissivityAtASpecifiedWavelength"))) (to (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::illuminance"))) (to (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::irradiance"))) (to (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::linearAbsorptionCoefficient"))) (to (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::linearAttenuationCoefficient"))) (to (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminance"))) (to (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminanceFactor"))) (to (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousAbsorptance"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfASource"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfRadiation"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousEfficiency"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousEnergy"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousExitance"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousExposure"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousFlux"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousReflectance"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::luminousTransmittance"))) (to (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::massAbsorptionCoefficient"))) (to (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::massAttenuationCoefficient"))) (to (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::maximumLuminousEfficacy"))) (to (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::molarAbsorptionCoefficient"))) (to (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::napierianAbsorbance"))) (to (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonExitance"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonExposure"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonFlux"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonIntensity"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonIrradiance"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonNumber"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::photonRadiance"))) (to (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiance"))) (to (node (document "d0") (qualified-name "ISQLight::RadianceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radianceFactor"))) (to (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiantEnergyDensity"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiantExitance"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiantExposure"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiantFlux"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::radiantIntensity"))) (to (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::reflectance"))) (to (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::reflectanceFactor"))) (to (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::refractiveIndex"))) (to (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralIrradiance"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficacy"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficiency"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiance"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergy"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavelength"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavenumber"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantExitance"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantExposure"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantFlux"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::spectralRadiantIntensity"))) (to (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::speedOfLightInAMedium"))) (to (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::transmittance"))) (to (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::transmittanceOpticalDensity"))) (to (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1931StandardColorimetricObserver"))) (to (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1964StandardColorimetricObserver"))) (to (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/isq_light.md"
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
        (range (start 20 19) (end 20 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 4) (end 23 795))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 36 8) (end 36 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 8) (end 36 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 37 8) (end 37 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 4) (end 42 370))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 8) (end 43 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 8) (end 44 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 45 8) (end 45 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 4) (end 49 974))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 4) (end 66 1036))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 4) (end 82 872))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 95 8) (end 95 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 8) (end 95 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 96 8) (end 96 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 4) (end 101 479))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 102 8) (end 102 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 8) (end 103 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 8) (end 104 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 105 8) (end 105 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 4) (end 109 983))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 122 8) (end 122 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 8) (end 122 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 123 8) (end 123 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 4) (end 128 479))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 8) (end 129 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 8) (end 130 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 8) (end 131 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 132 8) (end 132 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 4) (end 136 1181))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 149 8) (end 149 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 149 8) (end 149 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 150 8) (end 150 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 155 4) (end 155 506))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 156 8) (end 156 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 157 8) (end 157 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 8) (end 158 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 159 8) (end 159 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 828))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 176 8) (end 176 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 8) (end 176 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 177 8) (end 177 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 182 4) (end 182 392))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 8) (end 183 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 8) (end 184 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 185 8) (end 185 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 4) (end 189 836))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 202 8) (end 202 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 202 8) (end 202 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 203 8) (end 203 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 208 4) (end 208 469))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 209 8) (end 209 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 210 8) (end 210 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 211 8) (end 211 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 212 8) (end 212 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 4) (end 220 894))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 233 8) (end 233 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 233 8) (end 233 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 234 8) (end 234 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 239 4) (end 239 477))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 240 8) (end 240 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 241 8) (end 241 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 242 8) (end 242 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 243 8) (end 243 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 251 4) (end 251 1277))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 264 8) (end 264 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 264 8) (end 264 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 265 8) (end 265 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 270 4) (end 270 474))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 271 8) (end 271 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 272 8) (end 272 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 273 8) (end 273 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 274 8) (end 274 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 278 4) (end 278 890))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 291 8) (end 291 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 291 8) (end 291 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 292 8) (end 292 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 297 4) (end 297 482))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 298 8) (end 298 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 299 8) (end 299 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 300 8) (end 300 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 301 8) (end 301 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 305 4) (end 305 1155))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 318 8) (end 318 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 318 8) (end 318 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 319 8) (end 319 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 324 4) (end 324 353))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 325 8) (end 325 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 326 8) (end 326 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 327 8) (end 327 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 331 4) (end 331 1324))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 344 8) (end 344 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 344 8) (end 344 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 345 8) (end 345 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 350 4) (end 350 475))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 351 8) (end 351 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 352 8) (end 352 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 353 8) (end 353 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 354 8) (end 354 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 358 4) (end 358 1648))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 371 8) (end 371 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 371 8) (end 371 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 372 8) (end 372 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 377 4) (end 377 355))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 378 8) (end 378 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 379 8) (end 379 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 380 8) (end 380 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 384 4) (end 384 861))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 397 8) (end 397 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 397 8) (end 397 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 398 8) (end 398 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 403 4) (end 403 477))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 404 8) (end 404 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 405 8) (end 405 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 406 8) (end 406 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 407 8) (end 407 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 411 4) (end 411 1061))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 424 8) (end 424 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 424 8) (end 424 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 425 8) (end 425 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 430 4) (end 430 360))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 431 8) (end 431 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 432 8) (end 432 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 433 8) (end 433 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 4) (end 441 900))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 454 8) (end 454 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 454 8) (end 454 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 455 8) (end 455 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 460 4) (end 460 482))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 461 8) (end 461 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 462 8) (end 462 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 463 8) (end 463 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 464 8) (end 464 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 468 4) (end 468 899))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 481 8) (end 481 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 481 8) (end 481 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 482 8) (end 482 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 487 4) (end 487 360))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 488 8) (end 488 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 489 8) (end 489 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 490 8) (end 490 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 494 4) (end 494 900))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 507 8) (end 507 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 507 8) (end 507 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 508 8) (end 508 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 513 4) (end 513 482))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 514 8) (end 514 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 515 8) (end 515 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 516 8) (end 516 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 517 8) (end 517 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 521 4) (end 521 1315))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 538 4) (end 538 1449))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 555 4) (end 555 1206))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 568 8) (end 568 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 568 8) (end 568 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 569 8) (end 569 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 574 4) (end 574 621))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 575 8) (end 575 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 576 8) (end 576 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 577 8) (end 577 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 578 8) (end 578 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 579 8) (end 579 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 583 4) (end 583 1281))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 596 8) (end 596 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 596 8) (end 596 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 597 8) (end 597 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 602 4) (end 602 618))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 603 8) (end 603 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 604 8) (end 604 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 605 8) (end 605 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 606 8) (end 606 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 607 8) (end 607 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 611 4) (end 611 1322))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 624 8) (end 624 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 624 8) (end 624 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 625 8) (end 625 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 630 4) (end 630 617))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 631 8) (end 631 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 632 8) (end 632 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 633 8) (end 633 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 634 8) (end 634 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 635 8) (end 635 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 639 4) (end 639 779))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 652 8) (end 652 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 652 8) (end 652 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 653 8) (end 653 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 658 4) (end 658 619))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 659 8) (end 659 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 660 8) (end 660 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 661 8) (end 661 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 662 8) (end 662 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 663 8) (end 663 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 667 4) (end 667 1366))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 680 8) (end 680 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 680 8) (end 680 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 681 8) (end 681 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 686 4) (end 686 384))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 687 8) (end 687 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 688 8) (end 688 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 689 8) (end 689 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 697 4) (end 697 1341))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 710 8) (end 710 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 710 8) (end 710 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 711 8) (end 711 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 716 4) (end 716 263))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 717 8) (end 717 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 718 8) (end 718 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 725 4) (end 725 1405))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 738 8) (end 738 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 738 8) (end 738 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 739 8) (end 739 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 744 4) (end 744 376))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 745 8) (end 745 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 746 8) (end 746 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 747 8) (end 747 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 751 4) (end 751 1854))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 764 8) (end 764 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 764 8) (end 764 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 765 8) (end 765 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 770 4) (end 770 378))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 771 8) (end 771 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 772 8) (end 772 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 773 8) (end 773 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 777 4) (end 777 1341))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 790 8) (end 790 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 790 8) (end 790 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 791 8) (end 791 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 796 4) (end 796 383))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 797 8) (end 797 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 798 8) (end 798 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 799 8) (end 799 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 803 4) (end 803 1395))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 816 8) (end 816 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 816 8) (end 816 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 817 8) (end 817 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 822 4) (end 822 500))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 823 8) (end 823 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 824 8) (end 824 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 825 8) (end 825 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 826 8) (end 826 115))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 838 4) (end 838 832))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 857 4) (end 857 927))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 873 4) (end 873 1050))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 886 8) (end 886 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 886 8) (end 886 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 887 8) (end 887 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 892 4) (end 892 244))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 893 8) (end 893 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 894 8) (end 894 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 898 4) (end 898 1188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 911 8) (end 911 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 911 8) (end 911 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 912 8) (end 912 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 917 4) (end 917 249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 918 8) (end 918 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 919 8) (end 919 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 923 4) (end 923 987))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 936 8) (end 936 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 936 8) (end 936 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 937 8) (end 937 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 942 4) (end 942 364))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 943 8) (end 943 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 944 8) (end 944 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 945 8) (end 945 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 949 4) (end 949 879))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 962 8) (end 962 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 962 8) (end 962 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 963 8) (end 963 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 968 4) (end 968 366))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 969 8) (end 969 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 970 8) (end 970 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 971 8) (end 971 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 975 4) (end 975 879))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 988 8) (end 988 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 988 8) (end 988 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 989 8) (end 989 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 994 4) (end 994 364))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 995 8) (end 995 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 996 8) (end 996 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 997 8) (end 997 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1001 4) (end 1001 874))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1014 8) (end 1014 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1014 8) (end 1014 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1015 8) (end 1015 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1020 4) (end 1020 244))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1021 8) (end 1021 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1022 8) (end 1022 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1026 4) (end 1026 2021))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1039 8) (end 1039 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1039 8) (end 1039 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1040 8) (end 1040 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1045 4) (end 1045 425))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1046 8) (end 1046 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1047 8) (end 1047 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1048 8) (end 1048 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1052 4) (end 1052 2035))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1065 8) (end 1065 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1065 8) (end 1065 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1066 8) (end 1066 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1071 4) (end 1071 425))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1072 8) (end 1072 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1073 8) (end 1073 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1074 8) (end 1074 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1078 4) (end 1078 959))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1095 4) (end 1095 981))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1112 4) (end 1112 876))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1129 4) (end 1129 937))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1146 4) (end 1146 568))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1162 4) (end 1162 776))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1178 4) (end 1178 732))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1195 4) (end 1195 925))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1212 4) (end 1212 890))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1229 4) (end 1229 982))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1246 4) (end 1246 888))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1263 4) (end 1263 987))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1280 4) (end 1280 901))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1297 4) (end 1297 1026))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1314 4) (end 1314 854))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1337 4) (end 1337 793))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1354 4) (end 1354 1502))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1371 4) (end 1371 1160))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1388 4) (end 1388 1709))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1405 4) (end 1405 1126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1418 8) (end 1418 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1418 8) (end 1418 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1419 8) (end 1419 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1424 4) (end 1424 258))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1425 8) (end 1425 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1426 8) (end 1426 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1434 4) (end 1434 1341))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1447 8) (end 1447 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1447 8) (end 1447 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1448 8) (end 1448 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1453 4) (end 1453 257))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1454 8) (end 1454 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1455 8) (end 1455 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1459 4) (end 1459 901))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1472 8) (end 1472 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1472 8) (end 1472 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1473 8) (end 1473 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1478 4) (end 1478 367))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1479 8) (end 1479 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1480 8) (end 1480 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1481 8) (end 1481 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1485 4) (end 1485 896))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1498 8) (end 1498 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1498 8) (end 1498 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1499 8) (end 1499 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1504 4) (end 1504 366))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1505 8) (end 1505 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1506 8) (end 1506 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1507 8) (end 1507 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1511 4) (end 1511 910))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1524 8) (end 1524 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1524 8) (end 1524 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1525 8) (end 1525 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1530 4) (end 1530 393))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1531 8) (end 1531 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1532 8) (end 1532 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1533 8) (end 1533 103))
      )
    )
  )
)
~~~
