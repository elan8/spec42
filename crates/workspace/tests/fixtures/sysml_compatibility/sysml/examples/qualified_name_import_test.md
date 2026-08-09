# META
~~~ini
description=SysML Example (Import Tests): QualifiedNameImportTest
type=file
~~~
# SOURCE
~~~sysml
package QualifiedNameImportTest {
	package P1 {
		part def A;
	}
	package P2 {
		package P2a {
			public import P1::*;
		}
		// The following should not fail.
		// A is a member of P2a because of the import.
		part x: P2a::A;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
CloseCurly,
LineComment,
LineComment,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'QualifiedNameImportTest'
    (package_def 'P1'
      (part_def 'A'))
    (package_def 'P2'
      (package_def 'P2a'
        (import_decl public 'P1::*'))
      (line_comment)
      (line_comment)
      (part_usage 'x' : 'P2a::A'))))
~~~
# FORMAT
~~~sysml
package QualifiedNameImportTest {
    package P1 {
        part def A;
    }
    package P2 {
        package P2a {
            public import P1::*;
        }
        // The following should not fail.
        // A is a member of P2a because of the import.
        part x: P2a::A;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "QualifiedNameImportTest"))) (name "QualifiedNameImportTest") (declared-name "QualifiedNameImportTest")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P1"))) (name "P1") (declared-name "P1")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P1::A"))) (name "A") (declared-name "A") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2"))) (name "P2") (declared-name "P2")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a"))) (name "P2a") (declared-name "P2a")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::P2a::*"))) (name "*") (declared-name "*"))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::x"))) (name "x") (declared-name "x") (declared (properties (ordered false))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "QualifiedNameImportTest::P2::x"))) (to (node (document "d0") (qualified-name "QualifiedNameImportTest::P1::A"))))
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
  (document "sysml/examples/qualified_name_import_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 17) (end 6 19))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 10 10) (end 10 16))
      )
    )
  )
)
~~~
