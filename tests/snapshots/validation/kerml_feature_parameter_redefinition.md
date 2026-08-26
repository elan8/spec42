# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureParameterRedefinition requires matching inherited behavior or step parameters to be redefined
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureParameterRedefinition
blocked_by=lowering-gap-redefinition-parameter-position
type=file
~~~
# SOURCE
~~~kerml
package Redefinition { classifier Parent { feature inherited; } classifier Child :> Parent { feature inherited; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureParameterRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_parameter_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6cb36ecd4bd57a95ee2ec28832959a93c9d56ceb4fc662d24e8a335dd0529f44") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child::inherited"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent::inherited"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child::inherited"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child::inherited"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent::inherited"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent::inherited"))) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child")))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child::inherited")))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child")))
      (supertype (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent::inherited")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent")))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent::inherited")))
      (featured-by (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent")))
      (subtype (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child::inherited")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (range (start 0 84) (end 0 90)) (probe (position 0 84))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_parameter_redefinition.md") (qualified-name "Redefinition::Parent")))))
    )
  )
)
~~~
