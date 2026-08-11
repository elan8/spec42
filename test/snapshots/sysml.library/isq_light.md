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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_light.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 4) (end 49 974))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 4) (end 82 872))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 4) (end 109 983))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 4) (end 136 1181))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 828))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 4) (end 189 836))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 4) (end 220 894))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 251 4) (end 251 1277))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 278 4) (end 278 890))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 305 4) (end 305 1155))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 331 4) (end 331 1324))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 358 4) (end 358 1648))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 384 4) (end 384 861))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 411 4) (end 411 1061))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 4) (end 441 900))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 468 4) (end 468 899))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 494 4) (end 494 900))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 583 4) (end 583 1281))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 611 4) (end 611 1322))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 639 4) (end 639 779))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 667 4) (end 667 1366))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 697 4) (end 697 1341))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 725 4) (end 725 1405))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 751 4) (end 751 1854))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 777 4) (end 777 1341))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 803 4) (end 803 1395))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 838 4) (end 838 832))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 873 4) (end 873 1050))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 898 4) (end 898 1188))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 923 4) (end 923 987))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 949 4) (end 949 879))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 975 4) (end 975 879))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1001 4) (end 1001 874))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1026 4) (end 1026 2021))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1052 4) (end 1052 2035))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1434 4) (end 1434 1341))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1459 4) (end 1459 901))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1485 4) (end 1485 896))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1511 4) (end 1511 910))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "23d6e2488ab7d1a60b8bd75ba622527bab7a30e2108c49f3bdb0aa58d220a42c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQLight"))) (kind "package") (name "ISQLight") (declared-name "ISQLight") (range (start (line 0) (character 0)) (end (line 0) (character 100170))))
    (element (id (node (document "d0") (qualified-name "ISQLight::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))) (kind "attribute def") (name "AbsorptanceValue") (declared-name "AbsorptanceValue") (range (start (line 1212) (character 4)) (end (line 1212) (character 890))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1212) (character 4)) (end (line 1212) (character 890))) (parent (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))) (kind "attribute def") (name "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (declared-name "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (range (start (line 1112) (character 4)) (end (line 1112) (character 876))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1112) (character 4)) (end (line 1112) (character 876))) (parent (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))) (kind "attribute def") (name "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (declared-name "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (range (start (line 1129) (character 4)) (end (line 1129) (character 937))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1129) (character 4)) (end (line 1129) (character 937))) (parent (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))) (kind "attribute def") (name "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (declared-name "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (range (start (line 1078) (character 4)) (end (line 1078) (character 959))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1078) (character 4)) (end (line 1078) (character 959))) (parent (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))) (kind "attribute def") (name "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (declared-name "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (range (start (line 1095) (character 4)) (end (line 1095) (character 981))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1095) (character 4)) (end (line 1095) (character 981))) (parent (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))) (kind "attribute def") (name "EmissivityAtASpecifiedWavelengthValue") (declared-name "EmissivityAtASpecifiedWavelengthValue") (range (start (line 1195) (character 4)) (end (line 1195) (character 925))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1195) (character 4)) (end (line 1195) (character 925))) (parent (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))) (kind "attribute def") (name "EmissivityValue") (declared-name "EmissivityValue") (range (start (line 1178) (character 4)) (end (line 1178) (character 732))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::EmissivityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1178) (character 4)) (end (line 1178) (character 732))) (parent (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::EnergyValue"))) (kind "import") (name "EnergyValue") (declared-name "EnergyValue") (range (start (line 20) (character 4)) (end (line 20) (character 50))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQThermodynamics::EnergyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 19)) (end (line 20) (character 49))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (kind "attribute def") (name "IlluminanceUnit") (declared-name "IlluminanceUnit") (range (start (line 770) (character 4)) (end (line 770) (character 378))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 771) (character 8)) (end (line 771) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 772) (character 8)) (end (line 772) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 773) (character 8)) (end (line 773) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 773) (character 22)) (end (line 773) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))) (kind "attribute def") (name "IlluminanceValue") (declared-name "IlluminanceValue") (range (start (line 751) (character 4)) (end (line 751) (character 1854))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 751) (character 4)) (end (line 751) (character 1854))) (parent (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 765) (character 8)) (end (line 765) (character 47))) (parent (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IlluminanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 765) (character 22)) (end (line 765) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 764) (character 8)) (end (line 764) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 764) (character 22)) (end (line 764) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (kind "attribute def") (name "IrradianceUnit") (declared-name "IrradianceUnit") (range (start (line 377) (character 4)) (end (line 377) (character 355))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 379) (character 8)) (end (line 379) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 378) (character 8)) (end (line 378) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 380) (character 8)) (end (line 380) (character 92))) (parent (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 380) (character 22)) (end (line 380) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))) (kind "attribute def") (name "IrradianceValue") (declared-name "IrradianceValue") (range (start (line 358) (character 4)) (end (line 358) (character 1648))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 358) (character 4)) (end (line 358) (character 1648))) (parent (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 372) (character 8)) (end (line 372) (character 46))) (parent (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IrradianceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 372) (character 22)) (end (line 372) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 371) (character 8)) (end (line 371) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 371) (character 22)) (end (line 371) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LightExposureUnit"))) (kind "alias") (name "LightExposureUnit") (declared-name "LightExposureUnit") (range (start (line 833) (character 4)) (end (line 833) (character 53))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LightExposureValue"))) (kind "alias") (name "LightExposureValue") (declared-name "LightExposureValue") (range (start (line 834) (character 4)) (end (line 834) (character 55))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))) (kind "attribute def") (name "LinearAbsorptionCoefficientUnit") (declared-name "LinearAbsorptionCoefficientUnit") (range (start (line 1453) (character 4)) (end (line 1453) (character 257))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1454) (character 8)) (end (line 1454) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1455) (character 8)) (end (line 1455) (character 80))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1455) (character 22)) (end (line 1455) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))) (kind "attribute def") (name "LinearAbsorptionCoefficientValue") (declared-name "LinearAbsorptionCoefficientValue") (range (start (line 1434) (character 4)) (end (line 1434) (character 1341))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1434) (character 4)) (end (line 1434) (character 1341))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1448) (character 8)) (end (line 1448) (character 63))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LinearAbsorptionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1448) (character 22)) (end (line 1448) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1447) (character 8)) (end (line 1447) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1447) (character 22)) (end (line 1447) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))) (kind "attribute def") (name "LinearAttenuationCoefficientUnit") (declared-name "LinearAttenuationCoefficientUnit") (range (start (line 1424) (character 4)) (end (line 1424) (character 258))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1425) (character 8)) (end (line 1425) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1426) (character 8)) (end (line 1426) (character 80))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1426) (character 22)) (end (line 1426) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))) (kind "attribute def") (name "LinearAttenuationCoefficientValue") (declared-name "LinearAttenuationCoefficientValue") (range (start (line 1405) (character 4)) (end (line 1405) (character 1126))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1405) (character 4)) (end (line 1405) (character 1126))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1419) (character 8)) (end (line 1419) (character 64))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LinearAttenuationCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1419) (character 22)) (end (line 1419) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1418) (character 8)) (end (line 1418) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1418) (character 22)) (end (line 1418) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearExtinctionCoefficientUnit"))) (kind "alias") (name "LinearExtinctionCoefficientUnit") (declared-name "LinearExtinctionCoefficientUnit") (range (start (line 1429) (character 4)) (end (line 1429) (character 79))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LinearExtinctionCoefficientValue"))) (kind "alias") (name "LinearExtinctionCoefficientValue") (declared-name "LinearExtinctionCoefficientValue") (range (start (line 1430) (character 4)) (end (line 1430) (character 81))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))) (kind "attribute def") (name "LuminanceFactorValue") (declared-name "LuminanceFactorValue") (range (start (line 1371) (character 4)) (end (line 1371) (character 1160))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1371) (character 4)) (end (line 1371) (character 1160))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (kind "attribute def") (name "LuminanceUnit") (declared-name "LuminanceUnit") (range (start (line 744) (character 4)) (end (line 744) (character 376))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 745) (character 8)) (end (line 745) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 746) (character 8)) (end (line 746) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 747) (character 8)) (end (line 747) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 747) (character 22)) (end (line 747) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))) (kind "attribute def") (name "LuminanceValue") (declared-name "LuminanceValue") (range (start (line 725) (character 4)) (end (line 725) (character 1405))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 725) (character 4)) (end (line 725) (character 1405))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 739) (character 8)) (end (line 739) (character 45))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 739) (character 22)) (end (line 739) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 738) (character 8)) (end (line 738) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 738) (character 22)) (end (line 738) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))) (kind "attribute def") (name "LuminousAbsorptanceValue") (declared-name "LuminousAbsorptanceValue") (range (start (line 1229) (character 4)) (end (line 1229) (character 982))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1229) (character 4)) (end (line 1229) (character 982))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (kind "attribute def") (name "LuminousEfficacyOfASourceUnit") (declared-name "LuminousEfficacyOfASourceUnit") (range (start (line 658) (character 4)) (end (line 658) (character 619))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 661) (character 8)) (end (line 661) (character 104))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 659) (character 8)) (end (line 659) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 662) (character 8)) (end (line 662) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 660) (character 8)) (end (line 660) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 663) (character 8)) (end (line 663) (character 123))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 663) (character 22)) (end (line 663) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))) (kind "attribute def") (name "LuminousEfficacyOfASourceValue") (declared-name "LuminousEfficacyOfASourceValue") (range (start (line 639) (character 4)) (end (line 639) (character 779))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 639) (character 4)) (end (line 639) (character 779))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 653) (character 8)) (end (line 653) (character 61))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousEfficacyOfASourceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 653) (character 22)) (end (line 653) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 652) (character 8)) (end (line 652) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 652) (character 22)) (end (line 652) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (kind "attribute def") (name "LuminousEfficacyOfRadiationUnit") (declared-name "LuminousEfficacyOfRadiationUnit") (range (start (line 574) (character 4)) (end (line 574) (character 621))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 577) (character 8)) (end (line 577) (character 104))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 575) (character 8)) (end (line 575) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 578) (character 8)) (end (line 578) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 576) (character 8)) (end (line 576) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 579) (character 8)) (end (line 579) (character 123))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 579) (character 22)) (end (line 579) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))) (kind "attribute def") (name "LuminousEfficacyOfRadiationValue") (declared-name "LuminousEfficacyOfRadiationValue") (range (start (line 555) (character 4)) (end (line 555) (character 1206))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 555) (character 4)) (end (line 555) (character 1206))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 569) (character 8)) (end (line 569) (character 63))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousEfficacyOfRadiationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 569) (character 22)) (end (line 569) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 568) (character 8)) (end (line 568) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 568) (character 22)) (end (line 568) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))) (kind "attribute def") (name "LuminousEfficiencyValue") (declared-name "LuminousEfficiencyValue") (range (start (line 521) (character 4)) (end (line 521) (character 1315))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 521) (character 4)) (end (line 521) (character 1315))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (kind "attribute def") (name "LuminousEnergyUnit") (declared-name "LuminousEnergyUnit") (range (start (line 686) (character 4)) (end (line 686) (character 384))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 687) (character 8)) (end (line 687) (character 104))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 688) (character 8)) (end (line 688) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 689) (character 8)) (end (line 689) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 689) (character 22)) (end (line 689) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))) (kind "attribute def") (name "LuminousEnergyValue") (declared-name "LuminousEnergyValue") (range (start (line 667) (character 4)) (end (line 667) (character 1366))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 667) (character 4)) (end (line 667) (character 1366))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 681) (character 8)) (end (line 681) (character 50))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousEnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 681) (character 22)) (end (line 681) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 680) (character 8)) (end (line 680) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 680) (character 22)) (end (line 680) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (kind "attribute def") (name "LuminousExitanceUnit") (declared-name "LuminousExitanceUnit") (range (start (line 796) (character 4)) (end (line 796) (character 383))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 797) (character 8)) (end (line 797) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 798) (character 8)) (end (line 798) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 799) (character 8)) (end (line 799) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 799) (character 22)) (end (line 799) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))) (kind "attribute def") (name "LuminousExitanceValue") (declared-name "LuminousExitanceValue") (range (start (line 777) (character 4)) (end (line 777) (character 1341))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 777) (character 4)) (end (line 777) (character 1341))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 791) (character 8)) (end (line 791) (character 52))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousExitanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 791) (character 22)) (end (line 791) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 790) (character 8)) (end (line 790) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 790) (character 22)) (end (line 790) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (kind "attribute def") (name "LuminousExposureUnit") (declared-name "LuminousExposureUnit") (range (start (line 822) (character 4)) (end (line 822) (character 500))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 824) (character 8)) (end (line 824) (character 104))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 823) (character 8)) (end (line 823) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 825) (character 8)) (end (line 825) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 826) (character 8)) (end (line 826) (character 115))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 826) (character 22)) (end (line 826) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))) (kind "attribute def") (name "LuminousExposureValue") (declared-name "LuminousExposureValue") (range (start (line 803) (character 4)) (end (line 803) (character 1395))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 803) (character 4)) (end (line 803) (character 1395))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 817) (character 8)) (end (line 817) (character 52))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousExposureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 817) (character 22)) (end (line 817) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 816) (character 8)) (end (line 816) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 816) (character 22)) (end (line 816) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))) (kind "attribute def") (name "LuminousFluxUnit") (declared-name "LuminousFluxUnit") (range (start (line 716) (character 4)) (end (line 716) (character 263))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 717) (character 8)) (end (line 717) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 718) (character 8)) (end (line 718) (character 91))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 718) (character 22)) (end (line 718) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))) (kind "attribute def") (name "LuminousFluxValue") (declared-name "LuminousFluxValue") (range (start (line 697) (character 4)) (end (line 697) (character 1341))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::_documentation"))) (kind "documentation") (name "") (range (start (line 697) (character 4)) (end (line 697) (character 1341))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 711) (character 8)) (end (line 711) (character 48))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousFluxUnit") (range none)) (redefinition (reference "mRef") (range (start (line 711) (character 22)) (end (line 711) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 710) (character 8)) (end (line 710) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 710) (character 22)) (end (line 710) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))) (kind "attribute def") (name "LuminousReflectanceValue") (declared-name "LuminousReflectanceValue") (range (start (line 1263) (character 4)) (end (line 1263) (character 987))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1263) (character 4)) (end (line 1263) (character 987))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))) (kind "attribute def") (name "LuminousTransmittanceValue") (declared-name "LuminousTransmittanceValue") (range (start (line 1297) (character 4)) (end (line 1297) (character 1026))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1297) (character 4)) (end (line 1297) (character 1026))) (parent (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (kind "attribute def") (name "MassAbsorptionCoefficientUnit") (declared-name "MassAbsorptionCoefficientUnit") (range (start (line 1504) (character 4)) (end (line 1504) (character 366))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1505) (character 8)) (end (line 1505) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1506) (character 8)) (end (line 1506) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1507) (character 8)) (end (line 1507) (character 90))) (parent (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1507) (character 22)) (end (line 1507) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))) (kind "attribute def") (name "MassAbsorptionCoefficientValue") (declared-name "MassAbsorptionCoefficientValue") (range (start (line 1485) (character 4)) (end (line 1485) (character 896))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1485) (character 4)) (end (line 1485) (character 896))) (parent (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1499) (character 8)) (end (line 1499) (character 61))) (parent (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassAbsorptionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1499) (character 22)) (end (line 1499) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1498) (character 8)) (end (line 1498) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1498) (character 22)) (end (line 1498) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (kind "attribute def") (name "MassAttenuationCoefficientUnit") (declared-name "MassAttenuationCoefficientUnit") (range (start (line 1478) (character 4)) (end (line 1478) (character 367))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1479) (character 8)) (end (line 1479) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1480) (character 8)) (end (line 1480) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1481) (character 8)) (end (line 1481) (character 90))) (parent (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1481) (character 22)) (end (line 1481) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))) (kind "attribute def") (name "MassAttenuationCoefficientValue") (declared-name "MassAttenuationCoefficientValue") (range (start (line 1459) (character 4)) (end (line 1459) (character 901))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1459) (character 4)) (end (line 1459) (character 901))) (parent (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1473) (character 8)) (end (line 1473) (character 62))) (parent (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassAttenuationCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1473) (character 22)) (end (line 1473) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1472) (character 8)) (end (line 1472) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1472) (character 22)) (end (line 1472) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (kind "attribute def") (name "MaximumLuminousEfficacyUnit") (declared-name "MaximumLuminousEfficacyUnit") (range (start (line 630) (character 4)) (end (line 630) (character 617))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 633) (character 8)) (end (line 633) (character 104))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 631) (character 8)) (end (line 631) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 634) (character 8)) (end (line 634) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 632) (character 8)) (end (line 632) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 635) (character 8)) (end (line 635) (character 123))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 635) (character 22)) (end (line 635) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))) (kind "attribute def") (name "MaximumLuminousEfficacyValue") (declared-name "MaximumLuminousEfficacyValue") (range (start (line 611) (character 4)) (end (line 611) (character 1322))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 611) (character 4)) (end (line 611) (character 1322))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 625) (character 8)) (end (line 625) (character 59))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MaximumLuminousEfficacyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 625) (character 22)) (end (line 625) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 624) (character 8)) (end (line 624) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 624) (character 22)) (end (line 624) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (kind "attribute def") (name "MolarAbsorptionCoefficientUnit") (declared-name "MolarAbsorptionCoefficientUnit") (range (start (line 1530) (character 4)) (end (line 1530) (character 393))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (range (start (line 1532) (character 8)) (end (line 1532) (character 114))) (parent (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1531) (character 8)) (end (line 1531) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1533) (character 8)) (end (line 1533) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1533) (character 22)) (end (line 1533) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))) (kind "attribute def") (name "MolarAbsorptionCoefficientValue") (declared-name "MolarAbsorptionCoefficientValue") (range (start (line 1511) (character 4)) (end (line 1511) (character 910))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1511) (character 4)) (end (line 1511) (character 910))) (parent (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1525) (character 8)) (end (line 1525) (character 62))) (parent (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MolarAbsorptionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1525) (character 22)) (end (line 1525) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1524) (character 8)) (end (line 1524) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1524) (character 22)) (end (line 1524) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))) (kind "attribute def") (name "NapierianAbsorbanceValue") (declared-name "NapierianAbsorbanceValue") (range (start (line 1337) (character 4)) (end (line 1337) (character 793))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1337) (character 4)) (end (line 1337) (character 793))) (parent (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (kind "attribute def") (name "PhotonExitanceUnit") (declared-name "PhotonExitanceUnit") (range (start (line 994) (character 4)) (end (line 994) (character 364))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 996) (character 8)) (end (line 996) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 995) (character 8)) (end (line 995) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 997) (character 8)) (end (line 997) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 997) (character 22)) (end (line 997) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))) (kind "attribute def") (name "PhotonExitanceValue") (declared-name "PhotonExitanceValue") (range (start (line 975) (character 4)) (end (line 975) (character 879))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 975) (character 4)) (end (line 975) (character 879))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 989) (character 8)) (end (line 989) (character 50))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhotonExitanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 989) (character 22)) (end (line 989) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 988) (character 8)) (end (line 988) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 988) (character 22)) (end (line 988) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))) (kind "attribute def") (name "PhotonExposureUnit") (declared-name "PhotonExposureUnit") (range (start (line 1020) (character 4)) (end (line 1020) (character 244))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1021) (character 8)) (end (line 1021) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1022) (character 8)) (end (line 1022) (character 80))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1022) (character 22)) (end (line 1022) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))) (kind "attribute def") (name "PhotonExposureValue") (declared-name "PhotonExposureValue") (range (start (line 1001) (character 4)) (end (line 1001) (character 874))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1001) (character 4)) (end (line 1001) (character 874))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1015) (character 8)) (end (line 1015) (character 50))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhotonExposureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1015) (character 22)) (end (line 1015) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1014) (character 8)) (end (line 1014) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1014) (character 22)) (end (line 1014) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))) (kind "attribute def") (name "PhotonFluxUnit") (declared-name "PhotonFluxUnit") (range (start (line 892) (character 4)) (end (line 892) (character 244))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 893) (character 8)) (end (line 893) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 894) (character 8)) (end (line 894) (character 82))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 894) (character 22)) (end (line 894) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))) (kind "attribute def") (name "PhotonFluxValue") (declared-name "PhotonFluxValue") (range (start (line 873) (character 4)) (end (line 873) (character 1050))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::_documentation"))) (kind "documentation") (name "") (range (start (line 873) (character 4)) (end (line 873) (character 1050))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 887) (character 8)) (end (line 887) (character 46))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhotonFluxUnit") (range none)) (redefinition (reference "mRef") (range (start (line 887) (character 22)) (end (line 887) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 886) (character 8)) (end (line 886) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 886) (character 22)) (end (line 886) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))) (kind "attribute def") (name "PhotonIntensityUnit") (declared-name "PhotonIntensityUnit") (range (start (line 917) (character 4)) (end (line 917) (character 249))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 918) (character 8)) (end (line 918) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 919) (character 8)) (end (line 919) (character 82))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 919) (character 22)) (end (line 919) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))) (kind "attribute def") (name "PhotonIntensityValue") (declared-name "PhotonIntensityValue") (range (start (line 898) (character 4)) (end (line 898) (character 1188))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 898) (character 4)) (end (line 898) (character 1188))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 912) (character 8)) (end (line 912) (character 51))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhotonIntensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 912) (character 22)) (end (line 912) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 911) (character 8)) (end (line 911) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 911) (character 22)) (end (line 911) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (kind "attribute def") (name "PhotonIrradianceUnit") (declared-name "PhotonIrradianceUnit") (range (start (line 968) (character 4)) (end (line 968) (character 366))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 970) (character 8)) (end (line 970) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 969) (character 8)) (end (line 969) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 971) (character 8)) (end (line 971) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 971) (character 22)) (end (line 971) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))) (kind "attribute def") (name "PhotonIrradianceValue") (declared-name "PhotonIrradianceValue") (range (start (line 949) (character 4)) (end (line 949) (character 879))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 949) (character 4)) (end (line 949) (character 879))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 963) (character 8)) (end (line 963) (character 52))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhotonIrradianceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 963) (character 22)) (end (line 963) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 962) (character 8)) (end (line 962) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 962) (character 22)) (end (line 962) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))) (kind "attribute def") (name "PhotonNumberValue") (declared-name "PhotonNumberValue") (range (start (line 838) (character 4)) (end (line 838) (character 832))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 838) (character 4)) (end (line 838) (character 832))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (kind "attribute def") (name "PhotonRadianceUnit") (declared-name "PhotonRadianceUnit") (range (start (line 942) (character 4)) (end (line 942) (character 364))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 944) (character 8)) (end (line 944) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 943) (character 8)) (end (line 943) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 945) (character 8)) (end (line 945) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 945) (character 22)) (end (line 945) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))) (kind "attribute def") (name "PhotonRadianceValue") (declared-name "PhotonRadianceValue") (range (start (line 923) (character 4)) (end (line 923) (character 987))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 923) (character 4)) (end (line 923) (character 987))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 937) (character 8)) (end (line 937) (character 50))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhotonRadianceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 937) (character 22)) (end (line 937) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 936) (character 8)) (end (line 936) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 936) (character 22)) (end (line 936) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::QuantityOfIlluminationUnit"))) (kind "alias") (name "QuantityOfIlluminationUnit") (declared-name "QuantityOfIlluminationUnit") (range (start (line 829) (character 4)) (end (line 829) (character 62))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::QuantityOfIlluminationValue"))) (kind "alias") (name "QuantityOfIlluminationValue") (declared-name "QuantityOfIlluminationValue") (range (start (line 830) (character 4)) (end (line 830) (character 64))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::QuantityOfLightUnit"))) (kind "alias") (name "QuantityOfLightUnit") (declared-name "QuantityOfLightUnit") (range (start (line 692) (character 4)) (end (line 692) (character 53))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::QuantityOfLightValue"))) (kind "alias") (name "QuantityOfLightValue") (declared-name "QuantityOfLightValue") (range (start (line 693) (character 4)) (end (line 693) (character 55))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))) (kind "attribute def") (name "RadianceFactorValue") (declared-name "RadianceFactorValue") (range (start (line 1354) (character 4)) (end (line 1354) (character 1502))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1354) (character 4)) (end (line 1354) (character 1502))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (kind "attribute def") (name "RadianceUnit") (declared-name "RadianceUnit") (range (start (line 324) (character 4)) (end (line 324) (character 353))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 326) (character 8)) (end (line 326) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 325) (character 8)) (end (line 325) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 327) (character 8)) (end (line 327) (character 92))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 327) (character 22)) (end (line 327) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceValue"))) (kind "attribute def") (name "RadianceValue") (declared-name "RadianceValue") (range (start (line 305) (character 4)) (end (line 305) (character 1155))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 305) (character 4)) (end (line 305) (character 1155))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 319) (character 8)) (end (line 319) (character 44))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RadianceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 319) (character 22)) (end (line 319) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 318) (character 8)) (end (line 318) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::RadianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 318) (character 22)) (end (line 318) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEmittanceUnit"))) (kind "alias") (name "RadiantEmittanceUnit") (declared-name "RadiantEmittanceUnit") (range (start (line 436) (character 4)) (end (line 436) (character 55))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEmittanceValue"))) (kind "alias") (name "RadiantEmittanceValue") (declared-name "RadiantEmittanceValue") (range (start (line 437) (character 4)) (end (line 437) (character 57))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (kind "attribute def") (name "RadiantEnergyDensityUnit") (declared-name "RadiantEnergyDensityUnit") (range (start (line 128) (character 4)) (end (line 128) (character 479))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 131) (character 8)) (end (line 131) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 129) (character 8)) (end (line 129) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 130) (character 8)) (end (line 130) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 132) (character 8)) (end (line 132) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 132) (character 22)) (end (line 132) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))) (kind "attribute def") (name "RadiantEnergyDensityValue") (declared-name "RadiantEnergyDensityValue") (range (start (line 109) (character 4)) (end (line 109) (character 983))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 109) (character 4)) (end (line 109) (character 983))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 123) (character 8)) (end (line 123) (character 56))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RadiantEnergyDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 123) (character 22)) (end (line 123) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 122) (character 8)) (end (line 122) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 122) (character 22)) (end (line 122) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (kind "attribute def") (name "RadiantExitanceUnit") (declared-name "RadiantExitanceUnit") (range (start (line 430) (character 4)) (end (line 430) (character 360))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 432) (character 8)) (end (line 432) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 431) (character 8)) (end (line 431) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 433) (character 8)) (end (line 433) (character 92))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 433) (character 22)) (end (line 433) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))) (kind "attribute def") (name "RadiantExitanceValue") (declared-name "RadiantExitanceValue") (range (start (line 411) (character 4)) (end (line 411) (character 1061))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 411) (character 4)) (end (line 411) (character 1061))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 425) (character 8)) (end (line 425) (character 51))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RadiantExitanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 425) (character 22)) (end (line 425) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 424) (character 8)) (end (line 424) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 424) (character 22)) (end (line 424) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (kind "attribute def") (name "RadiantExposureUnit") (declared-name "RadiantExposureUnit") (range (start (line 487) (character 4)) (end (line 487) (character 360))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 489) (character 8)) (end (line 489) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 488) (character 8)) (end (line 488) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 490) (character 8)) (end (line 490) (character 92))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 490) (character 22)) (end (line 490) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))) (kind "attribute def") (name "RadiantExposureValue") (declared-name "RadiantExposureValue") (range (start (line 468) (character 4)) (end (line 468) (character 899))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 468) (character 4)) (end (line 468) (character 899))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 482) (character 8)) (end (line 482) (character 51))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RadiantExposureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 482) (character 22)) (end (line 482) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 481) (character 8)) (end (line 481) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 481) (character 22)) (end (line 481) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (kind "attribute def") (name "RadiantFluxUnit") (declared-name "RadiantFluxUnit") (range (start (line 208) (character 4)) (end (line 208) (character 469))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 211) (character 8)) (end (line 211) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 209) (character 8)) (end (line 209) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 210) (character 8)) (end (line 210) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 212) (character 8)) (end (line 212) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 212) (character 22)) (end (line 212) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))) (kind "attribute def") (name "RadiantFluxValue") (declared-name "RadiantFluxValue") (range (start (line 189) (character 4)) (end (line 189) (character 836))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::_documentation"))) (kind "documentation") (name "") (range (start (line 189) (character 4)) (end (line 189) (character 836))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 203) (character 8)) (end (line 203) (character 47))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RadiantFluxUnit") (range none)) (redefinition (reference "mRef") (range (start (line 203) (character 22)) (end (line 203) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 202) (character 8)) (end (line 202) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 202) (character 22)) (end (line 202) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (kind "attribute def") (name "RadiantIntensityUnit") (declared-name "RadiantIntensityUnit") (range (start (line 270) (character 4)) (end (line 270) (character 474))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 273) (character 8)) (end (line 273) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 271) (character 8)) (end (line 271) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 272) (character 8)) (end (line 272) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 274) (character 8)) (end (line 274) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 274) (character 22)) (end (line 274) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))) (kind "attribute def") (name "RadiantIntensityValue") (declared-name "RadiantIntensityValue") (range (start (line 251) (character 4)) (end (line 251) (character 1277))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 251) (character 4)) (end (line 251) (character 1277))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 265) (character 8)) (end (line 265) (character 52))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RadiantIntensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 265) (character 22)) (end (line 265) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 264) (character 8)) (end (line 264) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 264) (character 22)) (end (line 264) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantPowerUnit"))) (kind "alias") (name "RadiantPowerUnit") (declared-name "RadiantPowerUnit") (range (start (line 215) (character 4)) (end (line 215) (character 47))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RadiantPowerValue"))) (kind "alias") (name "RadiantPowerValue") (declared-name "RadiantPowerValue") (range (start (line 216) (character 4)) (end (line 216) (character 49))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))) (kind "attribute def") (name "ReflectanceFactorValue") (declared-name "ReflectanceFactorValue") (range (start (line 1388) (character 4)) (end (line 1388) (character 1709))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1388) (character 4)) (end (line 1388) (character 1709))) (parent (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))) (kind "attribute def") (name "ReflectanceValue") (declared-name "ReflectanceValue") (range (start (line 1246) (character 4)) (end (line 1246) (character 888))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::ReflectanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1246) (character 4)) (end (line 1246) (character 888))) (parent (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))) (kind "attribute def") (name "RefractiveIndexValue") (declared-name "RefractiveIndexValue") (range (start (line 49) (character 4)) (end (line 49) (character 974))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue::_documentation"))) (kind "documentation") (name "") (range (start (line 49) (character 4)) (end (line 49) (character 974))) (parent (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (kind "attribute def") (name "SpectralIrradianceUnit") (declared-name "SpectralIrradianceUnit") (range (start (line 403) (character 4)) (end (line 403) (character 477))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 406) (character 8)) (end (line 406) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 404) (character 8)) (end (line 404) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 405) (character 8)) (end (line 405) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 407) (character 8)) (end (line 407) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 407) (character 22)) (end (line 407) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))) (kind "attribute def") (name "SpectralIrradianceValue") (declared-name "SpectralIrradianceValue") (range (start (line 384) (character 4)) (end (line 384) (character 861))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 384) (character 4)) (end (line 384) (character 861))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 398) (character 8)) (end (line 398) (character 54))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralIrradianceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 398) (character 22)) (end (line 398) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 397) (character 8)) (end (line 397) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 397) (character 22)) (end (line 397) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (kind "attribute def") (name "SpectralLuminousEfficacyUnit") (declared-name "SpectralLuminousEfficacyUnit") (range (start (line 602) (character 4)) (end (line 602) (character 618))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 605) (character 8)) (end (line 605) (character 104))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 603) (character 8)) (end (line 603) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 606) (character 8)) (end (line 606) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 604) (character 8)) (end (line 604) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 607) (character 8)) (end (line 607) (character 123))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 607) (character 22)) (end (line 607) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))) (kind "attribute def") (name "SpectralLuminousEfficacyValue") (declared-name "SpectralLuminousEfficacyValue") (range (start (line 583) (character 4)) (end (line 583) (character 1281))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 583) (character 4)) (end (line 583) (character 1281))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 597) (character 8)) (end (line 597) (character 60))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralLuminousEfficacyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 597) (character 22)) (end (line 597) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 596) (character 8)) (end (line 596) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 596) (character 22)) (end (line 596) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))) (kind "attribute def") (name "SpectralLuminousEfficiencyValue") (declared-name "SpectralLuminousEfficiencyValue") (range (start (line 538) (character 4)) (end (line 538) (character 1449))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 538) (character 4)) (end (line 538) (character 1449))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (kind "attribute def") (name "SpectralRadianceUnit") (declared-name "SpectralRadianceUnit") (range (start (line 350) (character 4)) (end (line 350) (character 475))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 353) (character 8)) (end (line 353) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 351) (character 8)) (end (line 351) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 352) (character 8)) (end (line 352) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 354) (character 8)) (end (line 354) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 354) (character 22)) (end (line 354) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))) (kind "attribute def") (name "SpectralRadianceValue") (declared-name "SpectralRadianceValue") (range (start (line 331) (character 4)) (end (line 331) (character 1324))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 331) (character 4)) (end (line 331) (character 1324))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 345) (character 8)) (end (line 345) (character 52))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadianceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 345) (character 22)) (end (line 345) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 344) (character 8)) (end (line 344) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 344) (character 22)) (end (line 344) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (kind "attribute def") (name "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range (start (line 155) (character 4)) (end (line 155) (character 506))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 158) (character 8)) (end (line 158) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 156) (character 8)) (end (line 156) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 157) (character 8)) (end (line 157) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 159) (character 8)) (end (line 159) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 159) (character 22)) (end (line 159) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))) (kind "attribute def") (name "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (range (start (line 136) (character 4)) (end (line 136) (character 1181))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::_documentation"))) (kind "documentation") (name "") (range (start (line 136) (character 4)) (end (line 136) (character 1181))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 150) (character 8)) (end (line 150) (character 83))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range none)) (redefinition (reference "mRef") (range (start (line 150) (character 22)) (end (line 150) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 149) (character 8)) (end (line 149) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 149) (character 22)) (end (line 149) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (kind "attribute def") (name "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (range (start (line 182) (character 4)) (end (line 182) (character 392))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 184) (character 8)) (end (line 184) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 183) (character 8)) (end (line 183) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 185) (character 8)) (end (line 185) (character 92))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 185) (character 22)) (end (line 185) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))) (kind "attribute def") (name "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (declared-name "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (range (start (line 163) (character 4)) (end (line 163) (character 828))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 163) (character 4)) (end (line 163) (character 828))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 177) (character 8)) (end (line 177) (character 83))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (range none)) (redefinition (reference "mRef") (range (start (line 177) (character 22)) (end (line 177) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 176) (character 8)) (end (line 176) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 176) (character 22)) (end (line 176) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (kind "attribute def") (name "SpectralRadiantEnergyUnit") (declared-name "SpectralRadiantEnergyUnit") (range (start (line 101) (character 4)) (end (line 101) (character 479))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 104) (character 8)) (end (line 104) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 102) (character 8)) (end (line 102) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 103) (character 8)) (end (line 103) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 105) (character 8)) (end (line 105) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 105) (character 22)) (end (line 105) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))) (kind "attribute def") (name "SpectralRadiantEnergyValue") (declared-name "SpectralRadiantEnergyValue") (range (start (line 82) (character 4)) (end (line 82) (character 872))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 82) (character 4)) (end (line 82) (character 872))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 96) (character 8)) (end (line 96) (character 57))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantEnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 96) (character 22)) (end (line 96) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 95) (character 8)) (end (line 95) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 95) (character 22)) (end (line 95) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (kind "attribute def") (name "SpectralRadiantExitanceUnit") (declared-name "SpectralRadiantExitanceUnit") (range (start (line 460) (character 4)) (end (line 460) (character 482))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 463) (character 8)) (end (line 463) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 461) (character 8)) (end (line 461) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 462) (character 8)) (end (line 462) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 464) (character 8)) (end (line 464) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 464) (character 22)) (end (line 464) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))) (kind "attribute def") (name "SpectralRadiantExitanceValue") (declared-name "SpectralRadiantExitanceValue") (range (start (line 441) (character 4)) (end (line 441) (character 900))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 441) (character 4)) (end (line 441) (character 900))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 455) (character 8)) (end (line 455) (character 59))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantExitanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 455) (character 22)) (end (line 455) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 454) (character 8)) (end (line 454) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 454) (character 22)) (end (line 454) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (kind "attribute def") (name "SpectralRadiantExposureUnit") (declared-name "SpectralRadiantExposureUnit") (range (start (line 513) (character 4)) (end (line 513) (character 482))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 516) (character 8)) (end (line 516) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 514) (character 8)) (end (line 514) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 515) (character 8)) (end (line 515) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 517) (character 8)) (end (line 517) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 517) (character 22)) (end (line 517) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))) (kind "attribute def") (name "SpectralRadiantExposureValue") (declared-name "SpectralRadiantExposureValue") (range (start (line 494) (character 4)) (end (line 494) (character 900))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 494) (character 4)) (end (line 494) (character 900))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 508) (character 8)) (end (line 508) (character 59))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantExposureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 508) (character 22)) (end (line 508) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 507) (character 8)) (end (line 507) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 507) (character 22)) (end (line 507) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (kind "attribute def") (name "SpectralRadiantFluxUnit") (declared-name "SpectralRadiantFluxUnit") (range (start (line 239) (character 4)) (end (line 239) (character 477))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 242) (character 8)) (end (line 242) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 240) (character 8)) (end (line 240) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 241) (character 8)) (end (line 241) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 243) (character 8)) (end (line 243) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 243) (character 22)) (end (line 243) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))) (kind "attribute def") (name "SpectralRadiantFluxValue") (declared-name "SpectralRadiantFluxValue") (range (start (line 220) (character 4)) (end (line 220) (character 894))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::_documentation"))) (kind "documentation") (name "") (range (start (line 220) (character 4)) (end (line 220) (character 894))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 234) (character 8)) (end (line 234) (character 55))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantFluxUnit") (range none)) (redefinition (reference "mRef") (range (start (line 234) (character 22)) (end (line 234) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 233) (character 8)) (end (line 233) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 233) (character 22)) (end (line 233) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (kind "attribute def") (name "SpectralRadiantIntensityUnit") (declared-name "SpectralRadiantIntensityUnit") (range (start (line 297) (character 4)) (end (line 297) (character 482))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 300) (character 8)) (end (line 300) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 298) (character 8)) (end (line 298) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 299) (character 8)) (end (line 299) (character 100))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 301) (character 8)) (end (line 301) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 301) (character 22)) (end (line 301) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))) (kind "attribute def") (name "SpectralRadiantIntensityValue") (declared-name "SpectralRadiantIntensityValue") (range (start (line 278) (character 4)) (end (line 278) (character 890))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 278) (character 4)) (end (line 278) (character 890))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 292) (character 8)) (end (line 292) (character 60))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpectralRadiantIntensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 292) (character 22)) (end (line 292) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 291) (character 8)) (end (line 291) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 291) (character 22)) (end (line 291) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantPowerUnit"))) (kind "alias") (name "SpectralRadiantPowerUnit") (declared-name "SpectralRadiantPowerUnit") (range (start (line 246) (character 4)) (end (line 246) (character 63))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpectralRadiantPowerValue"))) (kind "alias") (name "SpectralRadiantPowerValue") (declared-name "SpectralRadiantPowerValue") (range (start (line 247) (character 4)) (end (line 247) (character 65))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (kind "attribute def") (name "SpeedOfLightInAMediumUnit") (declared-name "SpeedOfLightInAMediumUnit") (range (start (line 42) (character 4)) (end (line 42) (character 370))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 44) (character 8)) (end (line 44) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 43) (character 8)) (end (line 43) (character 102))) (parent (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 45) (character 8)) (end (line 45) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 45) (character 22)) (end (line 45) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))) (kind "attribute def") (name "SpeedOfLightInAMediumValue") (declared-name "SpeedOfLightInAMediumValue") (range (start (line 23) (character 4)) (end (line 23) (character 795))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::_documentation"))) (kind "documentation") (name "") (range (start (line 23) (character 4)) (end (line 23) (character 795))) (parent (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 37) (character 8)) (end (line 37) (character 57))) (parent (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedOfLightInAMediumUnit") (range none)) (redefinition (reference "mRef") (range (start (line 37) (character 22)) (end (line 37) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 36) (character 8)) (end (line 36) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 36) (character 22)) (end (line 36) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))) (kind "attribute def") (name "TransmittanceOpticalDensityValue") (declared-name "TransmittanceOpticalDensityValue") (range (start (line 1314) (character 4)) (end (line 1314) (character 854))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1314) (character 4)) (end (line 1314) (character 854))) (parent (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))) (kind "attribute def") (name "TransmittanceValue") (declared-name "TransmittanceValue") (range (start (line 1280) (character 4)) (end (line 1280) (character 901))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TransmittanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1280) (character 4)) (end (line 1280) (character 901))) (parent (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (kind "attribute def") (name "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (declared-name "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (range (start (line 1045) (character 4)) (end (line 1045) (character 425))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1046) (character 8)) (end (line 1046) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 1047) (character 8)) (end (line 1047) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1048) (character 8)) (end (line 1048) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1048) (character 22)) (end (line 1048) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))) (kind "attribute def") (name "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (declared-name "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (range (start (line 1026) (character 4)) (end (line 1026) (character 2021))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1026) (character 4)) (end (line 1026) (character 2021))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1040) (character 8)) (end (line 1040) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1040) (character 22)) (end (line 1040) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1039) (character 8)) (end (line 1039) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1039) (character 22)) (end (line 1039) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (kind "attribute def") (name "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (declared-name "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (range (start (line 1071) (character 4)) (end (line 1071) (character 425))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1072) (character 8)) (end (line 1072) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (range (start (line 1073) (character 8)) (end (line 1073) (character 113))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1074) (character 8)) (end (line 1074) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1074) (character 22)) (end (line 1074) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))) (kind "attribute def") (name "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (declared-name "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (range (start (line 1052) (character 4)) (end (line 1052) (character 2035))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1052) (character 4)) (end (line 1052) (character 2035))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1066) (character 8)) (end (line 1066) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1066) (character 22)) (end (line 1066) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1065) (character 8)) (end (line 1065) (character 32))) (parent (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1065) (character 22)) (end (line 1065) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQLight::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 100170))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::absorptance"))) (kind "attribute def") (name "absorptance") (declared-name "absorptance") (range (start (line 1226) (character 4)) (end (line 1226) (character 64))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsorptanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem"))) (kind "attribute def") (name "chromaticityCoordinatesInTheCie1931StandardColorimetricSystem") (declared-name "chromaticityCoordinatesInTheCie1931StandardColorimetricSystem") (range (start (line 1126) (character 4)) (end (line 1126) (character 164))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem"))) (kind "attribute def") (name "chromaticityCoordinatesInTheCie1964StandardColorimetricSystem") (declared-name "chromaticityCoordinatesInTheCie1964StandardColorimetricSystem") (range (start (line 1143) (character 4)) (end (line 1143) (character 164))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver"))) (kind "attribute def") (name "cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver") (declared-name "cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver") (range (start (line 1092) (character 4)) (end (line 1092) (character 176))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver"))) (kind "attribute def") (name "cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver") (declared-name "cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver") (range (start (line 1109) (character 4)) (end (line 1109) (character 176))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::colourTemperature"))) (kind "attribute def") (name "colourTemperature") (declared-name "colourTemperature") (range (start (line 1146) (character 4)) (end (line 1146) (character 568))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::colourTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 1146) (character 4)) (end (line 1146) (character 568))) (parent (node (document "d0") (qualified-name "ISQLight::colourTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature"))) (kind "attribute def") (name "correlatedColourTemperature") (declared-name "correlatedColourTemperature") (range (start (line 1162) (character 4)) (end (line 1162) (character 776))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 1162) (character 4)) (end (line 1162) (character 776))) (parent (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::decadicAbsorbance"))) (kind "alias") (name "decadicAbsorbance") (declared-name "decadicAbsorbance") (range (start (line 1334) (character 4)) (end (line 1334) (character 60))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::emissivity"))) (kind "attribute def") (name "emissivity") (declared-name "emissivity") (range (start (line 1192) (character 4)) (end (line 1192) (character 62))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "EmissivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::emissivityAtASpecifiedWavelength"))) (kind "attribute def") (name "emissivityAtASpecifiedWavelength") (declared-name "emissivityAtASpecifiedWavelength") (range (start (line 1209) (character 4)) (end (line 1209) (character 106))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "EmissivityAtASpecifiedWavelengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::illuminance"))) (kind "attribute def") (name "illuminance") (declared-name "illuminance") (range (start (line 768) (character 4)) (end (line 768) (character 77))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::irradiance"))) (kind "attribute def") (name "irradiance") (declared-name "irradiance") (range (start (line 375) (character 4)) (end (line 375) (character 75))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "IrradianceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::lightExposure"))) (kind "alias") (name "lightExposure") (declared-name "lightExposure") (range (start (line 835) (character 4)) (end (line 835) (character 45))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::linearAbsorptionCoefficient"))) (kind "attribute def") (name "linearAbsorptionCoefficient") (declared-name "linearAbsorptionCoefficient") (range (start (line 1451) (character 4)) (end (line 1451) (character 109))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearAbsorptionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::linearAttenuationCoefficient"))) (kind "attribute def") (name "linearAttenuationCoefficient") (declared-name "linearAttenuationCoefficient") (range (start (line 1422) (character 4)) (end (line 1422) (character 111))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearAttenuationCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::linearExtinctionCoefficient"))) (kind "alias") (name "linearExtinctionCoefficient") (declared-name "linearExtinctionCoefficient") (range (start (line 1431) (character 4)) (end (line 1431) (character 71))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminance"))) (kind "attribute def") (name "luminance") (declared-name "luminance") (range (start (line 742) (character 4)) (end (line 742) (character 73))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminanceFactor"))) (kind "attribute def") (name "luminanceFactor") (declared-name "luminanceFactor") (range (start (line 1385) (character 4)) (end (line 1385) (character 72))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminanceFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousAbsorptance"))) (kind "attribute def") (name "luminousAbsorptance") (declared-name "luminousAbsorptance") (range (start (line 1243) (character 4)) (end (line 1243) (character 80))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousAbsorptanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfASource"))) (kind "attribute def") (name "luminousEfficacyOfASource") (declared-name "luminousEfficacyOfASource") (range (start (line 656) (character 4)) (end (line 656) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficacyOfASourceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfRadiation"))) (kind "attribute def") (name "luminousEfficacyOfRadiation") (declared-name "luminousEfficacyOfRadiation") (range (start (line 572) (character 4)) (end (line 572) (character 109))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficacyOfRadiationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousEfficiency"))) (kind "attribute def") (name "luminousEfficiency") (declared-name "luminousEfficiency") (range (start (line 535) (character 4)) (end (line 535) (character 78))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficiencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousEnergy"))) (kind "attribute def") (name "luminousEnergy") (declared-name "luminousEnergy") (range (start (line 684) (character 4)) (end (line 684) (character 83))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousExitance"))) (kind "attribute def") (name "luminousExitance") (declared-name "luminousExitance") (range (start (line 794) (character 4)) (end (line 794) (character 87))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExitanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousExposure"))) (kind "attribute def") (name "luminousExposure") (declared-name "luminousExposure") (range (start (line 820) (character 4)) (end (line 820) (character 87))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExposureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousFlux"))) (kind "attribute def") (name "luminousFlux") (declared-name "luminousFlux") (range (start (line 714) (character 4)) (end (line 714) (character 79))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousFluxValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousReflectance"))) (kind "attribute def") (name "luminousReflectance") (declared-name "luminousReflectance") (range (start (line 1277) (character 4)) (end (line 1277) (character 80))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousReflectanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::luminousTransmittance"))) (kind "attribute def") (name "luminousTransmittance") (declared-name "luminousTransmittance") (range (start (line 1311) (character 4)) (end (line 1311) (character 84))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousTransmittanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::massAbsorptionCoefficient"))) (kind "attribute def") (name "massAbsorptionCoefficient") (declared-name "massAbsorptionCoefficient") (range (start (line 1502) (character 4)) (end (line 1502) (character 105))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassAbsorptionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::massAttenuationCoefficient"))) (kind "attribute def") (name "massAttenuationCoefficient") (declared-name "massAttenuationCoefficient") (range (start (line 1476) (character 4)) (end (line 1476) (character 107))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassAttenuationCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::maximumLuminousEfficacy"))) (kind "attribute def") (name "maximumLuminousEfficacy") (declared-name "maximumLuminousEfficacy") (range (start (line 628) (character 4)) (end (line 628) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "MaximumLuminousEfficacyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::molarAbsorptionCoefficient"))) (kind "attribute def") (name "molarAbsorptionCoefficient") (declared-name "molarAbsorptionCoefficient") (range (start (line 1528) (character 4)) (end (line 1528) (character 107))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarAbsorptionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::napierianAbsorbance"))) (kind "attribute def") (name "napierianAbsorbance") (declared-name "napierianAbsorbance") (range (start (line 1351) (character 4)) (end (line 1351) (character 80))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "NapierianAbsorbanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::numberOfPhotons"))) (kind "alias") (name "numberOfPhotons") (declared-name "numberOfPhotons") (range (start (line 854) (character 4)) (end (line 854) (character 43))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::opticalDensity"))) (kind "alias") (name "opticalDensity") (declared-name "opticalDensity") (range (start (line 1330) (character 4)) (end (line 1330) (character 57))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonEnergy"))) (kind "attribute def") (name "photonEnergy") (declared-name "photonEnergy") (range (start (line 857) (character 4)) (end (line 857) (character 927))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 857) (character 4)) (end (line 857) (character 927))) (parent (node (document "d0") (qualified-name "ISQLight::photonEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonExitance"))) (kind "attribute def") (name "photonExitance") (declared-name "photonExitance") (range (start (line 992) (character 4)) (end (line 992) (character 83))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonExitanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonExposure"))) (kind "attribute def") (name "photonExposure") (declared-name "photonExposure") (range (start (line 1018) (character 4)) (end (line 1018) (character 83))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonExposureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonFlux"))) (kind "attribute def") (name "photonFlux") (declared-name "photonFlux") (range (start (line 890) (character 4)) (end (line 890) (character 75))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonFluxValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonIntensity"))) (kind "attribute def") (name "photonIntensity") (declared-name "photonIntensity") (range (start (line 915) (character 4)) (end (line 915) (character 85))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonIntensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonIrradiance"))) (kind "attribute def") (name "photonIrradiance") (declared-name "photonIrradiance") (range (start (line 966) (character 4)) (end (line 966) (character 87))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonIrradianceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonNumber"))) (kind "attribute def") (name "photonNumber") (declared-name "photonNumber") (range (start (line 852) (character 4)) (end (line 852) (character 66))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::photonRadiance"))) (kind "attribute def") (name "photonRadiance") (declared-name "photonRadiance") (range (start (line 940) (character 4)) (end (line 940) (character 83))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonRadianceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::quantityOfIllumination"))) (kind "alias") (name "quantityOfIllumination") (declared-name "quantityOfIllumination") (range (start (line 831) (character 4)) (end (line 831) (character 54))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::quantityOfLight"))) (kind "alias") (name "quantityOfLight") (declared-name "quantityOfLight") (range (start (line 694) (character 4)) (end (line 694) (character 45))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiance"))) (kind "attribute def") (name "radiance") (declared-name "radiance") (range (start (line 322) (character 4)) (end (line 322) (character 71))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadianceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radianceFactor"))) (kind "attribute def") (name "radianceFactor") (declared-name "radianceFactor") (range (start (line 1368) (character 4)) (end (line 1368) (character 70))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadianceFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantEmittance"))) (kind "alias") (name "radiantEmittance") (declared-name "radiantEmittance") (range (start (line 438) (character 4)) (end (line 438) (character 47))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))) (kind "attribute def") (name "radiantEnergy") (declared-name "radiantEnergy") (range (start (line 66) (character 4)) (end (line 66) (character 1036))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 66) (character 4)) (end (line 66) (character 1036))) (parent (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantEnergyDensity"))) (kind "attribute def") (name "radiantEnergyDensity") (declared-name "radiantEnergyDensity") (range (start (line 126) (character 4)) (end (line 126) (character 95))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantEnergyDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantExitance"))) (kind "attribute def") (name "radiantExitance") (declared-name "radiantExitance") (range (start (line 428) (character 4)) (end (line 428) (character 85))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantExitanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantExposure"))) (kind "attribute def") (name "radiantExposure") (declared-name "radiantExposure") (range (start (line 485) (character 4)) (end (line 485) (character 85))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantExposureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantFlux"))) (kind "attribute def") (name "radiantFlux") (declared-name "radiantFlux") (range (start (line 206) (character 4)) (end (line 206) (character 77))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantFluxValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantIntensity"))) (kind "attribute def") (name "radiantIntensity") (declared-name "radiantIntensity") (range (start (line 268) (character 4)) (end (line 268) (character 87))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantIntensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::radiantPower"))) (kind "alias") (name "radiantPower") (declared-name "radiantPower") (range (start (line 217) (character 4)) (end (line 217) (character 39))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::reflectance"))) (kind "attribute def") (name "reflectance") (declared-name "reflectance") (range (start (line 1260) (character 4)) (end (line 1260) (character 64))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReflectanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::reflectanceFactor"))) (kind "attribute def") (name "reflectanceFactor") (declared-name "reflectanceFactor") (range (start (line 1402) (character 4)) (end (line 1402) (character 76))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReflectanceFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::refractiveIndex"))) (kind "attribute def") (name "refractiveIndex") (declared-name "refractiveIndex") (range (start (line 63) (character 4)) (end (line 63) (character 72))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "RefractiveIndexValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralIrradiance"))) (kind "attribute def") (name "spectralIrradiance") (declared-name "spectralIrradiance") (range (start (line 401) (character 4)) (end (line 401) (character 91))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralIrradianceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficacy"))) (kind "attribute def") (name "spectralLuminousEfficacy") (declared-name "spectralLuminousEfficacy") (range (start (line 600) (character 4)) (end (line 600) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralLuminousEfficacyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficiency"))) (kind "attribute def") (name "spectralLuminousEfficiency") (declared-name "spectralLuminousEfficiency") (range (start (line 552) (character 4)) (end (line 552) (character 94))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralLuminousEfficiencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiance"))) (kind "attribute def") (name "spectralRadiance") (declared-name "spectralRadiance") (range (start (line 348) (character 4)) (end (line 348) (character 87))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadianceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergy"))) (kind "attribute def") (name "spectralRadiantEnergy") (declared-name "spectralRadiantEnergy") (range (start (line 99) (character 4)) (end (line 99) (character 97))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavelength"))) (kind "attribute def") (name "spectralRadiantEnergyDensityInTermsOfWavelength") (declared-name "spectralRadiantEnergyDensityInTermsOfWavelength") (range (start (line 153) (character 4)) (end (line 153) (character 149))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavenumber"))) (kind "attribute def") (name "spectralRadiantEnergyDensityInTermsOfWavenumber") (declared-name "spectralRadiantEnergyDensityInTermsOfWavenumber") (range (start (line 180) (character 4)) (end (line 180) (character 149))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantExitance"))) (kind "attribute def") (name "spectralRadiantExitance") (declared-name "spectralRadiantExitance") (range (start (line 458) (character 4)) (end (line 458) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantExitanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantExposure"))) (kind "attribute def") (name "spectralRadiantExposure") (declared-name "spectralRadiantExposure") (range (start (line 511) (character 4)) (end (line 511) (character 101))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantExposureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantFlux"))) (kind "attribute def") (name "spectralRadiantFlux") (declared-name "spectralRadiantFlux") (range (start (line 237) (character 4)) (end (line 237) (character 93))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantFluxValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantIntensity"))) (kind "attribute def") (name "spectralRadiantIntensity") (declared-name "spectralRadiantIntensity") (range (start (line 295) (character 4)) (end (line 295) (character 103))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantIntensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::spectralRadiantPower"))) (kind "alias") (name "spectralRadiantPower") (declared-name "spectralRadiantPower") (range (start (line 248) (character 4)) (end (line 248) (character 55))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::speedOfLightInAMedium"))) (kind "attribute def") (name "speedOfLightInAMedium") (declared-name "speedOfLightInAMedium") (range (start (line 40) (character 4)) (end (line 40) (character 97))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedOfLightInAMediumValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::transmittance"))) (kind "attribute def") (name "transmittance") (declared-name "transmittance") (range (start (line 1294) (character 4)) (end (line 1294) (character 68))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransmittanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::transmittanceDensity"))) (kind "alias") (name "transmittanceDensity") (declared-name "transmittanceDensity") (range (start (line 1332) (character 4)) (end (line 1332) (character 63))) (parent (node (document "d0") (qualified-name "ISQLight"))))
    (element (id (node (document "d0") (qualified-name "ISQLight::transmittanceOpticalDensity"))) (kind "attribute def") (name "transmittanceOpticalDensity") (declared-name "transmittanceOpticalDensity") (range (start (line 1328) (character 4)) (end (line 1328) (character 96))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransmittanceOpticalDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1931StandardColorimetricObserver"))) (kind "attribute def") (name "tristimulusValuesForTheCie1931StandardColorimetricObserver") (declared-name "tristimulusValuesForTheCie1931StandardColorimetricObserver") (range (start (line 1043) (character 4)) (end (line 1043) (character 171))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1964StandardColorimetricObserver"))) (kind "attribute def") (name "tristimulusValuesForTheCie1964StandardColorimetricObserver") (declared-name "tristimulusValuesForTheCie1964StandardColorimetricObserver") (range (start (line 1069) (character 4)) (end (line 1069) (character 171))) (parent (node (document "d0") (qualified-name "ISQLight"))) (authored (membership (kind Owning)) (relationships (typing (reference "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::EnergyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQThermodynamics::EnergyValue") (range (start (line 20) (character 19)) (end (line 20) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 773) (character 22)) (end (line 773) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 765) (character 22)) (end (line 765) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 764) (character 22)) (end (line 764) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 380) (character 22)) (end (line 380) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IrradianceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IrradianceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 372) (character 22)) (end (line 372) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 371) (character 22)) (end (line 371) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1455) (character 22)) (end (line 1455) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearAbsorptionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1448) (character 22)) (end (line 1448) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1447) (character 22)) (end (line 1447) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1426) (character 22)) (end (line 1426) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearAttenuationCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1419) (character 22)) (end (line 1419) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1418) (character 22)) (end (line 1418) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 747) (character 22)) (end (line 747) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 739) (character 22)) (end (line 739) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 738) (character 22)) (end (line 738) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 663) (character 22)) (end (line 663) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfASourceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 653) (character 22)) (end (line 653) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 652) (character 22)) (end (line 652) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 579) (character 22)) (end (line 579) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfRadiationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 569) (character 22)) (end (line 569) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 568) (character 22)) (end (line 568) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 689) (character 22)) (end (line 689) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 681) (character 22)) (end (line 681) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 680) (character 22)) (end (line 680) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 799) (character 22)) (end (line 799) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExitanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 791) (character 22)) (end (line 791) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 790) (character 22)) (end (line 790) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 826) (character 22)) (end (line 826) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExposureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 817) (character 22)) (end (line 817) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 816) (character 22)) (end (line 816) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 718) (character 22)) (end (line 718) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousFluxUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 711) (character 22)) (end (line 711) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 710) (character 22)) (end (line 710) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1507) (character 22)) (end (line 1507) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAbsorptionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1499) (character 22)) (end (line 1499) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1498) (character 22)) (end (line 1498) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1481) (character 22)) (end (line 1481) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAttenuationCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1473) (character 22)) (end (line 1473) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1472) (character 22)) (end (line 1472) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 635) (character 22)) (end (line 635) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximumLuminousEfficacyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 625) (character 22)) (end (line 625) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 624) (character 22)) (end (line 624) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1533) (character 22)) (end (line 1533) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarAbsorptionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1525) (character 22)) (end (line 1525) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1524) (character 22)) (end (line 1524) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 997) (character 22)) (end (line 997) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonExitanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 989) (character 22)) (end (line 989) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 988) (character 22)) (end (line 988) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1022) (character 22)) (end (line 1022) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonExposureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1015) (character 22)) (end (line 1015) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1014) (character 22)) (end (line 1014) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 894) (character 22)) (end (line 894) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonFluxUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 887) (character 22)) (end (line 887) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 886) (character 22)) (end (line 886) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 919) (character 22)) (end (line 919) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIntensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 912) (character 22)) (end (line 912) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 911) (character 22)) (end (line 911) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 971) (character 22)) (end (line 971) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIrradianceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 963) (character 22)) (end (line 963) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 962) (character 22)) (end (line 962) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 945) (character 22)) (end (line 945) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonRadianceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 937) (character 22)) (end (line 937) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 936) (character 22)) (end (line 936) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 327) (character 22)) (end (line 327) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadianceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 319) (character 22)) (end (line 319) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 318) (character 22)) (end (line 318) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadianceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 132) (character 22)) (end (line 132) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantEnergyDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 123) (character 22)) (end (line 123) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 122) (character 22)) (end (line 122) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 433) (character 22)) (end (line 433) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantExitanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 425) (character 22)) (end (line 425) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 424) (character 22)) (end (line 424) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 490) (character 22)) (end (line 490) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantExposureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 482) (character 22)) (end (line 482) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 481) (character 22)) (end (line 481) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 212) (character 22)) (end (line 212) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantFluxUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 203) (character 22)) (end (line 203) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 202) (character 22)) (end (line 202) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 274) (character 22)) (end (line 274) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantIntensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 265) (character 22)) (end (line 265) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 264) (character 22)) (end (line 264) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 407) (character 22)) (end (line 407) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralIrradianceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 398) (character 22)) (end (line 398) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 397) (character 22)) (end (line 397) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 607) (character 22)) (end (line 607) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralLuminousEfficacyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 597) (character 22)) (end (line 597) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 596) (character 22)) (end (line 596) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 354) (character 22)) (end (line 354) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadianceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 345) (character 22)) (end (line 345) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 344) (character 22)) (end (line 344) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 159) (character 22)) (end (line 159) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 150) (character 22)) (end (line 150) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 149) (character 22)) (end (line 149) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 185) (character 22)) (end (line 185) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 177) (character 22)) (end (line 177) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 176) (character 22)) (end (line 176) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 105) (character 22)) (end (line 105) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 96) (character 22)) (end (line 96) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 95) (character 22)) (end (line 95) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 464) (character 22)) (end (line 464) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantExitanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 455) (character 22)) (end (line 455) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 454) (character 22)) (end (line 454) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 517) (character 22)) (end (line 517) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantExposureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 508) (character 22)) (end (line 508) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 507) (character 22)) (end (line 507) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 243) (character 22)) (end (line 243) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantFluxUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 234) (character 22)) (end (line 234) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 233) (character 22)) (end (line 233) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 301) (character 22)) (end (line 301) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantIntensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 292) (character 22)) (end (line 292) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 291) (character 22)) (end (line 291) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 45) (character 22)) (end (line 45) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedOfLightInAMediumUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 37) (character 22)) (end (line 37) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 36) (character 22)) (end (line 36) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1048) (character 22)) (end (line 1048) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1040) (character 22)) (end (line 1040) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1039) (character 22)) (end (line 1039) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1074) (character 22)) (end (line 1074) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1066) (character 22)) (end (line 1066) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1065) (character 22)) (end (line 1065) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::absorptance"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsorptanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)) (authored-target "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)) (authored-target "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::colourTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::correlatedColourTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::emissivity"))) (kind featureTyping) (ordinal 0)) (authored-target "EmissivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::EmissivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::emissivityAtASpecifiedWavelength"))) (kind featureTyping) (ordinal 0)) (authored-target "EmissivityAtASpecifiedWavelengthValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::illuminance"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::irradiance"))) (kind featureTyping) (ordinal 0)) (authored-target "IrradianceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::IrradianceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::linearAbsorptionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearAbsorptionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::linearAttenuationCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearAttenuationCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminance"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminanceFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousAbsorptance"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousAbsorptanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfASource"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfASourceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfRadiation"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfRadiationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficiencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousExitance"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExitanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousExposure"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExposureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousFlux"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousFluxValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousReflectance"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousReflectanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::luminousTransmittance"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousTransmittanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::massAbsorptionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAbsorptionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::massAttenuationCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAttenuationCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::maximumLuminousEfficacy"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximumLuminousEfficacyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::molarAbsorptionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarAbsorptionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::napierianAbsorbance"))) (kind featureTyping) (ordinal 0)) (authored-target "NapierianAbsorbanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonExitance"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonExitanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonExposure"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonExposureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonFlux"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonFluxValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonIntensity"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIntensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonIrradiance"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIrradianceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::photonRadiance"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonRadianceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiance"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadianceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radianceFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiantEnergyDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantEnergyDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiantExitance"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantExitanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiantExposure"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantExposureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiantFlux"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantFluxValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::radiantIntensity"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantIntensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::reflectance"))) (kind featureTyping) (ordinal 0)) (authored-target "ReflectanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::ReflectanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::reflectanceFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "ReflectanceFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::refractiveIndex"))) (kind featureTyping) (ordinal 0)) (authored-target "RefractiveIndexValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralIrradiance"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralIrradianceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficacy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralLuminousEfficacyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralLuminousEfficiencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiance"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadianceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavelength"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavenumber"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantExitance"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantExitanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantExposure"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantExposureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantFlux"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantFluxValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantIntensity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantIntensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::speedOfLightInAMedium"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedOfLightInAMediumValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::transmittance"))) (kind featureTyping) (ordinal 0)) (authored-target "TransmittanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TransmittanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::transmittanceOpticalDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "TransmittanceOpticalDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1931StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)) (authored-target "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1964StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)) (authored-target "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IlluminanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IrradianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::IrradianceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::IrradianceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadianceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadianceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::absorptance"))) (target (node (document "d0") (qualified-name "ISQLight::AbsorptanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::absorptance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem"))) (target (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1931StandardColorimetricSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem"))) (target (node (document "d0") (qualified-name "ISQLight::ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::chromaticityCoordinatesInTheCie1964StandardColorimetricSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver"))) (target (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver"))) (target (node (document "d0") (qualified-name "ISQLight::CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::emissivity"))) (target (node (document "d0") (qualified-name "ISQLight::EmissivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::emissivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::emissivityAtASpecifiedWavelength"))) (target (node (document "d0") (qualified-name "ISQLight::EmissivityAtASpecifiedWavelengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::emissivityAtASpecifiedWavelength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::illuminance"))) (target (node (document "d0") (qualified-name "ISQLight::IlluminanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::illuminance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::irradiance"))) (target (node (document "d0") (qualified-name "ISQLight::IrradianceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::irradiance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::linearAbsorptionCoefficient"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAbsorptionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::linearAbsorptionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::linearAttenuationCoefficient"))) (target (node (document "d0") (qualified-name "ISQLight::LinearAttenuationCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::linearAttenuationCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminance"))) (target (node (document "d0") (qualified-name "ISQLight::LuminanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminanceFactor"))) (target (node (document "d0") (qualified-name "ISQLight::LuminanceFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminanceFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousAbsorptance"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousAbsorptanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousAbsorptance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfASource"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfASourceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfASource"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfRadiation"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficacyOfRadiationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousEfficacyOfRadiation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousEfficiency"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEfficiencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousEfficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousEnergy"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousExitance"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExitanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousExitance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousExposure"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousExposureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousExposure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousFlux"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousFluxValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousFlux"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousReflectance"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousReflectanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousReflectance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::luminousTransmittance"))) (target (node (document "d0") (qualified-name "ISQLight::LuminousTransmittanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::luminousTransmittance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::massAbsorptionCoefficient"))) (target (node (document "d0") (qualified-name "ISQLight::MassAbsorptionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::massAbsorptionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::massAttenuationCoefficient"))) (target (node (document "d0") (qualified-name "ISQLight::MassAttenuationCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::massAttenuationCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::maximumLuminousEfficacy"))) (target (node (document "d0") (qualified-name "ISQLight::MaximumLuminousEfficacyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::maximumLuminousEfficacy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::molarAbsorptionCoefficient"))) (target (node (document "d0") (qualified-name "ISQLight::MolarAbsorptionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::molarAbsorptionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::napierianAbsorbance"))) (target (node (document "d0") (qualified-name "ISQLight::NapierianAbsorbanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::napierianAbsorbance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonEnergy"))) (target (node (document "d0") (qualified-name "ISQLight::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonExitance"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExitanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonExitance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonExposure"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonExposureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonExposure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonFlux"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonFluxValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonFlux"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonIntensity"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIntensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonIrradiance"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonIrradianceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonIrradiance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonNumber"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::photonRadiance"))) (target (node (document "d0") (qualified-name "ISQLight::PhotonRadianceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::photonRadiance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiance"))) (target (node (document "d0") (qualified-name "ISQLight::RadianceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radianceFactor"))) (target (node (document "d0") (qualified-name "ISQLight::RadianceFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radianceFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))) (target (node (document "d0") (qualified-name "ISQLight::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiantEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiantEnergyDensity"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantEnergyDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiantEnergyDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiantExitance"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExitanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiantExitance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiantExposure"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantExposureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiantExposure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiantFlux"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantFluxValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiantFlux"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::radiantIntensity"))) (target (node (document "d0") (qualified-name "ISQLight::RadiantIntensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::radiantIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::reflectance"))) (target (node (document "d0") (qualified-name "ISQLight::ReflectanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::reflectance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::reflectanceFactor"))) (target (node (document "d0") (qualified-name "ISQLight::ReflectanceFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::reflectanceFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::refractiveIndex"))) (target (node (document "d0") (qualified-name "ISQLight::RefractiveIndexValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::refractiveIndex"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralIrradiance"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralIrradianceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralIrradiance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficacy"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficacyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficacy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficiency"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralLuminousEfficiencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralLuminousEfficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiance"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadianceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergy"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavelength"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavelengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavelength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavenumber"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantEnergyDensityInTermsOfWavenumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantEnergyDensityInTermsOfWavenumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantExitance"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExitanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantExitance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantExposure"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantExposureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantExposure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantFlux"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantFluxValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantFlux"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantIntensity"))) (target (node (document "d0") (qualified-name "ISQLight::SpectralRadiantIntensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::spectralRadiantIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::speedOfLightInAMedium"))) (target (node (document "d0") (qualified-name "ISQLight::SpeedOfLightInAMediumValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::speedOfLightInAMedium"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::transmittance"))) (target (node (document "d0") (qualified-name "ISQLight::TransmittanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::transmittance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::transmittanceOpticalDensity"))) (target (node (document "d0") (qualified-name "ISQLight::TransmittanceOpticalDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::transmittanceOpticalDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1931StandardColorimetricObserver"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1931StandardColorimetricObserverValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1931StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1964StandardColorimetricObserver"))) (target (node (document "d0") (qualified-name "ISQLight::TristimulusValuesForTheCie1964StandardColorimetricObserverValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQLight::tristimulusValuesForTheCie1964StandardColorimetricObserver"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
