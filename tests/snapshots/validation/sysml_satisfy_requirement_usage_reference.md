# META
~~~ini
description=SysML 8.3.21.10 validateSatisfyRequirementUsageReference requires the featureTarget of the referencedFeature of a SatisfyRequirementUsage ownedReferenceSubsetting to be a RequirementUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.10 validateSatisfyRequirementUsageReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.21.10:validateSatisfyRequirementUsageReference
blocked_by=lowering-part-definition-members
type=file
~~~
# SOURCE
~~~sysml
package Requirements {
    part def Component;
    part def Library {
        requirement limit;
        part other : Component;
    }
    part def Holder {
        // Conforming: the satisfied feature is a requirement usage.
        satisfy Library::limit;

        // Invalid: the satisfied feature is a part usage, not a requirement usage.
        satisfy Library::other;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "satisfy_invalid_endpoint_kind")
        (source "semantic")
        (range (start 11 8) (end 11 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "satisfy_invalid_endpoint_kind")
        (source "semantic")
        (range (start 11 16) (end 11 30))
        (related-information
          (related
            (uri "memory://snapshot/sysml_satisfy_requirement_usage_reference.md")
            (range (start 4 8) (end 4 31))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d31b68e8c76c8f6e57f30f215929a28e1d66bc57282bf41a3ad43a527a2320f8") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Library::limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Library::other")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "Library::limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0))
      (authored-target "Library::other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")))))
  )
  (relationships
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")))
      (subtype (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit")))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other")))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library")))
      (type (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 8 16) (end 8 30)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "Library::limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 11 16) (end 11 30)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0) (authored-target "Library::other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 4 21) (end 4 30)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")))))
    )
  )
)
~~~
