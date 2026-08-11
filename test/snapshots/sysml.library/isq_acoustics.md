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
  (document "isq_acoustics.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 4) (end 30 1479))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 4) (end 49 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 4) (end 85 747))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 4) (end 105 901))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 4) (end 125 893))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 1267))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 182 4) (end 182 477))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 8) (end 183 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 8) (end 184 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 8) (end 185 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 4) (end 222 734))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 241 4) (end 241 359))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 242 8) (end 242 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 243 8) (end 243 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 247 4) (end 247 780))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 266 4) (end 266 223))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 273 4) (end 273 1046))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 292 4) (end 292 472))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 293 8) (end 293 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 294 8) (end 294 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 295 8) (end 295 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 300 4) (end 300 1260))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 319 4) (end 319 511))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 320 8) (end 320 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 8) (end 321 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 322 8) (end 322 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 4) (end 327 1182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 346 4) (end 346 476))
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
        (range (start 349 8) (end 349 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 354 4) (end 354 1863))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 373 4) (end 373 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 377 4) (end 377 1405))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 396 4) (end 396 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 400 4) (end 400 1550))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 419 4) (end 419 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 423 4) (end 423 888))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b0414fe086e5282e304ca2da44a1fb1724d740f31678a2ca2c1bd21bbcf028cf") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQAcoustics"))) (kind "package") (name "ISQAcoustics") (declared-name "ISQAcoustics") (range (start (line 0) (character 0)) (end (line 0) (character 26005))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AccelerationValue"))) (kind "import") (name "AccelerationValue") (declared-name "AccelerationValue") (range (start (line 25) (character 4)) (end (line 25) (character 51))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::AccelerationValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 25) (character 19)) (end (line 25) (character 50))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (kind "attribute def") (name "AcousticImpedanceUnit") (declared-name "AcousticImpedanceUnit") (range (start (line 346) (character 4)) (end (line 346) (character 476))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 349) (character 8)) (end (line 349) (character 105))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 347) (character 8)) (end (line 347) (character 103))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 348) (character 8)) (end (line 348) (character 100))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 350) (character 8)) (end (line 350) (character 102))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 350) (character 22)) (end (line 350) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (kind "attribute def") (name "AcousticImpedanceValue") (declared-name "AcousticImpedanceValue") (range (start (line 327) (character 4)) (end (line 327) (character 1182))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 327) (character 4)) (end (line 327) (character 1182))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 341) (character 8)) (end (line 341) (character 53))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcousticImpedanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 341) (character 22)) (end (line 341) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 340) (character 8)) (end (line 340) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 340) (character 22)) (end (line 340) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianAcceleration3dCoordinateFrame"))) (kind "import") (name "CartesianAcceleration3dCoordinateFrame") (declared-name "CartesianAcceleration3dCoordinateFrame") (range (start (line 26) (character 4)) (end (line 26) (character 72))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 26) (character 19)) (end (line 26) (character 71))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (kind "attribute def") (name "CartesianSoundIntensity3dCoordinateFrame") (declared-name "CartesianSoundIntensity3dCoordinateFrame") (range (start (line 266) (character 4)) (end (line 266) (character 223))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 267) (character 8)) (end (line 267) (character 38))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 267) (character 22)) (end (line 267) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 268) (character 8)) (end (line 268) (character 42))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 268) (character 22)) (end (line 268) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 269) (character 8)) (end (line 269) (character 51))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundIntensityUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 269) (character 22)) (end (line 269) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (kind "attribute def") (name "CartesianSoundIntensity3dVector") (declared-name "CartesianSoundIntensity3dVector") (range (start (line 247) (character 4)) (end (line 247) (character 780))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 247) (character 4)) (end (line 247) (character 780))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 260) (character 8)) (end (line 260) (character 38))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 260) (character 22)) (end (line 260) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 261) (character 8)) (end (line 261) (character 72))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSoundIntensity3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 261) (character 22)) (end (line 261) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (kind "attribute def") (name "CartesianSoundParticleAcceleration3dVector") (declared-name "CartesianSoundParticleAcceleration3dVector") (range (start (line 125) (character 4)) (end (line 125) (character 893))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 125) (character 4)) (end (line 125) (character 893))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 138) (character 8)) (end (line 138) (character 38))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 138) (character 22)) (end (line 138) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 139) (character 8)) (end (line 139) (character 70))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAcceleration3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 139) (character 22)) (end (line 139) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (kind "attribute def") (name "CartesianSoundParticleDisplacement3dVector") (declared-name "CartesianSoundParticleDisplacement3dVector") (range (start (line 85) (character 4)) (end (line 85) (character 747))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 85) (character 4)) (end (line 85) (character 747))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 98) (character 8)) (end (line 98) (character 38))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 98) (character 22)) (end (line 98) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 99) (character 8)) (end (line 99) (character 65))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 99) (character 22)) (end (line 99) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (kind "attribute def") (name "CartesianSoundParticleVelocity3dVector") (declared-name "CartesianSoundParticleVelocity3dVector") (range (start (line 105) (character 4)) (end (line 105) (character 901))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 105) (character 4)) (end (line 105) (character 901))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 118) (character 8)) (end (line 118) (character 38))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 118) (character 22)) (end (line 118) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 119) (character 8)) (end (line 119) (character 66))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianVelocity3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 119) (character 22)) (end (line 119) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSpatial3dCoordinateFrame"))) (kind "import") (name "CartesianSpatial3dCoordinateFrame") (declared-name "CartesianSpatial3dCoordinateFrame") (range (start (line 22) (character 4)) (end (line 22) (character 67))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::CartesianSpatial3dCoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 22) (character 19)) (end (line 22) (character 66))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianVelocity3dCoordinateFrame"))) (kind "import") (name "CartesianVelocity3dCoordinateFrame") (declared-name "CartesianVelocity3dCoordinateFrame") (range (start (line 24) (character 4)) (end (line 24) (character 68))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::CartesianVelocity3dCoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 24) (character 19)) (end (line 24) (character 67))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (kind "attribute def") (name "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (declared-name "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (range (start (line 319) (character 4)) (end (line 319) (character 511))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 322) (character 8)) (end (line 322) (character 105))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 320) (character 8)) (end (line 320) (character 103))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 321) (character 8)) (end (line 321) (character 100))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 323) (character 8)) (end (line 323) (character 102))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 323) (character 22)) (end (line 323) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (kind "attribute def") (name "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (declared-name "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (range (start (line 300) (character 4)) (end (line 300) (character 1260))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::_documentation"))) (kind "documentation") (name "") (range (start (line 300) (character 4)) (end (line 300) (character 1260))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 314) (character 8)) (end (line 314) (character 88))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (range none)) (redefinition (reference "mRef") (range (start (line 314) (character 22)) (end (line 314) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 313) (character 8)) (end (line 313) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 313) (character 22)) (end (line 313) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::EnergyValue"))) (kind "import") (name "EnergyValue") (declared-name "EnergyValue") (range (start (line 27) (character 4)) (end (line 27) (character 50))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQThermodynamics::EnergyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 27) (character 19)) (end (line 27) (character 49))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (kind "attribute def") (name "LogarithmicFrequencyRangeUnit") (declared-name "LogarithmicFrequencyRangeUnit") (range (start (line 49) (character 4)) (end (line 49) (character 75))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (kind "attribute def") (name "LogarithmicFrequencyRangeValue") (declared-name "LogarithmicFrequencyRangeValue") (range (start (line 30) (character 4)) (end (line 30) (character 1479))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 30) (character 4)) (end (line 30) (character 1479))) (parent (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 44) (character 8)) (end (line 44) (character 61))) (parent (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LogarithmicFrequencyRangeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 44) (character 22)) (end (line 44) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 43) (character 8)) (end (line 43) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 43) (character 22)) (end (line 43) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::PowerValue"))) (kind "import") (name "PowerValue") (declared-name "PowerValue") (range (start (line 20) (character 4)) (end (line 20) (character 44))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQMechanics::PowerValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 19)) (end (line 20) (character 43))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::PressureValue"))) (kind "import") (name "PressureValue") (declared-name "PressureValue") (range (start (line 21) (character 4)) (end (line 21) (character 47))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQMechanics::PressureValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 19)) (end (line 21) (character 46))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (kind "attribute def") (name "SoundEnergyDensityUnit") (declared-name "SoundEnergyDensityUnit") (range (start (line 182) (character 4)) (end (line 182) (character 477))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 185) (character 8)) (end (line 185) (character 105))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 183) (character 8)) (end (line 183) (character 103))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 184) (character 8)) (end (line 184) (character 100))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 186) (character 8)) (end (line 186) (character 102))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 186) (character 22)) (end (line 186) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (kind "attribute def") (name "SoundEnergyDensityValue") (declared-name "SoundEnergyDensityValue") (range (start (line 163) (character 4)) (end (line 163) (character 1267))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 163) (character 4)) (end (line 163) (character 1267))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 177) (character 8)) (end (line 177) (character 54))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundEnergyDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 177) (character 22)) (end (line 177) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 176) (character 8)) (end (line 176) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 176) (character 22)) (end (line 176) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (kind "attribute def") (name "SoundExposureLevelUnit") (declared-name "SoundExposureLevelUnit") (range (start (line 419) (character 4)) (end (line 419) (character 68))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (kind "attribute def") (name "SoundExposureLevelValue") (declared-name "SoundExposureLevelValue") (range (start (line 400) (character 4)) (end (line 400) (character 1550))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::_documentation"))) (kind "documentation") (name "") (range (start (line 400) (character 4)) (end (line 400) (character 1550))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 414) (character 8)) (end (line 414) (character 54))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundExposureLevelUnit") (range none)) (redefinition (reference "mRef") (range (start (line 414) (character 22)) (end (line 414) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 413) (character 8)) (end (line 413) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 413) (character 22)) (end (line 413) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (kind "attribute def") (name "SoundExposureUnit") (declared-name "SoundExposureUnit") (range (start (line 292) (character 4)) (end (line 292) (character 472))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 295) (character 8)) (end (line 295) (character 105))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 293) (character 8)) (end (line 293) (character 103))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 294) (character 8)) (end (line 294) (character 100))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 296) (character 8)) (end (line 296) (character 102))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 296) (character 22)) (end (line 296) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))) (kind "attribute def") (name "SoundExposureValue") (declared-name "SoundExposureValue") (range (start (line 273) (character 4)) (end (line 273) (character 1046))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 273) (character 4)) (end (line 273) (character 1046))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 287) (character 8)) (end (line 287) (character 49))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundExposureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 287) (character 22)) (end (line 287) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 286) (character 8)) (end (line 286) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 286) (character 22)) (end (line 286) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (kind "attribute def") (name "SoundIntensityUnit") (declared-name "SoundIntensityUnit") (range (start (line 241) (character 4)) (end (line 241) (character 359))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 243) (character 8)) (end (line 243) (character 105))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 242) (character 8)) (end (line 242) (character 100))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 244) (character 8)) (end (line 244) (character 92))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 244) (character 22)) (end (line 244) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (kind "attribute def") (name "SoundIntensityValue") (declared-name "SoundIntensityValue") (range (start (line 222) (character 4)) (end (line 222) (character 734))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 222) (character 4)) (end (line 222) (character 734))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 236) (character 8)) (end (line 236) (character 50))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundIntensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 236) (character 22)) (end (line 236) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 235) (character 8)) (end (line 235) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 235) (character 22)) (end (line 235) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (kind "attribute def") (name "SoundPowerLevelUnit") (declared-name "SoundPowerLevelUnit") (range (start (line 396) (character 4)) (end (line 396) (character 65))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (kind "attribute def") (name "SoundPowerLevelValue") (declared-name "SoundPowerLevelValue") (range (start (line 377) (character 4)) (end (line 377) (character 1405))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::_documentation"))) (kind "documentation") (name "") (range (start (line 377) (character 4)) (end (line 377) (character 1405))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 391) (character 8)) (end (line 391) (character 51))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundPowerLevelUnit") (range none)) (redefinition (reference "mRef") (range (start (line 391) (character 22)) (end (line 391) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 390) (character 8)) (end (line 390) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 390) (character 22)) (end (line 390) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (kind "attribute def") (name "SoundPressureLevelUnit") (declared-name "SoundPressureLevelUnit") (range (start (line 373) (character 4)) (end (line 373) (character 68))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (kind "attribute def") (name "SoundPressureLevelValue") (declared-name "SoundPressureLevelValue") (range (start (line 354) (character 4)) (end (line 354) (character 1863))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::_documentation"))) (kind "documentation") (name "") (range (start (line 354) (character 4)) (end (line 354) (character 1863))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 368) (character 8)) (end (line 368) (character 54))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SoundPressureLevelUnit") (range none)) (redefinition (reference "mRef") (range (start (line 368) (character 22)) (end (line 368) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 367) (character 8)) (end (line 367) (character 32))) (parent (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 367) (character 22)) (end (line 367) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::SpeedValue"))) (kind "import") (name "SpeedValue") (declared-name "SpeedValue") (range (start (line 23) (character 4)) (end (line 23) (character 44))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::SpeedValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 23) (character 19)) (end (line 23) (character 43))))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 26005))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind "attribute def") (name "acousticImpedance") (declared-name "acousticImpedance") (range (start (line 344) (character 4)) (end (line 344) (character 89))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "AcousticImpedanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind "attribute def") (name "cartesianSoundIntensity3dVector") (declared-name "cartesianSoundIntensity3dVector") (range (start (line 264) (character 4)) (end (line 264) (character 99))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSoundIntensity3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind "attribute def") (name "cartesianSoundParticleAcceleration3dVector") (declared-name "cartesianSoundParticleAcceleration3dVector") (range (start (line 142) (character 4)) (end (line 142) (character 121))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSoundParticleAcceleration3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind "attribute def") (name "cartesianSoundParticleDisplacement3dVector") (declared-name "cartesianSoundParticleDisplacement3dVector") (range (start (line 102) (character 4)) (end (line 102) (character 121))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSoundParticleDisplacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind "attribute def") (name "cartesianSoundParticleVelocity3dVector") (declared-name "cartesianSoundParticleVelocity3dVector") (range (start (line 122) (character 4)) (end (line 122) (character 113))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSoundParticleVelocity3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind "attribute def") (name "characteristicImpedanceOfAMediumForLongitudinalWaves") (declared-name "characteristicImpedanceOfAMediumForLongitudinalWaves") (range (start (line 317) (character 4)) (end (line 317) (character 159))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind "attribute def") (name "logarithmicFrequencyRange") (declared-name "logarithmicFrequencyRange") (range (start (line 47) (character 4)) (end (line 47) (character 105))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicFrequencyRangeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime"))) (kind "attribute def") (name "reverberationTime") (declared-name "reverberationTime") (range (start (line 423) (character 4)) (end (line 423) (character 888))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime::_documentation"))) (kind "documentation") (name "") (range (start (line 423) (character 4)) (end (line 423) (character 888))) (parent (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))) (kind "attribute def") (name "soundEnergy") (declared-name "soundEnergy") (range (start (line 190) (character 4)) (end (line 190) (character 600))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 190) (character 4)) (end (line 190) (character 600))) (parent (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind "attribute def") (name "soundEnergyDensity") (declared-name "soundEnergyDensity") (range (start (line 180) (character 4)) (end (line 180) (character 91))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundEnergyDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundExposure"))) (kind "attribute def") (name "soundExposure") (declared-name "soundExposure") (range (start (line 290) (character 4)) (end (line 290) (character 81))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundExposureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind "attribute def") (name "soundExposureLevel") (declared-name "soundExposureLevel") (range (start (line 417) (character 4)) (end (line 417) (character 91))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundExposureLevelValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundIntensity"))) (kind "attribute def") (name "soundIntensity") (declared-name "soundIntensity") (range (start (line 239) (character 4)) (end (line 239) (character 83))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundIntensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))) (kind "attribute def") (name "soundPower") (declared-name "soundPower") (range (start (line 206) (character 4)) (end (line 206) (character 1123))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundPower::_documentation"))) (kind "documentation") (name "") (range (start (line 206) (character 4)) (end (line 206) (character 1123))) (parent (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind "attribute def") (name "soundPowerLevel") (declared-name "soundPowerLevel") (range (start (line 394) (character 4)) (end (line 394) (character 85))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundPowerLevelValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))) (kind "attribute def") (name "soundPressure") (declared-name "soundPressure") (range (start (line 69) (character 4)) (end (line 69) (character 520))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundPressure::_documentation"))) (kind "documentation") (name "") (range (start (line 69) (character 4)) (end (line 69) (character 520))) (parent (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind "attribute def") (name "soundPressureLevel") (declared-name "soundPressureLevel") (range (start (line 371) (character 4)) (end (line 371) (character 91))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundPressureLevelValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))) (kind "attribute def") (name "staticPressure") (declared-name "staticPressure") (range (start (line 53) (character 4)) (end (line 53) (character 555))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::staticPressure::_documentation"))) (kind "documentation") (name "") (range (start (line 53) (character 4)) (end (line 53) (character 555))) (parent (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::volumeFlowRate"))) (kind "alias") (name "volumeFlowRate") (declared-name "volumeFlowRate") (range (start (line 160) (character 4)) (end (line 160) (character 44))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))) (kind "attribute def") (name "volumeVelocity") (declared-name "volumeVelocity") (range (start (line 145) (character 4)) (end (line 145) (character 552))) (parent (node (document "d0") (qualified-name "ISQAcoustics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity::_documentation"))) (kind "documentation") (name "") (range (start (line 145) (character 4)) (end (line 145) (character 552))) (parent (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AccelerationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::AccelerationValue") (range (start (line 25) (character 19)) (end (line 25) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 350) (character 22)) (end (line 350) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AcousticImpedanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 341) (character 22)) (end (line 341) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 340) (character 22)) (end (line 340) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianAcceleration3dCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame") (range (start (line 26) (character 19)) (end (line 26) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 267) (character 22)) (end (line 267) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 268) (character 22)) (end (line 268) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundIntensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 269) (character 22)) (end (line 269) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 260) (character 22)) (end (line 260) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSoundIntensity3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 261) (character 22)) (end (line 261) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 138) (character 22)) (end (line 138) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAcceleration3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianAcceleration3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 139) (character 22)) (end (line 139) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 98) (character 22)) (end (line 98) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 99) (character 22)) (end (line 99) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 118) (character 22)) (end (line 118) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianVelocity3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 119) (character 22)) (end (line 119) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSpatial3dCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::CartesianSpatial3dCoordinateFrame") (range (start (line 22) (character 19)) (end (line 22) (character 66))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianVelocity3dCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::CartesianVelocity3dCoordinateFrame") (range (start (line 24) (character 19)) (end (line 24) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 323) (character 22)) (end (line 323) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 314) (character 22)) (end (line 314) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 313) (character 22)) (end (line 313) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::EnergyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQThermodynamics::EnergyValue") (range (start (line 27) (character 19)) (end (line 27) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicFrequencyRangeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 44) (character 22)) (end (line 44) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 43) (character 22)) (end (line 43) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::PowerValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQMechanics::PowerValue") (range (start (line 20) (character 19)) (end (line 20) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::PressureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQMechanics::PressureValue") (range (start (line 21) (character 19)) (end (line 21) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 186) (character 22)) (end (line 186) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundEnergyDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 177) (character 22)) (end (line 177) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 176) (character 22)) (end (line 176) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureLevelUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 414) (character 22)) (end (line 414) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 413) (character 22)) (end (line 413) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 296) (character 22)) (end (line 296) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 287) (character 22)) (end (line 287) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 286) (character 22)) (end (line 286) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 244) (character 22)) (end (line 244) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundIntensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 236) (character 22)) (end (line 236) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 235) (character 22)) (end (line 235) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundPowerLevelUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 391) (character 22)) (end (line 391) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 390) (character 22)) (end (line 390) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundPressureLevelUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 368) (character 22)) (end (line 368) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 367) (character 22)) (end (line 367) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::SpeedValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::SpeedValue") (range (start (line 23) (character 19)) (end (line 23) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind featureTyping) (ordinal 0)) (authored-target "AcousticImpedanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSoundIntensity3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSoundParticleAcceleration3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSoundParticleDisplacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSoundParticleVelocity3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind featureTyping) (ordinal 0)) (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicFrequencyRangeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundEnergyDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundExposure"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureLevelValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundIntensity"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundIntensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::PowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundPowerLevelValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::PressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundPressureLevelValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::PressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQAcoustics::SpeedValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianAcceleration3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianVelocity3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::acousticImpedance"))) (target (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::acousticImpedance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (target (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (target (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))) (target (node (document "d0") (qualified-name "ISQAcoustics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundExposure"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundExposure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundExposureLevel"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundExposureLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundIntensity"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))) (target (node (document "d0") (qualified-name "ISQAcoustics::PowerValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundPowerLevel"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundPowerLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))) (target (node (document "d0") (qualified-name "ISQAcoustics::PressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::soundPressureLevel"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::soundPressureLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))) (target (node (document "d0") (qualified-name "ISQAcoustics::PressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))) (target (node (document "d0") (qualified-name "ISQAcoustics::SpeedValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 43 22) (end 43 25)) (probe (position 43 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 43 22) (end 43 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num") (range (start 43 8) (end 43 32)))
        )
      )
    )
    (query (range (start 176 22) (end 176 25)) (probe (position 176 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 176 22) (end 176 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num") (range (start 176 8) (end 176 32)))
        )
      )
    )
    (query (range (start 235 22) (end 235 25)) (probe (position 235 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 235 22) (end 235 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num") (range (start 235 8) (end 235 32)))
        )
      )
    )
    (query (range (start 286 22) (end 286 25)) (probe (position 286 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 286 22) (end 286 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num") (range (start 286 8) (end 286 32)))
        )
      )
    )
    (query (range (start 313 22) (end 313 25)) (probe (position 313 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 313 22) (end 313 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num") (range (start 313 8) (end 313 32)))
        )
      )
    )
    (query (range (start 340 22) (end 340 25)) (probe (position 340 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 340 22) (end 340 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num") (range (start 340 8) (end 340 32)))
        )
      )
    )
    (query (range (start 367 22) (end 367 25)) (probe (position 367 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 367 22) (end 367 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num") (range (start 367 8) (end 367 32)))
        )
      )
    )
    (query (range (start 390 22) (end 390 25)) (probe (position 390 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 390 22) (end 390 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num") (range (start 390 8) (end 390 32)))
        )
      )
    )
    (query (range (start 413 22) (end 413 25)) (probe (position 413 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 413 22) (end 413 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num") (range (start 413 8) (end 413 32)))
        )
      )
    )
    (query (range (start 44 22) (end 44 26)) (probe (position 44 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 44 22) (end 44 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef") (range (start 44 8) (end 44 61)))
        )
      )
    )
    (query (range (start 99 22) (end 99 26)) (probe (position 99 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 99 22) (end 99 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef") (range (start 99 8) (end 99 65)))
        )
      )
    )
    (query (range (start 119 22) (end 119 26)) (probe (position 119 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 119 22) (end 119 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef") (range (start 119 8) (end 119 66)))
        )
      )
    )
    (query (range (start 139 22) (end 139 26)) (probe (position 139 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 139 22) (end 139 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef") (range (start 139 8) (end 139 70)))
        )
      )
    )
    (query (range (start 177 22) (end 177 26)) (probe (position 177 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 177 22) (end 177 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef") (range (start 177 8) (end 177 54)))
        )
      )
    )
    (query (range (start 236 22) (end 236 26)) (probe (position 236 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 236 22) (end 236 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef") (range (start 236 8) (end 236 50)))
        )
      )
    )
    (query (range (start 261 22) (end 261 26)) (probe (position 261 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 261 22) (end 261 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef") (range (start 261 8) (end 261 72)))
        )
      )
    )
    (query (range (start 287 22) (end 287 26)) (probe (position 287 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 287 22) (end 287 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef") (range (start 287 8) (end 287 49)))
        )
      )
    )
    (query (range (start 314 22) (end 314 26)) (probe (position 314 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 314 22) (end 314 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef") (range (start 314 8) (end 314 88)))
        )
      )
    )
    (query (range (start 341 22) (end 341 26)) (probe (position 341 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 341 22) (end 341 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef") (range (start 341 8) (end 341 53)))
        )
      )
    )
    (query (range (start 368 22) (end 368 26)) (probe (position 368 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 368 22) (end 368 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef") (range (start 368 8) (end 368 54)))
        )
      )
    )
    (query (range (start 391 22) (end 391 26)) (probe (position 391 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 391 22) (end 391 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef") (range (start 391 8) (end 391 51)))
        )
      )
    )
    (query (range (start 414 22) (end 414 26)) (probe (position 414 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 414 22) (end 414 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef") (range (start 414 8) (end 414 54)))
        )
      )
    )
    (query (range (start 269 22) (end 269 27)) (probe (position 269 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 269 22) (end 269 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs") (range (start 269 8) (end 269 51)))
        )
      )
    )
    (query (range (start 17 19) (end 17 26)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQBase::*")
        (range (start 17 19) (end 17 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 98 22) (end 98 29)) (probe (position 98 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 98 22) (end 98 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound") (range (start 98 8) (end 98 38)))
        )
      )
    )
    (query (range (start 118 22) (end 118 29)) (probe (position 118 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 118 22) (end 118 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound") (range (start 118 8) (end 118 38)))
        )
      )
    )
    (query (range (start 138 22) (end 138 29)) (probe (position 138 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 138 22) (end 138 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound") (range (start 138 8) (end 138 38)))
        )
      )
    )
    (query (range (start 260 22) (end 260 29)) (probe (position 260 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 260 22) (end 260 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound") (range (start 260 8) (end 260 38)))
        )
      )
    )
    (query (range (start 267 22) (end 267 29)) (probe (position 267 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 267 22) (end 267 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound") (range (start 267 8) (end 267 38)))
        )
      )
    )
    (query (range (start 15 19) (end 15 29)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 15 19) (end 15 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 268 22) (end 268 34)) (probe (position 268 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 268 22) (end 268 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal") (range (start 268 8) (end 268 42)))
        )
      )
    )
    (query (range (start 186 22) (end 186 39)) (probe (position 186 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 186 22) (end 186 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension") (range (start 186 8) (end 186 102)))
        )
      )
    )
    (query (range (start 244 22) (end 244 39)) (probe (position 244 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 244 22) (end 244 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension") (range (start 244 8) (end 244 92)))
        )
      )
    )
    (query (range (start 296 22) (end 296 39)) (probe (position 296 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 296 22) (end 296 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension") (range (start 296 8) (end 296 102)))
        )
      )
    )
    (query (range (start 323 22) (end 323 39)) (probe (position 323 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 323 22) (end 323 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension") (range (start 323 8) (end 323 102)))
        )
      )
    )
    (query (range (start 350 22) (end 350 39)) (probe (position 350 22))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 350 22) (end 350 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension") (range (start 350 8) (end 350 102)))
        )
      )
    )
    (query (range (start 14 19) (end 14 37)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 14 19) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 40)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 16 19) (end 16 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 19) (end 20 43)) (probe (position 20 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::PowerValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQMechanics::PowerValue")
        (range (start 20 19) (end 20 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 19) (end 23 43)) (probe (position 23 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::SpeedValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::SpeedValue")
        (range (start 23 19) (end 23 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 19) (end 21 46)) (probe (position 21 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::PressureValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQMechanics::PressureValue")
        (range (start 21 19) (end 21 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 19) (end 27 49)) (probe (position 27 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::EnergyValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQThermodynamics::EnergyValue")
        (range (start 27 19) (end 27 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 19) (end 25 50)) (probe (position 25 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::AccelerationValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::AccelerationValue")
        (range (start 25 19) (end 25 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 19) (end 22 66)) (probe (position 22 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianSpatial3dCoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")
        (range (start 22 19) (end 22 66))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 19) (end 24 67)) (probe (position 24 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianVelocity3dCoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")
        (range (start 24 19) (end 24 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 19) (end 26 71)) (probe (position 26 19))
      (reference
        (source (document "d0") (qualified-name "ISQAcoustics::CartesianAcceleration3dCoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")
        (range (start 26 19) (end 26 71))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
