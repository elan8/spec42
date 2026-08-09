# META
~~~ini
description=SysML Example (Simple Tests): FeaturePathTest
type=file
~~~
# SOURCE
~~~sysml
package Q {
  part def F {
  	part a : A;
  }
  
  part f : F;
  
  part def A {
    part g = f.a;
  }
  
  part def B {
  	part f : F;
  	part a : A;
  }
  
  part def C {
	part b : B {
	  connect f.a to a.g;
	  bind f.a = a.g;
	}
  
	part c subsets b.f {
	  	part aa subsets a;
	}
	
	flow b.f.a to c.aa;
  }
  
  part e1 {
  	attribute x : E;
  	// Ensure that "e1" resolves correctly.
  	bind e1.x = E::e2;
  }
  
  enum def E {
  	enum e1;
  	enum e2;
  }
  
  part g = new A().g.g.g;
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,Dot,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
LineComment,
KwBind,Ident,Dot,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Eq,Ident,Ident,OpenParen,CloseParen,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Q'
    (part_def 'F'
      (part_usage 'a' : 'A'))
    (part_usage 'f' : 'F')
    (part_def 'A'
      (part_usage 'g' value))
    (part_def 'B'
      (part_usage 'f' : 'F')
      (part_usage 'a' : 'A'))
    (part_def 'C'
      (part_usage 'b' : 'B'
        (connection_usage
          (connector_end)
          (connector_end))
        (binding_as_usage
          (connector_end)
          (connector_end)))
      (part_usage 'c' :> 'b.f'
        (part_usage 'aa' :> 'a'))
      (flow_usage 'b'))
    (part_usage 'e1'
      (attribute_usage 'x' : 'E')
      (line_comment)
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (enum_def 'E'
      (enum_value 'e1')
      (enum_value 'e2'))
    (part_usage 'g' value)))
~~~
# FORMAT
~~~sysml
package Q {
    part def F {
        part a : A;
    }

    part f : F;

    part def A {
        part g = f.a;
    }

    part def B {
        part f : F;
        part a : A;
    }

    part def C {
        part b : B {
            connect f.a to a.g;
            bind f.a = a.g;
        }

        part c subsets b.f {
            part aa subsets a;
        }

        flow b.f.a to c.aa;
    }

    part e1 {
        attribute x : E;
        // Ensure that "e1" resolves correctly.
        bind e1.x = E::e2;
    }

    enum def E {
        enum e1;
        enum e2;
    }

    part g = new A().g.g.g;

}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'b'
semantic.ambiguous_member 'b'
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'b'
semantic.ambiguous_member 'b'
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Q"))) (name "Q") (declared-name "Q")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Q::A"))) (name "A") (declared-name "A") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Q::A::g"))) (name "g") (declared-name "g") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "a") (children (expression (kind "featureReference") (reference "f")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::A"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Q::A::g"))) (role feature-value))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Q::B"))) (name "B") (declared-name "B") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Q::B::a"))) (name "a") (declared-name "a") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::B")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Q::B::f"))) (name "f") (declared-name "f") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::B")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Q::C"))) (name "C") (declared-name "C") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Q::C::b"))) (name "b") (declared-name "b") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::C")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Q::C::c"))) (name "c") (declared-name "c") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::C"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Q::C::c::aa"))) (name "aa") (declared-name "aa") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::C")))))
              )
            )
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "Q::E"))) (name "E") (declared-name "E")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Q::E::e1"))) (name "e1") (declared-name "e1") (effective (featuring-type (node (document "d0") (qualified-name "Q::E")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "Q::E::e2"))) (name "e2") (declared-name "e2") (effective (featuring-type (node (document "d0") (qualified-name "Q::E")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Q::F"))) (name "F") (declared-name "F") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Q::F::a"))) (name "a") (declared-name "a") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Q::F")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Q::e1"))) (name "e1") (declared-name "e1") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Q::e1::x"))) (name "x") (declared-name "x") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Q::f"))) (name "f") (declared-name "f") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Q::g"))) (name "g") (declared-name "g") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "g") (children (expression (kind "memberAccess") (reference "g") (children (expression (kind "memberAccess") (reference "g") (children (expression (kind "constructor") (reference "A")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Q::g"))) (role feature-value))))
      )
    )
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "Q::F::a"))) (to (node (document "d0") (qualified-name "Q::A::g"))) (connect (source-expression "f::a") (target-expression "a::g") (container-prefix "Q::C::b")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "Q::e1::x"))) (to (node (document "d0") (qualified-name "Q::E::e2"))) (connect (source-expression "e1::x") (target-expression "E::e2") (container-prefix "Q::e1")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "Q::F::a"))) (to (node (document "d0") (qualified-name "Q::A::g"))) (connect (source-expression "f::a") (target-expression "a::g") (container-prefix "Q::C::b")))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Q::F::a"))) (to (node (document "d0") (qualified-name "Q::C::c::aa"))) (flow (source-expression "b::f::a") (target-expression "c::aa")))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Q::B::a"))) (to (node (document "d0") (qualified-name "Q::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Q::B::f"))) (to (node (document "d0") (qualified-name "Q::F"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Q::C::b"))) (to (node (document "d0") (qualified-name "Q::B"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Q::F::a"))) (to (node (document "d0") (qualified-name "Q::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Q::e1::x"))) (to (node (document "d0") (qualified-name "Q::E"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Q::f"))) (to (node (document "d0") (qualified-name "Q::F"))))
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
  (document "sysml/examples/feature_path_test.md"
    (diagnostics
    )
  )
)
~~~
