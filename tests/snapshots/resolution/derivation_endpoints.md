# META
~~~ini
description=Derivation endpoint resolution coverage
type=file
observed_gap=Both derivation endpoint reference-subsetting facts resolve and are published; the snapshot pins endpoint coverage without assuming an additional derived relationship.
~~~
# SOURCE
~~~sysml
package DerivationCoverage {
    requirement def ParentRequirement;
    requirement def ChildRequirement;
    #derivation connection {
        end #original ::> ParentRequirement;
        end #derive ::> ChildRequirement;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/derivation_endpoints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 5) (end 3 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 13) (end 4 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 13) (end 5 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:63504bac244084514ff7b1bd7210642e77e165056951c67f0730d8bff785879c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "derivation")))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "ParentRequirement")))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "ChildRequirement")))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "original")))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "derive")))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "ParentRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement")))))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "ChildRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement")))))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "derivation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "original")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "derive")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)))))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (form prefix-keyword) (definition unresolved))
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0))))) (kind connection) (end (name (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (feature (resolved (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement"))))) (end (name (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (feature (resolved (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 4 26) (end 4 43)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "ParentRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement")))))
    )
  )
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 5 24) (end 5 40)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "ChildRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement")))))
    )
  )
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 3 5) (end 3 15)) (probe (position 3 5))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "derivation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 4 13) (end 4 21)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "original")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 5 13) (end 5 19)) (probe (position 5 13))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection) (ordinal 0)) (anonymous (kind connection) (ordinal 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "derive")
      (outcome (status unresolved)))
    )
  )
)
~~~
