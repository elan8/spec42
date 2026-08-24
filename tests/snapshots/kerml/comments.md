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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e8a112900c3b200845c2ff34e7933480efbd2709124e0ca7ae3f8141e9b216d9") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text "\n*AAA\n * BBB")) (comment (text "\n    *\n    *\n    * AAA  ***   \n    *BBB\n    \t\t\t\t\t\t\t\t")) (comment (text "\n *       AAAA\n *       BBBB           ")) (comment (text " AAAA\n \n \n  * BBBB\n *\n * CCCC\n ")) (comment (locale "en_US") (text "\n * AAAA\n * BBBB\n *    CCC DDD    \n ")) (comment (text " comment inside a package ")) (comment (text " Named Comment ")) (comment (text " Other Comment about Comment ")) (comment (text " abc "))))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments::C"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (locale "en_US") (text " Documentation on Class C ")) (comment (text " Comment in Class C ")) (comment (text " Comment about Package "))))
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
