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
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 39 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_attribute_body_element")
        (source "parser")
        (range (start 36 2) (end 39 1))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 36 2) (end 39 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 1) (end 44 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e8a112900c3b200845c2ff34e7933480efbd2709124e0ca7ae3f8141e9b216d9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/comments.md") (qualified-name "Comments"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
