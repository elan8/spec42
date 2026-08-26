# META
~~~ini
description=SysML 8.3.21.10 checkSatisfyRequirementUsageBindingConnector requires exactly one canonical binding connector between subjectParameter and the counterpart related element
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageBindingConnector
blocked_by=lowering-gap-binding-connector-satisfy-requirement-endpoints
type=file
~~~
# SOURCE
~~~sysml
package Requirements {
    part def Component;
    part def Library {
        requirement limit;
    }
    part def Holder {
        satisfy Library::limit;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6b91e33df934f8d420053d13166be1bead0e30adc56046f5cbd1e25585377501") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Library::limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library::limit"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "Library::limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library::limit")))))
  )
  (relationships
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library::limit"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library::limit")))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (range (start 6 16) (end 6 30)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (path (named (kind package) (name "Requirements")) (named (kind part-def) (name "Holder")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "Library::limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_usage_binding_connector.md") (qualified-name "Requirements::Library::limit")))))
    )
  )
)
~~~
