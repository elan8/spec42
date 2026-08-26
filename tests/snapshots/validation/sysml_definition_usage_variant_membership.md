# META
~~~ini
description=Definition and Usage variant derivations preserve distinct member and VariantMembership identities
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionVariant
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionVariantMembership
rule_id=sysml-2.0:8.3.6.4:deriveUsageVariant
rule_id=sysml-2.0:8.3.6.4:deriveUsageVariantMembership
libraries=none
~~~
# SOURCE
~~~sysml
package Model {
    part def Base;
    part external : Base;
    variation part def Choice {
        variant part first : Base;
        variant external;
    }
    variation part choice : Base {
        variant part second : Base;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionVariant") (source "Model::Choice") (target "Model::Choice::first") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionVariantMembership") (source "Model::Choice") (target "Model::Choice::first") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageVariant") (source "Model::choice") (target "Model::choice::second") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageVariantMembership") (source "Model::choice") (target "Model::choice::second") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_usage_variant_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1f0786faa77e0f44ac25d18c7af515d4f87dc4af70492e811c80c1654c4955e7") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (subsetting (reference "external")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "external")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first"))) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice"))) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second"))) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (source inherited) (from (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))))
      (supertype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first")))
      (type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice")))
      (type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second")))
      (type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external")))
      (type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (range (start 5 16) (end 5 24)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "external")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (range (start 4 29) (end 4 33)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Choice::first"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (range (start 7 28) (end 7 32)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (range (start 8 30) (end 8 34)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::choice::second"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (range (start 2 20) (end 2 24)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::external"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Base")))))
    )
  )
)
~~~
