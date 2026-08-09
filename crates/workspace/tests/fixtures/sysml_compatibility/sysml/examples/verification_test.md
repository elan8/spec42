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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VerificationTest"))) (name "VerificationTest") (declared-name "VerificationTest")
      (contains
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "VerificationTest::R"))) (name "R") (declared-name "R")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VerificationTest::R::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::R")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "VerificationTest::V"))) (name "V") (declared-name "V") (declared))
        (element (kind "verification def") (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (name "VerificationCase") (declared-name "VerificationCase")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))))
              (contains
                (element (kind "verified requirement") (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (name "R") (declared-name "R") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::VerificationCase")))))
              )
            )
            (element (kind "subject") (id (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::VerificationCase")))))
          )
        )
        (element (kind "verification def") (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (name "VerificationPlan") (declared-name "VerificationPlan")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))))
              (contains
                (element (kind "verified requirement") (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (name "r") (declared-name "r") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::VerificationPlan")))))
              )
            )
            (element (kind "subject") (id (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "VerificationTest::VerificationPlan")))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "VerificationTest::r"))) (name "r") (declared-name "r"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VerificationTest::verificationContext"))) (name "verificationContext") (declared-name "verificationContext") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "VerificationTest::vv"))) (name "vv") (declared-name "vv") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::R::_documentation"))) (to (node (document "d0") (qualified-name "VerificationTest::R"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (to (node (document "d0") (qualified-name "VerificationTest::R"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationCase"))) (to (node (document "d0") (qualified-name "VerificationTest::V"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (to (node (document "d0") (qualified-name "VerificationTest::V"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationPlan"))) (to (node (document "d0") (qualified-name "VerificationTest::r"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationCase::objective::R"))) (to (node (document "d0") (qualified-name "VerificationTest::R"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationCase::v"))) (to (node (document "d0") (qualified-name "VerificationTest::V"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::objective::r"))) (to (node (document "d0") (qualified-name "VerificationTest::r"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::VerificationPlan::v"))) (to (node (document "d0") (qualified-name "VerificationTest::V"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::r"))) (to (node (document "d0") (qualified-name "VerificationTest::R"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VerificationTest::vv"))) (to (node (document "d0") (qualified-name "VerificationTest::V"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
