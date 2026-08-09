# META
~~~ini
description=SysML Example (Simple Tests): VariabilityTest
type=file
~~~
# SOURCE
~~~sysml
package VariabilityTest {
	part def P {
		attribute a;
	}
	
	part def Q :> P;
	attribute def B;
	variation part def V :> P {
		variant part x : Q {
			attribute b : B :>> a;
		}
	}
	
	part q : Q;
	variation part v : P {
		variant q {
			attribute b : B :>> a;
		}
	}
	
	part y : P = v::q;
	
	variation action def A {
		variant action a1;
		variant action a2;
	}
	
	variation use case uc1 {
    	variant use case uc11;
    	variant use case uc12;
    }

    variation analysis a1;
    
    variation verification v1;
    
    variation requirement r {
    	variant requirement r1;
    }
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwVariation,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwVariation,KwPart,Ident,Colon,Ident,OpenCurly,
KwVariant,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwVariation,KwAction,KwDef,Ident,OpenCurly,
KwVariant,KwAction,Ident,Semicolon,
KwVariant,KwAction,Ident,Semicolon,
CloseCurly,
KwVariation,KwUse,KwCase,Ident,OpenCurly,
KwVariant,KwUse,KwCase,Ident,Semicolon,
KwVariant,KwUse,KwCase,Ident,Semicolon,
CloseCurly,
KwVariation,KwAnalysis,Ident,Semicolon,
KwVariation,KwVerification,Ident,Semicolon,
KwVariation,KwRequirement,Ident,OpenCurly,
KwVariant,KwRequirement,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VariabilityTest'
    (part_def 'P'
      (attribute_usage 'a'))
    (part_def 'Q' :> 'P')
    (attribute_def 'B')
    (part_def variation 'V' :> 'P'
      (variant_usage
        (part_usage 'x' : 'Q'
          (attribute_usage 'b' : 'B' :>> 'a'))))
    (part_usage 'q' : 'Q')
    (part_usage variation 'v' : 'P'
      (variant_usage
        (default_ref_usage 'q'
          (attribute_usage 'b' : 'B' :>> 'a'))))
    (part_usage 'y' : 'P' value)
    (action_def variation 'A'
      (variant_usage
        (action_usage 'a1'))
      (variant_usage
        (action_usage 'a2')))
    (sysml_decl variation 'uc1'
      (variant_usage
        (malformed))
      (sysml_decl 'uc11')
      (variant_usage
        (malformed))
      (sysml_decl 'uc12'))
    (sysml_decl variation 'a1')
    (sysml_decl variation 'v1')
    (requirement_usage variation 'r'
      (variant_usage
        (requirement_usage 'r1')))))
~~~
# FORMAT
~~~sysml
package VariabilityTest {
	part def P {
		attribute a;
	}
	
	part def Q :> P;
	attribute def B;
	variation part def V :> P {
		variant part x : Q {
			attribute b : B :>> a;
		}
	}
	
	part q : Q;
	variation part v : P {
		variant q {
			attribute b : B :>> a;
		}
	}
	
	part y : P = v::q;
	
	variation action def A {
		variant action a1;
		variant action a2;
	}
	
	variation use case uc1 {
    	variant use case uc11;
    	variant use case uc12;
    }

    variation analysis a1;
    
    variation verification v1;
    
    variation requirement r {
    	variant requirement r1;
    }
	
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VariabilityTest"))) (name "VariabilityTest") (declared-name "VariabilityTest")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "VariabilityTest::A"))) (name "A") (declared-name "A"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "VariabilityTest::B"))) (name "B") (declared-name "B") (declared (properties (ordered false) (unique true))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VariabilityTest::P"))) (name "P") (declared-name "P") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (name "a") (declared-name "a") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VariabilityTest::P")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "VariabilityTest::Q"))) (name "Q") (declared-name "Q") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VariabilityTest::V"))) (name "V") (declared-name "V") (declared (properties (variation true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (name "x") (declared-name "x") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VariabilityTest::V"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (name "b") (declared-name "b") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VariabilityTest::Q")))))
              )
            )
          )
        )
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "VariabilityTest::a1"))) (name "a1") (declared-name "a1"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VariabilityTest::q"))) (name "q") (declared-name "q") (declared (properties (ordered false))))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "VariabilityTest::r"))) (name "r") (declared-name "r"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "VariabilityTest::uc1"))) (name "uc1") (declared-name "uc1"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VariabilityTest::v"))) (name "v") (declared-name "v") (declared (properties (variation true) (ordered false))))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "VariabilityTest::v1"))) (name "v1") (declared-name "v1"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VariabilityTest::y"))) (name "y") (declared-name "y") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "v::q")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VariabilityTest::y"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (to (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::Q"))) (to (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::V"))) (to (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (to (node (document "d0") (qualified-name "VariabilityTest::Q"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (to (node (document "d0") (qualified-name "VariabilityTest::B"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::q"))) (to (node (document "d0") (qualified-name "VariabilityTest::Q"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::v"))) (to (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VariabilityTest::y"))) (to (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::B"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::P"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::Q"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::V"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::q"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::r"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::v"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VariabilityTest::y"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/variability_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 15 2) (end 15 45))
      )
    )
  )
)
~~~
