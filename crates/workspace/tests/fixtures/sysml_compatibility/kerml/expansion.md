# META
~~~ini
description=KerML Simple Tests: Expansion
type=file
~~~
# SOURCE
~~~kerml
package Expansion {
	private import ControlFunctions::select;
	feature x = x->select {in y; in w; in z; w+1}; 
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,Ident,Plus,DecimalValue,CloseCurly,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Expansion'
    (import_decl private 'ControlFunctions::select')
    (feature_def 'x' value)))
~~~
# FORMAT
~~~sysml
package Expansion {
    private import ControlFunctions::select;
    feature x = x->select {in y; in w; in z; w+1};
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Expansion"))) (name "Expansion") (declared-name "Expansion")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Expansion::select"))) (name "select") (declared-name "select"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Expansion::x"))) (name "x") (declared-name "x"))
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
