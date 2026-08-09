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
        port p1 : P;
        port p2 : ~P;
    }

    connection def A {
        end p1 : P;
        end p2 : P;
    }

    interface def I {
        end p1 : P;
        end p2 : P;
    }

    part def B1 {
        part p {
            port p1 : P;
            port p2 : ~P;
        }

        connection a : A {
            end p3 : P ::> p.p1;
            end p4 : P ::> p.p2;
        }
        interface i : I {
            end p3 : P ::> p.p1;
            end p4 : P ::> p.p2;
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
(model
  (namespace
    (package 'ConjugationTest'
      (port_def 'P')
      (part_def 'B'
        (port_usage composite 'p1' : 'ConjugationTest::P'[port_def])
        (port_usage composite 'p2' : 'ConjugationTest::P'[port_def] ~ 'ConjugationTest::P'[port_def]))
      (connection_def 'A'
        (port_usage end 'p1' : 'ConjugationTest::P'[port_def])
        (port_usage end 'p2' : 'ConjugationTest::P'[port_def]))
      (interface_def 'I'
        (port_usage end 'p1' : 'ConjugationTest::P'[port_def])
        (port_usage end 'p2' : 'ConjugationTest::P'[port_def]))
      (part_def 'B1'
        (part_usage composite 'p'
          (port_usage composite 'p1' : 'ConjugationTest::P'[port_def])
          (port_usage composite 'p2' : 'ConjugationTest::P'[port_def] ~ 'ConjugationTest::P'[port_def]))
        (connection_usage composite 'a' : 'ConjugationTest::A'[connection_def]
          (port_usage end 'p3' : 'ConjugationTest::P'[port_def] :> 'ConjugationTest::B1::p::p1'[port_usage])
          (port_usage end 'p4' : 'ConjugationTest::P'[port_def] :> 'ConjugationTest::B1::p::p2'[port_usage]))
        (interface_usage composite 'i' : 'ConjugationTest::I'[interface_def]
          (port_usage end 'p3' : 'ConjugationTest::P'[port_def] :> 'ConjugationTest::B1::p::p1'[port_usage])
          (port_usage end 'p4' : 'ConjugationTest::P'[port_def] :> 'ConjugationTest::B1::p::p2'[port_usage]))))))
~~~
