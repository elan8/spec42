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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:fd7fe7b1e7f45b7fe7d690ba43ef5a1b5ad324737d823591f04faac4852cb030") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementStatusKind")))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UserRequirement")))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ManagedRequirement")))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Need")))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "status")))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "RequirementStatusKind::approved")))))
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
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
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "status")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status")))))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "RequirementStatusKind::approved")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved"))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")))
      (subtype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status")))
      (featured-by (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")))
      (type (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")) (source direct))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")) (scopes any))
      (subtype (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")))
      (subtype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved")))
      (featured-by (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need")))
      (type (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")) (provenance authored))
      (effective-type (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")) (source direct))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")) (scopes any))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")) (scopes any))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need")))
      (effective-type (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")) (source inherited) (from (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status"))))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status")) (scopes any feature))
      (supertype (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
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
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 8 28) (end 8 43)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need"))) (kind specialization) (ordinal 0) (authored-target "UserRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement")))))
    )
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 7 39) (end 7 57)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::UserRequirement"))) (kind specialization) (ordinal 0) (authored-target "ManagedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement")))))
    )
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 9 23) (end 9 27)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::need"))) (kind featureTyping) (ordinal 0) (authored-target "Need")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::Need")))))
    )
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 10 22) (end 10 28)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "status")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::ManagedRequirement::status")))))
    )
  )
  (query (document "memory://snapshot/enum_status_redefinition.md") (range (start 10 31) (end 10 62)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/enum_status_redefinition.md") (path (named (kind package) (name "Demo")) (named (kind requirement) (name "need")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "RequirementStatusKind::approved")
      (outcome (status resolved) (target (node (document "memory://snapshot/enum_status_redefinition.md") (qualified-name "Demo::RequirementStatusKind::approved")))))
    )
  )
)
~~~
