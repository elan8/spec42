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
    part f : A;

    public part def A {
        part <'1'> b : B;
        protected port c : C;
        const attribute x [0..2];
        derived const ref attribute y :> x;
        ref z : ScalarValues::Integer;
    }

    item def S;

    abstract part def <xx> B {
        public abstract part a : A [1..2];
        public abstract part b subsets a;
        public abstract part c subsets a [0..1];
        port x : ~C {
            port p;
            ref port q;
        }
        package P { }

        succession flow x;

        action a1 {
            accept S via x;
            action aa;
            accept S;
        }
        perform action a2;

        state s1;
        exhibit state s2;
    }

    private port def C {
        private in ref y : A, B {
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
(model
  (namespace
    (package 'PartTest'
      (part_usage 'f' : 'PartTest::A'[part_def])
      (part_def 'A'
        (part_usage composite 'b' : 'PartTest::B'[part_def])
        (port_usage composite 'c' : 'PartTest::C'[port_def])
        (attribute_usage composite 'x'
          (multiplicity_range [0..2]))
        (attribute_usage derived reference 'y' :> 'PartTest::A::x'[attribute_usage])
        (reference_usage reference 'z' : 'ScalarValues::Integer'[unresolved]))
      (item_def 'S')
      (part_def abstract 'B'
        (part_usage abstract composite 'a' : 'PartTest::A'[part_def]
          (multiplicity_range [1..2]))
        (part_usage abstract composite 'b' :> 'PartTest::B::a'[part_usage])
        (part_usage abstract composite 'c' :> 'PartTest::B::a'[part_usage]
          (multiplicity_range [0..1]))
        (port_usage composite 'x' : 'PartTest::C'[port_def] ~ 'PartTest::C'[port_def]
          (port_usage composite 'p')
          (port_usage reference 'q'))
        (package 'P')
        (succession_flow_usage composite 'x')
        (action_usage composite 'a1'
          (accept_action_usage)
          (action_usage composite 'aa')
          (accept_action_usage))
        (perform_action_usage 'a2')
        (state_usage composite 's1')
        (state_usage composite 's2'))
      (port_def 'C'
        (reference_usage in reference 'y' : 'PartTest::A'[part_def] : 'PartTest::B'[part_def]
          (part_usage composite 'B_b' :>> 'PartTest::B::b'[part_usage])
          (part_usage composite 'B_c' :>> 'PartTest::B::c'[part_usage])
          (port_usage composite 'B_x' :>> 'PartTest::B::x'[port_usage]))
        (alias_member 'z1' -> 'PartTest::C::y'[reference_usage])
        (alias_member 'z2' -> 'PartTest::C::y'[reference_usage])
        (port_usage composite 'c1' : 'PartTest::C'[port_def])
        (port_usage reference 'c2' : 'PartTest::C'[port_def]))
      (part_usage 'p1' :> 'PartTest::p2'[part_usage])
      (part_usage 'p2' :> 'PartTest::p3'[part_usage])
      (part_usage 'p3' :> 'PartTest::p1'[part_usage])
      (part_usage 'p4' :> 'p4'[unresolved]))))
~~~
