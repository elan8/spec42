# META
~~~ini
description=KerML Mass Roll-up: MassRollup_1
type=file
~~~
# SOURCE
~~~kerml
package MassRollup_1 {
	private import NumericalFunctions::*;

	class MassedThing {
		feature mass : ScalarValues::Real;	
		composite subcomponents: MassedThing[0..*];

		feature totalMass : ScalarValues::Real = 
			mass + sum(subcomponents.totalMass);
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup_1'
    (import_decl private 'NumericalFunctions::*')
    (class_def 'MassedThing'
      (feature_def 'mass' : 'ScalarValues::Real')
      (feature_def composite 'subcomponents' : 'MassedThing' multiplicity)
      (feature_def 'totalMass' : 'ScalarValues::Real' value))))
~~~
# FORMAT
~~~sysml
package MassRollup_1 {
    private import NumericalFunctions::*;

    class MassedThing {
        feature mass : ScalarValues::Real;
        composite subcomponents: MassedThing [0..*];

        feature totalMass : ScalarValues::Real = mass + sum(subcomponents.totalMass);
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# SMG
~~~
(model
  (namespace
    (package 'MassRollup_1'
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (class_def 'MassedThing'
        (feature_def 'mass' : 'ScalarValues::Real'[unresolved])
        (feature_def composite 'subcomponents' : 'MassRollup_1::MassedThing'[class_def]
          (multiplicity_range [0..*]))
        (feature_def 'totalMass' : 'ScalarValues::Real'[unresolved]
          (feature_value (=)))))))
~~~
