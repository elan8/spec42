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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (name "MeasurementRefCalculations") (declared-name "MeasurementRefCalculations")
      (contains
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::*"))) (name "*") (declared-name "*")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::*::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::*")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::*::y"))) (name "y") (declared-name "y") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::*")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::**"))) (name "**") (declared-name "**")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::**::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::**")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::**::y"))) (name "y") (declared-name "y") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::**")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::/"))) (name "/") (declared-name "/")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::/::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::/")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::/::y"))) (name "y") (declared-name "y") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::/")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame"))) (name "CoordinateFrame") (declared-name "CoordinateFrame"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (name "CoordinateFrame*") (declared-name "CoordinateFrame*")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*::y"))) (name "y") (declared-name "y") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (name "CoordinateFrame/") (declared-name "CoordinateFrame/")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/::y"))) (name "y") (declared-name "y") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::MeasurementUnit"))) (name "MeasurementUnit") (declared-name "MeasurementUnit"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ScalarMeasurementReference"))) (name "ScalarMeasurementReference") (declared-name "ScalarMeasurementReference"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::String"))) (name "String") (declared-name "String"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString"))) (name "ToString") (declared-name "ToString")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::^"))) (name "^") (declared-name "^")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::^::x"))) (name "x") (declared-name "x") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::^")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::^::y"))) (name "y") (declared-name "y") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "MeasurementRefCalculations::^")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementRefCalculations::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementRefCalculations::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::*"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::**"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::/"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MeasurementRefCalculations::^"))) (status missing-prerequisite) (target "Calculations::Calculation"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/measurement_ref_calculations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 50) (end 13 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 76) (end 13 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 50) (end 14 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 76) (end 14 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 52) (end 15 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 78) (end 15 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 50) (end 16 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 76) (end 16 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 65) (end 19 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 91) (end 19 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 65) (end 20 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 91) (end 20 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 8) (end 27 44))
      )
    )
  )
)
~~~
