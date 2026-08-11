# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/TrigFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package TrigFunctions {
    doc /*
	 * This package defines basic trigonometric functions on real numbers.
	 */

    public import ScalarValues::Real;

    feature pi : Real;
    inv piPrecision { RealFunctions::round(pi * 1E20) == 314159265358979323846.0 }

    function deg { in theta_rad : Real[1];
		return : Real[1] = theta_rad * 180 / pi;
	}
    function rad { in theta_deg : Real;
		return : Real[1] = theta_deg * pi / 180;
	}

    datatype UnitBoundedReal :> Real {
        inv unitBound { -1.0 <= that & that <= 1.0 }
    }

    function sin { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function cos { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function tan { in theta : Real[1]; 
        return : Real = sin(theta) / cos(theta);
	}
    function cot { in theta : Real; 
        return : Real = cos(theta) / sin(theta);
	}

    function arcsin { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arccos { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arctan { in x : Real[1]; return : Real[1]; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "trig_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 18) (end 5 36))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
KwInv,Ident,OpenCurly,Ident,ColonColon,Ident,OpenParen,Ident,Star,ExponentialValue,CloseParen,EqEq,DecimalValue,Dot,DecimalValue,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Star,DecimalValue,Slash,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Star,Ident,Slash,DecimalValue,Semicolon,
CloseCurly,
KwDatatype,Ident,ColonGt,Ident,OpenCurly,
KwInv,Ident,OpenCurly,Minus,DecimalValue,Dot,DecimalValue,LtEq,Ident,Ampersand,Ident,LtEq,DecimalValue,Dot,DecimalValue,CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Slash,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Slash,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'TrigFunctions'
    (documentation)
    (import_decl public 'ScalarValues::Real')
    (feature_def 'pi' : 'Real')
    (invariant_def
      (result_expr_member))
    (function_def
      (feature_def in 'theta_rad' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta_deg' : 'Real')
      (return_member))
    (datatype_def 'UnitBoundedReal' :> 'Real'
      (invariant_def
        (result_expr_member)))
    (function_def
      (feature_def in 'theta' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'theta' : 'Real')
      (return_member))
    (function_def
      (feature_def in 'x' : 'UnitBoundedReal' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'UnitBoundedReal' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# FORMAT
~~~sysml
standard library package TrigFunctions {
    doc /*
	 * This package defines basic trigonometric functions on real numbers.
	 */

    public import ScalarValues::Real;

    feature pi : Real;
    inv piPrecision { RealFunctions::round(pi * 1E20) == 314159265358979323846.0 }

    function deg { in theta_rad : Real[1];
		return : Real[1] = theta_rad * 180 / pi;
	}
    function rad { in theta_deg : Real;
		return : Real[1] = theta_deg * pi / 180;
	}

    datatype UnitBoundedReal :> Real {
        inv unitBound { -1.0 <= that & that <= 1.0 }
    }

    function sin { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function cos { in theta : Real[1]; return : UnitBoundedReal[1]; }
    function tan { in theta : Real[1]; 
        return : Real = sin(theta) / cos(theta);
	}
    function cot { in theta : Real; 
        return : Real = cos(theta) / sin(theta);
	}

    function arcsin { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arccos { in x : UnitBoundedReal[1]; return : Real[1]; }
    function arctan { in x : Real[1]; return : Real[1]; }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "65ff2948cdc10669c0328189ec8e9b4366ca9c78f5b16f385ae6bf504cbc00b1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TrigFunctions"))) (kind "package") (name "TrigFunctions") (declared-name "TrigFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 1070))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 5) (character 4)) (end (line 5) (character 37))) (parent (node (document "d0") (qualified-name "TrigFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 18)) (end (line 5) (character 36))))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::UnitBoundedReal"))) (kind "kermlDecl") (name "UnitBoundedReal") (declared-name "UnitBoundedReal") (range (start (line 17) (character 4)) (end (line 17) (character 97))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1070))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::arccos"))) (kind "kermlDecl") (name "arccos") (declared-name "arccos") (range (start (line 31) (character 4)) (end (line 31) (character 68))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::arcsin"))) (kind "kermlDecl") (name "arcsin") (declared-name "arcsin") (range (start (line 30) (character 4)) (end (line 30) (character 68))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::arctan"))) (kind "kermlDecl") (name "arctan") (declared-name "arctan") (range (start (line 32) (character 4)) (end (line 32) (character 57))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::cos"))) (kind "kermlDecl") (name "cos") (declared-name "cos") (range (start (line 22) (character 4)) (end (line 22) (character 69))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::cot"))) (kind "kermlDecl") (name "cot") (declared-name "cot") (range (start (line 26) (character 4)) (end (line 26) (character 88))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::deg"))) (kind "kermlDecl") (name "deg") (declared-name "deg") (range (start (line 10) (character 4)) (end (line 10) (character 88))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::pi"))) (kind "feature decl") (name "pi") (declared-name "pi") (range (start (line 7) (character 4)) (end (line 7) (character 22))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::piPrecision"))) (kind "kermlDecl") (name "piPrecision") (declared-name "piPrecision") (range (start (line 8) (character 4)) (end (line 8) (character 82))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::rad"))) (kind "kermlDecl") (name "rad") (declared-name "rad") (range (start (line 13) (character 4)) (end (line 13) (character 85))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::sin"))) (kind "kermlDecl") (name "sin") (declared-name "sin") (range (start (line 21) (character 4)) (end (line 21) (character 69))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::tan"))) (kind "kermlDecl") (name "tan") (declared-name "tan") (range (start (line 23) (character 4)) (end (line 23) (character 91))) (parent (node (document "d0") (qualified-name "TrigFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TrigFunctions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 5) (character 18)) (end (line 5) (character 36))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
