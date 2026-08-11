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
  (document "feature_inheritance.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "79adab37d07eb6b20d980a8c1d68360d81d281b5f183edabf4cc31dc649f0ac3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureInheritance"))) (kind "package") (name "FeatureInheritance") (declared-name "FeatureInheritance"))
    (element (id (node (document "d0") (qualified-name "FeatureInheritance::s"))) (kind "feature decl") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "FeatureInheritance"))))
    (element (id (node (document "d0") (qualified-name "FeatureInheritance::u"))) (kind "feature decl") (name "u") (declared-name "u") (parent (node (document "d0") (qualified-name "FeatureInheritance"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
