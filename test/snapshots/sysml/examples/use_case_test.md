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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "136f5674d5168da3277ded2c3ac8fde93ec4892e9e3129e3f0617301d79ddb29") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "UseCaseTest"))) (kind "package") (name "UseCaseTest") (declared-name "UseCaseTest"))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::System"))) (kind "part def") (name "System") (declared-name "System") (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UC1"))) (kind "use case def") (name "UC1") (declared-name "UC1") (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (kind "use case def") (name "UseSystem") (declared-name "UseSystem") (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (kind "subject") (name "system") (declared-name "system") (parent (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (authored (relationships (typing (reference "System")))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (kind "actor") (name "user") (declared-name "user") (parent (node (document "d0") (qualified-name "UseCaseTest::UseSystem"))) (authored (membership (kind Actor)) (relationships (typing (reference "User")))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::User"))) (kind "part def") (name "User") (declared-name "User") (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind "part") (name "system") (declared-name "system") (parent (node (document "d0") (qualified-name "UseCaseTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "System")) (perform (reference "UseCaseTest::system::u")))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::system::u"))) (kind "action") (name "u") (declared-name "u") (parent (node (document "d0") (qualified-name "UseCaseTest::system"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::u"))) (kind "use case") (name "u") (declared-name "u") (parent (node (document "d0") (qualified-name "UseCaseTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "UseSystem")))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::uc2"))) (kind "use case") (name "uc2") (declared-name "uc2") (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::uc3"))) (kind "use case") (name "uc3") (declared-name "uc3") (parent (node (document "d0") (qualified-name "UseCaseTest"))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (kind "include use case") (name "u") (declared-name "u") (parent (node (document "d0") (qualified-name "UseCaseTest::uc3"))) (authored (relationships (typing (reference "u")))))
    (element (id (node (document "d0") (qualified-name "UseCaseTest::user"))) (kind "part") (name "user") (declared-name "user") (parent (node (document "d0") (qualified-name "UseCaseTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "User")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0)) (authored-target "System") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0)) (authored-target "User") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::User")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0)) (authored-target "System") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::system"))) (kind performSource) (ordinal 0)) (authored-target "UseCaseTest::system::u") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::system::u")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0)) (authored-target "UseSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::UseSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::uc3::u"))) (kind featureTyping) (ordinal 0)) (authored-target "u") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::uc3::u")))))
    (reference (id (source (node (document "d0") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0)) (authored-target "User") (outcome (status resolved) (target (node (document "d0") (qualified-name "UseCaseTest::User")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 13) (end 22 17)) (probe (position 22 13))
      (reference
        (source (document "d0") (qualified-name "UseCaseTest::user"))
        (kind featureTyping) (ordinal 0) (authored-target "User")
        (range (start 22 13) (end 22 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "UseCaseTest::User") (range (start 3 1) (end 3 15)))
        )
      )
    )
    (query (range (start 31 15) (end 31 21)) (probe (position 31 15))
      (reference
        (source (document "d0") (qualified-name "UseCaseTest::system"))
        (kind featureTyping) (ordinal 0) (authored-target "System")
        (range (start 31 15) (end 31 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "UseCaseTest::System") (range (start 2 1) (end 2 17)))
        )
      )
    )
  )
)
~~~
