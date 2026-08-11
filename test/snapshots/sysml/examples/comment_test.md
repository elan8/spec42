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
  (document "comment_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 24 1) (end 24 92))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 31 1) (end 31 55))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "71a49d648e41b8100cb032c1a6868dd51734bc1a8124bfc011efb05fc1134669") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CommentTest"))) (kind "package") (name "CommentTest") (declared-name "CommentTest") (range (start (line 2) (character 0)) (end (line 2) (character 685))))
    (element (id (node (document "d0") (qualified-name "CommentTest::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 42) (character 1)) (end (line 42) (character 12))) (parent (node (document "d0") (qualified-name "CommentTest"))))
    (element (id (node (document "d0") (qualified-name "CommentTest::C"))) (kind "part def") (name "C") (declared-name "C") (range (start (line 36) (character 1)) (end (line 36) (character 161))) (parent (node (document "d0") (qualified-name "CommentTest"))))
    (element (id (node (document "d0") (qualified-name "CommentTest::C::_documentation"))) (kind "documentation") (name "") (range (start (line 36) (character 1)) (end (line 36) (character 161))) (parent (node (document "d0") (qualified-name "CommentTest::C"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
