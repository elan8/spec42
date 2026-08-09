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

    comment about Automobile /* This is an unnamed comment, annotating an 
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
(model
  (namespace
    (package 'Comment Example'
      (comment_annotating 'Comment1')
      (comment_annotating)
      (part_def 'Automobile')
      (alias_member 'Car' -> 'Comment Example::Automobile'[part_def])
      (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved]))))
~~~
