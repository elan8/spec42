# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQ
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQ {
    doc
    /*
     * International system of quantities (ISQ), as defined in ISO/IEC 80000
     */

	private import ScalarValues::Real;
	private import Quantities::*;
	private import MeasurementReferences::*;

	public import ISQBase::*;                  // ISO/IEC 80000 base quantities and general concepts
    public import ISQSpaceTime::*;             // ISO 80000-3 "Space and Time"
    public import ISQMechanics::*;             // ISO 80000-4 "Mechanics"
    public import ISQThermodynamics::*;        // ISO 80000-5 "Thermodynamics"
    public import ISQElectromagnetism::*;      // IEC 80000-6 "Electromagnetism"
    public import ISQLight::*;                 // ISO 80000-7 "Light"
    public import ISQAcoustics::*;             // ISO 80000-8 "Acoustics"
    public import ISQChemistryMolecular::*;    // ISO 80000-9 "Physical chemistry and molecular physics"
    public import ISQAtomicNuclear::*;         // ISO 80000-10 "Atomic and nuclear physics"
    public import ISQCharacteristicNumbers::*; // ISO 80000-11 "Characteristic numbers"
    public import ISQCondensedMatter::*;       // ISO 80000-12 "Condensed matter physics"
    public import ISQInformation::*;           // IEC 80000-13 "Information science and technology"

    /* Additional quantity declarations */

    attribute def TemperatureDifferenceValue :> ScalarQuantityValue {
        doc
        /*
         * temperature difference
         * A separate temperature difference quantity and unit are needed in order to support °C, °F and centrigrade temperature differences
         */
        attribute :>> num: Real;
        attribute :>> mRef: TemperatureDifferenceUnit[1];
    }
    
    attribute temperatureDifference: TemperatureDifferenceValue [*] nonunique :> scalarQuantities;

    attribute def TemperatureDifferenceUnit :> SimpleUnit {    
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 15) (end 10 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 18) (end 11 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 18) (end 12 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 18) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 18) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 18) (end 15 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 18) (end 16 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 18) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 18) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 18) (end 19 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 18) (end 20 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 18) (end 21 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 4) (end 25 378))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 4) (end 37 292))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 8) (end 38 123))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQ {
    doc
    /*
     * International system of quantities (ISQ), as defined in ISO/IEC 80000
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;

    public import ISQBase::*;                  // ISO/IEC 80000 base quantities and general concepts
    public import ISQSpaceTime::*;             // ISO 80000-3 "Space and Time"
    public import ISQMechanics::*;             // ISO 80000-4 "Mechanics"
    public import ISQThermodynamics::*;        // ISO 80000-5 "Thermodynamics"
    public import ISQElectromagnetism::*;      // IEC 80000-6 "Electromagnetism"
    public import ISQLight::*;                 // ISO 80000-7 "Light"
    public import ISQAcoustics::*;             // ISO 80000-8 "Acoustics"
    public import ISQChemistryMolecular::*;    // ISO 80000-9 "Physical chemistry and molecular physics"
    public import ISQAtomicNuclear::*;         // ISO 80000-10 "Atomic and nuclear physics"
    public import ISQCharacteristicNumbers::*; // ISO 80000-11 "Characteristic numbers"
    public import ISQCondensedMatter::*;       // ISO 80000-12 "Condensed matter physics"
    public import ISQInformation::*;           // IEC 80000-13 "Information science and technology"

    /* Additional quantity declarations */

    attribute def TemperatureDifferenceValue :> ScalarQuantityValue {
        doc
        /*
         * temperature difference
         * A separate temperature difference quantity and unit are needed in order to support °C, °F and centrigrade temperature differences
         */
        attribute :>> num: Real;
        attribute :>> mRef: TemperatureDifferenceUnit[1];
    }

    attribute temperatureDifference: TemperatureDifferenceValue [*] nonunique :> scalarQuantities;

    attribute def TemperatureDifferenceUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3412b67a19602820084fc353437f10e2390ead9e2eb511ed980ae52288e40d82") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQ"))) (kind "package") (name "ISQ") (declared-name "ISQ"))
    (element (id (node (document "d0") (qualified-name "ISQ::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import10"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQAtomicNuclear::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import11"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQCharacteristicNumbers::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import12"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQCondensedMatter::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import13"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQInformation::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQSpaceTime::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import4"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQMechanics::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import5"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQThermodynamics::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import6"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQElectromagnetism::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import7"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQLight::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import8"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQAcoustics::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import9"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQChemistryMolecular::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (kind "attribute def") (name "TemperatureDifferenceUnit") (declared-name "TemperatureDifferenceUnit") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (kind "attribute def") (name "TemperatureDifferenceValue") (declared-name "TemperatureDifferenceValue") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureDifferenceUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQ::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQ"))))
    (element (id (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (kind "attribute def") (name "temperatureDifference") (declared-name "temperatureDifference") (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Owning)) (relationships (typing (reference "TemperatureDifferenceValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import10"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQAtomicNuclear::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import11"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQCharacteristicNumbers::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import12"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQCondensedMatter::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import13"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQInformation::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQSpaceTime::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQMechanics::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import5"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQThermodynamics::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import6"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQElectromagnetism::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import7"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQLight::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import8"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQAcoustics::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import9"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQChemistryMolecular::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (target (node (document "d0") (qualified-name "ISQ::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 31 22) (end 31 25)) (probe (position 31 22))
      (reference
        (source (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 31 22) (end 31 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num") (range (start 31 8) (end 31 32)))
        )
      )
    )
    (query (range (start 32 22) (end 32 26)) (probe (position 32 22))
      (reference
        (source (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 32 22) (end 32 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef") (range (start 32 8) (end 32 57)))
        )
      )
    )
    (query (range (start 10 15) (end 10 22)) (probe (position 10 15))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQBase::*")
        (range (start 10 15) (end 10 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 18) (end 15 26)) (probe (position 15 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import7"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQLight::*")
        (range (start 15 18) (end 15 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 26)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "ISQ::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 7 16) (end 7 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 18) (end 11 30)) (probe (position 11 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQSpaceTime::*")
        (range (start 11 18) (end 11 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 18) (end 12 30)) (probe (position 12 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import4"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQMechanics::*")
        (range (start 12 18) (end 12 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 18) (end 16 30)) (probe (position 16 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import8"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQAcoustics::*")
        (range (start 16 18) (end 16 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 18) (end 21 32)) (probe (position 21 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import13"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQInformation::*")
        (range (start 21 18) (end 21 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 18) (end 18 34)) (probe (position 18 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import10"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQAtomicNuclear::*")
        (range (start 18 18) (end 18 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 18) (end 13 35)) (probe (position 13 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import5"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQThermodynamics::*")
        (range (start 13 18) (end 13 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 22) (end 39 39)) (probe (position 39 22))
      (reference
        (source (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 39 22) (end 39 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension") (range (start 39 8) (end 39 98)))
        )
      )
    )
    (query (range (start 6 16) (end 6 34)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "ISQ::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 6 16) (end 6 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 18) (end 20 36)) (probe (position 20 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import12"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQCondensedMatter::*")
        (range (start 20 18) (end 20 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 18) (end 14 37)) (probe (position 14 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import6"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQElectromagnetism::*")
        (range (start 14 18) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 37)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 8 16) (end 8 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 18) (end 17 39)) (probe (position 17 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import9"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQChemistryMolecular::*")
        (range (start 17 18) (end 17 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 18) (end 19 42)) (probe (position 19 18))
      (reference
        (source (document "d0") (qualified-name "ISQ::*#import11"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQCharacteristicNumbers::*")
        (range (start 19 18) (end 19 42))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
