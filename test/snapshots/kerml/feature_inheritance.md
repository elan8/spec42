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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwFeature,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'FeatureInheritance'
    (feature_def 's'
      (feature_def 't' : 'ISQ::TorqueValue'))
    (feature_def 'u' :> 's')))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::TorqueValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::TorqueValue'
~~~
# FORMAT
~~~sysml
package FeatureInheritance {
	feature s {
		feature t : ISQ::TorqueValue;
	}
	
	feature u subsets s;
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "79adab37d07eb6b20d980a8c1d68360d81d281b5f183edabf4cc31dc649f0ac3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureInheritance"))) (kind "package") (name "FeatureInheritance") (declared-name "FeatureInheritance") (range (start (line 0) (character 0)) (end (line 0) (character 102))))
    (element (id (node (document "d0") (qualified-name "FeatureInheritance::s"))) (kind "feature decl") (name "s") (declared-name "s") (range (start (line 1) (character 1)) (end (line 1) (character 47))) (parent (node (document "d0") (qualified-name "FeatureInheritance"))))
    (element (id (node (document "d0") (qualified-name "FeatureInheritance::u"))) (kind "feature decl") (name "u") (declared-name "u") (range (start (line 5) (character 1)) (end (line 5) (character 21))) (parent (node (document "d0") (qualified-name "FeatureInheritance"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
