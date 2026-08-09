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
        variant use
        case uc11;
        variant use
        case uc12;
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
(model
  (namespace
    (package 'VariabilityTest'
      (part_def 'P'
        (attribute_usage composite 'a'))
      (part_def 'Q' :> 'VariabilityTest::P'[part_def])
      (attribute_def 'B')
      (part_def variation 'V' :> 'VariabilityTest::P'[part_def]
        (variant_usage
          (part_usage composite 'x' : 'VariabilityTest::Q'[part_def]
            (attribute_usage composite 'b' : 'VariabilityTest::B'[attribute_def] :>> 'VariabilityTest::P::a'[attribute_usage]))))
      (part_usage 'q' : 'VariabilityTest::Q'[part_def])
      (part_usage variation 'v' : 'VariabilityTest::P'[part_def]
        (variant_usage
          (reference_usage reference 'q'
            (attribute_usage composite 'b' : 'VariabilityTest::B'[attribute_def] :>> 'VariabilityTest::P::a'[attribute_usage]))))
      (part_usage 'y' : 'VariabilityTest::P'[part_def]
        (feature_value (=)))
      (action_def variation 'A'
        (variant_usage
          (action_usage composite 'a1'))
        (variant_usage
          (action_usage composite 'a2')))
      (use_case_usage variation 'uc1'
        (variant_usage
          (not_implemented 'malformed'))
        (case_usage composite 'uc11')
        (variant_usage
          (not_implemented 'malformed'))
        (case_usage composite 'uc12'))
      (analysis_case_usage variation 'a1')
      (verification_case_usage variation 'v1')
      (requirement_usage variation 'r'
        (variant_usage
          (requirement_usage composite 'r1'))))))
~~~
