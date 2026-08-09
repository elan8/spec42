# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/MeasurementRefCalculations
type=file
~~~
# SOURCE
~~~sysml
standard library package MeasurementRefCalculations {
	doc
	/*
	 * This package package defines calculations on MeasurementUnits and CoordinateFrames.
	 */
	 
    private import ScalarValues::String;
    private import ScalarValues::Real;
    private import MeasurementReferences::MeasurementUnit;
    private import MeasurementReferences::ScalarMeasurementReference;
    private import MeasurementReferences::CoordinateFrame;
        
    /* MeasurementUnit operations */
    calc def '*' specializes DataFunctions::'*' { in x: MeasurementUnit[1]; in y: MeasurementUnit[1]; return : MeasurementUnit[1]; }
    calc def '/' specializes DataFunctions::'/' { in x: MeasurementUnit[1]; in y: MeasurementUnit[1]; return : MeasurementUnit[1]; }
    calc def '**' specializes DataFunctions::'**' { in x: MeasurementUnit[1]; in y: Real[1]; return : MeasurementUnit[1]; }
    calc def '^' specializes DataFunctions::'^' { in x: MeasurementUnit[1]; in y: Real[1]; return : MeasurementUnit[1]; }

    /* CoordinateFrame and MeasurementUnit operations */
    calc def 'CoordinateFrame*' specializes DataFunctions::'*' { in x: CoordinateFrame[1]; in y: MeasurementUnit[1]; return : CoordinateFrame[1]; }
    calc def 'CoordinateFrame/' specializes DataFunctions::'/' { in x: CoordinateFrame[1]; in y: MeasurementUnit[1]; return : CoordinateFrame[1]; }

    calc def ToString specializes BaseFunctions::ToString { 
        doc 
        /*
         * Returns the Unicode string symbol representing a scalar measurement reference.
         */
        in x: ScalarMeasurementReference[1]; return : String[1];
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::**'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::^'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'ScalarMeasurementReference'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::**'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::^'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'BaseFunctions::ToString'
semantic.unresolved_name 'ScalarMeasurementReference'
semantic.unresolved_name 'String'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
RegularComment,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'MeasurementRefCalculations'
    (documentation)
    (import_decl private 'ScalarValues::String')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'MeasurementReferences::MeasurementUnit')
    (import_decl private 'MeasurementReferences::ScalarMeasurementReference')
    (import_decl private 'MeasurementReferences::CoordinateFrame')
    (comment)
    (calc_def ''*'' :> 'DataFunctions::'*''
      (default_ref_usage in 'x' : 'MeasurementUnit' multiplicity)
      (default_ref_usage in 'y' : 'MeasurementUnit' multiplicity)
      (return_member))
    (calc_def ''/'' :> 'DataFunctions::'/''
      (default_ref_usage in 'x' : 'MeasurementUnit' multiplicity)
      (default_ref_usage in 'y' : 'MeasurementUnit' multiplicity)
      (return_member))
    (calc_def ''**'' :> 'DataFunctions::'**''
      (default_ref_usage in 'x' : 'MeasurementUnit' multiplicity)
      (default_ref_usage in 'y' : 'Real' multiplicity)
      (return_member))
    (calc_def ''^'' :> 'DataFunctions::'^''
      (default_ref_usage in 'x' : 'MeasurementUnit' multiplicity)
      (default_ref_usage in 'y' : 'Real' multiplicity)
      (return_member))
    (comment)
    (calc_def ''CoordinateFrame*'' :> 'DataFunctions::'*''
      (default_ref_usage in 'x' : 'CoordinateFrame' multiplicity)
      (default_ref_usage in 'y' : 'MeasurementUnit' multiplicity)
      (return_member))
    (calc_def ''CoordinateFrame/'' :> 'DataFunctions::'/''
      (default_ref_usage in 'x' : 'CoordinateFrame' multiplicity)
      (default_ref_usage in 'y' : 'MeasurementUnit' multiplicity)
      (return_member))
    (calc_def 'ToString' :> 'BaseFunctions::ToString'
      (documentation)
      (default_ref_usage in 'x' : 'ScalarMeasurementReference' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package MeasurementRefCalculations {
    doc /*
	 * This package package defines calculations on MeasurementUnits and CoordinateFrames.
	 */

    private import ScalarValues::String;
    private import ScalarValues::Real;
    private import MeasurementReferences::MeasurementUnit;
    private import MeasurementReferences::ScalarMeasurementReference;
    private import MeasurementReferences::CoordinateFrame;

    /* MeasurementUnit operations */
    calc def '*' specializes DataFunctions::'*' {
        in x : MeasurementUnit [1];
        in y : MeasurementUnit [1];
        return : MeasurementUnit[1];
    }
    calc def '/' specializes DataFunctions::'/' {
        in x : MeasurementUnit [1];
        in y : MeasurementUnit [1];
        return : MeasurementUnit[1];
    }
    calc def '**' specializes DataFunctions::'**' {
        in x : MeasurementUnit [1];
        in y : Real [1];
        return : MeasurementUnit[1];
    }
    calc def '^' specializes DataFunctions::'^' {
        in x : MeasurementUnit [1];
        in y : Real [1];
        return : MeasurementUnit[1];
    }

    /* CoordinateFrame and MeasurementUnit operations */
    calc def 'CoordinateFrame*' specializes DataFunctions::'*' {
        in x : CoordinateFrame [1];
        in y : MeasurementUnit [1];
        return : CoordinateFrame[1];
    }
    calc def 'CoordinateFrame/' specializes DataFunctions::'/' {
        in x : CoordinateFrame [1];
        in y : MeasurementUnit [1];
        return : CoordinateFrame[1];
    }

    calc def ToString specializes BaseFunctions::ToString {
        doc /*
         * Returns the Unicode string symbol representing a scalar measurement reference.
         */
        in x : ScalarMeasurementReference [1];
        return : String[1];
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'MeasurementRefCalculations'
      (documentation)
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'MeasurementReferences::MeasurementUnit'[unresolved])
      (membership_import private -> 'MeasurementReferences::ScalarMeasurementReference'[unresolved])
      (membership_import private -> 'MeasurementReferences::CoordinateFrame'[unresolved])
      (calculation_def '*' :> 'DataFunctions::*'[unresolved]
        (reference_usage in reference 'x' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'MeasurementUnit'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '/' :> 'DataFunctions::/'[unresolved]
        (reference_usage in reference 'x' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'MeasurementUnit'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '**' :> 'DataFunctions::**'[unresolved]
        (reference_usage in reference 'x' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'MeasurementUnit'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def '^' :> 'DataFunctions::^'[unresolved]
        (reference_usage in reference 'x' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'MeasurementUnit'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'CoordinateFrame*' :> 'DataFunctions::*'[unresolved]
        (reference_usage in reference 'x' : 'CoordinateFrame'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'CoordinateFrame'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'CoordinateFrame/' :> 'DataFunctions::/'[unresolved]
        (reference_usage in reference 'x' : 'CoordinateFrame'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'y' : 'MeasurementUnit'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'CoordinateFrame'[unresolved]
            (multiplicity_range [1]))))
      (calculation_def 'ToString' :> 'BaseFunctions::ToString'[unresolved]
        (documentation)
        (reference_usage in reference 'x' : 'ScalarMeasurementReference'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved]
            (multiplicity_range [1])))))))
~~~
