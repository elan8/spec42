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
# FORMAT
~~~sysml
standard library package ISQ {
    doc /*
     * International system of quantities (ISQ), as defined in ISO/IEC 80000
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;

    public import ISQBase::*;
    // ISO/IEC 80000 base quantities and general concepts
    public import ISQSpaceTime::*;
    // ISO 80000-3 "Space and Time"
    public import ISQMechanics::*;
    // ISO 80000-4 "Mechanics"
    public import ISQThermodynamics::*;
    // ISO 80000-5 "Thermodynamics"
    public import ISQElectromagnetism::*;
    // IEC 80000-6 "Electromagnetism"
    public import ISQLight::*;
    // ISO 80000-7 "Light"
    public import ISQAcoustics::*;
    // ISO 80000-8 "Acoustics"
    public import ISQChemistryMolecular::*;
    // ISO 80000-9 "Physical chemistry and molecular physics"
    public import ISQAtomicNuclear::*;
    // ISO 80000-10 "Atomic and nuclear physics"
    public import ISQCharacteristicNumbers::*;
    // ISO 80000-11 "Characteristic numbers"
    public import ISQCondensedMatter::*;
    // ISO 80000-12 "Condensed matter physics"
    public import ISQInformation::*;
    // IEC 80000-13 "Information science and technology"

    /* Additional quantity declarations */

    attribute def TemperatureDifferenceValue :> ScalarQuantityValue {
        doc /*
         * temperature difference
         * A separate temperature difference quantity and unit are needed in order to support °C, °F and centrigrade temperature differences
         */
        attribute :>> num : Real;
        attribute :>> mRef : TemperatureDifferenceUnit [1];
    }

    attribute temperatureDifference : TemperatureDifferenceValue :> scalarQuantities [*] nonunique;

    attribute def TemperatureDifferenceUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQ"))) (name "ISQ") (declared-name "ISQ")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import10"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import11"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import12"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import13"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import4"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import5"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import6"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import7"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import8"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::*#import9"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQ::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))) (name "TemperatureDifferenceUnit") (declared-name "TemperatureDifferenceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))) (name "TemperatureDifferenceValue") (declared-name "TemperatureDifferenceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQ::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (name "temperatureDifference") (declared-name "temperatureDifference") (declared (properties (ordered false) (unique false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQ::_documentation"))) (to (node (document "d0") (qualified-name "ISQ"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQ::temperatureDifference"))) (to (node (document "d0") (qualified-name "ISQ::TemperatureDifferenceValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
