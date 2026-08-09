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
(model
  (namespace
    (package 'FeatureInheritance'
      (feature_def 's'
        (feature_def 't' : 'ISQ::TorqueValue'[unresolved]))
      (feature_def 'u' :> 'FeatureInheritance::s'[feature_def]))))
~~~
