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
type=file
~~~
# SOURCE
~~~sysml
package Requirements {
    part def Component;
    requirement def Limit;
    part def Library {
        requirement limit;
        part other : Component;
    }
    part def Holder {
        // Conforming: the satisfied feature is a requirement usage.
        satisfy Library::limit;

        // Conforming: an anonymous requirement usage typed by the definition.
        satisfy requirement : Limit;

        // Invalid: the satisfied feature is a part usage, not a requirement usage.
        satisfy Library::other;

        // Invalid: a requirement definition is not a requirement usage.
        satisfy Limit;
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
        (range (start 15 16) (end 15 30))
        (related-information
          (related
            (uri "memory://snapshot/sysml_satisfy_requirement_usage_reference.md")
            (range (start 5 8) (end 5 31))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "satisfy_invalid_endpoint_kind")
        (source "semantic")
        (range (start 18 16) (end 18 21))
        (related-information
          (related
            (uri "memory://snapshot/sysml_satisfy_requirement_usage_reference.md")
            (range (start 2 4) (end 2 26))
          )
        )
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
        (range (start 15 16) (end 15 30))
        (related-information
          (related
            (uri "memory://snapshot/sysml_satisfy_requirement_usage_reference.md")
            (range (start 5 8) (end 5 31))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "satisfy_invalid_endpoint_kind")
        (source "semantic")
        (range (start 18 16) (end 18 21))
        (related-information
          (related
            (uri "memory://snapshot/sysml_satisfy_requirement_usage_reference.md")
            (range (start 2 4) (end 2 26))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fff3f13e6a16dc6f3835c09c9f2d08d2255106b102fe916fe2be15bbb1306ab7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Library::limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Library::other")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "Library::limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0))
      (authored-target "Library::other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfySource) (ordinal 0))
      (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder"))) (provenance implied))
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
      (type (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3)))))
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
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")))
      (subtype (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1)))) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 12 30) (end 12 35)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 9 16) (end 9 30)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "Library::limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::limit")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 15 16) (end 15 30)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0) (authored-target "Library::other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 18 16) (end 18 21)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfySource) (ordinal 0) (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Limit")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (range (start 5 21) (end 5 30)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Library::other"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_reference.md") (qualified-name "Requirements::Component")))))
    )
  )
)
~~~
