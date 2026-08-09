# META
~~~ini
description=KerML Mass Roll-up: MassRollup_2
type=file
~~~
# SOURCE
~~~kerml
package MassRollup_2 {
	private import NumericalFunctions::*;
	private import ISQ::*;
	
	class MassedThing {
		feature mass : ScalarValues::Real; 
		feature totalMass : ScalarValues::Real =
			mass + sum(subcomponents.totalMass);
			
		feature subcomponents redefines massedThings;	
	}
	
	feature massedThings: MassedThing[0..*];

}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup_2'
    (import_decl private 'NumericalFunctions::*')
    (import_decl private 'ISQ::*')
    (class_def 'MassedThing'
      (feature_def 'mass' : 'ScalarValues::Real')
      (feature_def 'totalMass' : 'ScalarValues::Real' value)
      (feature_def 'subcomponents' :>> 'massedThings'))
    (feature_def 'massedThings' : 'MassedThing' multiplicity)))
~~~
# FORMAT
~~~sysml
package MassRollup_2 {
    private import NumericalFunctions::*;
    private import ISQ::*;

    class MassedThing {
        feature mass : ScalarValues::Real;
        feature totalMass : ScalarValues::Real = mass + sum(subcomponents.totalMass);

        feature subcomponents redefines massedThings;
    }

    feature massedThings : MassedThing [0..*];
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
    (package 'MassRollup_2'
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (class_def 'MassedThing'
        (feature_def 'mass' : 'ScalarValues::Real'[unresolved])
        (feature_def 'totalMass' : 'ScalarValues::Real'[unresolved]
          (feature_value (=)))
        (feature_def 'subcomponents' :>> 'MassRollup_2::massedThings'[feature_def]))
      (feature_def 'massedThings' : 'MassRollup_2::MassedThing'[class_def]
        (multiplicity_range [0..*])))))
~~~
