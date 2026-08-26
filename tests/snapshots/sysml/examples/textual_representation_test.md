# META
~~~ini
description=SysML Example (Simple Tests): TextualRepresentationTest
type=file
~~~
# SOURCE
~~~sysml
package TextualRepresentationTest {
	private import ScalarValues::Real;
	
	item def C {
	    attribute x: Real;
	    assert constraint x_constraint {
		    rep inOCL language "ocl" 
		        /* self.x > 0.0 */
	    }
	}
	
	action def setX {
		in c : C;
		in newX : Real;
		
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
  (document "memory://snapshot/textual_representation_test.md"
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
        (range (start 4 18) (end 4 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 12) (end 13 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9878b50105e80aa203bd56e16e9462ea885ff7764f47f23e5e1f5f51de0394e6") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (path (named (kind package) (name "TextualRepresentationTest")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind assert-constraint) (membership (kind feature) (visibility default)) (documentation (rep (language "ocl") (text " self.x > 0.0 "))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (rep (language "alf") (text " c.x = newX;\n\t         * WriteLine(\"Set new x\");\n\t         "))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C") (direction in)))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (path (named (kind package) (name "TextualRepresentationTest")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")))))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX"))) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")))
      (subtype (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x")))
      (featured-by (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint")))
      (featured-by (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c")))
      (featured-by (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX")))
      (type (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")) (source direct))
      (supertype (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX")))
      (featured-by (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (path (named (kind package) (name "TextualRepresentationTest")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 4 18) (end 4 22)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 12 9) (end 12 10)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")))))
    )
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 13 12) (end 13 16)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
