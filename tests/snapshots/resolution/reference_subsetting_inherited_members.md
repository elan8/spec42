# META
~~~ini
description=A reference-subsetting feature inherits nested members from its target
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.3:referenceSubsetting
libraries=none
~~~
# SOURCE
~~~kerml
package P {
    class Container {
        feature target {
            feature nested;
        }
        feature proxy references target;
        feature result subsets proxy.nested;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/reference_subsetting_inherited_members.md"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind subsetting)
    (source "P::Container::result")
    (target "P::Container::target::nested")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/reference_subsetting_inherited_members.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7ed8800addd6ca83309a4754762d56ab074bc3e18656d993d15e0f62a25436fc"))
  (declarations
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "target")))))
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "proxy::nested")))))
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target")))))
    (reference (id (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result"))) (kind subsetting) (ordinal 0))
      (authored-target "proxy::nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy"))) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result"))) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy"))) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result"))) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target"))) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested"))) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy")))
      (featured-by (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container")))
      (supertype (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result")))
      (featured-by (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container")))
      (supertype (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target")))
      (featured-by (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container")))
      (subtype (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested")))
      (featured-by (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target")))
      (subtype (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/reference_subsetting_inherited_members.md") (range (start 5 33) (end 5 39)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::proxy"))) (kind referenceSubsetting) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target")))))
    )
  )
  (query (document "memory://snapshot/reference_subsetting_inherited_members.md") (range (start 6 31) (end 6 43)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::result"))) (kind subsetting) (ordinal 0) (authored-target "proxy::nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/reference_subsetting_inherited_members.md") (qualified-name "P::Container::target::nested")))))
    )
  )
)
~~~
