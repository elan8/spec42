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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/isq_base.md"
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
        (range (start 11 19) (end 11 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 33) (end 15 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 22) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 27) (end 28 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 22) (end 29 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 32) (end 34 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 36) (end 35 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 65) (end 35 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 87) (end 35 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 22) (end 36 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 46) (end 36 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 40 35) (end 40 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 53 22) (end 53 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 27) (end 53 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 22) (end 54 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 59 34) (end 59 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 38) (end 60 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 67) (end 60 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 89) (end 60 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 22) (end 61 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 46) (end 61 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 65 31) (end 65 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 78 22) (end 78 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 27) (end 78 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 22) (end 79 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 84 30) (end 84 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 34) (end 85 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 85 63) (end 85 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 85 85) (end 85 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 86 22) (end 86 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 86 46) (end 86 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 90 51) (end 90 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 103 22) (end 103 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 27) (end 103 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 104 22) (end 104 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 109 50) (end 109 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 54) (end 110 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 110 83) (end 110 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 110 108) (end 110 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 111 22) (end 111 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 111 46) (end 111 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 115 42) (end 115 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 128 22) (end 128 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 27) (end 128 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 129 22) (end 129 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 134 41) (end 134 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 45) (end 135 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 135 74) (end 135 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 135 96) (end 135 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 136 22) (end 136 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 136 46) (end 136 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 140 44) (end 140 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 153 22) (end 153 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 27) (end 153 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 154 22) (end 154 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 159 43) (end 159 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 47) (end 160 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 160 76) (end 160 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 160 98) (end 160 106))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 161 22) (end 161 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 161 46) (end 161 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 165 44) (end 165 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 178 22) (end 178 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 178 27) (end 178 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 179 22) (end 179 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 184 43) (end 184 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 47) (end 185 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 185 76) (end 185 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 185 98) (end 185 106))
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
        (range (start 189 58) (end 189 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 195 22) (end 195 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c37b6be2a9efcad37642429be38c79fae58e8a70dfa4079e13b476f643e3837a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit::amountOfSubstancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit::electricCurrentPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricCurrentUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SystemOfQuantities"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseQuantities"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::I"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricCurrentValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::J"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LuminousIntensityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::L"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::M"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::N"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::T"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThermodynamicTemperatureValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit::luminousIntensityPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LuminousIntensityUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::thermodynamicTemperaturePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThermodynamicTemperatureUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::amountOfSubstance"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AmountOfSubstanceValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::duration"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DurationValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::electricCurrent"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ElectricCurrentValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::length"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::luminousIntensity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LuminousIntensityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::mass"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ThermodynamicTemperatureValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricCurrentUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities"))) (kind featureTyping) (ordinal 0))
      (authored-target "SystemOfQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "baseQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::I"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::J"))) (kind featureTyping) (ordinal 0))
      (authored-target "LuminousIntensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::L"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::M"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::N"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::T"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermodynamicTemperatureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "LuminousIntensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (kind specialization) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermodynamicTemperatureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::amountOfSubstance"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmountOfSubstanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::duration"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::electricCurrent"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::length"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::luminousIntensity"))) (kind featureTyping) (ordinal 0))
      (authored-target "LuminousIntensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThermodynamicTemperatureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::I"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::I"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::J"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::J"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::L"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::L"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::M"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::M"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::N"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::N"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::T"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::T"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::Θ"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::amountOfSubstance"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::amountOfSubstance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::duration"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::duration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::electricCurrent"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::electricCurrent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::length"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::length"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::luminousIntensity"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::luminousIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::mass"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::thermodynamicTemperature"))) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/isq_base.md") (range (start 11 19) (end 11 32)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 12 19) (end 12 43)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 10 19) (end 10 37)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 184 43) (end 184 53)) (probe (position 184 43))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 186 22) (end 186 39)) (probe (position 186 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 186 46) (end 186 66)) (probe (position 186 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 185 47) (end 185 66)) (probe (position 185 47))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 185 76) (end 185 84)) (probe (position 185 76))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 185 98) (end 185 106)) (probe (position 185 98))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 165 44) (end 165 63)) (probe (position 165 44))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 178 27) (end 178 31)) (probe (position 178 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 179 28) (end 179 49)) (probe (position 179 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 178 22) (end 178 25)) (probe (position 178 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 179 22) (end 179 26)) (probe (position 179 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 59 34) (end 59 44)) (probe (position 59 34))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 61 22) (end 61 39)) (probe (position 61 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 61 46) (end 61 66)) (probe (position 61 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 60 38) (end 60 57)) (probe (position 60 38))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 60 67) (end 60 75)) (probe (position 60 67))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 60 89) (end 60 97)) (probe (position 60 89))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 40 35) (end 40 54)) (probe (position 40 35))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 53 27) (end 53 31)) (probe (position 53 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 54 28) (end 54 40)) (probe (position 54 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "DurationUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 53 22) (end 53 25)) (probe (position 53 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 54 22) (end 54 26)) (probe (position 54 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 134 41) (end 134 51)) (probe (position 134 41))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 136 22) (end 136 39)) (probe (position 136 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 136 46) (end 136 66)) (probe (position 136 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 135 45) (end 135 64)) (probe (position 135 45))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 135 74) (end 135 82)) (probe (position 135 74))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 135 96) (end 135 104)) (probe (position 135 96))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 115 42) (end 115 61)) (probe (position 115 42))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 128 27) (end 128 31)) (probe (position 128 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 129 28) (end 129 47)) (probe (position 129 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "ElectricCurrentUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 128 22) (end 128 25)) (probe (position 128 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 129 22) (end 129 26)) (probe (position 129 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 189 58) (end 189 76)) (probe (position 189 58))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities"))) (kind featureTyping) (ordinal 0) (authored-target "SystemOfQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 195 22) (end 195 36)) (probe (position 195 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "baseQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 200 21) (end 200 41)) (probe (position 200 21))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::I"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 203 21) (end 203 43)) (probe (position 203 21))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::J"))) (kind featureTyping) (ordinal 0) (authored-target "LuminousIntensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 197 21) (end 197 32)) (probe (position 197 21))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::L"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 198 21) (end 198 30)) (probe (position 198 21))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::M"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 202 21) (end 202 43)) (probe (position 202 21))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::N"))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 199 21) (end 199 34)) (probe (position 199 21))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::T"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 201 24) (end 201 53)) (probe (position 201 24))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind featureTyping) (ordinal 0) (authored-target "ThermodynamicTemperatureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 34 32) (end 34 42)) (probe (position 34 32))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 36 22) (end 36 39)) (probe (position 36 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 36 46) (end 36 66)) (probe (position 36 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 35 36) (end 35 55)) (probe (position 35 36))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 35 65) (end 35 73)) (probe (position 35 65))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 35 87) (end 35 95)) (probe (position 35 87))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 15 33) (end 15 52)) (probe (position 15 33))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 28 27) (end 28 31)) (probe (position 28 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 29 28) (end 29 38)) (probe (position 29 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "LengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 28 22) (end 28 25)) (probe (position 28 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 29 22) (end 29 26)) (probe (position 29 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 159 43) (end 159 53)) (probe (position 159 43))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 161 22) (end 161 39)) (probe (position 161 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 161 46) (end 161 66)) (probe (position 161 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 160 47) (end 160 66)) (probe (position 160 47))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 160 76) (end 160 84)) (probe (position 160 76))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 160 98) (end 160 106)) (probe (position 160 98))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 140 44) (end 140 63)) (probe (position 140 44))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 153 27) (end 153 31)) (probe (position 153 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 154 28) (end 154 49)) (probe (position 154 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "LuminousIntensityUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 153 22) (end 153 25)) (probe (position 153 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 154 22) (end 154 26)) (probe (position 154 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 84 30) (end 84 40)) (probe (position 84 30))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 86 22) (end 86 39)) (probe (position 86 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 86 46) (end 86 66)) (probe (position 86 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 85 34) (end 85 53)) (probe (position 85 34))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 85 63) (end 85 71)) (probe (position 85 63))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 85 85) (end 85 93)) (probe (position 85 85))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 65 31) (end 65 50)) (probe (position 65 31))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 78 27) (end 78 31)) (probe (position 78 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 79 28) (end 79 36)) (probe (position 79 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MassUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 78 22) (end 78 25)) (probe (position 78 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 79 22) (end 79 26)) (probe (position 79 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 109 50) (end 109 60)) (probe (position 109 50))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (kind specialization) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 111 22) (end 111 39)) (probe (position 111 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 111 46) (end 111 66)) (probe (position 111 46))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 110 54) (end 110 73)) (probe (position 110 54))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 110 83) (end 110 91)) (probe (position 110 83))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 110 108) (end 110 116)) (probe (position 110 108))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 90 51) (end 90 70)) (probe (position 90 51))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 103 27) (end 103 31)) (probe (position 103 27))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 104 28) (end 104 56)) (probe (position 104 28))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "ThermodynamicTemperatureUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureUnit")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 103 22) (end 103 25)) (probe (position 103 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 104 22) (end 104 26)) (probe (position 104 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 182 33) (end 182 55)) (probe (position 182 33))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::amountOfSubstance"))) (kind featureTyping) (ordinal 0) (authored-target "AmountOfSubstanceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 57 24) (end 57 37)) (probe (position 57 24))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::duration"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 132 31) (end 132 51)) (probe (position 132 31))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::electricCurrent"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 32 22) (end 32 33)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::length"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LengthValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 157 33) (end 157 55)) (probe (position 157 33))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::luminousIntensity"))) (kind featureTyping) (ordinal 0) (authored-target "LuminousIntensityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::LuminousIntensityValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 82 20) (end 82 29)) (probe (position 82 20))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::MassValue")))))
  )
  (query (document "memory://snapshot/isq_base.md") (range (start 107 40) (end 107 69)) (probe (position 107 40))
    (reference (id (source (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind featureTyping) (ordinal 0) (authored-target "ThermodynamicTemperatureValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_base.md") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
  )
)
~~~
