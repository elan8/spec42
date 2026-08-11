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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQ'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl public 'ISQBase::*')
    (line_comment)
    (import_decl public 'ISQSpaceTime::*')
    (line_comment)
    (import_decl public 'ISQMechanics::*')
    (line_comment)
    (import_decl public 'ISQThermodynamics::*')
    (line_comment)
    (import_decl public 'ISQElectromagnetism::*')
    (line_comment)
    (import_decl public 'ISQLight::*')
    (line_comment)
    (import_decl public 'ISQAcoustics::*')
    (line_comment)
    (import_decl public 'ISQChemistryMolecular::*')
    (line_comment)
    (import_decl public 'ISQAtomicNuclear::*')
    (line_comment)
    (import_decl public 'ISQCharacteristicNumbers::*')
    (line_comment)
    (import_decl public 'ISQCondensedMatter::*')
    (line_comment)
    (import_decl public 'ISQInformation::*')
    (line_comment)
    (comment)
    (attribute_def 'TemperatureDifferenceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TemperatureDifferenceUnit' multiplicity))
    (attribute_usage 'temperatureDifference' : 'TemperatureDifferenceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TemperatureDifferenceUnit' :> 'SimpleUnit'
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
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
semantic.unresolved_name 'SimpleUnit'
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
semantic.unresolved_name 'SimpleUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
    (element (id (node (document "d0") (qualified-name "ISQ"))) (kind "package") (name "ISQ") (declared-name "ISQ") (range (start (line 0) (character 0)) (end (line 0) (character 2095))))
    (element (id (node (document "d0") (qualified-name "ISQ::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 30))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 41))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import10"))) (kind "import") (name "*") (declared-name "*") (range (start (line 18) (character 4)) (end (line 18) (character 38))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQAtomicNuclear::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 18) (character 18)) (end (line 18) (character 34))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import11"))) (kind "import") (name "*") (declared-name "*") (range (start (line 19) (character 4)) (end (line 19) (character 46))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQCharacteristicNumbers::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 19) (character 18)) (end (line 19) (character 42))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import12"))) (kind "import") (name "*") (declared-name "*") (range (start (line 20) (character 4)) (end (line 20) (character 40))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQCondensedMatter::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 20) (character 18)) (end (line 20) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import13"))) (kind "import") (name "*") (declared-name "*") (range (start (line 21) (character 4)) (end (line 21) (character 36))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQInformation::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 21) (character 18)) (end (line 21) (character 32))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 1)) (end (line 10) (character 26))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 15)) (end (line 10) (character 22))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 4)) (end (line 11) (character 34))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQSpaceTime::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 18)) (end (line 11) (character 30))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import4"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 4)) (end (line 12) (character 34))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQMechanics::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 18)) (end (line 12) (character 30))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import5"))) (kind "import") (name "*") (declared-name "*") (range (start (line 13) (character 4)) (end (line 13) (character 39))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQThermodynamics::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 13) (character 18)) (end (line 13) (character 35))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import6"))) (kind "import") (name "*") (declared-name "*") (range (start (line 14) (character 4)) (end (line 14) (character 41))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQElectromagnetism::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 14) (character 18)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import7"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 30))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQLight::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 18)) (end (line 15) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import8"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 34))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQAcoustics::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 18)) (end (line 16) (character 30))))))
    (element (id (node (document "d0") (qualified-name "ISQ::*#import9"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 43))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQChemistryMolecular::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 18)) (end (line 17) (character 39))))))
    (element (id (node (document "d0") (qualified-name "ISQ::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 6) (character 1)) (end (line 6) (character 35))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 34))))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (kind "attribute def") (name "TemperatureDifferenceUnit") (declared-name "TemperatureDifferenceUnit") (range (start (line 37) (character 4)) (end (line 37) (character 292))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Owning)) (relationships (typing (reference "SimpleUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 39) (character 8)) (end (line 39) (character 98))) (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 39) (character 22)) (end (line 39) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 38) (character 8)) (end (line 38) (character 123))) (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (kind "attribute def") (name "TemperatureDifferenceValue") (declared-name "TemperatureDifferenceValue") (range (start (line 25) (character 4)) (end (line 25) (character 378))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 25) (character 4)) (end (line 25) (character 378))) (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 32) (character 8)) (end (line 32) (character 57))) (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureDifferenceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 32) (character 22)) (end (line 32) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 31) (character 8)) (end (line 31) (character 32))) (parent (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 31) (character 22)) (end (line 31) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQ::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2095))) (parent (node (document "d0") (qualified-name "ISQ"))))
    (element (id (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (kind "attribute def") (name "temperatureDifference") (declared-name "temperatureDifference") (range (start (line 35) (character 4)) (end (line 35) (character 98))) (parent (node (document "d0") (qualified-name "ISQ"))) (authored (membership (kind Owning)) (relationships (typing (reference "TemperatureDifferenceValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 7) (character 16)) (end (line 7) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 8) (character 16)) (end (line 8) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import10"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQAtomicNuclear::*") (range (start (line 18) (character 18)) (end (line 18) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import11"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQCharacteristicNumbers::*") (range (start (line 19) (character 18)) (end (line 19) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import12"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQCondensedMatter::*") (range (start (line 20) (character 18)) (end (line 20) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import13"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQInformation::*") (range (start (line 21) (character 18)) (end (line 21) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 10) (character 15)) (end (line 10) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQSpaceTime::*") (range (start (line 11) (character 18)) (end (line 11) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQMechanics::*") (range (start (line 12) (character 18)) (end (line 12) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import5"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQThermodynamics::*") (range (start (line 13) (character 18)) (end (line 13) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import6"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQElectromagnetism::*") (range (start (line 14) (character 18)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import7"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQLight::*") (range (start (line 15) (character 18)) (end (line 15) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import8"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQAcoustics::*") (range (start (line 16) (character 18)) (end (line 16) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::*#import9"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQChemistryMolecular::*") (range (start (line 17) (character 18)) (end (line 17) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 6) (character 16)) (end (line 6) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 39) (character 22)) (end (line 39) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 32) (character 22)) (end (line 32) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 31) (character 22)) (end (line 31) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue")))))
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
