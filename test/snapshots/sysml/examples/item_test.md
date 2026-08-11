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
    (element (kind "package") (id (node (document "d0") (qualified-name "ItemTest"))) (name "ItemTest") (declared-name "ItemTest")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "ItemTest::A"))) (name "A") (declared-name "A")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "ItemTest::A::c"))) (name "c") (declared-name "c") (declared (properties (composite false) (reference true) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "ItemTest::A")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ItemTest::B"))) (name "B") (declared-name "B")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ItemTest::B::a"))) (name "a") (declared-name "a") (declared (properties (abstract true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ItemTest::B")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "ItemTest::C"))) (name "C") (declared-name "C") (declared))
        (element (kind "port def") (id (node (document "d0") (qualified-name "ItemTest::P"))) (name "P") (declared-name "P")
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "ItemTest::P::a1"))) (name "a1") (declared-name "a1") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "ItemTest::P")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "ItemTest::P::a2"))) (name "a2") (declared-name "a2") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "ItemTest::P")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "ItemTest::P::~P"))) (name "~P") (declared-name "~P") (effective (featuring-type (node (document "d0") (qualified-name "ItemTest::P")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ItemTest::f"))) (name "f") (declared-name "f"))
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ItemTest::P::~P"))) (to (node (document "d0") (qualified-name "ItemTest::P"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ItemTest::f"))) (to (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ItemTest::A::c"))) (to (node (document "d0") (qualified-name "ItemTest::C"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ItemTest::B::a"))) (to (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ItemTest::P::a1"))) (to (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ItemTest::P::a2"))) (to (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::A"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::B"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::B::a"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::C"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::P"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::P::a1"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::P::a2"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::P::~P"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ItemTest::f"))) (status missing-prerequisite) (target "Items::Item"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/item_test.md"
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
