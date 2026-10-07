# META
~~~ini
description=SysML Example (Comment): Comments
type=file
~~~
# SOURCE
~~~sysml
package Comments {
	doc /* Documentation Comment */

	doc /* Documentation about Package */

	comment cmt /* Named Comment */	
	comment cmt_cmt about cmt /* Comment about Comment */
	
	comment about C /* Documention Comment on Part Def */
	part def C {
		doc /* Documentation in Part Def */
		comment /* Comment in Part Def */
		comment about Comments /* Comment about Package */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/comments.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:10c9223c97aac2807da14524934bddebcb193354881d2a0b2c5800c57607cc55"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Documentation Comment "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind documentation) (ordinal 1))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Documentation about Package "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Documention Comment on Part Def "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "C")))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Documentation in Part Def "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Comment in Part Def "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Comment about Package "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Comments")))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Named Comment "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Comment about Comment "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "cmt")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C")))))
    (reference (id (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0))
      (authored-target "Comments")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments")))))
    (reference (id (source (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind annotation) (ordinal 0))
      (authored-target "cmt")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt")))))
  )
  (relationships
    (relationship (kind annotation) (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 0))))) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind annotation) (ordinal 0)))
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
  (query (document "memory://snapshot/comments.md") (range (start 8 15) (end 8 16)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C")))))
    )
  )
  (query (document "memory://snapshot/comments.md") (range (start 12 16) (end 12 24)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0) (authored-target "Comments")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments")))))
    )
  )
  (query (document "memory://snapshot/comments.md") (range (start 6 23) (end 6 26)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind annotation) (ordinal 0) (authored-target "cmt")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt")))))
    )
  )
)
~~~
