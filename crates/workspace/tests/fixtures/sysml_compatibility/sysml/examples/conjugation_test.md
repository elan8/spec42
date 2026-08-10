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
            (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (name "p1") (declared-name "p1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (name "p2") (declared-name "p2") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B")))))
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
            (element (kind "part") (id (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (name "p") (declared-name "p") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1"))))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (name "p1") (declared-name "p1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (name "p2") (declared-name "p2") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ConjugationTest::B1")))))
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
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (to (node (document "d0") (qualified-name "ConjugationTest::A"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (to (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (to (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::A"))) (status missing-prerequisite) (target "Connections::Connection"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (status missing-prerequisite) (target "Connections::connections"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::I"))) (status missing-prerequisite) (target "Interfaces::Interface"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::P"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (status missing-prerequisite) (target "Ports::Port"))
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
