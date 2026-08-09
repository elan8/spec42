# META
~~~ini
description=SysML Example (Simple Tests): RootPackageTest
type=file
~~~
# SOURCE
~~~sysml
package P1 {
	part def A;
}

package P2 {
	private import P1::*;
	part a : A;
}

private import P2::*;

package P3 {
	part b subsets a;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P1'
    (part_def 'A'))
  (package_def 'P2'
    (import_decl private 'P1::*')
    (part_usage 'a' : 'A'))
  (import_decl private 'P2::*')
  (package_def 'P3'
    (part_usage 'b' :> 'a')))
~~~
# FORMAT
~~~sysml
package P1 {
    part def A;
}

package P2 {
    private import P1::*;
    part a : A;
}

private import P2::*;

package P3 {
    part b subsets a;
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
    (element (kind "import") (id (node (document "d0") (qualified-name "*"))) (name "*") (declared-name "*"))
    (element (kind "package") (id (node (document "d0") (qualified-name "P1"))) (name "P1") (declared-name "P1")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "P1::A"))) (name "A") (declared-name "A") (declared))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "P2"))) (name "P2") (declared-name "P2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "P2::*"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "P2::a"))) (name "a") (declared-name "a") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "P3"))) (name "P3") (declared-name "P3")
      (contains
        (element (kind "part") (id (node (document "d0") (qualified-name "P3::b"))) (name "b") (declared-name "b") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "P2::a"))) (to (node (document "d0") (qualified-name "P1::A"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/root_package_test.md"
    (diagnostics
    )
  )
)
~~~
