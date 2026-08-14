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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ea4aaf9defd5a72a774f78ec051d0f93df00a89e5bd9bad4addb379df381cfe2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)))))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "ParentRequirement"))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "ChildRequirement"))))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))) (kind connectorEnd) (ordinal 0))
      (authored-target "ParentRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement")))))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))) (kind connectorEnd) (ordinal 0))
      (authored-target "ChildRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))) (kind connectorEnd) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 4 26) (end 4 43)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 0)))))) (kind connectorEnd) (ordinal 0) (authored-target "ParentRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ParentRequirement")))))
  )
  (query (document "memory://snapshot/derivation_endpoints.md") (range (start 5 24) (end 5 40)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/derivation_endpoints.md") (path (named (kind package) (name "DerivationCoverage")) (anonymous (kind connection-def) (ordinal 0)) (anonymous (kind connection) (ordinal 1)))))) (kind connectorEnd) (ordinal 0) (authored-target "ChildRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/derivation_endpoints.md") (qualified-name "DerivationCoverage::ChildRequirement")))))
  )
)
~~~
