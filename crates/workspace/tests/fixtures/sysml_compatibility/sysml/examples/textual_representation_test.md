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
        attribute x : Real;
        assert constraint x_constraint {
            rep inOCL language "ocl" /* self.x > 0.0 */
        }
    }

    action def setX {
        in c : C;
        in newX : Real;

        language "alf" /* c.x = newX;
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
(model
  (namespace
    (package 'TextualRepresentationTest'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (item_def 'C'
        (attribute_usage composite 'x' : 'Real'[unresolved])
        (assert_constraint_usage 'x_constraint'
          (textual_rep 'inOCL')))
      (action_def 'setX'
        (reference_usage in reference 'c' : 'TextualRepresentationTest::C'[item_def])
        (reference_usage in reference 'newX' : 'Real'[unresolved])
        (textual_rep)))))
~~~
