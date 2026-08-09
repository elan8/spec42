# META
~~~ini
description=KerML Simple Tests: Circular
type=file
~~~
# SOURCE
~~~kerml
package Circular {
	class A { }
	feature a: A;
	alias Circ for Circular;
	package P {
		public import Circular::*;
	}
	
	feature x :> z;
	feature y :> x;
	feature z :> y;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,CloseCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
CloseCurly,
KwFeature,Ident,ColonGt,Ident,Semicolon,
KwFeature,Ident,ColonGt,Ident,Semicolon,
KwFeature,Ident,ColonGt,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Circular'
    (class_def 'A')
    (feature_def 'a' : 'A')
    (alias_member 'Circ' for 'Circular')
    (package_def 'P'
      (import_decl public 'Circular::*'))
    (feature_def 'x' :> 'z')
    (feature_def 'y' :> 'x')
    (feature_def 'z' :> 'y')))
~~~
# FORMAT
~~~sysml
package Circular {
    class A { }
    feature a : A;
    alias Circ for Circular;
    package P {
        public import Circular::*;
    }

    feature x :> z;
    feature y :> x;
    feature z :> y;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Circular"))) (name "Circular") (declared-name "Circular")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Circular::A"))) (name "A") (declared-name "A"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Circular::Circ"))) (name "Circ") (declared-name "Circ"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Circular::P"))) (name "P") (declared-name "P")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Circular::P::*"))) (name "*") (declared-name "*"))
          )
        )
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Circular::a"))) (name "a") (declared-name "a"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Circular::x"))) (name "x") (declared-name "x"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Circular::y"))) (name "y") (declared-name "y"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Circular::z"))) (name "z") (declared-name "z"))
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
