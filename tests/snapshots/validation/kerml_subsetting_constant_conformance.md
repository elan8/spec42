# META
~~~ini
description=KerML 8.3.3.3.10 validateSubsettingConstantConformance requires a variable subsettingFeature of a constant subsettedFeature to be constant
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.10 validateSubsettingConstantConformance
type=file
skip_validation=both sides now publish their variability -- (modifiers constant) and (modifiers var) -- but no semantic rule pairs a subsetting feature's constancy with the subsetted feature's; the canonical code subsetting_constant_mismatch does not exist yet
~~~
# SOURCE
~~~kerml
package Subsettings {
    classifier Thing;
    class Happening {
        const feature base : Thing;

        // Conforming: the variable subsetting feature is also constant.
        const feature fixed : Thing subsets base;

        // Invalid: a variable feature subsetting a constant feature must be constant.
        var feature loose : Thing subsets base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subsetting_constant_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "subsetting_constant_mismatch")
        (source "semantic")
        (range (start 9 8) (end 9 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subsetting_constant_conformance.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ee2a3b456c9ac51decc3d70ba901e9d8e5b6b018fc4d7eb8d185a120966832b9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers constant)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers constant)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "base")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "base")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind subsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind subsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening")))
      (type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening")))
      (type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))))
      (supertype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening")))
      (type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))))
      (supertype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (range (start 6 44) (end 6 48)) (probe (position 6 44))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::fixed"))) (kind subsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (range (start 9 28) (end 9 33)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (range (start 9 42) (end 9 46)) (probe (position 9 42))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::loose"))) (kind subsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_constant_conformance.md") (qualified-name "Subsettings::Happening::base")))))
    )
  )
)
~~~
