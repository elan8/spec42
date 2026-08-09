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
    doc /*
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
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : LogarithmicFrequencyRangeUnit [1];
    }

    attribute logarithmicFrequencyRange : LogarithmicFrequencyRangeValue :> scalarQuantities [*] nonunique;

    attribute def LogarithmicFrequencyRangeUnit :> DimensionOneUnit { }

    /* ISO-80000-8 item 8-2.1 static pressure */
    attribute staticPressure : PressureValue :> scalarQuantities {
        doc /*
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
    attribute soundPressure : PressureValue :> scalarQuantities {
        doc /*
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
        doc /*
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
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }

    attribute cartesianSoundParticleDisplacement3dVector : CartesianSoundParticleDisplacement3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-4 sound particle velocity */
    attribute def CartesianSoundParticleVelocity3dVector :> '3dVectorQuantityValue' {
        doc /*
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
        attribute :>> mRef : CartesianVelocity3dCoordinateFrame [1];
    }

    attribute cartesianSoundParticleVelocity3dVector : CartesianSoundParticleVelocity3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-5 sound particle acceleration */
    attribute def CartesianSoundParticleAcceleration3dVector :> '3dVectorQuantityValue' {
        doc /*
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
        attribute :>> mRef : CartesianAcceleration3dCoordinateFrame [1];
    }

    attribute cartesianSoundParticleAcceleration3dVector : CartesianSoundParticleAcceleration3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-6 volume velocity, volume flow rate */
    attribute volumeVelocity : SpeedValue :> scalarQuantities {
        doc /*
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
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : SoundEnergyDensityUnit [1];
    }

    attribute soundEnergyDensity : SoundEnergyDensityValue :> scalarQuantities [*] nonunique;

    attribute def SoundEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-8 item 8-8 sound energy */
    attribute soundEnergy : EnergyValue :> scalarQuantities {
        doc /*
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
    attribute soundPower : PowerValue :> scalarQuantities {
        doc /*
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
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : SoundIntensityUnit [1];
    }

    attribute soundIntensity : SoundIntensityValue :> scalarQuantities [*] nonunique;

    attribute def SoundIntensityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF);
        }
    }

    attribute def CartesianSoundIntensity3dVector :> '3dVectorQuantityValue' {
        doc /*
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
        attribute :>> mRef : CartesianSoundIntensity3dCoordinateFrame [1];
    }

    attribute cartesianSoundIntensity3dVector : CartesianSoundIntensity3dVector :> vectorQuantities;

    attribute def CartesianSoundIntensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : SoundIntensityUnit [3];
    }

    /* ISO-80000-8 item 8-11 sound exposure */
    attribute def SoundExposureValue :> ScalarQuantityValue {
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : SoundExposureUnit [1];
    }

    attribute soundExposure : SoundExposureValue :> scalarQuantities [*] nonunique;

    attribute def SoundExposureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-8 item 8-12 characteristic impedance of a medium for longitudinal waves */
    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesValue :> ScalarQuantityValue {
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit [1];
    }

    attribute characteristicImpedanceOfAMediumForLongitudinalWaves : CharacteristicImpedanceOfAMediumForLongitudinalWavesValue :> scalarQuantities [*] nonunique;

    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-8 item 8-13 acoustic impedance */
    attribute def AcousticImpedanceValue :> ScalarQuantityValue {
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : AcousticImpedanceUnit [1];
    }

    attribute acousticImpedance : AcousticImpedanceValue :> scalarQuantities [*] nonunique;

    attribute def AcousticImpedanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -4;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-8 item 8-14 sound pressure level */
    attribute def SoundPressureLevelValue :> ScalarQuantityValue {
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : SoundPressureLevelUnit [1];
    }

    attribute soundPressureLevel : SoundPressureLevelValue :> scalarQuantities [*] nonunique;

    attribute def SoundPressureLevelUnit :> DimensionOneUnit { }

    /* ISO-80000-8 item 8-15 sound power level */
    attribute def SoundPowerLevelValue :> ScalarQuantityValue {
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : SoundPowerLevelUnit [1];
    }

    attribute soundPowerLevel : SoundPowerLevelValue :> scalarQuantities [*] nonunique;

    attribute def SoundPowerLevelUnit :> DimensionOneUnit { }

    /* ISO-80000-8 item 8-16 sound exposure level */
    attribute def SoundExposureLevelValue :> ScalarQuantityValue {
        doc /*
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
        attribute :>> num : Real;
        attribute :>> mRef : SoundExposureLevelUnit [1];
    }

    attribute soundExposureLevel : SoundExposureLevelValue :> scalarQuantities [*] nonunique;

    attribute def SoundExposureLevelUnit :> DimensionOneUnit { }

    /* ISO-80000-8 item 8-17 reverberation time */
    attribute reverberationTime : DurationValue :> scalarQuantities {
        doc /*
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
(model
  (namespace
    (library_package 'ISQAcoustics'
      (documentation)
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'ISQBase'[unresolved])
      (membership_import private -> 'ISQMechanics::PowerValue'[unresolved])
      (membership_import private -> 'ISQMechanics::PressureValue'[unresolved])
      (membership_import private -> 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame'[unresolved])
      (membership_import private -> 'ISQSpaceTime::SpeedValue'[unresolved])
      (membership_import private -> 'ISQSpaceTime::CartesianVelocity3dCoordinateFrame'[unresolved])
      (membership_import private -> 'ISQSpaceTime::AccelerationValue'[unresolved])
      (membership_import private -> 'ISQSpaceTime::CartesianAcceleration3dCoordinateFrame'[unresolved])
      (membership_import private -> 'ISQThermodynamics::EnergyValue'[unresolved])
      (attribute_def 'LogarithmicFrequencyRangeValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::LogarithmicFrequencyRangeUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'logarithmicFrequencyRange' : 'ISQAcoustics::LogarithmicFrequencyRangeValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'LogarithmicFrequencyRangeUnit' :> 'DimensionOneUnit'[unresolved])
      (attribute_usage 'staticPressure' : 'PressureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'soundPressure' : 'PressureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'CartesianSoundParticleDisplacement3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianSpatial3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianSoundParticleDisplacement3dVector' : 'ISQAcoustics::CartesianSoundParticleDisplacement3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianSoundParticleVelocity3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianVelocity3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianSoundParticleVelocity3dVector' : 'ISQAcoustics::CartesianSoundParticleVelocity3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianSoundParticleAcceleration3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'CartesianAcceleration3dCoordinateFrame'[unresolved]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianSoundParticleAcceleration3dVector' : 'ISQAcoustics::CartesianSoundParticleAcceleration3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_usage 'volumeVelocity' : 'SpeedValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'volumeFlowRate' -> 'ISQAcoustics::volumeVelocity'[attribute_usage])
      (attribute_def 'SoundEnergyDensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::SoundEnergyDensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'soundEnergyDensity' : 'ISQAcoustics::SoundEnergyDensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SoundEnergyDensityUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_usage 'soundEnergy' : 'EnergyValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'soundPower' : 'PowerValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'SoundIntensityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::SoundIntensityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'soundIntensity' : 'ISQAcoustics::SoundIntensityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SoundIntensityUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_def 'CartesianSoundIntensity3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::CartesianSoundIntensity3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianSoundIntensity3dVector' : 'ISQAcoustics::CartesianSoundIntensity3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianSoundIntensity3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQAcoustics::SoundIntensityUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_def 'SoundExposureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::SoundExposureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'soundExposure' : 'ISQAcoustics::SoundExposureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SoundExposureUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_def 'CharacteristicImpedanceOfAMediumForLongitudinalWavesValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'characteristicImpedanceOfAMediumForLongitudinalWaves' : 'ISQAcoustics::CharacteristicImpedanceOfAMediumForLongitudinalWavesValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_def 'AcousticImpedanceValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::AcousticImpedanceUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'acousticImpedance' : 'ISQAcoustics::AcousticImpedanceValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AcousticImpedanceUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_def 'SoundPressureLevelValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::SoundPressureLevelUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'soundPressureLevel' : 'ISQAcoustics::SoundPressureLevelValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SoundPressureLevelUnit' :> 'DimensionOneUnit'[unresolved])
      (attribute_def 'SoundPowerLevelValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::SoundPowerLevelUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'soundPowerLevel' : 'ISQAcoustics::SoundPowerLevelValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SoundPowerLevelUnit' :> 'DimensionOneUnit'[unresolved])
      (attribute_def 'SoundExposureLevelValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQAcoustics::SoundExposureLevelUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'soundExposureLevel' : 'ISQAcoustics::SoundExposureLevelValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SoundExposureLevelUnit' :> 'DimensionOneUnit'[unresolved])
      (attribute_usage 'reverberationTime' : 'DurationValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation)))))
~~~
