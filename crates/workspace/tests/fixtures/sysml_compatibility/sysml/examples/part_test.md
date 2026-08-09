# META
~~~ini
description=SysML Example (Simple Tests): PartTest
type=file
~~~
# SOURCE
~~~sysml
package PartTest {
	
	part f: A;

	public part def A {
		part <'1'> b: B;
		protected port c: C;
		constant attribute x[0..2];
		derived constant ref attribute y :> x;
		ref z : ScalarValues::Integer;
	}
	
	item def S;
	
	abstract part def <xx> B {
		public abstract part a: A[1..2];
		public abstract part b subsets a;
		public abstract part c[0..1] subsets a;
		port x: ~C {
		    port p;
		    ref port q;
		}
		package P { }
		
		succession flow x.p to a1.aa.receiver;
		
		action a1 {
			accept S via x;
			action aa accept S;
		}
		perform action a2;
		
		state s1;
		exhibit state s2;
	}
	
	private port def C {
		private in ref y: A, B {
		    part B_b redefines B::b;
		    part B_c redefines B::c;
		    port B_x redefines B::x;
		}
		alias z1 for y;
		alias z2 for y;
		port c1 : C;
		ref port c2 : C;
	}
	
    part p1 :> p2;
    part p2 :> p3; 
    part p3 :> p1;
    
    part p4 :> p4;
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPublic,KwPart,KwDef,Ident,OpenCurly,
KwPart,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Semicolon,
KwProtected,KwPort,Ident,Colon,Ident,Semicolon,
KwConstant,KwAttribute,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwConstant,KwRef,KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwRef,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,Semicolon,
KwAbstract,KwPart,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwPublic,KwAbstract,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPublic,KwAbstract,KwPart,Ident,KwSubsets,Ident,Semicolon,
KwPublic,KwAbstract,KwPart,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwRef,KwPort,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,CloseCurly,
KwSuccession,KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwAccept,Ident,KwVia,Ident,Semicolon,
KwAction,Ident,KwAccept,Ident,Semicolon,
CloseCurly,
KwPerform,KwAction,Ident,Semicolon,
KwState,Ident,Semicolon,
KwExhibit,KwState,Ident,Semicolon,
CloseCurly,
KwPrivate,KwPort,KwDef,Ident,OpenCurly,
KwPrivate,KwIn,KwRef,Ident,Colon,Ident,Comma,Ident,OpenCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwPort,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwRef,KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'PartTest'
    (part_usage 'f' : 'A')
    (part_def public 'A'
      (part_usage 'b' : 'B')
      (port_usage protected 'c' : 'C')
      (attribute_usage const 'x' multiplicity)
      (attribute_usage derived ref const 'y' :> 'x')
      (ref_usage ref 'z' : 'ScalarValues::Integer'))
    (item_def 'S')
    (part_def abstract 'B'
      (part_usage public abstract 'a' : 'A' multiplicity)
      (part_usage public abstract 'b' :> 'a')
      (part_usage public abstract 'c' :> 'a' multiplicity)
      (port_usage 'x' : ~'C'
        (port_usage 'p')
        (port_usage ref 'q'))
      (package_def 'P')
      (succession_flow_usage 'x')
      (action_usage 'a1'
        (accept_node)
        (action_usage 'aa')
        (accept_node))
      (perform_action 'a2')
      (state_usage 's1')
      (exhibit_state 's2'))
    (port_def private 'C'
      (ref_usage private in ref 'y' : 'A', 'B'
        (part_usage 'B_b' :>> 'B::b')
        (part_usage 'B_c' :>> 'B::c')
        (port_usage 'B_x' :>> 'B::x'))
      (alias_member 'z1' for 'y')
      (alias_member 'z2' for 'y')
      (port_usage 'c1' : 'C')
      (port_usage ref 'c2' : 'C'))
    (part_usage 'p1' :> 'p2')
    (part_usage 'p2' :> 'p3')
    (part_usage 'p3' :> 'p1')
    (part_usage 'p4' :> 'p4')))
