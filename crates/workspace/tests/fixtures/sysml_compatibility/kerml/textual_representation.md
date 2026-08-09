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
        feature x : Real;
        inv x_constraint {
		    rep inOCL language "ocl" 
		        /* self.x > 0.0 */
	    }
    }

    behavior setX {
        in c: C;
        in newX: Real;
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
    (package 'TextualRepresentation'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (class_def 'C'
        (feature_def 'x' : 'Real'[unresolved])
        (invariant_def 'x_constraint'
          (textual_rep 'inOCL')))
      (behavior_def 'setX'
        (feature_def in 'c' : 'TextualRepresentation::C'[class_def])
        (feature_def in 'newX' : 'Real'[unresolved])
        (textual_rep)))))
~~~
