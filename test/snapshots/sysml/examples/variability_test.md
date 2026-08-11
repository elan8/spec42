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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "variability_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 15 2) (end 15 45))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 20 1) (end 20 19))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6443efe3ada14b41055651ca59d4b7d3d790c425c070af47caaf83c8a03ed0ed") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VariabilityTest"))) (kind "package") (name "VariabilityTest") (declared-name "VariabilityTest") (range (start (line 0) (character 0)) (end (line 0) (character 589))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (range (start (line 22) (character 1)) (end (line 22) (character 70))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::B"))) (kind "attribute def") (name "B") (declared-name "B") (range (start (line 6) (character 1)) (end (line 6) (character 17))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::P"))) (kind "part def") (name "P") (declared-name "P") (range (start (line 1) (character 1)) (end (line 1) (character 31))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (kind "attribute") (name "a") (declared-name "a") (range (start (line 2) (character 2)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "VariabilityTest::P"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::Q"))) (kind "part def") (name "Q") (declared-name "Q") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P") (range (start (line 5) (character 15)) (end (line 5) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::V"))) (kind "part def") (name "V") (declared-name "V") (range (start (line 7) (character 1)) (end (line 7) (character 84))) (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P") (range (start (line 7) (character 25)) (end (line 7) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (kind "part") (name "x") (declared-name "x") (range (start (line 8) (character 10)) (end (line 8) (character 52))) (parent (node (document "d0") (qualified-name "VariabilityTest::V"))) (authored (membership (kind Feature)) (relationships (typing (reference "Q") (range (start (line 8) (character 19)) (end (line 8) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind "attribute") (name "b") (declared-name "b") (range (start (line 9) (character 3)) (end (line 9) (character 25))) (parent (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range none)) (typing (reference "B") (range (start (line 9) (character 17)) (end (line 9) (character 18)))) (redefinition (reference "a") (range (start (line 9) (character 23)) (end (line 9) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::a1"))) (kind "kermlDecl") (name "a1") (declared-name "a1") (range (start (line 32) (character 4)) (end (line 32) (character 26))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::q"))) (kind "part") (name "q") (declared-name "q") (range (start (line 13) (character 1)) (end (line 13) (character 12))) (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Q") (range (start (line 13) (character 10)) (end (line 13) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::r"))) (kind "requirement") (name "r") (declared-name "r") (range (start (line 36) (character 4)) (end (line 36) (character 64))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::uc1"))) (kind "kermlDecl") (name "uc1") (declared-name "uc1") (range (start (line 27) (character 1)) (end (line 27) (character 87))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::v"))) (kind "part") (name "v") (declared-name "v") (range (start (line 14) (character 1)) (end (line 14) (character 70))) (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range (start (line 14) (character 20)) (end (line 14) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::v1"))) (kind "kermlDecl") (name "v1") (declared-name "v1") (range (start (line 34) (character 4)) (end (line 34) (character 30))) (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::y"))) (kind "part") (name "y") (declared-name "y") (range (start (line 20) (character 1)) (end (line 20) (character 19))) (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range (start (line 20) (character 10)) (end (line 20) (character 11)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0)) (authored-target "P") (range (start (line 5) (character 15)) (end (line 5) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0)) (authored-target "P") (range (start (line 7) (character 25)) (end (line 7) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Q") (range (start (line 8) (character 19)) (end (line 8) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 1)) (authored-target "B") (range (start (line 9) (character 17)) (end (line 9) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0)) (authored-target "a") (range (start (line 9) (character 23)) (end (line 9) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P::a")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0)) (authored-target "Q") (range (start (line 13) (character 10)) (end (line 13) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range (start (line 14) (character 20)) (end (line 14) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range (start (line 20) (character 10)) (end (line 20) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "VariabilityTest::Q"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "VariabilityTest::V"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (target (node (document "d0") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "d0") (qualified-name "VariabilityTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "d0") (qualified-name "VariabilityTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::q"))) (target (node (document "d0") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::v"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::y"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VariabilityTest::y")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
