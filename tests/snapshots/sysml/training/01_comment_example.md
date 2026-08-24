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
  (document "memory://snapshot/01_comment_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 18) (end 22 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:28c2d48f5427c64c78239d51d094404aed41b924871a7a17ba5f795e6c1b7b47") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " This is a comment, which is a part of the model, \n\t * annotating (by default) it's owning namespace. ")) (comment (text " This is a named comment. ")) (comment (text " This is an unnamed comment, annotating an \n\t * explicitly specified element. \n\t "))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (kind alias) (membership (kind alias) (visibility default)) (documentation (comment (text "\n\t\t * This is a comment annotating its owning\n\t\t * element.\n\t\t "))) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Automobile")))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile")))))
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/01_comment_example.md") (range (start 13 15) (end 13 25)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (kind aliasBinding) (ordinal 0) (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile")))))
    )
  )
  (query (document "memory://snapshot/01_comment_example.md") (range (start 22 18) (end 22 34)) (probe (position 22 18))
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
