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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3a615c3d20ff6700dfad8657acf88416607715b5e33b0ca55e3d607da2a54aa9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "This is a comment, which is a part of the model, \nannotating (by default) it's owning namespace. "))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (anonymous (kind comment) (ordinal 1))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "This is an unnamed comment, annotating an \nexplicitly specified element. \n"))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Automobile")))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Automobile")))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (named (kind alias) (name "Car")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "This is a comment annotating its owning\nelement.\n"))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Comment1"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "This is a named comment. "))))
    (declaration (id (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0))
      (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile")))))
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Car"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile")))))
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind annotation) (source (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (anonymous (kind comment) (ordinal 1))))) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0)))
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
  (query (document "memory://snapshot/01_comment_example.md") (range (start 6 15) (end 6 25)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/01_comment_example.md") (path (named (kind package) (name "Comment Example")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0) (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_comment_example.md") (qualified-name "Comment Example::Automobile")))))
    )
  )
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
