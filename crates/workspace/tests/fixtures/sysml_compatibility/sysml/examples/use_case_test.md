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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "UseCaseTest"))) (name "UseCaseTest") (declared-name "UseCaseTest")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "UseCaseTest::System"))) (name "System") (declared-name "System") (declared))
        (element (kind "use case def") (id (node (document "d0") (qualified-name "UseCaseTest::UC1"))) (name "UC1") (declared-name "UC1"))
        (element (kind "use case def") (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (name "UseSystem") (declared-name "UseSystem")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "UseCaseTest::UseSystem")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (name "system") (declared-name "system") (effective (featuring-type (node (document "d0") (qualified-name "UseCaseTest::UseSystem")))))
            (element (kind "actor") (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (name "user") (declared-name "user") (effective (featuring-type (node (document "d0") (qualified-name "UseCaseTest::UseSystem")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "UseCaseTest::User"))) (name "User") (declared-name "User") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "UseCaseTest::system"))) (name "system") (declared-name "system") (declared (properties (ordered false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "UseCaseTest::system::u"))) (name "u") (declared-name "u") (effective (featuring-type (node (document "d0") (qualified-name "UseCaseTest::System")))))
          )
        )
        (element (kind "use case") (id (node (document "d0") (qualified-name "UseCaseTest::u"))) (name "u") (declared-name "u"))
        (element (kind "use case") (id (node (document "d0") (qualified-name "UseCaseTest::uc2"))) (name "uc2") (declared-name "uc2"))
        (element (kind "use case") (id (node (document "d0") (qualified-name "UseCaseTest::uc3"))) (name "uc3") (declared-name "uc3")
          (contains
            (element (kind "include use case") (id (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (name "u") (declared-name "u"))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "UseCaseTest::user"))) (name "user") (declared-name "user") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::system"))) (to (node (document "d0") (qualified-name "UseCaseTest::system::u"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (to (node (document "d0") (qualified-name "UseCaseTest::System"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (to (node (document "d0") (qualified-name "UseCaseTest::System"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (to (node (document "d0") (qualified-name "UseCaseTest::User"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::system"))) (to (node (document "d0") (qualified-name "UseCaseTest::System"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::u"))) (to (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "UseCaseTest::user"))) (to (node (document "d0") (qualified-name "UseCaseTest::User"))))
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
  (document "sysml/examples/use_case_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 32 2) (end 32 17))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 34 2) (end 34 23))
      )
      (diagnostic
        (severity warning)
        (code "ambiguous_name_reference")
        (source "semantic")
        (range (start 38 5) (end 38 15))
      )
    )
  )
)
~~~
