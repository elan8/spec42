# META
~~~ini
description=Inherited attribute value reports both redefinition and type mismatch diagnostics
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
        attribute status = "approved";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inherited_attribute_value_type_mismatch.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 10 8) (end 10 38))
      )
      (diagnostic
        (severity error)
        (code "inherited_attribute_value_type_mismatch")
        (source "semantic")
        (range (start 10 8) (end 10 38))
      )
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
        attribute status = "approved";
    }
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "83e9d8c3c0491b9b6cb3d8b4f0bf678db04d6d876f29e37c585c20709a9c6922") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Demo"))) (kind "package") (name "Demo") (declared-name "Demo"))
    (element (id (node (document "d0") (qualified-name "Demo::ManagedRequirement"))) (kind "requirement def") (name "ManagedRequirement") (declared-name "ManagedRequirement") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (kind "attribute") (name "status") (declared-name "status") (parent (node (document "d0") (qualified-name "Demo::ManagedRequirement"))) (authored (relationships (typing (reference "RequirementStatusKind")))))
    (element (id (node (document "d0") (qualified-name "Demo::Need"))) (kind "requirement def") (name "Need") (declared-name "Need") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Owning)) (relationships (specializes (reference "UserRequirement")))))
    (element (id (node (document "d0") (qualified-name "Demo::RequirementStatusKind"))) (kind "enum def") (name "RequirementStatusKind") (declared-name "RequirementStatusKind") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::RequirementStatusKind::approved"))) (kind "enumerated value") (name "approved") (declared-name "approved") (parent (node (document "d0") (qualified-name "Demo::RequirementStatusKind"))))
    (element (id (node (document "d0") (qualified-name "Demo::UserRequirement"))) (kind "requirement def") (name "UserRequirement") (declared-name "UserRequirement") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ManagedRequirement")))))
    (element (id (node (document "d0") (qualified-name "Demo::need"))) (kind "requirement") (name "need") (declared-name "need") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Need")))))
    (element (id (node (document "d0") (qualified-name "Demo::need::status"))) (kind "attribute") (name "status") (declared-name "status") (parent (node (document "d0") (qualified-name "Demo::need"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementStatusKind") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::RequirementStatusKind")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)) (authored-target "UserRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::UserRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)) (authored-target "ManagedRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::ManagedRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)) (authored-target "Need") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Need")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (target (node (document "d0") (qualified-name "Demo::RequirementStatusKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Demo::Need"))) (target (node (document "d0") (qualified-name "Demo::UserRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Demo::UserRequirement"))) (target (node (document "d0") (qualified-name "Demo::ManagedRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::need"))) (target (node (document "d0") (qualified-name "Demo::Need"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Demo::need::status")) (expression (status "ok") (value (string "approved"))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
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
