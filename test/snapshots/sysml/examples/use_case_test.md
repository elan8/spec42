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
  (document "memory://snapshot/use_case_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 7 2) (end 7 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 9 2) (end 11 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 13 10) (end 13 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 13 14) (end 13 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 13 19) (end 13 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 13 23) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 14 2) (end 14 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 14 10) (end 14 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 14 14) (end 14 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 14 19) (end 14 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 14 23) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 27 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 32 2) (end 32 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 34 2) (end 34 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 37 1) (end 40 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c9177640324998bea3516aa02bd078a8abb67fdfab27000797838daab7c9a5a1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System"))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System"))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/use_case_test.md") (range (start 6 19) (end 6 25)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 31 15) (end 31 21)) (probe (position 31 15))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 22 13) (end 22 17)) (probe (position 22 13))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
  )
)
~~~
