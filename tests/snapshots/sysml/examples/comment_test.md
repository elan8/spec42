# META
~~~ini
description=SysML Example (Simple Tests): CommentTest
type=file
~~~
# SOURCE
~~~sysml
  /* AAA */
  //a lexical comment ("note") is not a part of model
package CommentTest {
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
	doc locale "en_US" /* Documentation about Package */
	comment cmt /* Named Comment */	
	comment cmt_cmt about cmt /* Comment about Comment */
	
	comment about C /* Documention Comment about Part Def */
	part def C {
		doc /* Documentation in Part Def */
		comment /* Comment in Part Def */
		comment about CommentTest locale "en_US" /* Comment about Package */
	}
	/* abc */
	part def A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/comment_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:88cdcf53e43681b273eecd82e7ff17e79726c77a2c9dac90ed36ea1a5fb2dee4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "AAA\nBBB"))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 1))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "\n\nAAA  ***   \nBBB\n"))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 2))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "      AAAA\n      BBBB           "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 3))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "AAAA\n\n\nBBBB\n\nCCCC\n"))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 4))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (locale "en_US") (text "AAAA\nBBBB\n   CCC DDD    \n"))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 5))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "comment inside a package "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (locale "en_US") (text "Documentation about Package "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 6))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Documention Comment about Part Def "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "C")))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 7))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "abc "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::C"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Documentation in Part Def "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Comment in Part Def "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (locale "en_US") (text "Comment about Package "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "CommentTest")))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Named Comment "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt_cmt"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "Comment about Comment "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "cmt")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 6))))) (kind annotation) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::C")))))
    (reference (id (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0))
      (authored-target "CommentTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest")))))
    (reference (id (source (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt_cmt"))) (kind annotation) (ordinal 0))
      (authored-target "cmt")
      (outcome (status resolved) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt")))))
  )
  (relationships
    (relationship (kind annotation) (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 6))))) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 6))))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt_cmt"))) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt_cmt"))) (kind annotation) (ordinal 0)))
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
  (query (document "memory://snapshot/comment_test.md") (range (start 35 15) (end 35 16)) (probe (position 35 15))
    (reference (id (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (anonymous (kind comment) (ordinal 6))))) (kind annotation) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::C")))))
    )
  )
  (query (document "memory://snapshot/comment_test.md") (range (start 39 16) (end 39 27)) (probe (position 39 16))
    (reference (id (source (node (document "memory://snapshot/comment_test.md") (path (named (kind package) (name "CommentTest")) (named (kind part-def) (name "C")) (anonymous (kind comment) (ordinal 1))))) (kind annotation) (ordinal 0) (authored-target "CommentTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest")))))
    )
  )
  (query (document "memory://snapshot/comment_test.md") (range (start 33 23) (end 33 26)) (probe (position 33 23))
    (reference (id (source (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt_cmt"))) (kind annotation) (ordinal 0) (authored-target "cmt")
      (outcome (status resolved) (target (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::cmt")))))
    )
  )
)
~~~
