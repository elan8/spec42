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
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 5) (end 5 5))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 5) (end 9 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 37) (end 11 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 5) (end 12 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:d4f81f9512462339d824ecced521ffffcb6f645d3746d27d76d38c13f7c91b29") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "language"))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C") (direction in))))
    (declaration (id (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (kind expressionOperand) (ordinal 0))
      (authored-target "language")
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
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (value (kind string) (value "alf")))
    (evaluated (declaration (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (value (kind string) (value "alf")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/textual_representation.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation.md") (range (start 12 5) (end 12 13)) (probe (position 12 5))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX"))) (kind expressionOperand) (ordinal 0) (authored-target "language")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation.md") (range (start 11 24) (end 11 25)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::C")))))
  )
  (query (document "memory://snapshot/textual_representation.md") (range (start 11 37) (end 11 41)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/textual_representation.md") (qualified-name "TextualRepresentation::setX::newX"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
