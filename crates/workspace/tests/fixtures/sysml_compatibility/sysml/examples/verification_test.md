# META
~~~ini
description=SysML Example (Simple Tests): VerificationTest
type=file
~~~
# SOURCE
~~~sysml
package VerificationTest {

	part def V {
		m : ScalarValues::Integer;
	}
	
	part vv : V;
	
	requirement def R {
		doc /* ... */
	}
	
	requirement r : R;

	verification def VerificationCase {		
		subject v : V;	
		objective {
			verify requirement : R;
		}
		
		VerificationCases::PassIf(v.m == 0)
	}
	
	verification def VerificationPlan {
		subject v : V;
		
		objective {
			verify r;
		}
		
		verification verificationCase : VerificationCase;
	}
	
	part verificationContext {
		verification verificationPlan : VerificationPlan {
			subject v = vv;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwRequirement,Ident,Colon,Ident,Semicolon,
KwVerification,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
KwVerify,KwRequirement,Colon,Ident,Semicolon,
CloseCurly,
Ident,ColonColon,Ident,OpenParen,Ident,Dot,Ident,EqEq,DecimalValue,CloseParen,
CloseCurly,
KwVerification,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
KwVerify,Ident,Semicolon,
CloseCurly,
KwVerification,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwVerification,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VerificationTest'
    (part_def 'V'
      (default_ref_usage 'm' : 'ScalarValues::Integer'))
    (part_usage 'vv' : 'V')
    (requirement_def 'R'
      (documentation))
    (requirement_usage 'r' : 'R')
    (verification_case_def 'VerificationCase'
      (sysml_decl 'v' : 'V')
      (objective_member)
      (result_expr_member))
    (verification_case_def 'VerificationPlan'
      (sysml_decl 'v' : 'V')
      (objective_member)
      (sysml_decl 'verificationCase' : 'VerificationCase'))
    (part_usage 'verificationContext'
      (sysml_decl 'verificationPlan' : 'VerificationPlan'
        (sysml_decl 'v' value)))))
~~~
# FORMAT
~~~sysml
package VerificationTest {
    part def V {
        m : ScalarValues::Integer;
    }

    part vv : V;

    requirement def R {
        doc /* ... */
    }

    requirement r : R;

    verification def VerificationCase {
        subject v : V;
        objective {
            verify : R;
        }

        = VerificationCases::PassIf(v.m == 0);
    }

    verification def VerificationPlan {
        subject v : V;

        objective {
            verify r;
        }

        verification verificationCase : VerificationCase;
    }

    part verificationContext {
        verification verificationPlan : VerificationPlan {
            subject v = vv;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Integer'
~~~
# SMG
~~~
(model
  (namespace
    (package 'VerificationTest'
      (part_def 'V'
        (reference_usage reference 'm' : 'ScalarValues::Integer'[unresolved]))
      (part_usage 'vv' : 'VerificationTest::V'[part_def])
      (requirement_def 'R'
        (documentation))
      (requirement_usage 'r' : 'VerificationTest::R'[requirement_def])
      (verification_case_def 'VerificationCase'
        (subject_membership in 'v' : 'VerificationTest::V'[part_def])
        (objective_membership composite
          (verify_requirement_membership : 'VerificationTest::R'[requirement_def]))
        (result_expr_membership))
      (verification_case_def 'VerificationPlan'
        (subject_membership in 'v' : 'VerificationTest::V'[part_def])
        (objective_membership composite
          (verify_requirement_membership 'r'))
        (verification_case_usage composite 'verificationCase' : 'VerificationTest::VerificationCase'[verification_case_def]))
      (part_usage 'verificationContext'
        (verification_case_usage composite 'verificationPlan' : 'VerificationTest::VerificationPlan'[verification_case_def]
          (subject_membership in 'v'
            (feature_value (=))))))))
~~~
