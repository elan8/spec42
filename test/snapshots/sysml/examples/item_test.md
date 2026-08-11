# META
~~~ini
description=SysML Example (Simple Tests): ItemTest
type=file
~~~
# SOURCE
~~~sysml
package ItemTest {
	
	item f: A;

	public item def A {
		item b: B;
		protected ref part c: C;
	}
	
	abstract item def B {
		public abstract part a: A;
	}
	
	private part def C {
		private in ref y: A, B;
	}
	
	port def P {
		in item a1: A;
		out item a2: A;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "item_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 14 2) (end 14 27))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwPublic,KwItem,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwProtected,KwRef,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwItem,KwDef,Ident,OpenCurly,
KwPublic,KwAbstract,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPrivate,KwPart,KwDef,Ident,OpenCurly,
KwPrivate,KwIn,KwRef,Ident,Colon,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ItemTest'
    (item_usage 'f' : 'A')
    (item_def public 'A'
      (item_usage 'b' : 'B')
      (part_usage protected ref 'c' : 'C'))
    (item_def abstract 'B'
      (part_usage public abstract 'a' : 'A'))
    (part_def private 'C'
      (ref_usage private in ref 'y' : 'A', 'B'))
    (port_def 'P'
      (item_usage in 'a1' : 'A')
      (item_usage out 'a2' : 'A'))))
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
package ItemTest {

    item f: A;

    public item def A {
        item b: B;
        protected ref part c: C;
    }

    abstract item def B {
        public abstract part a: A;
    }

    private part def C {
        private in ref y: A, B;
    }

    port def P {
        in item a1: A;
        out item a2: A;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "09e5cf33075e0e311dfda961522cdeab1acab26936fa23dc135fbcf677496910") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ItemTest"))) (kind "package") (name "ItemTest") (declared-name "ItemTest") (range (start (line 0) (character 0)) (end (line 0) (character 265))))
    (element (id (node (document "d0") (qualified-name "ItemTest::A"))) (kind "item def") (name "A") (declared-name "A") (range (start (line 4) (character 1)) (end (line 4) (character 63))) (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::A::c"))) (kind "ref") (name "c") (declared-name "c") (range (start (line 6) (character 2)) (end (line 6) (character 26))) (parent (node (document "d0") (qualified-name "ItemTest::A"))) (authored (membership (kind Feature) (visibility "protected")) (relationships (typing (reference "C") (range (start (line 6) (character 24)) (end (line 6) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ItemTest::B"))) (kind "item def") (name "B") (declared-name "B") (range (start (line 9) (character 1)) (end (line 9) (character 54))) (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::B::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 10) (character 2)) (end (line 10) (character 28))) (parent (node (document "d0") (qualified-name "ItemTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (typing (reference "A") (range (start (line 10) (character 26)) (end (line 10) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ItemTest::C"))) (kind "part def") (name "C") (declared-name "C") (range (start (line 13) (character 1)) (end (line 13) (character 50))) (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P"))) (kind "port def") (name "P") (declared-name "P") (range (start (line 17) (character 1)) (end (line 17) (character 51))) (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P::a1"))) (kind "item") (name "a1") (declared-name "a1") (range (start (line 18) (character 2)) (end (line 18) (character 16))) (parent (node (document "d0") (qualified-name "ItemTest::P"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P::a2"))) (kind "item") (name "a2") (declared-name "a2") (range (start (line 19) (character 2)) (end (line 19) (character 17))) (parent (node (document "d0") (qualified-name "ItemTest::P"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P::~P"))) (kind "conjugated port definition") (name "~P") (declared-name "~P") (range (start (line 17) (character 1)) (end (line 17) (character 51))) (parent (node (document "d0") (qualified-name "ItemTest::P"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::f"))) (kind "item def") (name "f") (declared-name "f") (range (start (line 2) (character 1)) (end (line 2) (character 11))) (parent (node (document "d0") (qualified-name "ItemTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "A") (range (start (line 0) (character 0)) (end (line 0) (character 1)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range (start (line 6) (character 24)) (end (line 6) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 10) (character 26)) (end (line 10) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::f"))) (kind specialization) (ordinal 0)) (authored-target "A") (range (start (line 0) (character 0)) (end (line 0) (character 1))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::A::c"))) (target (node (document "d0") (qualified-name "ItemTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::B::a"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::P::a1"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::P::a2"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ItemTest::f"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::f"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
