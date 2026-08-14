# META
~~~ini
description=KerML Simple Tests: FeatureInheritance
type=file
~~~
# SOURCE
~~~kerml
package FeatureInheritance {
	feature s {
		feature t : ISQ::TorqueValue;
	}
	
	feature u subsets s;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/feature_inheritance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 14) (end 2 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:df909931b8c58721d9ed14e94f921f5c9963b522afef74efb37af7c23562a2a1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s::t"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::TorqueValue"))))
    (declaration (id (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::u"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "s"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s::t"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::u"))) (kind subsetting) (ordinal 0))
      (authored-target "s")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::u"))) (target (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::u"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/feature_inheritance.md") (range (start 2 14) (end 2 30)) (probe (position 2 14))
    (reference (id (source (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s::t"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_inheritance.md") (range (start 5 19) (end 5 20)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::u"))) (kind subsetting) (ordinal 0) (authored-target "s")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_inheritance.md") (qualified-name "FeatureInheritance::s")))))
  )
)
~~~
