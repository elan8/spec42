# META
~~~ini
description=KerML Simple Tests: TextualRepresentation
type=file
~~~
# SOURCE
~~~kerml
package TextualRepresentation {
	private import ScalarValues::Real;
	
	class C {
	    feature x: Real;
	    inv x_constraint {
		    rep inOCL language "ocl" 
		        /* self.x > 0.0 */
	    }
	}
	
	behavior setX { in c : C; in newX : Real;
	    language "alf" 
	        /* c.x = newX;
	         * WriteLine("Set new x");
	         */
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwInv,Ident,OpenCurly,
KwRep,Ident,KwLanguage,StringValue,
RegularComment,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwLanguage,StringValue,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TextualRepresentation'
    (import_decl private 'ScalarValues::Real')
    (class_def 'C'
      (feature_def 'x' : 'Real')
      (invariant_def
        (textual_rep 'inOCL' language '"ocl"')))
    (behavior_def
      (feature_def in 'c' : 'C')
      (feature_def in 'newX' : 'Real')
      (textual_rep language '"alf"'))))
~~~
# FORMAT
~~~sysml
package TextualRepresentation {
	private import ScalarValues::Real;
	
	class C {
	    feature x: Real;
	    inv x_constraint {
		    rep inOCL language "ocl" 
		        /* self.x > 0.0 */
	    }
	}
	
	behavior setX { in c : C; in newX : Real;
	    language "alf" 
	        /* c.x = newX;
	         * WriteLine("Set new x");
	         */
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TextualRepresentation"))) (name "TextualRepresentation") (declared-name "TextualRepresentation")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TextualRepresentation::C"))) (name "C") (declared-name "C"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TextualRepresentation::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "TextualRepresentation::setX"))) (name "setX") (declared-name "setX"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/textual_representation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
    )
  )
)
~~~
