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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cab89b300cc637b967bd447c61b4cac265bebc1c080a13e79122f6699df11964") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text "\n*AAA\n * BBB")) (comment (text "\n    *\n    *\n    * AAA  ***   \n    *BBB\n    \t\t\t\t\t\t\t\t")) (comment (text "\n *       AAAA\n *       BBBB           ")) (comment (text " AAAA\n \n \n  * BBBB\n *\n * CCCC\n ")) (comment (locale "en_US") (text "\n * AAAA\n * BBBB\n *    CCC DDD    \n ")) (comment (text " comment inside a package ")) (doc (locale "en_US") (text " Documentation about Package ")) (comment (text " Named Comment ")) (comment (text " Comment about Comment ")) (comment (text " Documention Comment about Part Def ")) (comment (text " abc "))))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comment_test.md") (qualified-name "CommentTest::C"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Documentation in Part Def ")) (comment (text " Comment in Part Def ")) (comment (locale "en_US") (text " Comment about Package "))))
  )
  (references
  )
  (relationships
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
)
~~~
