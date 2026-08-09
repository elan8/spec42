# META
~~~ini
description=Coverage: Connector from/to forms, binding connector variants, connector specializations
type=file
~~~
# SOURCE
~~~sysml
part def A { port p1; port p2; }
part def B { port q1; port q2; }

part def System {
    part a : A;
    part b : B;

    connector c1 from a.p1 to b.q1;
    connector c2 :> c1 from a.p2 to b.q2;

    binding b1 of a.p1 = b.q1;
    binding of a.p2 = b.q2;

    ref part engine : A;
    individual part myA : A;
}
~~~
# EXPECTED
~~~
parse.expected_keyword_to
~~~
# PROBLEMS
~~~
parse.expected_keyword_to
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,KwPort,Ident,Semicolon,KwPort,Ident,Semicolon,CloseCurly,
KwPart,KwDef,Ident,OpenCurly,KwPort,Ident,Semicolon,KwPort,Ident,Semicolon,CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwConnector,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,ColonGt,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBinding,Ident,KwOf,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBinding,KwOf,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
KwIndividual,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'A'
    (port_usage 'p1')
    (port_usage 'p2'))
  (part_def 'B'
    (port_usage 'q1')
    (port_usage 'q2'))
  (part_def 'System'
    (part_usage 'a' : 'A')
    (part_usage 'b' : 'B')
    (connector_def 'c1'
      (connector_end)
      (connector_end))
    (malformed)
    (binding_connector 'b1'
      (connector_end)
      (connector_end))
    (binding_connector
      (connector_end)
      (connector_end))
    (part_usage ref 'engine' : 'A')
    (part_usage individual 'myA' : 'A')))
~~~
# FORMAT
~~~sysml
part def A { port p1; port p2; }
part def B { port q1; port q2; }

part def System {
    part a : A;
    part b : B;

    connector c1 from a.p1 to b.q1;
    connector c2 :> c1 from a.p2 to b.q2;

    binding b1 of a.p1 = b.q1;
    binding of a.p2 = b.q2;

    ref part engine : A;
    individual part myA : A;
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "part def") (id (node (document "d0") (qualified-name "A"))) (name "A") (declared-name "A") (declared)
      (contains
        (element (kind "port") (id (node (document "d0") (qualified-name "A::p1"))) (name "p1") (declared-name "p1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "A")))))
        (element (kind "port") (id (node (document "d0") (qualified-name "A::p2"))) (name "p2") (declared-name "p2") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "A")))))
      )
    )
    (element (kind "part def") (id (node (document "d0") (qualified-name "B"))) (name "B") (declared-name "B") (declared)
      (contains
        (element (kind "port") (id (node (document "d0") (qualified-name "B::q1"))) (name "q1") (declared-name "q1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "B")))))
        (element (kind "port") (id (node (document "d0") (qualified-name "B::q2"))) (name "q2") (declared-name "q2") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "B")))))
      )
    )
    (element (kind "part def") (id (node (document "d0") (qualified-name "System"))) (name "System") (declared-name "System") (declared)
      (contains
        (element (kind "part") (id (node (document "d0") (qualified-name "System::a"))) (name "a") (declared-name "a") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "System")))))
        (element (kind "part") (id (node (document "d0") (qualified-name "System::b"))) (name "b") (declared-name "b") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "System")))))
        (element (kind "ref") (id (node (document "d0") (qualified-name "System::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "System")))))
        (element (kind "part") (id (node (document "d0") (qualified-name "System::myA"))) (name "myA") (declared-name "myA") (declared (properties (individual true) (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "System")))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "System::a"))) (to (node (document "d0") (qualified-name "A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "System::b"))) (to (node (document "d0") (qualified-name "B"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "System::engine"))) (to (node (document "d0") (qualified-name "A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "System::myA"))) (to (node (document "d0") (qualified-name "A"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
