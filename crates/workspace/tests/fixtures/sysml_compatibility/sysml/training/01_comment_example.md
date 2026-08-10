# META
~~~ini
description=SysML Training 01 (Packages): Comment Example
type=file
~~~
# SOURCE
~~~sysml
package 'Comment Example' {
	/* This is a comment, which is a part of the model, 
	 * annotating (by default) it's owning namespace. */
	
	comment Comment1 /* This is a named comment. */
	
	comment about Automobile
	/* This is an unnamed comment, annotating an 
	 * explicitly specified element. 
	 */
	 
	part def Automobile;
	
	alias Car for Automobile {
		/*
		 * This is a comment annotating its owning
		 * element.
		 */
	}	                         
	
	// This is a note. It is in the text, but not part 
	// of the model.
	alias Torque for ISQ::TorqueValue;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwComment,Ident,RegularComment,
KwComment,KwAbout,Ident,
RegularComment,
KwPart,KwDef,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,OpenCurly,
RegularComment,
CloseCurly,
LineComment,
LineComment,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Comment Example''
    (comment)
    (comment_annotating 'Comment1')
    (comment_annotating about 'Automobile')
    (part_def 'Automobile')
    (alias_member 'Car' for 'Automobile'
      (comment))
    (line_comment)
    (line_comment)
    (alias_member 'Torque' for 'ISQ::TorqueValue')))
~~~
# FORMAT
~~~sysml
package 'Comment Example' {
    /* This is a comment, which is a part of the model, 
	 * annotating (by default) it's owning namespace. */

    comment Comment1 /* This is a named comment. */

    comment about Automobile
    /* This is an unnamed comment, annotating an 
	 * explicitly specified element. 
	 */

    part def Automobile;

    alias Car for Automobile {
        /*
		 * This is a comment annotating its owning
		 * element.
		 */
    }

    // This is a note. It is in the text, but not part
    // of the model.
    alias Torque for ISQ::TorqueValue;
}

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Comment Example"))) (name "Comment Example") (declared-name "Comment Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Comment Example::Automobile"))) (name "Automobile") (declared-name "Automobile") (declared))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Comment Example::Car"))) (name "Car") (declared-name "Car"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Comment Example::Torque"))) (name "Torque") (declared-name "Torque"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Comment Example::Automobile"))) (status missing-prerequisite) (target "Parts::Part"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/01_comment_example.md"
    (diagnostics
    )
  )
)
~~~
