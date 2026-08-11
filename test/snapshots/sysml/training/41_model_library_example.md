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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5bd0a41806ce8d848de10dafd6cdd989e7e9c96d5537bfaf589053d3ce2c7ae8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Model Library Example"))) (kind "package") (name "Model Library Example") (declared-name "Model Library Example") (range (start (line 0) (character 0)) (end (line 0) (character 867))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Causation"))) (kind "connection def") (name "Causation") (declared-name "Causation") (range (start (line 20) (character 1)) (end (line 20) (character 136))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Occurrences::HappensBefore") (range (start (line 20) (character 38)) (end (line 20) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Causation::cause"))) (kind "interface end") (name "cause") (declared-name "cause") (range (start (line 21) (character 2)) (end (line 21) (character 32))) (parent (node (document "d0") (qualified-name "Model Library Example::Causation"))) (authored (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Causation::effect"))) (kind "interface end") (name "effect") (declared-name "effect") (range (start (line 22) (character 2)) (end (line 22) (character 33))) (parent (node (document "d0") (qualified-name "Model Library Example::Causation"))) (authored (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Cause"))) (kind "occurrence def") (name "Cause") (declared-name "Cause") (range (start (line 8) (character 1)) (end (line 8) (character 67))) (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (kind "attribute") (name "probability") (declared-name "probability") (range (start (line 9) (character 2)) (end (line 9) (character 31))) (parent (node (document "d0") (qualified-name "Model Library Example::Cause"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Failure"))) (kind "occurrence def") (name "Failure") (declared-name "Failure") (range (start (line 14) (character 1)) (end (line 14) (character 67))) (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (kind "attribute") (name "severity") (declared-name "severity") (range (start (line 15) (character 2)) (end (line 15) (character 29))) (parent (node (document "d0") (qualified-name "Model Library Example::Failure"))) (authored (membership (kind Feature)) (relationships (typing (reference "Level") (range none)))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Level"))) (kind "import") (name "Level") (declared-name "Level") (range (start (line 2) (character 1)) (end (line 2) (character 36))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::Level") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (kind "item def") (name "Scenario") (declared-name "Scenario") (range (start (line 27) (character 1)) (end (line 27) (character 132))) (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (kind "occurrence") (name "") (range (start (line 28) (character 13)) (end (line 28) (character 28))) (parent (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "situations") (range (start (line 28) (character 17)) (end (line 28) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind "occurrence") (name "") (range (start (line 29) (character 13)) (end (line 29) (character 38))) (parent (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "situations") (range (start (line 29) (character 27)) (end (line 29) (character 37)))) (redefinition (reference "causes") (range (start (line 29) (character 17)) (end (line 29) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind "occurrence") (name "") (range (start (line 30) (character 13)) (end (line 30) (character 40))) (parent (node (document "d0") (qualified-name "Model Library Example::Scenario"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "situations") (range (start (line 30) (character 29)) (end (line 30) (character 39)))) (redefinition (reference "failures") (range (start (line 30) (character 17)) (end (line 30) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::Situation"))) (kind "occurrence def") (name "Situation") (declared-name "Situation") (range (start (line 4) (character 1)) (end (line 4) (character 35))) (parent (node (document "d0") (qualified-name "Model Library Example"))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::causations"))) (kind "connection def") (name "causations") (declared-name "causations") (range (start (line 25) (character 1)) (end (line 25) (character 57))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Causation") (range (start (line 0) (character 0)) (end (line 0) (character 9)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind "occurrence") (name "causes") (declared-name "causes") (range (start (line 12) (character 21)) (end (line 12) (character 63))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cause") (range none)) (subsetting (reference "situations") (range (start (line 12) (character 52)) (end (line 12) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind "occurrence") (name "failures") (declared-name "failures") (range (start (line 18) (character 21)) (end (line 18) (character 67))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Failure") (range none)) (subsetting (reference "situations") (range (start (line 18) (character 56)) (end (line 18) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (kind "item def") (name "scenarios") (declared-name "scenarios") (range (start (line 33) (character 1)) (end (line 33) (character 40))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Scenario") (range (start (line 0) (character 0)) (end (line 0) (character 8)))))))
    (element (id (node (document "d0") (qualified-name "Model Library Example::situations"))) (kind "occurrence") (name "situations") (declared-name "situations") (range (start (line 6) (character 21)) (end (line 6) (character 57))) (parent (node (document "d0") (qualified-name "Model Library Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Situation") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Causation"))) (kind specialization) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 20) (character 38)) (end (line 20) (character 64))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Causation::cause"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Causation::effect"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Cause::probability"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Failure::severity"))) (kind featureTyping) (ordinal 0)) (authored-target "Level") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Level"))) (kind membershipImport) (ordinal 0)) (authored-target "RiskMetadata::Level") (range (start (line 2) (character 16)) (end (line 2) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::"))) (kind redefinition) (ordinal 0)) (authored-target "situations") (range (start (line 28) (character 17)) (end (line 28) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (range (start (line 29) (character 27)) (end (line 29) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "causes") (range (start (line 29) (character 17)) (end (line 29) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::causes")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (range (start (line 30) (character 29)) (end (line 30) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::Scenario::#occurrence2"))) (kind redefinition) (ordinal 0)) (authored-target "failures") (range (start (line 30) (character 17)) (end (line 30) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::failures")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::causations"))) (kind specialization) (ordinal 0)) (authored-target "Causation") (range (start (line 0) (character 0)) (end (line 0) (character 9))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Causation")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind featureTyping) (ordinal 0)) (authored-target "Cause") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Cause")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::causes"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (range (start (line 12) (character 52)) (end (line 12) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind featureTyping) (ordinal 0)) (authored-target "Failure") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Failure")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::failures"))) (kind subsetting) (ordinal 0)) (authored-target "situations") (range (start (line 18) (character 56)) (end (line 18) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::situations")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::scenarios"))) (kind specialization) (ordinal 0)) (authored-target "Scenario") (range (start (line 0) (character 0)) (end (line 0) (character 8))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Scenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Model Library Example::situations"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Model Library Example::Situation")))))
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
