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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 2) (end 13 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 10) (end 13 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 14) (end 13 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 2) (end 14 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 10) (end 14 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 14) (end 14 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 14 23) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 26 2) (end 26 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 32 2) (end 32 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 5) (end 39 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 13) (end 39 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:c9177640324998bea3516aa02bd078a8abb67fdfab27000797838daab7c9a5a1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "include")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "uc1")) (expressionOperand (reference "include")) (expressionOperand (reference "use")) (expressionOperand (reference "case")) (expressionOperand (reference "uc2")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UC1")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseSystem")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc2"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "include")) (includeUseCase (reference "u")) (memberAccessOperand (reference "system::uc1")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 0))
      (authored-target "include")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 1))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 2))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 3))
      (authored-target "uc1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 4))
      (authored-target "include")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 5))
      (authored-target "use")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 6))
      (authored-target "case")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 7))
      (authored-target "uc2")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc2")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (kind featureTyping) (ordinal 0))
      (authored-target "UC1")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0))
      (authored-target "UseSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind expressionOperand) (ordinal 0))
      (authored-target "include")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind includeUseCase) (ordinal 0))
      (authored-target "u")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "system::uc1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 7)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/use_case_test.md") (range (start 13 2) (end 13 9)) (probe (position 13 2))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 0) (authored-target "include")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 13 10) (end 13 13)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 1) (authored-target "use")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 13 14) (end 13 18)) (probe (position 13 14))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 2) (authored-target "case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 13 19) (end 13 22)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 3) (authored-target "uc1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 14 2) (end 14 9)) (probe (position 14 2))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 4) (authored-target "include")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 14 10) (end 14 13)) (probe (position 14 10))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 5) (authored-target "use")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 14 14) (end 14 18)) (probe (position 14 14))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 6) (authored-target "case")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 14 19) (end 14 22)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind expressionOperand) (ordinal 7) (authored-target "uc2")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc2")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 6 19) (end 6 25)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 7 15) (end 7 19)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 31 15) (end 31 21)) (probe (position 31 15))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 34 17) (end 34 20)) (probe (position 34 17))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (kind featureTyping) (ordinal 0) (authored-target "UC1")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 29 14) (end 29 23)) (probe (position 29 14))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0) (authored-target "UseSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 39 5) (end 39 12)) (probe (position 39 5))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind expressionOperand) (ordinal 0) (authored-target "include")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 38 13) (end 38 14)) (probe (position 38 13))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind includeUseCase) (ordinal 0) (authored-target "u")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")))))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 39 13) (end 39 23)) (probe (position 39 13))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind memberAccessOperand) (ordinal 0) (authored-target "system::uc1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 22 13) (end 22 17)) (probe (position 22 13))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
    )
  )
)
~~~
