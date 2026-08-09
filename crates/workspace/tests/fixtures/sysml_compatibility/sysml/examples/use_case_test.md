# META
~~~ini
description=SysML Example (Simple Tests): UseCaseTest
type=file
~~~
# SOURCE
~~~sysml
package UseCaseTest {

	part def System;	
	part def User;
	
	use case def UseSystem {
		subject system : System;
		actor user : User;
		
		objective  { 
			/* Goal */
		}
		
		include use case uc1 : UC1;	
		include use case uc2 {
			subject = system;
			actor user = UseSystem::user;
		}
	}
	
	use case def UC1;
	
	part user : User;
	
	use case uc2 {
	    subject;
		actor :>> user;
	}
	
	use case u : UseSystem;
	
	part system : System {
		include uc2;
		perform u;
		use case uc1 : UC1;
	}
	
	use case uc3 {
	    include u;
	    include system.uc1;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwUse,KwCase,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
RegularComment,
CloseCurly,
KwInclude,KwUse,KwCase,Ident,Colon,Ident,Semicolon,
KwInclude,KwUse,KwCase,Ident,OpenCurly,
KwSubject,Eq,Ident,Semicolon,
KwActor,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwUse,KwCase,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwUse,KwCase,Ident,OpenCurly,
KwSubject,Semicolon,
KwActor,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwUse,KwCase,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwInclude,Ident,Semicolon,
KwPerform,Ident,Semicolon,
KwUse,KwCase,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwUse,KwCase,Ident,OpenCurly,
KwInclude,Ident,Semicolon,
KwInclude,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'UseCaseTest'
    (part_def 'System')
    (part_def 'User')
    (use_case_def 'UseSystem'
      (sysml_decl 'system' : 'System')
      (sysml_decl 'user' : 'User')
      (objective_member)
      (include_use_case)
      (include_use_case))
    (use_case_def 'UC1')
    (part_usage 'user' : 'User')
    (sysml_decl 'uc2'
      (sysml_decl)
      (sysml_decl :>> 'user'))
    (sysml_decl 'u' : 'UseSystem')
    (part_usage 'system' : 'System'
      (include_use_case)
      (perform_action :>> 'u')
      (sysml_decl 'uc1' : 'UC1'))
    (sysml_decl 'uc3'
      (include_use_case)
      (malformed))))
~~~
# FORMAT
~~~sysml
package UseCaseTest {
    part def System;
    part def User;

    use case def UseSystem {
        subject system : System;
        actor user : User;

        objective {
            /* Goal */
        }

        include use case uc1 : UC1;
        include use case uc2 {
            subject = system;
            actor user = UseSystem::user;
        }
    }

    use case def UC1;

    part user : User;

    use case uc2 {
        subject;
        actor :>> user;
    }

    use case u : UseSystem;

    part system : System {
        include uc2;
        perform :>> u;
        use case uc1 : UC1;
    }

    use case uc3 {
        include u;
        .uc1;
    }
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
~~~
# SMG
~~~
(model
  (namespace
    (package 'UseCaseTest'
      (part_def 'System')
      (part_def 'User')
      (use_case_def 'UseSystem'
        (subject_membership in 'system' : 'UseCaseTest::System'[part_def])
        (actor_membership in 'user' : 'UseCaseTest::User'[part_def])
        (objective_membership composite)
        (include_use_case_usage 'uc1' : 'UseCaseTest::UC1'[use_case_def])
        (include_use_case_usage 'uc2'
          (subject_membership in
            (feature_value (=)))
          (actor_membership in 'user'
            (feature_value (=)))))
      (use_case_def 'UC1')
      (part_usage 'user' : 'UseCaseTest::User'[part_def])
      (use_case_usage 'uc2'
        (subject_membership in)
        (actor_membership in :>> 'UseCaseTest::user'[part_usage]))
      (use_case_usage 'u' : 'UseCaseTest::UseSystem'[use_case_def])
      (part_usage 'system' : 'UseCaseTest::System'[part_def]
        (include_use_case_usage 'uc2')
        (perform_action_usage :>> 'UseCaseTest::u'[use_case_usage])
        (use_case_usage composite 'uc1' : 'UseCaseTest::UC1'[use_case_def]))
      (use_case_usage 'uc3'
        (include_use_case_usage 'u')
        (not_implemented 'malformed')))))
~~~
