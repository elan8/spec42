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
  (document "memory://snapshot/enum_status_redefinition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 22) (end 10 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fd7fe7b1e7f45b7fe7d690ba43ef5a1b5ad324737d823591f04faac4852cb030") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementStatusKind"))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UserRequirement"))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ManagedRequirement"))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Need"))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "status"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementStatusKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")))))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0))
      (authored-target "UserRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")))))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0))
      (authored-target "ManagedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")))))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0))
      (authored-target "Need")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")))))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "status")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 5 27) (end 5 48)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementStatusKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")))))
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 8 28) (end 8 43)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0) (authored-target "UserRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")))))
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 7 39) (end 7 57)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0) (authored-target "ManagedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")))))
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 9 23) (end 9 27)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0) (authored-target "Need")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")))))
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 10 22) (end 10 28)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "status")
      (outcome (status unresolved)))
  )
)
~~~
