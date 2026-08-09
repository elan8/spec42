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
(model
  (namespace
    (library_package 'ISQ'
      (documentation)
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import public -> 'ISQBase'[unresolved])
      (namespace_import public -> 'ISQSpaceTime'[unresolved])
      (namespace_import public -> 'ISQMechanics'[unresolved])
      (namespace_import public -> 'ISQThermodynamics'[unresolved])
      (namespace_import public -> 'ISQElectromagnetism'[unresolved])
      (namespace_import public -> 'ISQLight'[unresolved])
      (namespace_import public -> 'ISQAcoustics'[unresolved])
      (namespace_import public -> 'ISQChemistryMolecular'[unresolved])
      (namespace_import public -> 'ISQAtomicNuclear'[unresolved])
      (namespace_import public -> 'ISQCharacteristicNumbers'[unresolved])
      (namespace_import public -> 'ISQCondensedMatter'[unresolved])
      (namespace_import public -> 'ISQInformation'[unresolved])
      (attribute_def 'TemperatureDifferenceValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQ::TemperatureDifferenceUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'temperatureDifference' : 'ISQ::TemperatureDifferenceValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'TemperatureDifferenceUnit' :> 'SimpleUnit'[unresolved]
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=))))))))
~~~
