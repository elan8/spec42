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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAssert,KwConstraint,Ident,OpenCurly,
KwRep,Ident,KwLanguage,StringValue,
RegularComment,
CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwLanguage,StringValue,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TextualRepresentationTest'
    (import_decl private 'ScalarValues::Real')
    (item_def 'C'
      (attribute_usage 'x' : 'Real')
      (sysml_decl 'x_constraint'
        (textual_rep 'inOCL' language '"ocl"')))
    (action_def 'setX'
      (default_ref_usage in 'c' : 'C')
      (default_ref_usage in 'newX' : 'Real')
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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a0410c70f61383bc6d8308a8dd1dc8ddc2a2105cad2b6f217cb9d4e43baa491a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest"))) (kind "package") (name "TextualRepresentationTest") (declared-name "TextualRepresentationTest") (range (start (line 0) (character 0)) (end (line 0) (character 375))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::C"))) (kind "item def") (name "C") (declared-name "C") (range (start (line 3) (character 1)) (end (line 3) (character 146))) (parent (node (document "d0") (qualified-name "TextualRepresentationTest"))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 4) (character 5)) (end (line 4) (character 23))) (parent (node (document "d0") (qualified-name "TextualRepresentationTest::C"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "TextualRepresentationTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (kind "action def") (name "setX") (declared-name "setX") (range (start (line 11) (character 1)) (end (line 11) (character 148))) (parent (node (document "d0") (qualified-name "TextualRepresentationTest"))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (kind "in out parameter") (name "c") (declared-name "c") (range (start (line 12) (character 2)) (end (line 12) (character 11))) (parent (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (authored (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind "in out parameter") (name "newX") (declared-name "newX") (range (start (line 13) (character 2)) (end (line 13) (character 17))) (parent (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (authored (relationships (typing (reference "Real") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TextualRepresentationTest::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TextualRepresentationTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TextualRepresentationTest::Real")))))
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
