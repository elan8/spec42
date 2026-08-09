# META
~~~ini
description=SysML Example (Simple Tests): ConjugationTest
type=file
~~~
# SOURCE
~~~sysml
package ConjugationTest {
	port def P;
	
	part def B {
		port p1: P;
		port p2: ~P;
	}
	
	connection def A {
		end port p1: P;
		end port p2: ~P;
	}
	
	interface def I {
		end p1: P;
		end p2: ~P;
	}
	
	part def B1 {
		part p {
			port p1: P;
			port p2: ~P;		
		}
	
		connection a: A {
			end port p3: P ::> p.p1;
			end port p4: ~P ::> p.p2;
		}
		interface i: I {
			end port p3: P ::> p.p1;
			end port p4: ~P ::> p.p2;
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPort,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,KwPort,Ident,Colon,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwConnection,Ident,Colon,Ident,OpenCurly,
KwEnd,KwPort,Ident,Colon,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Tilde,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwInterface,Ident,Colon,Ident,OpenCurly,
KwEnd,KwPort,Ident,Colon,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,KwPort,Ident,Colon,Tilde,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ConjugationTest'
    (port_def 'P')
    (part_def 'B'
      (port_usage 'p1' : 'P')
      (port_usage 'p2' : ~'P'))
    (connection_def 'A'
      (interface_end end 'p1' : 'P')
      (interface_end end 'p2' : 'P'))
    (interface_def 'I'
      (interface_end end 'p1' : 'P')
      (interface_end end 'p2' : 'P'))
    (part_def 'B1'
      (part_usage 'p'
        (port_usage 'p1' : 'P')
        (port_usage 'p2' : ~'P'))
      (connection_usage 'A' 'a'
        (interface_end end 'p3' : 'P' references 'p.p1')
        (interface_end end 'p4' : 'P' references 'p.p2'))
      (interface_usage 'I' 'i'
        (interface_end end 'p3' : 'P' references 'p.p1')
        (interface_end end 'p4' : 'P' references 'p.p2')))))
~~~
# FORMAT
~~~sysml
package ConjugationTest {
    port def P;

    part def B {
        port p1: P;
        port p2: ~P;
    }

    connection def A {
        end port p1: P;
        end port p2: ~P;
    }

    interface def I {
        end p1: P;
        end p2: ~P;
    }

    part def B1 {
        part p {
            port p1: P;
            port p2: ~P;
        }

        connection a: A {
            end port p3: P ::> p.p1;
            end port p4: ~P ::> p.p2;
        }
        interface i: I {
            end port p3: P ::> p.p1;
            end port p4: ~P ::> p.p2;
        }
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
    (element (kind "package") (id (node (document "d0") (qualified-name "ConjugationTest"))) (name "ConjugationTest") (declared-name "ConjugationTest")
      (contains
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ConjugationTest::A"))) (name "A") (declared-name "A")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (name "p1") (declared-name "p1") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::A")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (name "p2") (declared-name "p2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::A")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "ConjugationTest::B"))) (name "B") (declared-name "B") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (name "p1") (declared-name "p1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (name "p2") (declared-name "p2") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "ConjugationTest::B1"))) (name "B1") (declared-name "B1") (declared)
          (contains
            (element (kind "connection") (id (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1"))))
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (name "p3") (declared-name "p3") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::A")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (name "p4") (declared-name "p4") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::A")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (name "p") (declared-name "p") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1"))))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (name "p1") (declared-name "p1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (name "p2") (declared-name "p2") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1")))))
              )
            )
          )
        )
        (element (kind "interface def") (id (node (document "d0") (qualified-name "ConjugationTest::I"))) (name "I") (declared-name "I")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (name "p1") (declared-name "p1") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::I")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (name "p2") (declared-name "p2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::I")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "ConjugationTest::P"))) (name "P") (declared-name "P")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (name "~P") (declared-name "~P") (effective (featuring-type (node (document "d0") (qualified-name "ConjugationTest::P")))))
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))))
    (referenceSubsetting (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (to (node (document "d0") (qualified-name "ConjugationTest::A::p1"))))
    (referenceSubsetting (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (to (node (document "d0") (qualified-name "ConjugationTest::A::p2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (to (node (document "d0") (qualified-name "ConjugationTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))))
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
  (document "sysml/examples/conjugation_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 20 3) (end 20 14))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 21 3) (end 21 15))
      )
    )
  )
)
~~~
