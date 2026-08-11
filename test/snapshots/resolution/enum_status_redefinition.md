# META
~~~ini
description=Explicit enum status redefinition suppresses implicit redefinition diagnostics
type=file
~~~
# SOURCE
~~~sysml
package Demo {
    enum def RequirementStatusKind {
        enum approved;
    }
    requirement def ManagedRequirement {
        attribute status : RequirementStatusKind;
    }
    requirement def UserRequirement :> ManagedRequirement;
    requirement def Need :> UserRequirement;
    requirement need : Need {
        attribute :>> status = RequirementStatusKind::approved;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enum_status_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Demo {
    enum def RequirementStatusKind {
        enum approved;
    }
    requirement def ManagedRequirement {
        attribute status : RequirementStatusKind;
    }
    requirement def UserRequirement :> ManagedRequirement;
    requirement def Need :> UserRequirement;
    requirement need : Need {
        attribute :>> status = RequirementStatusKind::approved;
    }
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "91df9a9d4c01e6e6add66dd3f41fc0f5c40c1a6c3170d1aabf789201a71abfdf") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Demo"))) (kind "package") (name "Demo") (declared-name "Demo"))
    (element (id (node (document "d0") (qualified-name "Demo::ManagedRequirement"))) (kind "requirement def") (name "ManagedRequirement") (declared-name "ManagedRequirement") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (kind "attribute") (name "status") (declared-name "status") (parent (node (document "d0") (qualified-name "Demo::ManagedRequirement"))) (authored (relationships (typing (reference "RequirementStatusKind")))))
    (element (id (node (document "d0") (qualified-name "Demo::Need"))) (kind "requirement def") (name "Need") (declared-name "Need") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Owning)) (relationships (specializes (reference "UserRequirement")))))
    (element (id (node (document "d0") (qualified-name "Demo::RequirementStatusKind"))) (kind "enum def") (name "RequirementStatusKind") (declared-name "RequirementStatusKind") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::RequirementStatusKind::approved"))) (kind "enumerated value") (name "approved") (declared-name "approved") (parent (node (document "d0") (qualified-name "Demo::RequirementStatusKind"))))
    (element (id (node (document "d0") (qualified-name "Demo::UserRequirement"))) (kind "requirement def") (name "UserRequirement") (declared-name "UserRequirement") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ManagedRequirement")))))
    (element (id (node (document "d0") (qualified-name "Demo::need"))) (kind "requirement") (name "need") (declared-name "need") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Need")))))
    (element (id (node (document "d0") (qualified-name "Demo::need::status"))) (kind "attribute") (name "status") (declared-name "status") (parent (node (document "d0") (qualified-name "Demo::need"))) (authored (relationships (redefinition (reference "status")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementStatusKind") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::RequirementStatusKind")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)) (authored-target "UserRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::UserRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)) (authored-target "ManagedRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::ManagedRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)) (authored-target "Need") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Need")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::need::status"))) (kind redefinition) (ordinal 0)) (authored-target "status") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::need::status")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (target (node (document "d0") (qualified-name "Demo::RequirementStatusKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Demo::Need"))) (target (node (document "d0") (qualified-name "Demo::UserRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Demo::UserRequirement"))) (target (node (document "d0") (qualified-name "Demo::ManagedRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::need"))) (target (node (document "d0") (qualified-name "Demo::Need"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Demo::need::status"))) (target (node (document "d0") (qualified-name "Demo::need::status"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::need::status"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Demo::need::status")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 10 22) (end 10 28)) (probe (position 10 22))
      (reference
        (source (document "d0") (qualified-name "Demo::need::status"))
        (kind redefinition) (ordinal 0) (authored-target "status")
        (range (start 10 22) (end 10 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::need::status") (range (start 10 8) (end 10 63)))
        )
      )
    )
    (query (range (start 8 28) (end 8 43)) (probe (position 8 28))
      (reference
        (source (document "d0") (qualified-name "Demo::Need"))
        (kind specialization) (ordinal 0) (authored-target "UserRequirement")
        (range (start 8 28) (end 8 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::UserRequirement") (range (start 7 4) (end 7 58)))
        )
      )
    )
    (query (range (start 7 39) (end 7 57)) (probe (position 7 39))
      (reference
        (source (document "d0") (qualified-name "Demo::UserRequirement"))
        (kind specialization) (ordinal 0) (authored-target "ManagedRequirement")
        (range (start 7 39) (end 7 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::ManagedRequirement") (range (start 4 4) (end 4 96)))
        )
      )
    )
  )
)
~~~
