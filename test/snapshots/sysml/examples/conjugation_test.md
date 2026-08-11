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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "conjugation_test.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "37db3f2c358db743c9a5295c9b31a7bac93052fb337f6ccf80b98b26d5ae2ca9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConjugationTest"))) (kind "package") (name "ConjugationTest") (declared-name "ConjugationTest") (range (start (line 0) (character 0)) (end (line 0) (character 434))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::A"))) (kind "connection def") (name "A") (declared-name "A") (range (start (line 8) (character 1)) (end (line 8) (character 59))) (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (kind "interface end") (name "p1") (declared-name "p1") (range (start (line 9) (character 2)) (end (line 9) (character 17))) (parent (node (document "d0") (qualified-name "ConjugationTest::A"))) (authored (relationships (typing (reference "P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (kind "interface end") (name "p2") (declared-name "p2") (range (start (line 10) (character 2)) (end (line 10) (character 18))) (parent (node (document "d0") (qualified-name "ConjugationTest::A"))) (authored (relationships (typing (reference "~P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 3) (character 1)) (end (line 3) (character 45))) (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1"))) (kind "part def") (name "B1") (declared-name "B1") (range (start (line 18) (character 1)) (end (line 18) (character 228))) (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (kind "connection") (name "a") (declared-name "a") (range (start (line 24) (character 2)) (end (line 24) (character 80))) (parent (node (document "d0") (qualified-name "ConjugationTest::B1"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind "interface end") (name "p3") (declared-name "p3") (range (start (line 25) (character 3)) (end (line 25) (character 27))) (parent (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (authored (relationships (typing (reference "P") (range none)) (reference-subsetting (reference "p.p1") (range (start (line 25) (character 22)) (end (line 25) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind "interface end") (name "p4") (declared-name "p4") (range (start (line 26) (character 3)) (end (line 26) (character 28))) (parent (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (authored (relationships (typing (reference "~P") (range none)) (reference-subsetting (reference "p.p2") (range (start (line 26) (character 23)) (end (line 26) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 19) (character 2)) (end (line 19) (character 47))) (parent (node (document "d0") (qualified-name "ConjugationTest::B1"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (kind "port") (name "p1") (declared-name "p1") (range (start (line 20) (character 3)) (end (line 20) (character 14))) (parent (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (kind "port") (name "p2") (declared-name "p2") (range (start (line 21) (character 3)) (end (line 21) (character 15))) (parent (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (authored (membership (kind Feature)) (relationships (typing (reference "~P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (kind "port") (name "p1") (declared-name "p1") (range (start (line 4) (character 2)) (end (line 4) (character 13))) (parent (node (document "d0") (qualified-name "ConjugationTest::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (kind "port") (name "p2") (declared-name "p2") (range (start (line 5) (character 2)) (end (line 5) (character 14))) (parent (node (document "d0") (qualified-name "ConjugationTest::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "~P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::I"))) (kind "interface def") (name "I") (declared-name "I") (range (start (line 13) (character 1)) (end (line 13) (character 48))) (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (kind "interface end") (name "p1") (declared-name "p1") (range (start (line 14) (character 2)) (end (line 14) (character 12))) (parent (node (document "d0") (qualified-name "ConjugationTest::I"))) (authored (relationships (typing (reference "P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (kind "interface end") (name "p2") (declared-name "p2") (range (start (line 15) (character 2)) (end (line 15) (character 13))) (parent (node (document "d0") (qualified-name "ConjugationTest::I"))) (authored (relationships (typing (reference "~P") (range none)))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::P"))) (kind "port def") (name "P") (declared-name "P") (range (start (line 1) (character 1)) (end (line 1) (character 12))) (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (kind "conjugated port definition") (name "~P") (declared-name "~P") (range (start (line 1) (character 1)) (end (line 1) (character 12))) (parent (node (document "d0") (qualified-name "ConjugationTest::P"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "p.p1") (range (start (line 25) (character 22)) (end (line 25) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "p.p2") (range (start (line 26) (character 23)) (end (line 26) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (target (node (document "d0") (qualified-name "ConjugationTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
