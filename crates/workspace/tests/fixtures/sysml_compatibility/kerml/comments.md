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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Comments"))) (name "Comments") (declared-name "Comments")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Comments::A"))) (name "A") (declared-name "A"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Comments::C"))) (name "C") (declared-name "C"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
