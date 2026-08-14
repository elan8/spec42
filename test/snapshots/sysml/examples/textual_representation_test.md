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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 6) (end 6 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 10) (end 6 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 16) (end 6 24))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:9878b50105e80aa203bd56e16e9462ea885ff7764f47f23e5e1f5f51de0394e6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (path (named (kind package) (name "TextualRepresentationTest")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "rep")) (expressionOperand (reference "inOCL")) (expressionOperand (reference "language"))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (rep (language "alf") (text " c.x = newX;\n\t         * WriteLine(\"Set new x\");\n\t         "))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C") (direction in))))
    (declaration (id (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (path (named (kind package) (name "TextualRepresentationTest")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "rep")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind expressionOperand) (ordinal 1))
      (authored-target "inOCL")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind expressionOperand) (ordinal 2))
      (authored-target "language")
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
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (value (kind string) (value "ocl")))
    (evaluated (declaration (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (value (kind string) (value "ocl")))
    (evaluated (declaration (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (value (kind string) (value "ocl")))
    (evaluated (declaration (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (value (kind string) (value "ocl")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (path (named (kind package) (name "TextualRepresentationTest")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 4 18) (end 4 22)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 6 6) (end 6 9)) (probe (position 6 6))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind expressionOperand) (ordinal 0) (authored-target "rep")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 6 10) (end 6 15)) (probe (position 6 10))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind expressionOperand) (ordinal 1) (authored-target "inOCL")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 6 16) (end 6 24)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C::x_constraint"))) (kind expressionOperand) (ordinal 2) (authored-target "language")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 12 9) (end 12 10)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::C")))))
  )
  (query (document "memory://snapshot/textual_representation_test.md") (range (start 13 12) (end 13 16)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/textual_representation_test.md") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
