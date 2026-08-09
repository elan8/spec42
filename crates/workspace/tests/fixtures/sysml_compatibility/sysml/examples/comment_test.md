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
KwDoc,KwLocale,StringValue,RegularComment,
KwComment,Ident,RegularComment,
KwComment,Ident,KwAbout,Ident,RegularComment,
KwComment,KwAbout,Ident,RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwComment,RegularComment,
KwComment,KwAbout,Ident,KwLocale,StringValue,RegularComment,
CloseCurly,
RegularComment,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (comment)
  (line_comment)
  (package_def 'CommentTest'
    (line_comment)
    (comment)
    (comment)
    (comment)
    (comment)
    (comment_annotating locale "en_US")
    (comment)
    (documentation locale "en_US")
    (comment_annotating 'cmt')
    (comment_annotating 'cmt_cmt' about 'cmt')
    (comment_annotating about 'C')
    (part_def 'C'
      (documentation)
      (comment_annotating)
      (comment_annotating about 'CommentTest' locale "en_US"))
    (comment)
    (part_def 'A')))
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
    comment locale "en_US" /*
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
(model
  (namespace
    (package 'CommentTest'
      (comment_annotating)
      (documentation)
      (comment_annotating 'cmt')
      (comment_annotating 'cmt_cmt')
      (comment_annotating)
      (part_def 'C'
        (documentation)
        (comment_annotating)
        (comment_annotating))
      (part_def 'A'))))
~~~
