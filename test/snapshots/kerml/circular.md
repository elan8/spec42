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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "circular.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "00aebb6ba004439ee30b63301928598e49736318f230bb516c5dbd7f3ceebae1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Circular"))) (kind "package") (name "Circular") (declared-name "Circular") (range (start (line 0) (character 0)) (end (line 0) (character 172))))
    (element (id (node (document "d0") (qualified-name "Circular::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 12))) (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::Circ"))) (kind "alias") (name "Circ") (declared-name "Circ") (range (start (line 3) (character 1)) (end (line 3) (character 25))) (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 4) (character 1)) (end (line 4) (character 44))) (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::P::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 2)) (end (line 5) (character 28))) (parent (node (document "d0") (qualified-name "Circular::P"))) (authored (membership (kind Import) (visibility "public") (import (reference "Circular::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 24))))))
    (element (id (node (document "d0") (qualified-name "Circular::a"))) (kind "feature decl") (name "a") (declared-name "a") (range (start (line 2) (character 1)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 8) (character 1)) (end (line 8) (character 16))) (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::y"))) (kind "feature decl") (name "y") (declared-name "y") (range (start (line 9) (character 1)) (end (line 9) (character 16))) (parent (node (document "d0") (qualified-name "Circular"))))
    (element (id (node (document "d0") (qualified-name "Circular::z"))) (kind "feature decl") (name "z") (declared-name "z") (range (start (line 10) (character 1)) (end (line 10) (character 16))) (parent (node (document "d0") (qualified-name "Circular"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Circular::P::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Circular::*") (range (start (line 5) (character 16)) (end (line 5) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Circular")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
