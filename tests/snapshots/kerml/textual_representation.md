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
  (document "memory://snapshot/textual_representation.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 16) (end 4 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 37) (end 11 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d4f81f9512462339d824ecced521ffffcb6f645d3746d27d76d38c13f7c91b29") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (path (named (kind package) (name "TextualRepresentation")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x_constraint"))) (kind kerml-invariant) (membership (kind feature) (visibility default)) (documentation (rep (language "ocl") (text " self.x > 0.0 "))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (documentation (rep (language "alf") (text " c.x = newX;\n\t         * WriteLine(\"Set new x\");\n\t         "))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C") (direction in)))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (path (named (kind package) (name "TextualRepresentation")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")))))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x"))) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x_constraint"))) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX"))) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")))
      (subtype (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x")))
      (featured-by (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x_constraint")))
      (featured-by (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c")))
      (featured-by (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX")))
      (type (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")) (source direct))
      (supertype (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX")))
      (featured-by (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/textual_representation.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (path (named (kind package) (name "TextualRepresentation")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/textual_representation.md") (range (start 4 16) (end 4 20)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/textual_representation.md") (range (start 11 24) (end 11 25)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")))))
    )
  )
  (query (document "memory://snapshot/textual_representation.md") (range (start 11 37) (end 11 41)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
