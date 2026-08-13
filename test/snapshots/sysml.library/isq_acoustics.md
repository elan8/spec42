# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQAcoustics
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQAcoustics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-8:2020 "Acoustics"
     * see also https://www.iso.org/standard/64978.html
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
    private import ISQMechanics::PowerValue;
    private import ISQMechanics::PressureValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::SpeedValue;
    private import ISQSpaceTime::CartesianVelocity3dCoordinateFrame;
    private import ISQSpaceTime::AccelerationValue;
    private import ISQSpaceTime::CartesianAcceleration3dCoordinateFrame;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-8 item 8-1 logarithmic frequency range */
    attribute def LogarithmicFrequencyRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-1 logarithmic frequency range
         * symbol(s): `G`
         * application domain: generic
         * name: LogarithmicFrequencyRange
         * quantity dimension: 1
         * measurement unit(s): oct, dec
         * tensor order: 0
         * definition: quantity given by: `G = log_2(f_2/f_1) "[oct]" = log_10(f_2/f_1) "[dec]"`, where `f_1` and `f_2` are two frequencies (ISO 80000-3)
         * remarks: One octave (oct) is the logarithmic frequency range between `f_1` and `f_2` when `f_2/f_1 = 2`. Similarly, one decade (dec) is the logarithmic frequency range between `f_1` and `f_2` when `f_2/f_1 = 10`; thus `1 "[dec]" = log_2(10) "[oct]" ≈ 3.322 "[oct]"`. ISO 266 specifies preferred frequencies for acoustics separated by logarithmic frequency ranges equal to one tenth of a decade (`0.1 "[dec]"`). Each `0.1 "[dec]"` logarithmic frequency range is referred to in ISO 266 as a "one-third-octave interval" because `0.1 "[dec]"` is approximately equal to `1/3 "[oct]"`. Similarly, a logarithmic frequency range of `0.3 "[dec]"` is referred to as a "one-octave interval" because `0.3 "[dec]"` is approximately equal to `1 "[oct]"`. A logarithmic frequency range equal to one tenth of a decade can be referred to as a decidecade.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LogarithmicFrequencyRangeUnit[1];
    }

    attribute logarithmicFrequencyRange: LogarithmicFrequencyRangeValue[*] nonunique :> scalarQuantities;

    attribute def LogarithmicFrequencyRangeUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-2.1 static pressure */
    attribute staticPressure: PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 8-2.1 static pressure
         * symbol(s): `p_s`
         * application domain: generic
         * name: StaticPressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure (ISO 80000-4) in a medium when no sound wave is present
         * remarks: This definition applies to a medium with zero flow.
         */
    }

    /* ISO-80000-8 item 8-2.2 sound pressure */
    attribute soundPressure: PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 8-2.2 sound pressure
         * symbol(s): `p`
         * application domain: generic
         * name: SoundPressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: difference between instantaneous total pressure and static pressure (item 8-2.1)
         * remarks: None.
         */
    }

    /* ISO-80000-8 item 8-3 sound particle displacement */
    attribute def CartesianSoundParticleDisplacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-3 sound particle displacement
         * symbol(s): `vec(δ)`
         * application domain: generic
         * name: SoundParticleDisplacement (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the instantaneous displacement (ISO 80000-3) of a particle in a medium from what would be its position in the absence of sound waves
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianSoundParticleDisplacement3dVector: CartesianSoundParticleDisplacement3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-4 sound particle velocity */
    attribute def CartesianSoundParticleVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-4 sound particle velocity
         * symbol(s): `vec(u)`, `(vec(v))`
         * application domain: generic
         * name: SoundParticleVelocity (specializes Velocity)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(u) = del(vec(δ))/del(t)`, where `vec(δ)` is sound particle displacement (item 8-3) and `t` is time (ISO 80000-3)
         * remarks: The definition is limited to small-amplitude acoustic disturbances such that the magnitude of `vec(u)` is small relative to the phase speed (ISO 80000-3) of sound.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianVelocity3dCoordinateFrame[1];
    }

    attribute cartesianSoundParticleVelocity3dVector: CartesianSoundParticleVelocity3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-5 sound particle acceleration */
    attribute def CartesianSoundParticleAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-5 sound particle acceleration
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: SoundParticleAcceleration (specializes Acceleration)
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(a) = (del(vec(u)))/(del(t))`, where `vec(u)` is sound particle velocity (item 8-4) and `t` is time
         * remarks: The definition is limited to small-amplitude acoustic disturbances such that the magnitude of `vec(u)` is small relative to the phase speed (ISO 80000-3) of sound.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAcceleration3dCoordinateFrame[1];
    }

    attribute cartesianSoundParticleAcceleration3dVector: CartesianSoundParticleAcceleration3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-6 volume velocity, volume flow rate */
    attribute volumeVelocity: SpeedValue :> scalarQuantities {
        doc
        /*
         * source: item 8-6 volume velocity, volume flow rate
         * symbol(s): `q`, `q_v`
         * application domain: generic
         * name: VolumeVelocity (specializes Speed)
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: surface integral of the normal component of the sound particle velocity (item 8-4) over a defined surface
         * remarks: None.
         */
    }

    alias volumeFlowRate for volumeVelocity;

    /* ISO-80000-8 item 8-7 sound energy density */
    attribute def SoundEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-7 sound energy density
         * symbol(s): `w`
         * application domain: generic
         * name: SoundEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quantity given by: `w = 1/2 ρ_m u^2 + 1/2 p^2/(ρ_m c^2)`, where `ρ_m` is mean density (ISO 80000-4), `u` is the magnitude of the sound particle velocity (item 8-4), `p` is sound pressure (item 8-2.2), and `c` is the phase speed (ISO 80000-3) of sound
         * remarks: In formula form: `E = int_(t_1)^(t_2) p^2 dt`, where `t_1` and `t_2` are the starting and ending times for the integral and `p` is sound pressure (item 8-2.2). In airborne acoustics, the sound pressure is frequency-weighted and frequency-band-limited. If frequency weightings as specified in IEC 61672-1 are applied, this should be indicated by appropriate subscripts to the symbol `E`. In underwater acoustics, the term ""sound exposure"" indicates an unweighted quantity unless indicated otherwise.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundEnergyDensityUnit[1];
    }

    attribute soundEnergyDensity: SoundEnergyDensityValue[*] nonunique :> scalarQuantities;

    attribute def SoundEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-8 sound energy */
    attribute soundEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 8-8 sound energy
         * symbol(s): `Q`
         * application domain: generic
         * name: SoundEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: integral of sound energy density (item 8-7) over a specified volume
         * remarks: The sound energy in region `R` can be expressed by: `Q = oint_R w(x) d^3x`, where `d^3x` is an element of volume.
         */
    }

    /* ISO-80000-8 item 8-9 sound power */
    attribute soundPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 8-9 sound power
         * symbol(s): `P`, `W`
         * application domain: generic
         * name: SoundPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: integral over a surface of the product of sound pressure, `p` (item 8-2.2), and the component `u_n` of the particle velocity (item 8-4) in the direction normal to the surface, at a point on the surface
         * remarks: This definition holds for waves in the volume of homogenous fluids or gases. This definition can become inapplicable in situations with a high mean fluid flow. Sound power is for example used to indicate the rate at which energy is radiated by a sound source. Sound power is an oscillatory quantity that can be positive or negative. A positive sound power indicates that the sound power is radiated out of the surface. A negative sound power indicates that the sound power is absorbed into the surface.
         */
    }

    /* ISO-80000-8 item 8-10 sound intensity */
    attribute def SoundIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-10 sound intensity (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: SoundIntensity
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity given by: `vec(I) = p vec(u)`, where `p` is sound pressure (item 8-2.2) and `vec(u)` is sound particle velocity (item 8-4)
         * remarks: This definition can become inapplicable in situations with a high mean fluid flow.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundIntensityUnit[1];
    }

    attribute soundIntensity: SoundIntensityValue[*] nonunique :> scalarQuantities;

    attribute def SoundIntensityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    attribute def CartesianSoundIntensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-10 sound intensity (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: SoundIntensity
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(I) = p vec(u)`, where `p` is sound pressure (item 8-2.2) and `vec(u)` is sound particle velocity (item 8-4)
         * remarks: This definition can become inapplicable in situations with a high mean fluid flow.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSoundIntensity3dCoordinateFrame[1];
    }

    attribute cartesianSoundIntensity3dVector: CartesianSoundIntensity3dVector :> vectorQuantities;

    attribute def CartesianSoundIntensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SoundIntensityUnit[3];
    }

    /* ISO-80000-8 item 8-11 sound exposure */
    attribute def SoundExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-11 sound exposure
         * symbol(s): `E`
         * application domain: generic
         * name: SoundExposure
         * quantity dimension: L^-2*M^2*T^-3
         * measurement unit(s): Pa^2*s, kg^2*m^-2*s^-3
         * tensor order: 0
         * definition: time-integrated squared sound pressure (item 8-2.2)
         * remarks: In formula form: `E = int_(t_1)^(t_2) p^2 dt`, where `t_1` and `t_2` are the starting and ending times for the integral and `p` is sound pressure (item 8-2.2). In airborne acoustics, the sound pressure is frequency-weighted and frequency-band-limited. If frequency weightings as specified in IEC 61672-1 are applied, this should be indicated by appropriate subscripts to the symbol `E`. In underwater acoustics, the term "sound exposure" indicates an unweighted quantity unless indicated otherwise.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundExposureUnit[1];
    }

    attribute soundExposure: SoundExposureValue[*] nonunique :> scalarQuantities;

    attribute def SoundExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-12 characteristic impedance of a medium for longitudinal waves */
    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-12 characteristic impedance of a medium for longitudinal waves
         * symbol(s): `Z_c`
         * application domain: generic
         * name: CharacteristicImpedanceOfAMediumForLongitudinalWaves
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): Pa*s/m, kg*m^-2*s^-1
         * tensor order: 0
         * definition: quotient of sound pressure (item 8-2.2) and the component of the sound particle velocity (item 8-4) in the direction of the wave propagation
         * remarks: The definition is limited to a progressive plane wave in a non-dissipative homogenous gas or fluid. Characteristic impedance is a property of the medium and is equal to `ρ c` where `ρ` is the time-averaged density (ISO 80000-4) of the medium and `c` the phase speed of sound (ISO 80000-3). Longitudinal waves are waves in which the displacement of the medium is in the same direction as, or the opposite direction to, the direction of propagation of the wave.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit[1];
    }

    attribute characteristicImpedanceOfAMediumForLongitudinalWaves: CharacteristicImpedanceOfAMediumForLongitudinalWavesValue[*] nonunique :> scalarQuantities;

    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-13 acoustic impedance */
    attribute def AcousticImpedanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-13 acoustic impedance
         * symbol(s): `Z_a`
         * application domain: generic
         * name: AcousticImpedance
         * quantity dimension: L^-4*M^1*T^-1
         * measurement unit(s): Pa*s/m^3, kg*m^-4*s^-1
         * tensor order: 0
         * definition: at a surface, quotient of the average sound pressure (item 8-2.2) over that surface and the sound volume flow rate (item 8-6) through that surface
         * remarks: This definition applies to a sound pressure that is in phase with the volume flow rate. In this situation, the acoustic impedance is real. Both the sound pressure, `p`, and sound volume flow rate, `q`, are real quantities that fluctuate with time. If the fluctuations are in phase (phase difference equal to zero), the quotient `p/q` is a constant. If they are out of phase (phase difference not equal to zero), they can be represented by complex quantities in the frequency domain, the quotient of which is also complex.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AcousticImpedanceUnit[1];
    }

    attribute acousticImpedance: AcousticImpedanceValue[*] nonunique :> scalarQuantities;

    attribute def AcousticImpedanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -4; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-14 sound pressure level */
    attribute def SoundPressureLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-14 sound pressure level
         * symbol(s): `L_p`
         * application domain: generic
         * name: SoundPressureLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_p = 10 log_10((p_"RMS"^2)/p_0^2) "[dB]"`, where `p_"RMS"` is the root-mean-square sound pressure in the time domain and `p_0` is the reference value of sound pressure
         * remarks: For sound in air and other gases, the reference value of sound pressure is given by `p_0 = 20 "[μPa]"`. For sound in water and other liquids, the reference value of sound pressure is given by `p_0 = 1 "[μPa]"`. When stating a value of sound pressure level, the reference value shall be specified. The value of sound pressure level depends on the selected frequency range and time duration. When stating a value of sound pressure level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol. In some applications the level of the peak sound pressure is required. This is obtained by replacing the root-mean-square sound pressure, with the instantaneous sound pressure having the greatest absolute value during a stated time interval, in the definition of sound pressure level.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundPressureLevelUnit[1];
    }

    attribute soundPressureLevel: SoundPressureLevelValue[*] nonunique :> scalarQuantities;

    attribute def SoundPressureLevelUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-15 sound power level */
    attribute def SoundPowerLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-15 sound power level
         * symbol(s): `L_P`, `L_W`
         * application domain: generic
         * name: SoundPowerLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_P = 10 log_10 ((P_m)/P_0) "[dB]"`, where `P_m` is the magnitude of the time-averaged sound power (item 8-9) and `P_0` is the reference value of sound power
         * remarks: The reference value of sound power is given by `P_0 = 1 "[pW]"`. When stating a value of sound power level, the reference value shall be specified. The value of sound power level depends on the selected frequency range and time duration. When stating a value of sound power level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundPowerLevelUnit[1];
    }

    attribute soundPowerLevel: SoundPowerLevelValue[*] nonunique :> scalarQuantities;

    attribute def SoundPowerLevelUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-16 sound exposure level */
    attribute def SoundExposureLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-16 sound exposure level
         * symbol(s): `L_E`
         * application domain: generic
         * name: SoundExposureLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_E = 10 log_10(E/E_0) "[dB]"`, where `E` is sound exposure (item 8-11) and `E_0` is the reference value of sound exposure
         * remarks: For sound in air and other gases, the reference value of sound exposure is given by `E_0 = 400 "@"["μPa"^2*"s"]`. For sound in water and other liquids, the reference value of sound exposure is given by `E_0 = 1"@"["μPa"^2*"s"]`. When stating a value of sound exposure level, the reference value shall be specified. The value of sound exposure level depends on the selected frequency range and time duration. When stating a value of sound exposure level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundExposureLevelUnit[1];
    }

    attribute soundExposureLevel: SoundExposureLevelValue[*] nonunique :> scalarQuantities;

    attribute def SoundExposureLevelUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-17 reverberation time */
    attribute reverberationTime: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 8-17 reverberation time
         * symbol(s): `T`
         * application domain: generic
         * name: ReverberationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time duration (ISO 80000-3) required for the space-averaged sound energy density (item 8-7) to decrease to `10^(−6)` of its initial value (i.e. for its level to decrease by `60 "[dB]"`) after the source emission has stopped
         * remarks: The reverberation time can be evaluated based on a dynamic range smaller than `60 "[dB]"` and extrapolated to a decay time of `60 "[dB]"`. It is then labelled accordingly `T_n`, where `n` is the dynamic range in `"[dB]"`. See also ISO 3382-1.
         */
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/isq_acoustics.md"
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
        (range (start 20 19) (end 20 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 19) (end 21 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 19) (end 22 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 19) (end 23 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 19) (end 24 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 19) (end 25 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 19) (end 26 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 19) (end 27 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 30 52) (end 30 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 22) (end 43 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 27) (end 43 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 22) (end 44 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 49 51) (end 49 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 30) (end 53 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 29) (end 69 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 85 64) (end 85 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 98 22) (end 98 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 99 22) (end 99 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 28) (end 99 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 105 60) (end 105 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 118 22) (end 118 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 22) (end 119 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 28) (end 119 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 125 64) (end 125 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 138 22) (end 138 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 139 22) (end 139 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 28) (end 139 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 30) (end 145 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 163 45) (end 163 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 176 22) (end 176 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 27) (end 176 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 177 22) (end 177 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 182 44) (end 182 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 36) (end 183 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 183 65) (end 183 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 183 76) (end 183 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 183 87) (end 183 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 34) (end 184 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 63) (end 184 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 74) (end 184 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 85) (end 184 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 38) (end 185 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 185 67) (end 185 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 185 78) (end 185 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 185 89) (end 185 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 186 22) (end 186 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 186 46) (end 186 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 27) (end 190 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 26) (end 206 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 222 41) (end 222 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 235 22) (end 235 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 235 27) (end 235 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 236 22) (end 236 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 241 40) (end 241 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 242 34) (end 242 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 242 63) (end 242 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 242 74) (end 242 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 242 85) (end 242 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 243 38) (end 243 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 243 67) (end 243 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 243 78) (end 243 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 243 89) (end 243 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 244 22) (end 244 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 244 46) (end 244 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 247 53) (end 247 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 260 22) (end 260 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 261 22) (end 261 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 266 62) (end 266 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 267 22) (end 267 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 268 22) (end 268 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 269 22) (end 269 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 273 40) (end 273 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 286 22) (end 286 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 27) (end 286 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 287 22) (end 287 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 292 39) (end 292 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 293 36) (end 293 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 293 65) (end 293 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 293 76) (end 293 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 293 87) (end 293 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 294 34) (end 294 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 294 63) (end 294 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 294 74) (end 294 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 294 85) (end 294 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 295 38) (end 295 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 295 67) (end 295 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 295 78) (end 295 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 295 89) (end 295 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 296 22) (end 296 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 296 46) (end 296 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 300 79) (end 300 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 313 22) (end 313 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 313 27) (end 313 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 314 22) (end 314 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 319 78) (end 319 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 320 36) (end 320 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 320 65) (end 320 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 320 76) (end 320 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 320 87) (end 320 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 34) (end 321 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 321 63) (end 321 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 321 74) (end 321 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 321 85) (end 321 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 322 38) (end 322 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 322 67) (end 322 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 322 78) (end 322 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 322 89) (end 322 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 323 22) (end 323 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 323 46) (end 323 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 327 44) (end 327 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 347 65) (end 347 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 347 76) (end 347 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 348 63) (end 348 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 348 74) (end 348 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 348 85) (end 348 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 349 38) (end 349 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 349 67) (end 349 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 349 78) (end 349 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 349 89) (end 349 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 350 22) (end 350 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 350 46) (end 350 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 354 45) (end 354 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 367 22) (end 367 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 367 27) (end 367 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 368 22) (end 368 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 373 44) (end 373 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 377 42) (end 377 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 390 22) (end 390 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 390 27) (end 390 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 391 22) (end 391 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 396 41) (end 396 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 400 45) (end 400 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 413 22) (end 413 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 413 27) (end 413 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 414 22) (end 414 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 419 44) (end 419 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 423 33) (end 423 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5f9973660df4abce2416fa65cd9bb620d81764cbf79b7019f0df075e9d13d3c4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQBase") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQMechanics::PowerValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQMechanics::PressureValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::CartesianSpatial3dCoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::SpeedValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::CartesianVelocity3dCoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::AccelerationValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQThermodynamics::EnergyValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::T"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::L"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::M"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcousticImpedanceUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "3dCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isOrthogonal"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundIntensityUnit")) (redefinition (reference "mRefs"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "3dVectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSoundIntensity3dCoordinateFrame")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "3dVectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianAcceleration3dCoordinateFrame")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "3dVectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSpatial3dCoordinateFrame")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "3dVectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVelocity3dCoordinateFrame")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::T"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::L"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::M"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LogarithmicFrequencyRangeUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::T"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::L"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::M"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundEnergyDensityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundExposureLevelUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::T"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::L"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::M"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundExposureUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::T"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::M"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundIntensityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundPowerLevelUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SoundPressureLevelUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AcousticImpedanceValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CartesianSoundIntensity3dVector"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CartesianSoundParticleAcceleration3dVector"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CartesianSoundParticleDisplacement3dVector"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CartesianSoundParticleVelocity3dVector"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LogarithmicFrequencyRangeValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::reverberationTime"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DurationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EnergyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SoundEnergyDensityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposure"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SoundExposureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SoundExposureLevelValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundIntensity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SoundIntensityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPower"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PowerValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SoundPowerLevelValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressure"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PressureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SoundPressureLevelValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::staticPressure"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PressureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeFlowRate"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "volumeVelocity"))))
    (declaration (id (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeVelocity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SpeedValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQBase")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQMechanics::PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQMechanics::PressureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQThermodynamics::EnergyValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::L")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "AcousticImpedanceUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (kind specialization) (ordinal 0))
      (authored-target "3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundIntensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "isOrthogonal")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (kind specialization) (ordinal 0))
      (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSoundIntensity3dCoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (kind specialization) (ordinal 0))
      (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianAcceleration3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (kind specialization) (ordinal 0))
      (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (kind specialization) (ordinal 0))
      (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVelocity3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::L")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "LogarithmicFrequencyRangeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::L")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundEnergyDensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundExposureLevelUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::L")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundExposureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundIntensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundPowerLevelUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundPressureLevelUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind featureTyping) (ordinal 0))
      (authored-target "AcousticImpedanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSoundIntensity3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSoundParticleAcceleration3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSoundParticleDisplacement3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSoundParticleVelocity3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind featureTyping) (ordinal 0))
      (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind featureTyping) (ordinal 0))
      (authored-target "LogarithmicFrequencyRangeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::reverberationTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergy"))) (kind featureTyping) (ordinal 0))
      (authored-target "EnergyValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundEnergyDensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposure"))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundExposureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundExposureLevelValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundIntensity"))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundIntensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundPowerLevelValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressure"))) (kind featureTyping) (ordinal 0))
      (authored-target "PressureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "SoundPressureLevelValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::staticPressure"))) (kind featureTyping) (ordinal 0))
      (authored-target "PressureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeFlowRate"))) (kind aliasBinding) (ordinal 0))
      (authored-target "volumeVelocity")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeVelocity")))))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeVelocity"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::acousticImpedance"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposure"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposureLevel"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundIntensity"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPowerLevel"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressureLevel"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeFlowRate"))) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeVelocity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeFlowRate"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer 1)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 15 19) (end 15 32)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 16 19) (end 16 43)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 17 19) (end 17 29)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQBase")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 14 19) (end 14 37)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 20 19) (end 20 43)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ISQMechanics::PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 21 19) (end 21 46)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ISQMechanics::PressureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 22 19) (end 22 66)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 23 19) (end 23 43)) (probe (position 23 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 24 19) (end 24 67)) (probe (position 24 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 25 19) (end 25 50)) (probe (position 25 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 26 19) (end 26 71)) (probe (position 26 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 27 19) (end 27 49)) (probe (position 27 19))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "ISQThermodynamics::EnergyValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 346 43) (end 346 54)) (probe (position 346 43))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 350 22) (end 350 39)) (probe (position 350 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 350 46) (end 350 66)) (probe (position 350 46))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 349 38) (end 349 57)) (probe (position 349 38))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 349 67) (end 349 75)) (probe (position 349 67))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 349 89) (end 349 97)) (probe (position 349 89))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 349 78) (end 349 83)) (probe (position 349 78))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::T")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 347 36) (end 347 55)) (probe (position 347 36))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 347 65) (end 347 73)) (probe (position 347 65))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 347 87) (end 347 95)) (probe (position 347 87))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 347 76) (end 347 81)) (probe (position 347 76))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::L")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 348 34) (end 348 53)) (probe (position 348 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 348 63) (end 348 71)) (probe (position 348 63))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 348 85) (end 348 93)) (probe (position 348 85))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 348 74) (end 348 79)) (probe (position 348 74))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 327 44) (end 327 63)) (probe (position 327 44))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 340 27) (end 340 31)) (probe (position 340 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 341 28) (end 341 49)) (probe (position 341 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "AcousticImpedanceUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 340 22) (end 340 25)) (probe (position 340 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 341 22) (end 341 26)) (probe (position 341 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 266 62) (end 266 81)) (probe (position 266 62))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (kind specialization) (ordinal 0) (authored-target "3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 269 29) (end 269 47)) (probe (position 269 29))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (kind featureTyping) (ordinal 0) (authored-target "SoundIntensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 267 22) (end 267 29)) (probe (position 267 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 268 22) (end 268 34)) (probe (position 268 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 269 22) (end 269 27)) (probe (position 269 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 247 53) (end 247 76)) (probe (position 247 53))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (kind specialization) (ordinal 0) (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 261 28) (end 261 68)) (probe (position 261 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSoundIntensity3dCoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 260 22) (end 260 29)) (probe (position 260 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 261 22) (end 261 26)) (probe (position 261 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 125 64) (end 125 87)) (probe (position 125 64))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (kind specialization) (ordinal 0) (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 139 28) (end 139 66)) (probe (position 139 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianAcceleration3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 138 22) (end 138 29)) (probe (position 138 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 139 22) (end 139 26)) (probe (position 139 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 85 64) (end 85 87)) (probe (position 85 64))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (kind specialization) (ordinal 0) (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 99 28) (end 99 61)) (probe (position 99 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 98 22) (end 98 29)) (probe (position 98 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 99 22) (end 99 26)) (probe (position 99 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 105 60) (end 105 83)) (probe (position 105 60))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (kind specialization) (ordinal 0) (authored-target "3dVectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 119 28) (end 119 62)) (probe (position 119 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVelocity3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 118 22) (end 118 29)) (probe (position 118 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 119 22) (end 119 26)) (probe (position 119 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 319 78) (end 319 89)) (probe (position 319 78))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 323 22) (end 323 39)) (probe (position 323 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 323 46) (end 323 66)) (probe (position 323 46))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 322 38) (end 322 57)) (probe (position 322 38))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 322 67) (end 322 75)) (probe (position 322 67))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 322 89) (end 322 97)) (probe (position 322 89))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 322 78) (end 322 83)) (probe (position 322 78))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::T")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 320 36) (end 320 55)) (probe (position 320 36))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 320 65) (end 320 73)) (probe (position 320 65))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 320 87) (end 320 95)) (probe (position 320 87))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 320 76) (end 320 81)) (probe (position 320 76))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::L")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 321 34) (end 321 53)) (probe (position 321 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 321 63) (end 321 71)) (probe (position 321 63))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 321 85) (end 321 93)) (probe (position 321 85))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 321 74) (end 321 79)) (probe (position 321 74))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 300 79) (end 300 98)) (probe (position 300 79))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 313 27) (end 313 31)) (probe (position 313 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 314 28) (end 314 84)) (probe (position 314 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 313 22) (end 313 25)) (probe (position 313 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 314 22) (end 314 26)) (probe (position 314 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 49 51) (end 49 67)) (probe (position 49 51))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 30 52) (end 30 71)) (probe (position 30 52))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 43 27) (end 43 31)) (probe (position 43 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 44 28) (end 44 57)) (probe (position 44 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "LogarithmicFrequencyRangeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 43 22) (end 43 25)) (probe (position 43 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 44 22) (end 44 26)) (probe (position 44 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 182 44) (end 182 55)) (probe (position 182 44))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 186 22) (end 186 39)) (probe (position 186 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 186 46) (end 186 66)) (probe (position 186 46))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 185 38) (end 185 57)) (probe (position 185 38))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 185 67) (end 185 75)) (probe (position 185 67))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 185 89) (end 185 97)) (probe (position 185 89))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 185 78) (end 185 83)) (probe (position 185 78))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::T")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 183 36) (end 183 55)) (probe (position 183 36))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 183 65) (end 183 73)) (probe (position 183 65))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 183 87) (end 183 95)) (probe (position 183 87))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 183 76) (end 183 81)) (probe (position 183 76))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::L")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 184 34) (end 184 53)) (probe (position 184 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 184 63) (end 184 71)) (probe (position 184 63))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 184 85) (end 184 93)) (probe (position 184 85))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 184 74) (end 184 79)) (probe (position 184 74))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 163 45) (end 163 64)) (probe (position 163 45))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 176 27) (end 176 31)) (probe (position 176 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 177 28) (end 177 50)) (probe (position 177 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SoundEnergyDensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 176 22) (end 176 25)) (probe (position 176 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 177 22) (end 177 26)) (probe (position 177 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 419 44) (end 419 60)) (probe (position 419 44))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 400 45) (end 400 64)) (probe (position 400 45))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 413 27) (end 413 31)) (probe (position 413 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 414 28) (end 414 50)) (probe (position 414 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SoundExposureLevelUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 413 22) (end 413 25)) (probe (position 413 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 414 22) (end 414 26)) (probe (position 414 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 292 39) (end 292 50)) (probe (position 292 39))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 296 22) (end 296 39)) (probe (position 296 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 296 46) (end 296 66)) (probe (position 296 46))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 295 38) (end 295 57)) (probe (position 295 38))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 295 67) (end 295 75)) (probe (position 295 67))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 295 89) (end 295 97)) (probe (position 295 89))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 295 78) (end 295 83)) (probe (position 295 78))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::T")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 293 36) (end 293 55)) (probe (position 293 36))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 293 65) (end 293 73)) (probe (position 293 65))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 293 87) (end 293 95)) (probe (position 293 87))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 293 76) (end 293 81)) (probe (position 293 76))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::L")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 294 34) (end 294 53)) (probe (position 294 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 294 63) (end 294 71)) (probe (position 294 63))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 294 85) (end 294 93)) (probe (position 294 85))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 294 74) (end 294 79)) (probe (position 294 74))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 273 40) (end 273 59)) (probe (position 273 40))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 286 27) (end 286 31)) (probe (position 286 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 287 28) (end 287 45)) (probe (position 287 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SoundExposureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 286 22) (end 286 25)) (probe (position 286 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 287 22) (end 287 26)) (probe (position 287 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 241 40) (end 241 51)) (probe (position 241 40))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 244 22) (end 244 39)) (probe (position 244 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 244 46) (end 244 66)) (probe (position 244 46))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 243 38) (end 243 57)) (probe (position 243 38))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 243 67) (end 243 75)) (probe (position 243 67))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 243 89) (end 243 97)) (probe (position 243 89))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 243 78) (end 243 83)) (probe (position 243 78))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::T")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 242 34) (end 242 53)) (probe (position 242 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 242 63) (end 242 71)) (probe (position 242 63))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 242 85) (end 242 93)) (probe (position 242 85))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 242 74) (end 242 79)) (probe (position 242 74))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 222 41) (end 222 60)) (probe (position 222 41))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 235 27) (end 235 31)) (probe (position 235 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 236 28) (end 236 46)) (probe (position 236 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SoundIntensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 235 22) (end 235 25)) (probe (position 235 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 236 22) (end 236 26)) (probe (position 236 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 396 41) (end 396 57)) (probe (position 396 41))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 377 42) (end 377 61)) (probe (position 377 42))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 390 27) (end 390 31)) (probe (position 390 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 391 28) (end 391 47)) (probe (position 391 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SoundPowerLevelUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 390 22) (end 390 25)) (probe (position 390 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 391 22) (end 391 26)) (probe (position 391 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 373 44) (end 373 60)) (probe (position 373 44))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 354 45) (end 354 64)) (probe (position 354 45))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 367 27) (end 367 31)) (probe (position 367 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 368 28) (end 368 50)) (probe (position 368 28))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SoundPressureLevelUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelUnit")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 367 22) (end 367 25)) (probe (position 367 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 368 22) (end 368 26)) (probe (position 368 22))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 344 33) (end 344 55)) (probe (position 344 33))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind featureTyping) (ordinal 0) (authored-target "AcousticImpedanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::AcousticImpedanceValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 264 47) (end 264 78)) (probe (position 264 47))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSoundIntensity3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 142 58) (end 142 100)) (probe (position 142 58))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSoundParticleAcceleration3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 102 58) (end 102 100)) (probe (position 102 58))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSoundParticleDisplacement3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 122 54) (end 122 92)) (probe (position 122 54))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSoundParticleVelocity3dVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 317 68) (end 317 125)) (probe (position 317 68))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind featureTyping) (ordinal 0) (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 47 41) (end 47 71)) (probe (position 47 41))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind featureTyping) (ordinal 0) (authored-target "LogarithmicFrequencyRangeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 423 33) (end 423 46)) (probe (position 423 33))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::reverberationTime"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 190 27) (end 190 38)) (probe (position 190 27))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergy"))) (kind featureTyping) (ordinal 0) (authored-target "EnergyValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 180 34) (end 180 57)) (probe (position 180 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind featureTyping) (ordinal 0) (authored-target "SoundEnergyDensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundEnergyDensityValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 290 29) (end 290 47)) (probe (position 290 29))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposure"))) (kind featureTyping) (ordinal 0) (authored-target "SoundExposureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 417 34) (end 417 57)) (probe (position 417 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind featureTyping) (ordinal 0) (authored-target "SoundExposureLevelValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundExposureLevelValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 239 30) (end 239 49)) (probe (position 239 30))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundIntensity"))) (kind featureTyping) (ordinal 0) (authored-target "SoundIntensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundIntensityValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 206 26) (end 206 36)) (probe (position 206 26))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPower"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 394 31) (end 394 51)) (probe (position 394 31))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind featureTyping) (ordinal 0) (authored-target "SoundPowerLevelValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPowerLevelValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 69 29) (end 69 42)) (probe (position 69 29))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressure"))) (kind featureTyping) (ordinal 0) (authored-target "PressureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 371 34) (end 371 57)) (probe (position 371 34))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind featureTyping) (ordinal 0) (authored-target "SoundPressureLevelValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::SoundPressureLevelValue")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 53 30) (end 53 43)) (probe (position 53 30))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::staticPressure"))) (kind featureTyping) (ordinal 0) (authored-target "PressureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 160 29) (end 160 43)) (probe (position 160 29))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeFlowRate"))) (kind aliasBinding) (ordinal 0) (authored-target "volumeVelocity")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeVelocity")))))
  )
  (query (document "memory://snapshot/isq_acoustics.md") (range (start 145 30) (end 145 40)) (probe (position 145 30))
    (reference (id (source (node (document "memory://snapshot/isq_acoustics.md") (qualified-name "ISQAcoustics::volumeVelocity"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
)
~~~