~~~
# FORMAT
~~~sysml
package PartTest {

    part f: A;

    public part def A {
        part <'1'> b: B;
        protected port c: C;
        constant attribute x[0..2];
        derived constant ref attribute y :> x;
        ref z : ScalarValues::Integer;
    }

    item def S;

    abstract part def <xx> B {
        public abstract part a: A[1..2];
        public abstract part b subsets a;
        public abstract part c[0..1] subsets a;
        port x: ~C {
            port p;
            ref port q;
        }
        package P { }

        succession flow x.p to a1.aa.receiver;

        action a1 {
            accept S via x;
            action aa accept S;
        }
        perform action a2;

        state s1;
        exhibit state s2;
    }

    private port def C {
        private in ref y: A, B {
            part B_b redefines B::b;
            part B_c redefines B::c;
            port B_x redefines B::x;
        }
        alias z1 for y;
        alias z2 for y;
        port c1 : C;
        ref port c2 : C;
    }

    part p1 :> p2;
    part p2 :> p3;
    part p3 :> p1;

    part p4 :> p4;

}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'x'
semantic.ambiguous_member 'x'
semantic.invalid_connection_end_count
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'p4'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'x'
semantic.ambiguous_member 'x'
semantic.invalid_connection_end_count
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'p4'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "PartTest"))) (name "PartTest") (declared-name "PartTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "PartTest::A"))) (name "A") (declared-name "A") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::A::b"))) (name "b") (declared-name "b") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::A")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "PartTest::A::c"))) (name "c") (declared-name "c") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::A")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "PartTest::A::x"))) (name "x") (declared-name "x") (declared (properties (constant true) (ordered false) (unique true)) (multiplicity (lower 0) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::A")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "PartTest::A::z"))) (name "z") (declared-name "z") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "PartTest::A")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "PartTest::B"))) (name "B") (declared-name "B") (declared (properties (abstract true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::B::a"))) (name "a") (declared-name "a") (declared (properties (abstract true) (ordered false)) (multiplicity (lower 1) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "PartTest::B::a1"))) (name "a1") (declared-name "a1") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "PartTest::B::a1::aa"))) (name "aa") (declared-name "aa") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "PartTest::B::a2"))) (name "a2") (declared-name "a2") (effective (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::B::b"))) (name "b") (declared-name "b") (declared (properties (abstract true) (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::B::c"))) (name "c") (declared-name "c") (declared (properties (abstract true) (ordered false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "PartTest::B::s1"))) (name "s1") (declared-name "s1") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
            (element (kind "exhibit state") (id (node (document "d0") (qualified-name "PartTest::B::s2"))) (name "s2") (declared-name "s2") (effective (featuring-type (node (document "d0") (qualified-name "PartTest::B")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "PartTest::B::x"))) (name "x") (declared-name "x") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::B"))))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "PartTest::B::x::p"))) (name "p") (declared-name "p") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::C::~C")))))
              )
            )
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "PartTest::C"))) (name "C") (declared-name "C")
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "PartTest::C::c1"))) (name "c1") (declared-name "c1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "PartTest::C")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "PartTest::C::~C"))) (name "~C") (declared-name "~C") (effective (featuring-type (node (document "d0") (qualified-name "PartTest::C")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "PartTest::S"))) (name "S") (declared-name "S"))
        (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::f"))) (name "f") (declared-name "f") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::p1"))) (name "p1") (declared-name "p1") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::p2"))) (name "p2") (declared-name "p2") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::p3"))) (name "p3") (declared-name "p3") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "PartTest::p4"))) (name "p4") (declared-name "p4") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "PartTest::B"))) (to (node (document "d0") (qualified-name "PartTest::B::a2"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "PartTest::B::a1"))) (to (node (document "d0") (qualified-name "PartTest::B::a1::aa"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "PartTest::C::~C"))) (to (node (document "d0") (qualified-name "PartTest::C"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "PartTest::B::b"))) (to (node (document "d0") (qualified-name "PartTest::B::a"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "PartTest::B::c"))) (to (node (document "d0") (qualified-name "PartTest::B::a"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "PartTest::p1"))) (to (node (document "d0") (qualified-name "PartTest::p2"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "PartTest::p2"))) (to (node (document "d0") (qualified-name "PartTest::p3"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "PartTest::p3"))) (to (node (document "d0") (qualified-name "PartTest::p1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PartTest::A::b"))) (to (node (document "d0") (qualified-name "PartTest::B"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PartTest::A::c"))) (to (node (document "d0") (qualified-name "PartTest::C"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PartTest::B::a"))) (to (node (document "d0") (qualified-name "PartTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PartTest::B::x"))) (to (node (document "d0") (qualified-name "PartTest::C::~C"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PartTest::C::c1"))) (to (node (document "d0") (qualified-name "PartTest::C"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "PartTest::f"))) (to (node (document "d0") (qualified-name "PartTest::A"))))
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
  (document "sysml/examples/part_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 2) (end 8 43))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 9 10) (end 9 31))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 22 2) (end 22 21))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 37 2) (end 37 126))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 42 2) (end 42 20))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 43 2) (end 43 20))
      )
    )
  )
)
~~~
