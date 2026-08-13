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
  (document "memory://snapshot/inherited_attribute_value_type_mismatch.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9e7eb73ae52a115ef9f7d78ba27f3da87510fa8f2abbda70e1af4273a42c7d57") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementStatusKind"))))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UserRequirement"))))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::RequirementStatusKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::RequirementStatusKind::approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ManagedRequirement"))))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::need"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Need"))))
    (declaration (id (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::need::status"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementStatusKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::RequirementStatusKind")))))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0))
      (authored-target "UserRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement")))))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0))
      (authored-target "ManagedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement")))))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0))
      (authored-target "Need")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement::status"))) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::RequirementStatusKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need"))) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement"))) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::need"))) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (range (start 5 27) (end 5 48)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementStatusKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::RequirementStatusKind")))))
  )
  (query (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (range (start 8 28) (end 8 43)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0) (authored-target "UserRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement")))))
  )
  (query (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (range (start 7 39) (end 7 57)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0) (authored-target "ManagedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::ManagedRequirement")))))
  )
  (query (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (range (start 9 23) (end 9 27)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0) (authored-target "Need")
      (outcome (status resolved) (target (node (document "memory://snapshot/inherited_attribute_value_type_mismatch.md") (qualified-name "Demo::Need")))))
  )
)
~~~
