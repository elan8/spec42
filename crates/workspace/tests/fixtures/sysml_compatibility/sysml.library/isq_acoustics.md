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
# EXPECTED
~~~
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'PressureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PressureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianVelocity3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianAcceleration3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'SpeedValue'
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
semantic.unresolved_name 'PowerValue'
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
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
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
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'PressureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PressureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianSpatial3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianVelocity3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'CartesianAcceleration3dCoordinateFrame'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'SpeedValue'
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
semantic.unresolved_name 'PowerValue'
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
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
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
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQAcoustics'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQMechanics::PowerValue')
    (import_decl private 'ISQMechanics::PressureValue')
    (import_decl private 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame')
    (import_decl private 'ISQSpaceTime::SpeedValue')
    (import_decl private 'ISQSpaceTime::CartesianVelocity3dCoordinateFrame')
    (import_decl private 'ISQSpaceTime::AccelerationValue')
    (import_decl private 'ISQSpaceTime::CartesianAcceleration3dCoordinateFrame')
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (attribute_def 'LogarithmicFrequencyRangeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LogarithmicFrequencyRangeUnit' multiplicity))
    (attribute_usage 'logarithmicFrequencyRange' : 'LogarithmicFrequencyRangeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LogarithmicFrequencyRangeUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_usage 'staticPressure' : 'PressureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'soundPressure' : 'PressureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'CartesianSoundParticleDisplacement3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianSoundParticleDisplacement3dVector' : 'CartesianSoundParticleDisplacement3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianSoundParticleVelocity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianVelocity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianSoundParticleVelocity3dVector' : 'CartesianSoundParticleVelocity3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianSoundParticleAcceleration3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAcceleration3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianSoundParticleAcceleration3dVector' : 'CartesianSoundParticleAcceleration3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_usage 'volumeVelocity' : 'SpeedValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'volumeFlowRate' for 'volumeVelocity')
    (comment)
    (attribute_def 'SoundEnergyDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SoundEnergyDensityUnit' multiplicity))
    (attribute_usage 'soundEnergyDensity' : 'SoundEnergyDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SoundEnergyDensityUnit' :> 'DerivedUnit'
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
    (attribute_usage 'soundEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'soundPower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'SoundIntensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SoundIntensityUnit' multiplicity))
    (attribute_usage 'soundIntensity' : 'SoundIntensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SoundIntensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianSoundIntensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianSoundIntensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianSoundIntensity3dVector' : 'CartesianSoundIntensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianSoundIntensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'SoundIntensityUnit' multiplicity))
    (comment)
    (attribute_def 'SoundExposureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SoundExposureUnit' multiplicity))
    (attribute_usage 'soundExposure' : 'SoundExposureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SoundExposureUnit' :> 'DerivedUnit'
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
    (attribute_def 'CharacteristicImpedanceOfAMediumForLongitudinalWavesValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit' multiplicity))
    (attribute_usage 'characteristicImpedanceOfAMediumForLongitudinalWaves' : 'CharacteristicImpedanceOfAMediumForLongitudinalWavesValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit' :> 'DerivedUnit'
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
    (attribute_def 'AcousticImpedanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AcousticImpedanceUnit' multiplicity))
    (attribute_usage 'acousticImpedance' : 'AcousticImpedanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AcousticImpedanceUnit' :> 'DerivedUnit'
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
    (attribute_def 'SoundPressureLevelValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SoundPressureLevelUnit' multiplicity))
    (attribute_usage 'soundPressureLevel' : 'SoundPressureLevelValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SoundPressureLevelUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'SoundPowerLevelValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SoundPowerLevelUnit' multiplicity))
    (attribute_usage 'soundPowerLevel' : 'SoundPowerLevelValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SoundPowerLevelUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_def 'SoundExposureLevelValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SoundExposureLevelUnit' multiplicity))
    (attribute_usage 'soundExposureLevel' : 'SoundExposureLevelValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SoundExposureLevelUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_usage 'reverberationTime' : 'DurationValue' :> 'scalarQuantities'
      (documentation))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQAcoustics"))) (name "ISQAcoustics") (declared-name "ISQAcoustics")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::AccelerationValue"))) (name "AccelerationValue") (declared-name "AccelerationValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))) (name "AcousticImpedanceUnit") (declared-name "AcousticImpedanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))) (name "AcousticImpedanceValue") (declared-name "AcousticImpedanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianAcceleration3dCoordinateFrame"))) (name "CartesianAcceleration3dCoordinateFrame") (declared-name "CartesianAcceleration3dCoordinateFrame"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))) (name "CartesianSoundIntensity3dCoordinateFrame") (declared-name "CartesianSoundIntensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))) (name "CartesianSoundIntensity3dVector") (declared-name "CartesianSoundIntensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))) (name "CartesianSoundParticleAcceleration3dVector") (declared-name "CartesianSoundParticleAcceleration3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))) (name "CartesianSoundParticleDisplacement3dVector") (declared-name "CartesianSoundParticleDisplacement3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))) (name "CartesianSoundParticleVelocity3dVector") (declared-name "CartesianSoundParticleVelocity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianSpatial3dCoordinateFrame"))) (name "CartesianSpatial3dCoordinateFrame") (declared-name "CartesianSpatial3dCoordinateFrame"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::CartesianVelocity3dCoordinateFrame"))) (name "CartesianVelocity3dCoordinateFrame") (declared-name "CartesianVelocity3dCoordinateFrame"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))) (name "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (declared-name "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))) (name "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (declared-name "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))) (name "LogarithmicFrequencyRangeUnit") (declared-name "LogarithmicFrequencyRangeUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))) (name "LogarithmicFrequencyRangeValue") (declared-name "LogarithmicFrequencyRangeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::PowerValue"))) (name "PowerValue") (declared-name "PowerValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::PressureValue"))) (name "PressureValue") (declared-name "PressureValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))) (name "SoundEnergyDensityUnit") (declared-name "SoundEnergyDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))) (name "SoundEnergyDensityValue") (declared-name "SoundEnergyDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))) (name "SoundExposureLevelUnit") (declared-name "SoundExposureLevelUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))) (name "SoundExposureLevelValue") (declared-name "SoundExposureLevelValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))) (name "SoundExposureUnit") (declared-name "SoundExposureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))) (name "SoundExposureValue") (declared-name "SoundExposureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))) (name "SoundIntensityUnit") (declared-name "SoundIntensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))) (name "SoundIntensityValue") (declared-name "SoundIntensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))) (name "SoundPowerLevelUnit") (declared-name "SoundPowerLevelUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))) (name "SoundPowerLevelValue") (declared-name "SoundPowerLevelValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))) (name "SoundPressureLevelUnit") (declared-name "SoundPressureLevelUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))) (name "SoundPressureLevelValue") (declared-name "SoundPressureLevelValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQAcoustics::SpeedValue"))) (name "SpeedValue") (declared-name "SpeedValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::acousticImpedance"))) (name "acousticImpedance") (declared-name "acousticImpedance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (name "cartesianSoundIntensity3dVector") (declared-name "cartesianSoundIntensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (name "cartesianSoundParticleAcceleration3dVector") (declared-name "cartesianSoundParticleAcceleration3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (name "cartesianSoundParticleDisplacement3dVector") (declared-name "cartesianSoundParticleDisplacement3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (name "cartesianSoundParticleVelocity3dVector") (declared-name "cartesianSoundParticleVelocity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (name "characteristicImpedanceOfAMediumForLongitudinalWaves") (declared-name "characteristicImpedanceOfAMediumForLongitudinalWaves") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (name "logarithmicFrequencyRange") (declared-name "logarithmicFrequencyRange") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime"))) (name "reverberationTime") (declared-name "reverberationTime") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))) (name "soundEnergy") (declared-name "soundEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (name "soundEnergyDensity") (declared-name "soundEnergyDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundExposure"))) (name "soundExposure") (declared-name "soundExposure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundExposureLevel"))) (name "soundExposureLevel") (declared-name "soundExposureLevel") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundIntensity"))) (name "soundIntensity") (declared-name "soundIntensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))) (name "soundPower") (declared-name "soundPower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::soundPower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::soundPower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundPowerLevel"))) (name "soundPowerLevel") (declared-name "soundPowerLevel") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))) (name "soundPressure") (declared-name "soundPressure") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::soundPressure::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::soundPressure")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::soundPressureLevel"))) (name "soundPressureLevel") (declared-name "soundPressureLevel") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))) (name "staticPressure") (declared-name "staticPressure") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::staticPressure::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::staticPressure")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQAcoustics::volumeFlowRate"))) (name "volumeFlowRate") (declared-name "volumeFlowRate"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))) (name "volumeVelocity") (declared-name "volumeVelocity") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::reverberationTime"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::soundEnergy"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundPower::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::soundPower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundPressure::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::soundPressure"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::staticPressure::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::staticPressure"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity::_documentation"))) (to (node (document "d0") (qualified-name "ISQAcoustics::volumeVelocity"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue::mRef"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::acousticImpedance"))) (to (node (document "d0") (qualified-name "ISQAcoustics::AcousticImpedanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundIntensity3dVector"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundIntensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleAcceleration3dVector"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleAcceleration3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleDisplacement3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::cartesianSoundParticleVelocity3dVector"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CartesianSoundParticleVelocity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::characteristicImpedanceOfAMediumForLongitudinalWaves"))) (to (node (document "d0") (qualified-name "ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::logarithmicFrequencyRange"))) (to (node (document "d0") (qualified-name "ISQAcoustics::LogarithmicFrequencyRangeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundEnergyDensity"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundEnergyDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundExposure"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundExposureLevel"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundExposureLevelValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundIntensity"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundIntensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundPowerLevel"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundPowerLevelValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQAcoustics::soundPressureLevel"))) (to (node (document "d0") (qualified-name "ISQAcoustics::SoundPressureLevelValue"))))
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
  (document "sysml.library/isq_acoustics.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 8) (end 43 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 8) (end 43 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 44 8) (end 44 61))
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
        (range (start 53 4) (end 53 555))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 4) (end 69 520))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 4) (end 85 747))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 98 8) (end 98 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 99 8) (end 99 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 8) (end 99 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 4) (end 105 901))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 118 8) (end 118 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 119 8) (end 119 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 8) (end 119 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 4) (end 125 893))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 138 8) (end 138 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 139 8) (end 139 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 8) (end 139 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 4) (end 145 552))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 1267))
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
        (range (start 177 8) (end 177 54))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 186 8) (end 186 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 4) (end 190 600))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 4) (end 206 1123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 4) (end 222 734))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 235 8) (end 235 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 235 8) (end 235 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 236 8) (end 236 50))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 244 8) (end 244 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 247 4) (end 247 780))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 260 8) (end 260 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 261 8) (end 261 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 266 4) (end 266 223))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 267 8) (end 267 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 268 8) (end 268 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 269 8) (end 269 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 273 4) (end 273 1046))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 286 8) (end 286 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 8) (end 286 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 287 8) (end 287 49))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 296 8) (end 296 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 300 4) (end 300 1260))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 313 8) (end 313 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 313 8) (end 313 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 314 8) (end 314 88))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 323 8) (end 323 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 4) (end 327 1182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 340 8) (end 340 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 340 8) (end 340 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 341 8) (end 341 53))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 350 8) (end 350 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 354 4) (end 354 1863))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 367 8) (end 367 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 367 8) (end 367 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 368 8) (end 368 54))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 390 8) (end 390 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 390 8) (end 390 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 391 8) (end 391 51))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 413 8) (end 413 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 413 8) (end 413 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 414 8) (end 414 54))
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
