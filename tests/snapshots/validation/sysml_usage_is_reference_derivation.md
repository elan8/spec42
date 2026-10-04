# META
~~~ini
description=SysML 8.3.6.4 deriveUsageIsReference (isReference = not isComposite) makes every directed usage, end usage, usage without a featuring type, and port usage outside a port referential, so validateUsageIsReferential holds by construction
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:deriveUsageIsReference
coverage_role=secondary
libraries=none
type=file
~~~
# SOURCE
~~~sysml
package References {
    part def Base;
    part def Holder {
        in part directed : Base;
        part owned : Base;
        port access;
    }
    part loose : Base;
    connection def Link {
        end part source : Base;
        end part target : Base;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageIsReference") (source "References::Holder::directed") (outcome true))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageIsReference") (source "References::Holder::owned") (outcome false))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageIsReference") (source "References::Holder::access") (outcome true))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageIsReference") (source "References::loose") (outcome true))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageIsReference") (source "References::Link::source") (outcome true)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_is_reference_derivation.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2f891c943e93f6a56e5ee10ee4c2618c26de7e8c093ec4ea1c0e097e97ef39af"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::access"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed"))) (kind part) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::access"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target"))) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::access")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link")))
      (type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link")))
      (type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose")))
      (type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")) (scopes any))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link"))) (kind connection))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (range (start 3 27) (end 3 31)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::directed"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (range (start 4 21) (end 4 25)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Holder::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (range (start 9 26) (end 9 30)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (range (start 10 26) (end 10 30)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (range (start 7 17) (end 7 21)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::loose"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_reference_derivation.md") (qualified-name "References::Base")))))
    )
  )
)
~~~
