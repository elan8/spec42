# META
~~~ini
description=SysML Example (Import Tests): CircularImport
type=file
~~~
# SOURCE
~~~sysml
package CircularImport {

	package P1 {
		public import P2::*;
		part def A;
	}
	package P2 {
		public import P1::*;
		part def B;
	}
	package Test1 {
		public import P1::*;
		part x: A;
		part y: B;
	}
	package Test2 {
		public import P2::*;
		part x: A;
		part y: B;
	}
	
	part x: P1::A;
	
	// The following should not fail.
	part y: P1::B;
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CircularImport'
    (package_def 'P1'
      (import_decl public 'P2::*')
      (part_def 'A'))
    (package_def 'P2'
      (import_decl public 'P1::*')
      (part_def 'B'))
    (package_def 'Test1'
      (import_decl public 'P1::*')
      (part_usage 'x' : 'A')
      (part_usage 'y' : 'B'))
    (package_def 'Test2'
      (import_decl public 'P2::*')
      (part_usage 'x' : 'A')
      (part_usage 'y' : 'B'))
    (part_usage 'x' : 'P1::A')
    (line_comment)
    (part_usage 'y' : 'P1::B')))
~~~
# FORMAT
~~~sysml
package CircularImport {

    package P1 {
        public import P2::*;
        part def A;
    }
    package P2 {
        public import P1::*;
        part def B;
    }
    package Test1 {
        public import P1::*;
        part x: A;
        part y: B;
    }
    package Test2 {
        public import P2::*;
        part x: A;
        part y: B;
    }

    part x: P1::A;

    // The following should not fail.
    part y: P1::B;

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
    (element (kind "package") (id (node (document "d0") (qualified-name "CircularImport"))) (name "CircularImport") (declared-name "CircularImport")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "CircularImport::P1"))) (name "P1") (declared-name "P1")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "CircularImport::P1::*"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "CircularImport::P1::A"))) (name "A") (declared-name "A") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "CircularImport::P2"))) (name "P2") (declared-name "P2")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "CircularImport::P2::*"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "CircularImport::P2::B"))) (name "B") (declared-name "B") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "CircularImport::Test1"))) (name "Test1") (declared-name "Test1")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "CircularImport::Test1::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "CircularImport::Test1::x"))) (name "x") (declared-name "x") (declared (properties (composite true) (reference false) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "CircularImport::Test1::y"))) (name "y") (declared-name "y") (declared (properties (composite true) (reference false) (ordered false))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "CircularImport::Test2"))) (name "Test2") (declared-name "Test2")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "CircularImport::Test2::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "CircularImport::Test2::x"))) (name "x") (declared-name "x") (declared (properties (composite true) (reference false) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "CircularImport::Test2::y"))) (name "y") (declared-name "y") (declared (properties (composite true) (reference false) (ordered false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "CircularImport::x"))) (name "x") (declared-name "x") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "CircularImport::y"))) (name "y") (declared-name "y") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "CircularImport::Test1::x"))) (to (node (document "d0") (qualified-name "CircularImport::P1::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CircularImport::Test1::y"))) (to (node (document "d0") (qualified-name "CircularImport::P2::B"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CircularImport::Test2::x"))) (to (node (document "d0") (qualified-name "CircularImport::P1::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CircularImport::Test2::y"))) (to (node (document "d0") (qualified-name "CircularImport::P2::B"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CircularImport::x"))) (to (node (document "d0") (qualified-name "CircularImport::P1::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CircularImport::y"))) (to (node (document "d0") (qualified-name "CircularImport::P2::B"))))
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
  (document "sysml/examples/circular_import.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 2) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 2) (end 7 22))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 21 1) (end 21 15))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 24 1) (end 24 15))
      )
    )
  )
)
~~~
