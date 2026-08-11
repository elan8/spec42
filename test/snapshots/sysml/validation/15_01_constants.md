# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_01-Constants
type=file
~~~
# SOURCE
~~~sysml
package '15_01-Constants' {
    private import MeasurementReferences::*;
    private import SI::*;
    private import RealFunctions::*;

    /* Note: Value properties that are bound to specific values are constants and have the specified
     * values in all contexts. It is not legal to redefine them.
     */    
    
    package 'Mathematical Constants' {
	    doc
	    /*
	     * Standard mathematical constants
	     * 
	     * Irrational constants cannot be represented exactly with finite precision.
	     * However, they can be required to be implemented with a attribute that is accurate
	     * to at least a certain precision.
	     * 
	     * (The decimal literals here should be interpreted as being fixed point and exact.)
	     */
    
        attribute e: Real {
        	assert constraint { round(e * 1E20) == 271828182845904523536.0 }
        }
        attribute pi: Real {
        	assert constraint { round(pi * 1E20) == 314159265358979323846.0 }
        }
    }

    package 'Fundamental Physical Constants' {
	    doc
	    /*
	     * Standard fundamental physical constants
	     * 
	     * Physical constants have a standard measured attribute to a finite precision.
	     *
	     * The reference source is:
	     * CODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values
	     * See https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/
	     * For the actual values see https://pml.nist.gov/cuu/Constants/ 
	     */
    
        attribute 'fine structure constant'      : DimensionOneValue = 7.2973525693E-3[one];  // 2018 CODATA attribute 7.2973525693E-3;  uncertainty = 0.0000000011E-3
        attribute 'electron to proton mass ratio': DimensionOneValue = 5.44617021487E-4[one]; // 2018 CODATA attribute 5.44617021487E-4; uncertainty = 0.00000000033E-4 
        attribute 'speed of light in vacuum'     : SpeedValue = 299792458[m/s];               // 2018 CODATA attribute 299792458 m s^-1; (exact)
     }

    package 'Global Context' {
        attribute 'nominal earth gravitational acceleration': AccelerationValue = 9.80665['m/s²'];
    }

    package 'Model X Context' {
        attribute 'amplifier gain': DimensionOneValue = 3.5[one];
    }
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,Star,ExponentialValue,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,CloseCurly,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,Star,ExponentialValue,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,UnrestrictedName,Colon,Ident,Eq,DecimalValue,Dot,ExponentialValue,OpenSquare,Ident,CloseSquare,Semicolon,LineComment,
KwAttribute,UnrestrictedName,Colon,Ident,Eq,DecimalValue,Dot,ExponentialValue,OpenSquare,Ident,CloseSquare,Semicolon,LineComment,
KwAttribute,UnrestrictedName,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,LineComment,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,UnrestrictedName,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_01-Constants''
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'SI::*')
    (import_decl private 'RealFunctions::*')
    (comment)
    (package_def ''Mathematical Constants''
      (documentation)
      (attribute_usage 'e' : 'Real'
        (sysml_decl
          (result_expr_member)))
      (attribute_usage 'pi' : 'Real'
        (sysml_decl
          (result_expr_member))))
    (package_def ''Fundamental Physical Constants''
      (documentation)
      (attribute_usage ''fine structure constant'' : 'DimensionOneValue' value)
      (line_comment)
      (attribute_usage ''electron to proton mass ratio'' : 'DimensionOneValue' value)
      (line_comment)
      (attribute_usage ''speed of light in vacuum'' : 'SpeedValue' value)
      (line_comment))
    (package_def ''Global Context''
      (attribute_usage ''nominal earth gravitational acceleration'' : 'AccelerationValue' value))
    (package_def ''Model X Context''
      (attribute_usage ''amplifier gain'' : 'DimensionOneValue' value))))
~~~
# FORMAT
~~~sysml
package '15_01-Constants' {
    private import MeasurementReferences::*;
    private import SI::*;
    private import RealFunctions::*;

    /* Note: Value properties that are bound to specific values are constants and have the specified
     * values in all contexts. It is not legal to redefine them.
     */    

    package 'Mathematical Constants' {
        doc
        /*
	     * Standard mathematical constants
	     * 
	     * Irrational constants cannot be represented exactly with finite precision.
	     * However, they can be required to be implemented with a attribute that is accurate
	     * to at least a certain precision.
	     * 
	     * (The decimal literals here should be interpreted as being fixed point and exact.)
	     */

        attribute e: Real {
            assert constraint { round(e * 1E20) == 271828182845904523536.0 }
        }
        attribute pi: Real {
            assert constraint { round(pi * 1E20) == 314159265358979323846.0 }
        }
    }

    package 'Fundamental Physical Constants' {
        doc
        /*
	     * Standard fundamental physical constants
	     * 
	     * Physical constants have a standard measured attribute to a finite precision.
	     *
	     * The reference source is:
	     * CODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values
	     * See https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/
	     * For the actual values see https://pml.nist.gov/cuu/Constants/ 
	     */

        attribute 'fine structure constant'      : DimensionOneValue = 7.2973525693E-3[one];  // 2018 CODATA attribute 7.2973525693E-3;  uncertainty = 0.0000000011E-3
        attribute 'electron to proton mass ratio': DimensionOneValue = 5.44617021487E-4[one]; // 2018 CODATA attribute 5.44617021487E-4; uncertainty = 0.00000000033E-4
        attribute 'speed of light in vacuum'     : SpeedValue = 299792458[m/s];               // 2018 CODATA attribute 299792458 m s^-1; (exact)
    }

    package 'Global Context' {
        attribute 'nominal earth gravitational acceleration': AccelerationValue = 9.80665['m/s²'];
    }

    package 'Model X Context' {
        attribute 'amplifier gain': DimensionOneValue = 3.5[one];
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'DimensionOneValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'DimensionOneValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_01-Constants"))) (name "15_01-Constants") (declared-name "15_01-Constants")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_01-Constants::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_01-Constants::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_01-Constants::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (name "Fundamental Physical Constants") (declared-name "Fundamental Physical Constants")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::_documentation"))) (name ""))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (name "electron to proton mass ratio") (declared-name "electron to proton mass ratio") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal (real "5.44617021487E-4"))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "one")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (name "fine structure constant") (declared-name "fine structure constant") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal (real "7.2973525693E-3"))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "one")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (name "speed of light in vacuum") (declared-name "speed of light in vacuum") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 299792458))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m/s")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "15_01-Constants::Global Context"))) (name "Global Context") (declared-name "Global Context")
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (name "nominal earth gravitational acceleration") (declared-name "nominal earth gravitational acceleration") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal (real "9.80665"))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m/s²")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants"))) (name "Mathematical Constants") (declared-name "Mathematical Constants")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::_documentation"))) (name ""))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (name "e") (declared-name "e") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (name "pi") (declared-name "pi") (declared (properties (ordered false) (unique true))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "15_01-Constants::Model X Context"))) (name "Model X Context") (declared-name "Model X Context")
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (name "amplifier gain") (declared-name "amplifier gain") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal (real "3.5"))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "one")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::_documentation"))) (to (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::_documentation"))) (to (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::electron to proton mass ratio"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::fine structure constant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Fundamental Physical Constants::speed of light in vacuum"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Global Context::nominal earth gravitational acceleration"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::e"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Mathematical Constants::pi"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_01-Constants::Model X Context::amplifier gain"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/15_01_constants.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 8) (end 21 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 8) (end 24 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 8) (end 42 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 8) (end 43 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 8) (end 44 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 8) (end 48 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 8) (end 52 65))
      )
    )
  )
)
~~~
