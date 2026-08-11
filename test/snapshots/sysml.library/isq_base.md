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
  (document "isq_base.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 4) (end 40 683))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 728))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 4) (end 90 2181))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 4) (end 115 963))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 140 4) (end 140 1591))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 4) (end 165 1473))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 4) (end 189 666))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a43078cfbe4593bdc71f5e92f2b9fa52e97f3c969c5bf5c1dffeb71f5fc4c62d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQBase"))) (kind "package") (name "ISQBase") (declared-name "ISQBase"))
    (element (id (node (document "d0") (qualified-name "ISQBase::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQBase::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (kind "attribute def") (name "AmountOfSubstanceUnit") (declared-name "AmountOfSubstanceUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::amountOfSubstancePF"))) (kind "attribute") (name "amountOfSubstancePF") (declared-name "amountOfSubstancePF") (parent (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (kind "attribute def") (name "AmountOfSubstanceValue") (declared-name "AmountOfSubstanceValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmountOfSubstanceUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationUnit"))) (kind "attribute def") (name "DurationUnit") (declared-name "DurationUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQBase::DurationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::DurationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (kind "attribute def") (name "DurationValue") (declared-name "DurationValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::DurationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))) (kind "attribute def") (name "ElectricCurrentUnit") (declared-name "ElectricCurrentUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::electricCurrentPF"))) (kind "attribute") (name "electricCurrentPF") (declared-name "electricCurrentPF") (parent (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (kind "attribute def") (name "ElectricCurrentValue") (declared-name "ElectricCurrentValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ElectricCurrentUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (kind "attribute def") (name "International System of Quantities") (declared-name "International System of Quantities") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SystemOfQuantities")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::I"))) (kind "attribute") (name "I") (declared-name "I") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "ElectricCurrentValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::J"))) (kind "attribute") (name "J") (declared-name "J") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousIntensityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::L"))) (kind "attribute") (name "L") (declared-name "L") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::M"))) (kind "attribute") (name "M") (declared-name "M") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::N"))) (kind "attribute") (name "N") (declared-name "N") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "AmountOfSubstanceValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::T"))) (kind "attribute") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))) (kind "attribute") (name "baseQuantities") (declared-name "baseQuantities") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseQuantities")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind "attribute") (name "Θ") (declared-name "Θ") (parent (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermodynamicTemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthUnit"))) (kind "attribute def") (name "LengthUnit") (declared-name "LengthUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQBase::LengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::LengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (kind "attribute def") (name "LengthValue") (declared-name "LengthValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::LengthValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))) (kind "attribute def") (name "LuminousIntensityUnit") (declared-name "LuminousIntensityUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::luminousIntensityPF"))) (kind "attribute") (name "luminousIntensityPF") (declared-name "luminousIntensityPF") (parent (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (kind "attribute def") (name "LuminousIntensityValue") (declared-name "LuminousIntensityValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LuminousIntensityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassUnit"))) (kind "attribute def") (name "MassUnit") (declared-name "MassUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "ISQBase::MassUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::MassUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassValue"))) (kind "attribute def") (name "MassValue") (declared-name "MassValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::MassValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::MassValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::MassValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (kind "attribute def") (name "ThermodynamicTemperatureUnit") (declared-name "ThermodynamicTemperatureUnit") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (parent (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (kind "attribute def") (name "ThermodynamicTemperatureValue") (declared-name "ThermodynamicTemperatureValue") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermodynamicTemperatureUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQBase"))))
    (element (id (node (document "d0") (qualified-name "ISQBase::amountOfSubstance"))) (kind "attribute def") (name "amountOfSubstance") (declared-name "amountOfSubstance") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::duration"))) (kind "attribute def") (name "duration") (declared-name "duration") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::electricCurrent"))) (kind "attribute def") (name "electricCurrent") (declared-name "electricCurrent") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricCurrentValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::length"))) (kind "attribute def") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::luminousIntensity"))) (kind "attribute def") (name "luminousIntensity") (declared-name "luminousIntensity") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousIntensityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::mass"))) (kind "attribute def") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind "attribute def") (name "thermodynamicTemperature") (declared-name "thermodynamicTemperature") (parent (node (document "d0") (qualified-name "ISQBase"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::amountOfSubstancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::DurationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::DurationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::electricCurrentPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities"))) (kind featureTyping) (ordinal 0)) (authored-target "SystemOfQuantities") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::I"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::J"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousIntensityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::L"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::M"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::MassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::N"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::T"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))) (kind redefinition) (ordinal 0)) (authored-target "baseQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::luminousIntensityPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousIntensityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::MassUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::MassValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::MassValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::amountOfSubstance"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::duration"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::electricCurrent"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::length"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::luminousIntensity"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousIntensityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::MassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::DurationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::DurationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::I"))) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::I"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::J"))) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::J"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::L"))) (target (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::L"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::M"))) (target (node (document "d0") (qualified-name "ISQBase::MassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::M"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::N"))) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::N"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::T"))) (target (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::T"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))) (target (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::Θ"))) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::International System of Quantities::Θ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::LengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LengthValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::MassUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::MassValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::MassValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::amountOfSubstance"))) (target (node (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::amountOfSubstance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::duration"))) (target (node (document "d0") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::duration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::electricCurrent"))) (target (node (document "d0") (qualified-name "ISQBase::ElectricCurrentValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::electricCurrent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::length"))) (target (node (document "d0") (qualified-name "ISQBase::LengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::length"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::luminousIntensity"))) (target (node (document "d0") (qualified-name "ISQBase::LuminousIntensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::luminousIntensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::mass"))) (target (node (document "d0") (qualified-name "ISQBase::MassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQBase::thermodynamicTemperature"))) (target (node (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQBase::thermodynamicTemperature"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 28 22) (end 28 25)) (probe (position 28 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::LengthValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 28 22) (end 28 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::LengthValue::num") (range (start 28 8) (end 28 32)))
        )
      )
    )
    (query (range (start 53 22) (end 53 25)) (probe (position 53 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::DurationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 53 22) (end 53 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::DurationValue::num") (range (start 53 8) (end 53 32)))
        )
      )
    )
    (query (range (start 78 22) (end 78 25)) (probe (position 78 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::MassValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 78 22) (end 78 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::MassValue::num") (range (start 78 8) (end 78 32)))
        )
      )
    )
    (query (range (start 103 22) (end 103 25)) (probe (position 103 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 103 22) (end 103 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::num") (range (start 103 8) (end 103 32)))
        )
      )
    )
    (query (range (start 128 22) (end 128 25)) (probe (position 128 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 128 22) (end 128 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::num") (range (start 128 8) (end 128 32)))
        )
      )
    )
    (query (range (start 153 22) (end 153 25)) (probe (position 153 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 153 22) (end 153 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::num") (range (start 153 8) (end 153 32)))
        )
      )
    )
    (query (range (start 178 22) (end 178 25)) (probe (position 178 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 178 22) (end 178 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::num") (range (start 178 8) (end 178 32)))
        )
      )
    )
    (query (range (start 29 22) (end 29 26)) (probe (position 29 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::LengthValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 29 22) (end 29 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::LengthValue::mRef") (range (start 29 8) (end 29 42)))
        )
      )
    )
    (query (range (start 54 22) (end 54 26)) (probe (position 54 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::DurationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 54 22) (end 54 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::DurationValue::mRef") (range (start 54 8) (end 54 44)))
        )
      )
    )
    (query (range (start 79 22) (end 79 26)) (probe (position 79 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::MassValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 79 22) (end 79 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::MassValue::mRef") (range (start 79 8) (end 79 40)))
        )
      )
    )
    (query (range (start 104 22) (end 104 26)) (probe (position 104 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 104 22) (end 104 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureValue::mRef") (range (start 104 8) (end 104 60)))
        )
      )
    )
    (query (range (start 129 22) (end 129 26)) (probe (position 129 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 129 22) (end 129 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::ElectricCurrentValue::mRef") (range (start 129 8) (end 129 51)))
        )
      )
    )
    (query (range (start 154 22) (end 154 26)) (probe (position 154 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 154 22) (end 154 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::LuminousIntensityValue::mRef") (range (start 154 8) (end 154 53)))
        )
      )
    )
    (query (range (start 179 22) (end 179 26)) (probe (position 179 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 179 22) (end 179 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::AmountOfSubstanceValue::mRef") (range (start 179 8) (end 179 53)))
        )
      )
    )
    (query (range (start 11 19) (end 11 29)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "ISQBase::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 11 19) (end 11 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 195 22) (end 195 36)) (probe (position 195 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities"))
        (kind redefinition) (ordinal 0) (authored-target "baseQuantities")
        (range (start 195 22) (end 195 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::International System of Quantities::baseQuantities") (range (start 195 8) (end 195 66)))
        )
      )
    )
    (query (range (start 36 22) (end 36 39)) (probe (position 36 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 36 22) (end 36 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::LengthUnit::quantityDimension") (range (start 36 8) (end 36 80)))
        )
      )
    )
    (query (range (start 61 22) (end 61 39)) (probe (position 61 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 61 22) (end 61 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::DurationUnit::quantityDimension") (range (start 61 8) (end 61 82)))
        )
      )
    )
    (query (range (start 86 22) (end 86 39)) (probe (position 86 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 86 22) (end 86 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::MassUnit::quantityDimension") (range (start 86 8) (end 86 78)))
        )
      )
    )
    (query (range (start 111 22) (end 111 39)) (probe (position 111 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 111 22) (end 111 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::ThermodynamicTemperatureUnit::quantityDimension") (range (start 111 8) (end 111 98)))
        )
      )
    )
    (query (range (start 136 22) (end 136 39)) (probe (position 136 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 136 22) (end 136 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::ElectricCurrentUnit::quantityDimension") (range (start 136 8) (end 136 89)))
        )
      )
    )
    (query (range (start 161 22) (end 161 39)) (probe (position 161 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 161 22) (end 161 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::LuminousIntensityUnit::quantityDimension") (range (start 161 8) (end 161 91)))
        )
      )
    )
    (query (range (start 186 22) (end 186 39)) (probe (position 186 22))
      (reference
        (source (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 186 22) (end 186 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQBase::AmountOfSubstanceUnit::quantityDimension") (range (start 186 8) (end 186 91)))
        )
      )
    )
    (query (range (start 10 19) (end 10 37)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "ISQBase::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 10 19) (end 10 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 40)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "ISQBase::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 12 19) (end 12 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
