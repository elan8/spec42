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
part def A {
    port p1;
    port p2;
}
part def B {
    port q1;
    port q2;
}

part def System {
    part a : A;
    part b : B;

    connector c1 from a.p1 to b.q1;
    connector c2 :> c1 from a.p2 to b.q2;

    binding b1 of a.p1 = b.q1;
    binding a.p2 = b.q2;

    ref part engine : A;
    individual part myA : A;
}
~~~
# SMG
~~~
(model
  (namespace
    (part_def 'A'
      (port_usage composite 'p1')
      (port_usage composite 'p2'))
    (part_def 'B'
      (port_usage composite 'q1')
      (port_usage composite 'q2'))
    (part_def 'System'
      (part_usage composite 'a' : 'A'[part_def])
      (part_usage composite 'b' : 'B'[part_def])
      (connector_def 'c1'
        (connector_end 'a.p1')
        (connector_end 'b.q1'))
      (not_implemented 'malformed')
      (binding_connector_def 'b1'
        (connector_end 'a.p1')
        (connector_end 'b.q1'))
      (binding_connector_def
        (connector_end 'a.p2')
        (connector_end 'b.q2'))
      (part_usage reference 'engine' : 'A'[part_def])
      (part_usage individual composite 'myA' : 'A'[part_def]))))
~~~
