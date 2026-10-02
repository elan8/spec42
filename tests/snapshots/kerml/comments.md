# META
~~~ini
description=KerML Simple Tests: Comments
type=file
~~~
# SOURCE
~~~kerml
/* AAA */
//a lexical comment ("note") is not a part of model
package Comments {
	// inside package
	/*
*AAA
 * BBB*/	
 /*
    *
    *
    * AAA  ***   
    *BBB
    								*/

   /*
 *       AAAA
 *       BBBB           */	
 /* AAAA
 
 
  * BBBB
 *
 * CCCC
 */
 locale "en_US" /*
 * AAAA
 * BBBB
 *    CCC DDD    
 */
	
	/* comment inside a package */
	comment cmt /* Named Comment */	
	comment cmt_cmt about cmt /* Other Comment about Comment */
	
	class C {
		doc locale "en_US"/* Documentation on Class C */
		comment /* Comment in Class C */
		comment about Comments /* Comment about Package */
		
	}
	/* abc */
	class A {
		doc <a> /* Documentation comment on A*/
		comment about a locale "en_US" /* Comment about documenation with ID 'a' */		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/comments.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 42 2) (end 44 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e8a112900c3b200845c2ff34e7933480efbd2709124e0ca7ae3f8141e9b216d9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "\n*AAA\n * BBB"))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 1))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "\n    *\n    *\n    * AAA  ***   \n    *BBB\n    \t\t\t\t\t\t\t\t"))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 2))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "\n *       AAAA\n *       BBBB           "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 3))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " AAAA\n \n \n  * BBBB\n *\n * CCCC\n "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 4))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (locale "en_US") (text "\n * AAAA\n * BBBB\n *    CCC DDD    \n "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 5))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " comment inside a package "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (anonymous (kind comment) (ordinal 6))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " abc "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (locale "en_US") (text " Documentation on Class C "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " Comment in Class C "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " Comment about Package "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Comments")))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " Named Comment "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " Other Comment about Comment "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "cmt")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0))
      (authored-target "Comments")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments")))))
    (reference (id (source (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind annotation) (ordinal 0))
      (authored-target "cmt")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt")))))
  )
  (relationships
    (relationship (kind annotation) (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0)))
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
  (query (document "memory://snapshot/comments.md") (range (start 37 16) (end 37 24)) (probe (position 37 16))
    (reference (id (source (node (document "memory://snapshot/comments.md") (path (named (kind package) (name "Comments")) (named (kind class-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0) (authored-target "Comments")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments")))))
    )
  )
  (query (document "memory://snapshot/comments.md") (range (start 32 23) (end 32 26)) (probe (position 32 23))
    (reference (id (source (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt_cmt"))) (kind annotation) (ordinal 0) (authored-target "cmt")
      (outcome (status resolved) (target (node (document "memory://snapshot/comments.md") (qualified-name "Comments::cmt")))))
    )
  )
)
~~~
