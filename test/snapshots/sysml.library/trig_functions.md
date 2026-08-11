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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fdc591eb39d947e851856edecd97abb016a4cf7a60975489d9e748777bd0ad7a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TrigFunctions"))) (kind "package") (name "TrigFunctions") (declared-name "TrigFunctions"))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "TrigFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::UnitBoundedReal"))) (kind "kermlDecl") (name "UnitBoundedReal") (declared-name "UnitBoundedReal") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::arccos"))) (kind "kermlDecl") (name "arccos") (declared-name "arccos") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::arcsin"))) (kind "kermlDecl") (name "arcsin") (declared-name "arcsin") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::arctan"))) (kind "kermlDecl") (name "arctan") (declared-name "arctan") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::cos"))) (kind "kermlDecl") (name "cos") (declared-name "cos") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::cot"))) (kind "kermlDecl") (name "cot") (declared-name "cot") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::deg"))) (kind "kermlDecl") (name "deg") (declared-name "deg") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::pi"))) (kind "feature decl") (name "pi") (declared-name "pi") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::piPrecision"))) (kind "kermlDecl") (name "piPrecision") (declared-name "piPrecision") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::rad"))) (kind "kermlDecl") (name "rad") (declared-name "rad") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::sin"))) (kind "kermlDecl") (name "sin") (declared-name "sin") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
    (element (id (node (document "d0") (qualified-name "TrigFunctions::tan"))) (kind "kermlDecl") (name "tan") (declared-name "tan") (parent (node (document "d0") (qualified-name "TrigFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TrigFunctions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 18) (end 5 36)) (probe (position 5 18))
      (reference
        (source (document "d0") (qualified-name "TrigFunctions::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 5 18) (end 5 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
