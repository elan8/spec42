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
        (severity error)
        (code "recovered_use_case_body_element")
        (source "parser")
        (range (start 16 3) (end 16 32))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 16 3) (end 16 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 32 2) (end 32 14))
      )
      (diagnostic
        (severity warning)
        (code "perform_target_invalid_kind")
        (source "semantic")
        (range (start 33 10) (end 33 11))
        (related-information
          (related
            (uri "memory://snapshot/use_case_test.md")
            (range (start 29 1) (end 29 24))
          )
        )
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
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation true) (source-digest "blake3:c9177640324998bea3516aa02bd078a8abb67fdfab27000797838daab7c9a5a1") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (comment (text " Goal "))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UC1")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc2"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "System")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "u")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UC1")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UseSystem")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc2"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "include")) (includeUseCase (reference "u")) (memberAccessOperand (reference "system::uc1")))))
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1"))) (kind featureTyping) (ordinal 0))
      (authored-target "UC1")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "u")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")))))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::objective"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc2"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc2"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1"))) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::uc3"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system")) (scopes any))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1")) (scopes any))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::objective")))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::system")))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1")))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc2")))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc2")))
      (supertype (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind use-case-def) (name "UseSystem")) (named (kind use-case) (name "uc2")) (anonymous (kind subject) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user")))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::user")) (scopes any))
      (subtype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::System")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system")))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system::uc1")))
      (featured-by (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::system")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::user")))
      (type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (source direct))
      (supertype (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::User")) (scopes any))
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
  )
  (query (document "memory://snapshot/use_case_test.md") (range (start 13 25) (end 13 28)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UseSystem::uc1"))) (kind featureTyping) (ordinal 0) (authored-target "UC1")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::UC1")))))
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
  (query (document "memory://snapshot/use_case_test.md") (range (start 33 10) (end 33 11)) (probe (position 33 10))
    (reference (id (source (node (document "memory://snapshot/use_case_test.md") (path (named (kind package) (name "UseCaseTest")) (named (kind part) (name "system")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "u")
      (outcome (status resolved) (target (node (document "memory://snapshot/use_case_test.md") (qualified-name "UseCaseTest::u")))))
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
