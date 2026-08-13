# META
~~~ini
description=SysML Training 41 (Language Extension): Model Library Example
type=file
~~~
# SOURCE
~~~sysml
library package 'Model Library Example' {
	private import ScalarValues::Real;
	private import RiskMetadata::Level;
	
	abstract occurrence def Situation;
	
	abstract occurrence situations : Situation[*] nonunique;
	
	abstract occurrence def Cause {
		attribute probability : Real;
	}
	
	abstract occurrence causes : Cause[*] nonunique :> situations;
	
	abstract occurrence def Failure {
		attribute severity : Level;
	}
	
	abstract occurrence failures : Failure[*] nonunique :> situations;
	
	abstract connection def Causation :> Occurrences::HappensBefore {
		end [*] ref cause : Situation;
		end [*] ref effect : Situation;
	}
	
	abstract connection causations : Causation[*] nonunique;
	
	item def Scenario {
		occurrence :>> situations;
		occurrence :>> causes :> situations;
		occurrence :>> failures :> situations;
	}
	
	item scenarios : Scenario[*] nonunique;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/41_model_library_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 21) (end 6 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 10 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 21) (end 12 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 16 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 21) (end 18 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 38) (end 20 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 22) (end 21 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 23) (end 22 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 28 2) (end 28 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 29 2) (end 29 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 30 2) (end 30 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:54921bd4339ab3a2b45c81d208000b87bda288dc0e55c677ab676c2cb17aee14") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "RiskMetadata::Level") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Occurrences::HappensBefore"))))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation::cause"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Situation"))))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation::effect"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Situation"))))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Scenario"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::causations"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Causation"))))
    (declaration (id (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::scenarios"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Scenario"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "RiskMetadata::Level")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation"))) (kind specialization) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation::cause"))) (kind featureTyping) (ordinal 0))
      (authored-target "Situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation::effect"))) (kind featureTyping) (ordinal 0))
      (authored-target "Situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::causations"))) (kind featureTyping) (ordinal 0))
      (authored-target "Causation")
      (outcome (status resolved) (target (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation")))))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::scenarios"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Scenario")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::causations"))) (target (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::causations"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::scenarios"))) (target (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::scenarios"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 2 16) (end 2 35)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "RiskMetadata::Level")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 20 38) (end 20 64)) (probe (position 20 38))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation"))) (kind specialization) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 21 22) (end 21 31)) (probe (position 21 22))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation::cause"))) (kind featureTyping) (ordinal 0) (authored-target "Situation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 22 23) (end 22 32)) (probe (position 22 23))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation::effect"))) (kind featureTyping) (ordinal 0) (authored-target "Situation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 25 34) (end 25 43)) (probe (position 25 34))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::causations"))) (kind featureTyping) (ordinal 0) (authored-target "Causation")
      (outcome (status resolved) (target (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Causation")))))
  )
  (query (document "memory://snapshot/41_model_library_example.md") (range (start 33 18) (end 33 26)) (probe (position 33 18))
    (reference (id (source (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::scenarios"))) (kind featureTyping) (ordinal 0) (authored-target "Scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/41_model_library_example.md") (qualified-name "Model Library Example::Scenario")))))
  )
)
~~~
