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
# FORMAT
~~~sysml
package FeatureInheritance {
    feature s {
        feature t : ISQ::TorqueValue;
    }

    feature u subsets s;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::TorqueValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::TorqueValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FeatureInheritance"))) (name "FeatureInheritance") (declared-name "FeatureInheritance")
      (contains
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureInheritance::s"))) (name "s") (declared-name "s"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureInheritance::u"))) (name "u") (declared-name "u"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
