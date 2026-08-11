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
  (document "41_model_library_example.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 38) (end 20 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 2) (end 21 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 2) (end 22 33))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5bd0a41806ce8d848de10dafd6cdd989e7e9c96d5537bfaf589053d3ce2c7ae8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Model Library Example"))) (kind "package") (name "Model Library Example") (declared-name "Model Library Example"))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Causation"))) (kind "connection def") (name "Causation") (declared-name "Causation") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Occurrences::HappensBefore")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Causation::cause"))) (kind "interface end") (name "cause") (declared-name "cause") (parent (node (document "d0") (qualified-name "Model Library Example::Causation"))) (authored (relationships (typing (reference "Situation")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Causation::effect"))) (kind "interface end") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "Model Library Example::Causation"))) (authored (relationships (typing (reference "Situation")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Cause"))) (kind "occurrence def") (name "Cause") (declared-name "Cause") (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (kind "attribute") (name "probability") (declared-name "probability") (parent (node (document "d0") (qualified-name "Model Library Example::Cause"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Failure"))) (kind "occurrence def") (name "Failure") (declared-name "Failure") (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (kind "attribute") (name "severity") (declared-name "severity") (parent (node (document "d0") (qualified-name "Model Library Example::Failure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Level")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Level"))) (kind "import") (name "Level") (declared-name "Level") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::Level") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (kind "item def") (name "Scenario") (declared-name "Scenario") (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "situations")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "situations")) (redefinition (reference "causes")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "situations")) (redefinition (reference "failures")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Situation"))) (kind "occurrence def") (name "Situation") (declared-name "Situation") (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::causations"))) (kind "connection def") (name "causations") (declared-name "causations") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Causation")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind "occurrence") (name "causes") (declared-name "causes") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cause")) (subsetting (reference "situations")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind "occurrence") (name "failures") (declared-name "failures") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Failure")) (subsetting (reference "situations")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (kind "item def") (name "scenarios") (declared-name "scenarios") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Scenario")))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::situations"))) (kind "occurrence") (name "situations") (declared-name "situations") (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Situation")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Causation"))) (kind specialization) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Causation::cause"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Causation::effect"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (kind featureTyping) (ordinal 0)) (authored-target "Level") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Level"))) (kind membershipImport) (ordinal 0)) (authored-target "RiskMetadata::Level") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (kind redefinition) (ordinal 0)) (authored-target "situations") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "causes") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::causes")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind redefinition) (ordinal 0)) (authored-target "failures") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::failures")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::causations"))) (kind specialization) (ordinal 0)) (authored-target "Causation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Causation")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind featureTyping) (ordinal 0)) (authored-target "Cause") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Cause")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind featureTyping) (ordinal 0)) (authored-target "Failure") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Failure")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (kind specialization) (ordinal 0)) (authored-target "Scenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Scenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::situations"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Situation")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (target (node (document "d0") (qualified-name "Model Library Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (target (node (document "d0") (qualified-name "Model Library Example::Level"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (target (node (document "d0") (qualified-name "Model Library Example::situations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (target (node (document "d0") (qualified-name "Model Library Example::situations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (target (node (document "d0") (qualified-name "Model Library Example::causes"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (target (node (document "d0") (qualified-name "Model Library Example::situations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (target (node (document "d0") (qualified-name "Model Library Example::failures"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Model Library Example::causations"))) (target (node (document "d0") (qualified-name "Model Library Example::Causation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::causations"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (target (node (document "d0") (qualified-name "Model Library Example::Cause"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (target (node (document "d0") (qualified-name "Model Library Example::situations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (target (node (document "d0") (qualified-name "Model Library Example::Failure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (target (node (document "d0") (qualified-name "Model Library Example::situations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (target (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Model Library Example::situations"))) (target (node (document "d0") (qualified-name "Model Library Example::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Model Library Example::situations"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 29 17) (end 29 23)) (probe (position 29 17))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))
        (kind redefinition) (ordinal 0) (authored-target "causes")
        (range (start 29 17) (end 29 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::causes") (range (start 12 21) (end 12 63)))
        )
      )
    )
    (query (range (start 0 0) (end 0 8)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::scenarios"))
        (kind specialization) (ordinal 0) (authored-target "Scenario")
        (range (start 0 0) (end 0 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::Scenario") (range (start 27 1) (end 27 132)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Model Library Example::causations"))
        (kind specialization) (ordinal 0) (authored-target "Causation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::Causation") (range (start 20 1) (end 20 136)))
        )
      )
    )
    (query (range (start 30 17) (end 30 25)) (probe (position 30 17))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))
        (kind redefinition) (ordinal 0) (authored-target "failures")
        (range (start 30 17) (end 30 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::failures") (range (start 18 21) (end 18 67)))
        )
      )
    )
    (query (range (start 0 0) (end 0 9)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::scenarios"))
        (kind specialization) (ordinal 0) (authored-target "Scenario")
        (range (start 0 0) (end 0 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::Scenario") (range (start 27 1) (end 27 132)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Model Library Example::causations"))
        (kind specialization) (ordinal 0) (authored-target "Causation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::Causation") (range (start 20 1) (end 20 136)))
        )
      )
    )
    (query (range (start 12 52) (end 12 62)) (probe (position 12 52))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::causes"))
        (kind subsetting) (ordinal 0) (authored-target "situations")
        (range (start 12 52) (end 12 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::situations") (range (start 6 21) (end 6 57)))
        )
      )
    )
    (query (range (start 18 56) (end 18 66)) (probe (position 18 56))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::failures"))
        (kind subsetting) (ordinal 0) (authored-target "situations")
        (range (start 18 56) (end 18 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::situations") (range (start 6 21) (end 6 57)))
        )
      )
    )
    (query (range (start 28 17) (end 28 27)) (probe (position 28 17))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Scenario::"))
        (kind redefinition) (ordinal 0) (authored-target "situations")
        (range (start 28 17) (end 28 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::situations") (range (start 6 21) (end 6 57)))
        )
      )
    )
    (query (range (start 29 27) (end 29 37)) (probe (position 29 27))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))
        (kind subsetting) (ordinal 0) (authored-target "situations")
        (range (start 29 27) (end 29 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::situations") (range (start 6 21) (end 6 57)))
        )
      )
    )
    (query (range (start 30 29) (end 30 39)) (probe (position 30 29))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))
        (kind subsetting) (ordinal 0) (authored-target "situations")
        (range (start 30 29) (end 30 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Model Library Example::situations") (range (start 6 21) (end 6 57)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 35)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Level"))
        (kind membershipImport) (ordinal 0) (authored-target "RiskMetadata::Level")
        (range (start 2 16) (end 2 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 38) (end 20 64)) (probe (position 20 38))
      (reference
        (source (document "d0") (qualified-name "Model Library Example::Causation"))
        (kind specialization) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 20 38) (end 20 64))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
