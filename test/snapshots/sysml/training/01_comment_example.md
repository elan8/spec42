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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "01_comment_example.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d47ea1b6ed0749eaf1efd2263e2f3e6c13b8809ef82f571ab32980e10d54050f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Comment Example"))) (kind "package") (name "Comment Example") (declared-name "Comment Example") (range (start (line 0) (character 0)) (end (line 0) (character 566))))
    (element (id (node (document "d0") (qualified-name "Comment Example::Automobile"))) (kind "part def") (name "Automobile") (declared-name "Automobile") (range (start (line 11) (character 1)) (end (line 11) (character 21))) (parent (node (document "d0") (qualified-name "Comment Example"))))
    (element (id (node (document "d0") (qualified-name "Comment Example::Car"))) (kind "alias") (name "Car") (declared-name "Car") (range (start (line 13) (character 1)) (end (line 13) (character 100))) (parent (node (document "d0") (qualified-name "Comment Example"))))
    (element (id (node (document "d0") (qualified-name "Comment Example::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (range (start (line 22) (character 1)) (end (line 22) (character 35))) (parent (node (document "d0") (qualified-name "Comment Example"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
