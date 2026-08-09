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
    (element (kind "package") (id (node (document "d0") (qualified-name "TextualRepresentationTest"))) (name "TextualRepresentationTest") (declared-name "TextualRepresentationTest")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "TextualRepresentationTest::C"))) (name "C") (declared-name "C")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TextualRepresentationTest::C::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TextualRepresentationTest::C")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "TextualRepresentationTest::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX"))) (name "setX") (declared-name "setX")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (name "c") (declared-name "c") (effective (featuring-type (node (document "d0") (qualified-name "TextualRepresentationTest::setX")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TextualRepresentationTest::setX::newX"))) (name "newX") (declared-name "newX") (effective (featuring-type (node (document "d0") (qualified-name "TextualRepresentationTest::setX")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "TextualRepresentationTest::setX::c"))) (to (node (document "d0") (qualified-name "TextualRepresentationTest::C"))))
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
  (document "sysml/examples/textual_representation_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 5) (end 4 23))
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
