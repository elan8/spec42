# META
~~~ini
description=SysML Example (Simple Tests): AliasTest
type=file
~~~
# SOURCE
~~~sysml
package AliasTest {
	private import ISQSpaceTime::breadth; // import of an alias
	attribute b :> breadth;
	
    part def P1 {
        port porig1;
        alias po1 for porig1;
    }

    part p1 : P1 {
        port po1 :>> po1;
    }

    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }


    connect p1.po1 to p2.pdest;
	connect p1.po1 to p2.pd1;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,LineComment,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AliasTest'
    (import_decl private 'ISQSpaceTime::breadth')
    (line_comment)
    (attribute_usage 'b' :> 'breadth')
    (part_def 'P1'
      (port_usage 'porig1')
      (alias_member 'po1' for 'porig1'))
    (part_usage 'p1' : 'P1'
      (port_usage 'po1' :>> 'po1'))
    (part_usage 'p2' : 'P1'
      (port_usage 'pdest')
      (alias_member 'pd1' for 'pdest'))
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
package AliasTest {
    private import ISQSpaceTime::breadth; // import of an alias
    attribute b :> breadth;

    part def P1 {
        port porig1;
        alias po1 for porig1;
    }

    part p1 : P1 {
        port po1 :>> po1;
    }

    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }

    connect p1.po1 to p2.pdest;
    connect p1.po1 to p2.pd1;
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'breadth'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'breadth'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AliasTest"))) (name "AliasTest") (declared-name "AliasTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "AliasTest::P1"))) (name "P1") (declared-name "P1") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "AliasTest::P1::porig1"))) (name "porig1") (declared-name "porig1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AliasTest::P1")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "AliasTest::b"))) (name "b") (declared-name "b") (declared (properties (ordered false) (unique true))))
        (element (kind "import") (id (node (document "d0") (qualified-name "AliasTest::breadth"))) (name "breadth") (declared-name "breadth"))
        (element (kind "part") (id (node (document "d0") (qualified-name "AliasTest::p1"))) (name "p1") (declared-name "p1") (declared (properties (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (name "po1") (declared-name "po1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AliasTest::P1")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "AliasTest::p2"))) (name "p2") (declared-name "p2") (declared (properties (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "AliasTest::p2::pdest"))) (name "pdest") (declared-name "pdest") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AliasTest::P1")))))
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "AliasTest::p1::po1"))) (to (node (document "d0") (qualified-name "AliasTest::p2::pdest"))) (connect (source-expression "p1::po1") (target-expression "p2::pdest") (container-prefix "AliasTest")))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AliasTest::p1"))) (to (node (document "d0") (qualified-name "AliasTest::P1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AliasTest::p2"))) (to (node (document "d0") (qualified-name "AliasTest::P1"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (connection (status pending-expression) (document "d0") (source-expression "p1::po1") (target-expression "p2::pd1") (container-prefix "AliasTest"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/alias_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 1) (end 2 24))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 6 8) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 10 8) (end 10 25))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 15 8) (end 15 33))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 20 9) (end 20 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 20 9) (end 20 15))
      )
    )
  )
)
~~~
