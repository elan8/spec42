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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "use_case_test.md"
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
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "136f5674d5168da3277ded2c3ac8fde93ec4892e9e3129e3f0617301d79ddb29") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "UseCaseTest"))) (kind "package") (name "UseCaseTest") (declared-name "UseCaseTest") (range (start (line 0) (character 0)) (end (line 0) (character 557))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::System"))) (kind "part def") (name "System") (declared-name "System") (range (start (line 2) (character 1)) (end (line 2) (character 17))) (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UC1"))) (kind "use case def") (name "UC1") (declared-name "UC1") (range (start (line 20) (character 1)) (end (line 20) (character 18))) (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (kind "use case def") (name "UseSystem") (declared-name "UseSystem") (range (start (line 5) (character 1)) (end (line 5) (character 230))) (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 9) (character 2)) (end (line 9) (character 33))) (parent (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (kind "subject") (name "system") (declared-name "system") (range (start (line 6) (character 2)) (end (line 6) (character 26))) (parent (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (authored (relationships (typing (reference "System") (range none)))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (kind "actor") (name "user") (declared-name "user") (range (start (line 7) (character 2)) (end (line 7) (character 20))) (parent (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (authored (membership (kind Actor)) (relationships (typing (reference "User") (range none)))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::User"))) (kind "part def") (name "User") (declared-name "User") (range (start (line 3) (character 1)) (end (line 3) (character 15))) (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind "part") (name "system") (declared-name "system") (range (start (line 31) (character 1)) (end (line 31) (character 76))) (parent (node (document "d0") (qualified-name "UseCaseTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "System") (range (start (line 31) (character 15)) (end (line 31) (character 21)))) (perform (reference "UseCaseTest::system::u") (range none)))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::system::u"))) (kind "action") (name "u") (declared-name "u") (range (start (line 33) (character 2)) (end (line 33) (character 12))) (parent (node (document "d0") (qualified-name "UseCaseTest::system"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::u"))) (kind "use case") (name "u") (declared-name "u") (range (start (line 29) (character 1)) (end (line 29) (character 24))) (parent (node (document "d0") (qualified-name "UseCaseTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "UseSystem") (range none)))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::uc2"))) (kind "use case") (name "uc2") (declared-name "uc2") (range (start (line 24) (character 1)) (end (line 24) (character 50))) (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::uc3"))) (kind "use case") (name "uc3") (declared-name "uc3") (range (start (line 37) (character 1)) (end (line 37) (character 59))) (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (kind "include use case") (name "u") (declared-name "u") (range (start (line 38) (character 5)) (end (line 38) (character 15))) (parent (node (document "d0") (qualified-name "UseCaseTest::uc3"))) (authored (relationships (typing (reference "u") (range none)))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::user"))) (kind "part") (name "user") (declared-name "user") (range (start (line 22) (character 1)) (end (line 22) (character 18))) (parent (node (document "d0") (qualified-name "UseCaseTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "User") (range (start (line 22) (character 13)) (end (line 22) (character 17)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0)) (authored-target "System") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0)) (authored-target "User") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::User")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0)) (authored-target "System") (range (start (line 31) (character 15)) (end (line 31) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind performSource) (ordinal 0)) (authored-target "UseCaseTest::system::u") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::system::u")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0)) (authored-target "UseSystem") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::UseSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (kind featureTyping) (ordinal 0)) (authored-target "u") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::uc3::u")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0)) (authored-target "User") (range (start (line 22) (character 13)) (end (line 22) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::User")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (target (node (document "d0") (qualified-name "UseCaseTest::System"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (target (node (document "d0") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (target (node (document "d0") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (target (node (document "d0") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (target (node (document "d0") (qualified-name "UseCaseTest::system::u"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCaseTest::u"))) (target (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (target (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "UseCaseTest::user"))) (target (node (document "d0") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
