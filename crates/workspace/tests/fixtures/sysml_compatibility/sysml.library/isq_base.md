# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQBase
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQBase {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO/IEC 80000
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;

    /* ISO-80000-3 item 3-1.1 length */
    attribute def LengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-1.1 length
         * symbol(s): `l`, `L`
         * application domain: generic
         * name: Length
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: linear extent in space between any two points
         * remarks: Length does not need to be measured along a straight line. Length is one of the seven base quantities in the International System of Units (ISO 80000-1).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LengthUnit[1];
    }

    attribute length: LengthValue[*] nonunique :> scalarQuantities;

    attribute def LengthUnit :> SimpleUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-9 duration, time */
    attribute def DurationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-9 duration, time
         * symbol(s): `t`
         * application domain: generic
         * name: Duration
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: measure of the time difference between two events
         * remarks: Duration is often just called time. Time is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Duration is a measure of a time interval.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DurationUnit[1];
    }

    attribute duration: DurationValue[*] nonunique :> scalarQuantities;

    attribute def DurationUnit :> SimpleUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-4 item 4-1 mass */
    attribute def MassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-1 mass
         * symbol(s): `m`
         * application domain: generic
         * name: Mass
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: property of a body which expresses itself in terms of inertia with regard to changes in its state of motion as well as its gravitational attraction to other bodies
         * remarks: The kilogram (kg) is one of the seven base units (see ISO 80000-1) of the International System of Units, the SI. See also IEC 60050-113.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassUnit[1];
    }

    attribute mass: MassValue[*] nonunique :> scalarQuantities;

    attribute def MassUnit :> SimpleUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = massPF; }
    }

    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    attribute def ThermodynamicTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-1 thermodynamic temperature, temperature
         * symbol(s): `T`, `Θ`
         * application domain: generic
         * name: ThermodynamicTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: partial derivative of internal energy with respect to entropy at constant volume and constant number of particles in the system: `T = ((partial U)/(partial S))_(V,N)` where `U` is internal energy (item 5-20.2), `S` is entropy (item 5-18), `V` is volume (ISO 80000-3), and `N` is number of particles
         * remarks: It is measured with a primary thermometer, examples of which are gas thermometers of different kinds, noise thermometers, or radiation thermometers. The Boltzmann constant (ISO 80000-1) relates energy at the individual particle level with thermodynamic temperature. Differences of thermodynamic temperatures or changes may be expressed either in kelvin, symbol K, or in degrees Celsius, symbol °C (item 5-2). Thermodynamic temperature is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). The International Temperature Scale of 1990. For the purpose of practical measurements, the International Temperature Scale of 1990, ITS-90, was adopted by CIPM in 1989, which is a close approximation to the thermodynamic temperature scale. The quantities defined by this scale are denoted `T_90` and `t_90`, respectively (replacing `T_68` and `t_68` defined by the International Practical Temperature Scale of 1968, IPTS-68), where `t_90/(1 °C) = T_90/(1 K) - 273,15`. The units of `T_90` and `t_90` are the kelvin, symbol K, and the degree Celsius, symbol °C (item 5-2), respectively. For further information, see References [5], [6]. For ready conversion between temperatures reported on the International Temperature Scale and thermodynamic temperatures the systematic deviations can be found in Reference [7].
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermodynamicTemperatureUnit[1];
    }

    attribute thermodynamicTemperature: ThermodynamicTemperatureValue[*] nonunique :> scalarQuantities;

    attribute def ThermodynamicTemperatureUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* IEC-80000-6 item 6-1 electric current */
    attribute def ElectricCurrentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-1 electric current
         * symbol(s): `I`, `i`
         * application domain: generic
         * name: ElectricCurrent
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: electric current is one of the base quantities in the International System of Quantities, ISQ, on which the International System of Units, SI, is based
         * remarks: Electric current is the quantity that can often be measured with an ammeter. The electric current through a surface is the quotient of the electric charge (item 6-2) transferred through the surface during a time interval by the duration of that interval. For a more complete definition, see item 6-8 and IEC 60050-121, item 121-11-13.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricCurrentUnit[1];
    }

    attribute electricCurrent: ElectricCurrentValue[*] nonunique :> scalarQuantities;

    attribute def ElectricCurrentUnit :> SimpleUnit {
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = electricCurrentPF; }
    }

    /* ISO-80000-7 item 7-14 luminous intensity */
    attribute def LuminousIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-14 luminous intensity
         * symbol(s): `I_v`, `(I)`
         * application domain: generic
         * name: LuminousIntensity
         * quantity dimension: J^1
         * measurement unit(s): cd
         * tensor order: 0
         * definition: density of luminous flux with respect to solid angle in a specified direction, expressed by `I_v = (dΦ_v)/(dΩ)` where `Φ_v` is the luminous flux (item 7-13) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the luminous intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)`, is used to determine the luminous flux (item 7-13) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_v = int int_Ω I_v(θ,φ) sin(θ) dφ dθ`. Luminous intensity can be derived from the spectral radiant intensity distribution by `I_v = K_m int_0^∞ I_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `I_(e,λ)(λ)` is the spectral radiant intensity (item 7-5.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousIntensityUnit[1];
    }

    attribute luminousIntensity: LuminousIntensityValue[*] nonunique :> scalarQuantities;

    attribute def LuminousIntensityUnit :> SimpleUnit {
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = luminousIntensityPF; }
    }

    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    attribute def AmountOfSubstanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-2 amount of substance, number of moles
         * symbol(s): `n(X)`
         * application domain: generic
         * name: AmountOfSubstance
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: quotient of number `N` of specified elementary entities of kind `X` (item 9-1) in a sample, and the Avogadro constant `N_A` (ISO 80000-1): `n(X) = N(X)/N_A`
         * remarks: Amount of substance is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Elementary entities, such as molecules, atoms, ions, electrons, holes and other quasi-particles, double bonds can be used. It is necessary to specify precisely the entity involved, e.g. atoms of hydrogen `H` vs. molecules of hydrogen `H_2`, preferably by giving the molecular chemical formula of the material involved. In the name "amount of substance", the words "of substance" could be replaced by words specifying the substance concerned, e.g. "amount of hydrogen chloride, `HCl`", or "amount of benzene, `C_6H_6`". The name "number of moles" is often used for "amount of substance", but this is deprecated because the name of a quantity should be distinguished from the name of the unit.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AmountOfSubstanceUnit[1];
    }

    attribute amountOfSubstance: AmountOfSubstanceValue[*] nonunique :> scalarQuantities;

    attribute def AmountOfSubstanceUnit :> SimpleUnit {
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = amountOfSubstancePF; }
    }

    attribute <isq> 'International System of Quantities': SystemOfQuantities {
        doc
        /*
         * Declaration of the International System of Quantities (ISQ), 
         * including its base quantities and symbols as specified in ISO 80000-1:2009.
         */
        attribute :>> baseQuantities = ( L, M, T, I, 'Θ', N, J );
        
        attribute L: LengthValue[1];
        attribute M: MassValue[1];
        attribute T: DurationValue[1];
        attribute I: ElectricCurrentValue[1];
        attribute 'Θ': ThermodynamicTemperatureValue[1];
        attribute N: AmountOfSubstanceValue[1];
        attribute J: LuminousIntensityValue[1];
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
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'SystemOfQuantities'
semantic.unresolved_name 'baseQuantities'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'SystemOfQuantities'
semantic.unresolved_name 'baseQuantities'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,UnrestrictedName,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQBase'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (comment)
    (attribute_def 'LengthValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LengthUnit' multiplicity))
    (attribute_usage 'length' : 'LengthValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LengthUnit' :> 'SimpleUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'DurationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DurationUnit' multiplicity))
    (attribute_usage 'duration' : 'DurationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DurationUnit' :> 'SimpleUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassUnit' multiplicity))
    (attribute_usage 'mass' : 'MassValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassUnit' :> 'SimpleUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermodynamicTemperatureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermodynamicTemperatureUnit' multiplicity))
    (attribute_usage 'thermodynamicTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermodynamicTemperatureUnit' :> 'SimpleUnit'
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ElectricCurrentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricCurrentUnit' multiplicity))
    (attribute_usage 'electricCurrent' : 'ElectricCurrentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricCurrentUnit' :> 'SimpleUnit'
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LuminousIntensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LuminousIntensityUnit' multiplicity))
    (attribute_usage 'luminousIntensity' : 'LuminousIntensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LuminousIntensityUnit' :> 'SimpleUnit'
      (attribute_usage private 'luminousIntensityPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AmountOfSubstanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AmountOfSubstanceUnit' multiplicity))
    (attribute_usage 'amountOfSubstance' : 'AmountOfSubstanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AmountOfSubstanceUnit' :> 'SimpleUnit'
      (attribute_usage private 'amountOfSubstancePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_usage ''International System of Quantities'' : 'SystemOfQuantities'
      (documentation)
      (attribute_usage :>> 'baseQuantities' value)
      (attribute_usage 'L' : 'LengthValue' multiplicity)
      (attribute_usage 'M' : 'MassValue' multiplicity)
      (attribute_usage 'T' : 'DurationValue' multiplicity)
      (attribute_usage 'I' : 'ElectricCurrentValue' multiplicity)
      (attribute_usage ''Θ'' : 'ThermodynamicTemperatureValue' multiplicity)
      (attribute_usage 'N' : 'AmountOfSubstanceValue' multiplicity)
      (attribute_usage 'J' : 'LuminousIntensityValue' multiplicity))))
~~~
# FORMAT
~~~sysml
standard library package ISQBase {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO/IEC 80000
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;

    /* ISO-80000-3 item 3-1.1 length */
    attribute def LengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-1.1 length
         * symbol(s): `l`, `L`
         * application domain: generic
         * name: Length
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: linear extent in space between any two points
         * remarks: Length does not need to be measured along a straight line. Length is one of the seven base quantities in the International System of Units (ISO 80000-1).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LengthUnit[1];
    }

    attribute length: LengthValue[*] nonunique :> scalarQuantities;

    attribute def LengthUnit :> SimpleUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-9 duration, time */
    attribute def DurationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-9 duration, time
         * symbol(s): `t`
         * application domain: generic
         * name: Duration
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: measure of the time difference between two events
         * remarks: Duration is often just called time. Time is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Duration is a measure of a time interval.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DurationUnit[1];
    }

    attribute duration: DurationValue[*] nonunique :> scalarQuantities;

    attribute def DurationUnit :> SimpleUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-4 item 4-1 mass */
    attribute def MassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-1 mass
         * symbol(s): `m`
         * application domain: generic
         * name: Mass
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: property of a body which expresses itself in terms of inertia with regard to changes in its state of motion as well as its gravitational attraction to other bodies
         * remarks: The kilogram (kg) is one of the seven base units (see ISO 80000-1) of the International System of Units, the SI. See also IEC 60050-113.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassUnit[1];
    }

    attribute mass: MassValue[*] nonunique :> scalarQuantities;

    attribute def MassUnit :> SimpleUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = massPF; }
    }

    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    attribute def ThermodynamicTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-1 thermodynamic temperature, temperature
         * symbol(s): `T`, `Θ`
         * application domain: generic
         * name: ThermodynamicTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: partial derivative of internal energy with respect to entropy at constant volume and constant number of particles in the system: `T = ((partial U)/(partial S))_(V,N)` where `U` is internal energy (item 5-20.2), `S` is entropy (item 5-18), `V` is volume (ISO 80000-3), and `N` is number of particles
         * remarks: It is measured with a primary thermometer, examples of which are gas thermometers of different kinds, noise thermometers, or radiation thermometers. The Boltzmann constant (ISO 80000-1) relates energy at the individual particle level with thermodynamic temperature. Differences of thermodynamic temperatures or changes may be expressed either in kelvin, symbol K, or in degrees Celsius, symbol °C (item 5-2). Thermodynamic temperature is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). The International Temperature Scale of 1990. For the purpose of practical measurements, the International Temperature Scale of 1990, ITS-90, was adopted by CIPM in 1989, which is a close approximation to the thermodynamic temperature scale. The quantities defined by this scale are denoted `T_90` and `t_90`, respectively (replacing `T_68` and `t_68` defined by the International Practical Temperature Scale of 1968, IPTS-68), where `t_90/(1 °C) = T_90/(1 K) - 273,15`. The units of `T_90` and `t_90` are the kelvin, symbol K, and the degree Celsius, symbol °C (item 5-2), respectively. For further information, see References [5], [6]. For ready conversion between temperatures reported on the International Temperature Scale and thermodynamic temperatures the systematic deviations can be found in Reference [7].
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermodynamicTemperatureUnit[1];
    }

    attribute thermodynamicTemperature: ThermodynamicTemperatureValue[*] nonunique :> scalarQuantities;

    attribute def ThermodynamicTemperatureUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* IEC-80000-6 item 6-1 electric current */
    attribute def ElectricCurrentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-1 electric current
         * symbol(s): `I`, `i`
         * application domain: generic
         * name: ElectricCurrent
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: electric current is one of the base quantities in the International System of Quantities, ISQ, on which the International System of Units, SI, is based
         * remarks: Electric current is the quantity that can often be measured with an ammeter. The electric current through a surface is the quotient of the electric charge (item 6-2) transferred through the surface during a time interval by the duration of that interval. For a more complete definition, see item 6-8 and IEC 60050-121, item 121-11-13.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricCurrentUnit[1];
    }

    attribute electricCurrent: ElectricCurrentValue[*] nonunique :> scalarQuantities;

    attribute def ElectricCurrentUnit :> SimpleUnit {
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = electricCurrentPF; }
    }

    /* ISO-80000-7 item 7-14 luminous intensity */
    attribute def LuminousIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-14 luminous intensity
         * symbol(s): `I_v`, `(I)`
         * application domain: generic
         * name: LuminousIntensity
         * quantity dimension: J^1
         * measurement unit(s): cd
         * tensor order: 0
         * definition: density of luminous flux with respect to solid angle in a specified direction, expressed by `I_v = (dΦ_v)/(dΩ)` where `Φ_v` is the luminous flux (item 7-13) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the luminous intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)`, is used to determine the luminous flux (item 7-13) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_v = int int_Ω I_v(θ,φ) sin(θ) dφ dθ`. Luminous intensity can be derived from the spectral radiant intensity distribution by `I_v = K_m int_0^∞ I_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `I_(e,λ)(λ)` is the spectral radiant intensity (item 7-5.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousIntensityUnit[1];
    }

    attribute luminousIntensity: LuminousIntensityValue[*] nonunique :> scalarQuantities;

    attribute def LuminousIntensityUnit :> SimpleUnit {
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = luminousIntensityPF; }
    }

    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    attribute def AmountOfSubstanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-2 amount of substance, number of moles
         * symbol(s): `n(X)`
         * application domain: generic
         * name: AmountOfSubstance
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: quotient of number `N` of specified elementary entities of kind `X` (item 9-1) in a sample, and the Avogadro constant `N_A` (ISO 80000-1): `n(X) = N(X)/N_A`
         * remarks: Amount of substance is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Elementary entities, such as molecules, atoms, ions, electrons, holes and other quasi-particles, double bonds can be used. It is necessary to specify precisely the entity involved, e.g. atoms of hydrogen `H` vs. molecules of hydrogen `H_2`, preferably by giving the molecular chemical formula of the material involved. In the name "amount of substance", the words "of substance" could be replaced by words specifying the substance concerned, e.g. "amount of hydrogen chloride, `HCl`", or "amount of benzene, `C_6H_6`". The name "number of moles" is often used for "amount of substance", but this is deprecated because the name of a quantity should be distinguished from the name of the unit.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AmountOfSubstanceUnit[1];
    }

    attribute amountOfSubstance: AmountOfSubstanceValue[*] nonunique :> scalarQuantities;

    attribute def AmountOfSubstanceUnit :> SimpleUnit {
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = amountOfSubstancePF; }
    }

    attribute <isq> 'International System of Quantities': SystemOfQuantities {
        doc
        /*
         * Declaration of the International System of Quantities (ISQ), 
         * including its base quantities and symbols as specified in ISO 80000-1:2009.
         */
        attribute :>> baseQuantities = ( L, M, T, I, 'Θ', N, J );

        attribute L: LengthValue[1];
        attribute M: MassValue[1];
        attribute T: DurationValue[1];
        attribute I: ElectricCurrentValue[1];
        attribute 'Θ': ThermodynamicTemperatureValue[1];
        attribute N: AmountOfSubstanceValue[1];
        attribute J: LuminousIntensityValue[1];
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQBase"))) (name "ISQBase") (declared-name "ISQBase")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQBase::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQBase::*#import"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (name "AmountOfSubstanceUnit") (declared-name "AmountOfSubstanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::amountOfSubstancePF"))) (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (name "AmountOfSubstanceValue") (declared-name "AmountOfSubstanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::DurationUnit"))) (name "DurationUnit") (declared-name "DurationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::DurationUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::DurationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::DurationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (name "DurationValue") (declared-name "DurationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::DurationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::DurationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::DurationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::DurationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))) (name "ElectricCurrentUnit") (declared-name "ElectricCurrentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (name "ElectricCurrentValue") (declared-name "ElectricCurrentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (name "International System of Quantities") (declared-name "International System of Quantities") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::I"))) (name "I") (declared-name "I") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::J"))) (name "J") (declared-name "J") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::L"))) (name "L") (declared-name "L") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::M"))) (name "M") (declared-name "M") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::N"))) (name "N") (declared-name "N") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::T"))) (name "T") (declared-name "T") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))) (name "baseQuantities") (declared-name "baseQuantities") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::Θ"))) (name "Θ") (declared-name "Θ") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::International System of Quantities")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::LengthUnit"))) (name "LengthUnit") (declared-name "LengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LengthUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (name "LengthValue") (declared-name "LengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::LengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::LengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))) (name "LuminousIntensityUnit") (declared-name "LuminousIntensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::luminousIntensityPF"))) (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (name "LuminousIntensityValue") (declared-name "LuminousIntensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::MassUnit"))) (name "MassUnit") (declared-name "MassUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::MassUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::MassUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::MassUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::MassValue"))) (name "MassValue") (declared-name "MassValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::MassValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::MassValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::MassValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::MassValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQBase::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (name "ThermodynamicTemperatureUnit") (declared-name "ThermodynamicTemperatureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (name "ThermodynamicTemperatureValue") (declared-name "ThermodynamicTemperatureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQBase::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::amountOfSubstance"))) (name "amountOfSubstance") (declared-name "amountOfSubstance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::duration"))) (name "duration") (declared-name "duration") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::electricCurrent"))) (name "electricCurrent") (declared-name "electricCurrent") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::length"))) (name "length") (declared-name "length") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::luminousIntensity"))) (name "luminousIntensity") (declared-name "luminousIntensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQBase::thermodynamicTemperature"))) (name "thermodynamicTemperature") (declared-name "thermodynamicTemperature") (declared (properties (ordered false) (unique false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::DurationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::DurationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::LengthValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::LengthValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::MassValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::MassValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQBase::_documentation"))) (to (node (document "d0") (qualified-name "ISQBase"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::DurationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::I"))) (to (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::J"))) (to (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::L"))) (to (node (document "d0") (qualified-name "ISQBase::LengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::M"))) (to (node (document "d0") (qualified-name "ISQBase::MassValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::N"))) (to (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::T"))) (to (node (document "d0") (qualified-name "ISQBase::DurationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::International System of Quantities::Θ"))) (to (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::LengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::MassUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::amountOfSubstance"))) (to (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::duration"))) (to (node (document "d0") (qualified-name "ISQBase::DurationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::electricCurrent"))) (to (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::length"))) (to (node (document "d0") (qualified-name "ISQBase::LengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::luminousIntensity"))) (to (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::mass"))) (to (node (document "d0") (qualified-name "ISQBase::MassValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQBase::thermodynamicTemperature"))) (to (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))))
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
  (document "sysml.library/isq_base.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 4) (end 15 641))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 28 8) (end 28 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 8) (end 28 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 29 8) (end 29 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 4) (end 34 234))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 8) (end 35 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 36 8) (end 36 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 4) (end 40 683))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 53 8) (end 53 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 8) (end 53 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 54 8) (end 54 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 4) (end 59 240))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 8) (end 60 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 61 8) (end 61 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 728))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 78 8) (end 78 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 8) (end 78 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 79 8) (end 79 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 4) (end 84 228))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 8) (end 85 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 86 8) (end 86 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 4) (end 90 2181))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 103 8) (end 103 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 8) (end 103 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 104 8) (end 104 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 4) (end 109 291))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 8) (end 110 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 111 8) (end 111 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 4) (end 115 963))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 128 8) (end 128 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 8) (end 128 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 129 8) (end 129 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 134 4) (end 134 261))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 8) (end 135 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 136 8) (end 136 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 140 4) (end 140 1591))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 153 8) (end 153 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 8) (end 153 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 154 8) (end 154 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 4) (end 159 267))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 8) (end 160 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 161 8) (end 161 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 4) (end 165 1473))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 178 8) (end 178 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 178 8) (end 178 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 179 8) (end 179 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 4) (end 184 267))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 8) (end 185 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 186 8) (end 186 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 4) (end 189 666))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 195 8) (end 195 66))
      )
    )
  )
)
~~~
