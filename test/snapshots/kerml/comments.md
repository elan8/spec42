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
  (document "comments.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 24 1) (end 24 92))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
RegularComment,
LineComment,
KwPackage,Ident,OpenCurly,
LineComment,
RegularComment,
RegularComment,
RegularComment,
RegularComment,
KwLocale,StringValue,RegularComment,
RegularComment,
KwComment,Ident,RegularComment,
KwComment,Ident,KwAbout,Ident,RegularComment,
KwClass,Ident,OpenCurly,
KwDoc,KwLocale,StringValue,RegularComment,
KwComment,RegularComment,
KwComment,KwAbout,Ident,RegularComment,
CloseCurly,
RegularComment,
KwClass,Ident,OpenCurly,
KwDoc,OpenAngle,Ident,CloseAngle,RegularComment,
KwComment,KwAbout,Ident,KwLocale,StringValue,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (comment)
  (line_comment)
  (package_def 'Comments'
    (line_comment)
    (comment)
    (comment)
    (comment)
    (comment)
    (comment_annotating locale "en_US")
    (comment)
    (comment_annotating 'cmt')
    (comment_annotating 'cmt_cmt' about 'cmt')
    (class_def 'C'
      (documentation locale "en_US")
      (comment_annotating)
      (comment_annotating about 'Comments'))
    (comment)
    (class_def 'A'
      (documentation 'a')
      (comment_annotating about 'a' locale "en_US"))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "807b81015df756cfa6185f889f5edcc2ef6d73439150124fbb0e72297eeaac40") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Comments"))) (kind "package") (name "Comments") (declared-name "Comments") (range (start (line 2) (character 0)) (end (line 2) (character 693))))
    (element (id (node (document "d0") (qualified-name "Comments::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 41) (character 1)) (end (line 41) (character 135))) (parent (node (document "d0") (qualified-name "Comments"))))
    (element (id (node (document "d0") (qualified-name "Comments::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 34) (character 1)) (end (line 34) (character 155))) (parent (node (document "d0") (qualified-name "Comments"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
