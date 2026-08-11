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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "textual_representation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5bb99bb7a28f7328de54bcbeafe79f8ca9317e68e658348f156f8b8878ba0d86") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TextualRepresentation"))) (kind "package") (name "TextualRepresentation") (declared-name "TextualRepresentation") (range (start (line 0) (character 0)) (end (line 0) (character 343))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentation::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 3) (character 1)) (end (line 3) (character 127))) (parent (node (document "d0") (qualified-name "TextualRepresentation"))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentation::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "TextualRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentation::setX"))) (kind "kermlDecl") (name "setX") (declared-name "setX") (range (start (line 11) (character 1)) (end (line 11) (character 139))) (parent (node (document "d0") (qualified-name "TextualRepresentation"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentation::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
