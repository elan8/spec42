# META
~~~ini
description=KerML Simple Tests: Classifications
type=file
~~~
# SOURCE
~~~kerml
package Classifications {
	class T;
	x;
	y = x istype T or x hastype z;
	z = (all T)#(3);
	a = x as T;
	b = x meta KerML::Feature;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifications.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 2 1) (end 2 95))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
Ident,Semicolon,
Ident,Eq,Ident,KwIstype,Ident,KwOr,Ident,KwHastype,Ident,Semicolon,
Ident,Eq,OpenParen,KwAll,Ident,CloseParen,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
Ident,Eq,Ident,KwAs,Ident,Semicolon,
Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Classifications'
    (class_def 'T')
    (feature_def 'x')
    (feature_def 'y' value)
    (feature_def 'z' value)
    (feature_def 'a' value)
    (feature_def 'b' value)))
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
package Classifications {
    class T;
    x;
    y = x istype T or x hastype z;
    z = (all T)#(3);
    a = x as T;
    b = x meta KerML::Feature;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1c80543436423961605d7294767f49e0dd0564633255ad0f4ac8d5afae09eda1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Classifications"))) (kind "package") (name "Classifications") (declared-name "Classifications") (range (start (line 0) (character 0)) (end (line 0) (character 132))))
    (element (id (node (document "d0") (qualified-name "Classifications::T"))) (kind "classifier decl") (name "T") (declared-name "T") (range (start (line 1) (character 1)) (end (line 1) (character 9))) (parent (node (document "d0") (qualified-name "Classifications"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
