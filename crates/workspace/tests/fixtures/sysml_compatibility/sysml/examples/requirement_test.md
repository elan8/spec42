# META
~~~ini
description=SysML Example (Simple Tests): RequirementTest
type=file
~~~
# SOURCE
~~~sysml
package RequirementTest {
	constraint def C;
	constraint c : C;
	private import q::**;
	requirement def R {
		assume constraint c1 : C;
		require c;
		doc /* */
    	requirement;
    	requirement def <'1'> A {
    		doc /* Text */
    		subject s;
    	}
	}
	requirement def R1 {
		require constraint c1 :>> c;
	}
	part p;
	part q {
		requirement r : R;
		satisfy r by p;
		assert satisfy r by q;
	}
	
	requirement r1 : R1;
	not satisfy r1 by p;
	assert not satisfy r1 by q;
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwConstraint,KwDef,Ident,Semicolon,
KwConstraint,Ident,Colon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwRequirement,KwDef,Ident,OpenCurly,
KwAssume,KwConstraint,Ident,Colon,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwDoc,RegularComment,
KwRequirement,Semicolon,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwRequire,KwConstraint,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwRequirement,Ident,Colon,Ident,Semicolon,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
KwAssert,KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,Semicolon,
KwNot,KwSatisfy,Ident,KwBy,Ident,Semicolon,
KwAssert,KwNot,KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RequirementTest'
    (constraint_def 'C')
    (constraint_usage 'c' : 'C')
    (import_decl private 'q::**')
    (requirement_def 'R'
      (sysml_decl 'c1' : 'C')
      (sysml_decl 'c')
      (documentation)
      (malformed)
      (requirement_def 'A'
        (documentation)
        (sysml_decl 's')))
    (requirement_def 'R1'
      (sysml_decl 'c1' :>> 'c'))
    (part_usage 'p')
    (part_usage 'q'
      (requirement_usage 'r' : 'R')
      (sysml_decl 'r')
      (sysml_decl 'r'))
    (requirement_usage 'r1' : 'R1')
    (malformed)
    (sysml_decl 'r1')
    (sysml_decl 'r1')))
~~~
# FORMAT
~~~sysml
package RequirementTest {
    constraint def C;
    constraint c : C;
    private import q::**;
    requirement def R {
        assume constraint c1 : C;
        require constraint c;
        doc /* */
        requirement;
        requirement def <'1'> A {
            doc /* Text */
            subject s;
        }
    }
    requirement def R1 {
        require constraint c1 :>> c;
    }
    part p;
    part q {
        requirement r : R;
        satisfy r by p;
        assert satisfy r by q;
    }

    requirement r1 : R1;
    not
    satisfy r1 by p;
    assert not satisfy r1 by q;
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.unexpected_token
semantic.duplicate_name 'r'
semantic.duplicate_name 'r'
semantic.duplicate_name 'r1'
semantic.duplicate_name 'r1'
semantic.ambiguous_member 'r1'
semantic.ambiguous_member 'r1'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.unexpected_token
semantic.duplicate_name 'r'
semantic.duplicate_name 'r'
semantic.duplicate_name 'r1'
semantic.duplicate_name 'r1'
semantic.ambiguous_member 'r1'
semantic.ambiguous_member 'r1'
~~~
# SMG
~~~
(model
  (namespace
    (package 'RequirementTest'
      (constraint_def 'C')
      (constraint_usage 'c' : 'RequirementTest::C'[constraint_def])
      (membership_import private recursive -> 'RequirementTest::q'[part_usage])
      (requirement_def 'R'
        (assume_constraint_usage composite 'c1' : 'RequirementTest::C'[constraint_def])
        (require_constraint_usage composite 'c')
        (documentation)
        (not_implemented 'malformed')
        (requirement_def 'A'
          (documentation)
          (subject_membership in 's')))
      (requirement_def 'R1'
        (require_constraint_usage composite 'c1' :>> 'RequirementTest::c'[constraint_usage]))
      (part_usage 'p')
      (part_usage 'q'
        (requirement_usage composite 'r' : 'RequirementTest::R'[requirement_def])
        (satisfy_requirement_usage 'r' by 'RequirementTest::p'[part_usage])
        (satisfy_requirement_usage 'r' by 'RequirementTest::q'[part_usage]))
      (requirement_usage 'r1' : 'RequirementTest::R1'[requirement_def])
      (not_implemented 'malformed')
      (satisfy_requirement_usage 'r1' by 'RequirementTest::p'[part_usage])
      (satisfy_requirement_usage not 'r1' by 'RequirementTest::q'[part_usage]))))
~~~
