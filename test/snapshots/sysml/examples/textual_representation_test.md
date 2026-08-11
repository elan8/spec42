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
  (document "textual_representation_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 15 5) (end 15 95))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b636f0b185cce46df451c25762f3f9b202c50bc745d090a0aaf919ff58aa338e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest"))) (kind "package") (name "TextualRepresentationTest") (declared-name "TextualRepresentationTest"))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::C"))) (kind "item def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "TextualRepresentationTest"))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "TextualRepresentationTest::C"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "TextualRepresentationTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (kind "action def") (name "setX") (declared-name "setX") (parent (node (document "d0") (qualified-name "TextualRepresentationTest"))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (kind "in out parameter") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (authored (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind "in out parameter") (name "newX") (declared-name "newX") (parent (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (authored (relationships (typing (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "TextualRepresentationTest::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "TextualRepresentationTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "TextualRepresentationTest::Real")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (target (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (target (node (document "d0") (qualified-name "TextualRepresentationTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (target (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "TextualRepresentationTest::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
