# META
~~~ini
description=Fuzzer crash #2: malformed SysML input
type=file
formatter=skip
formatter_skip_reason=source fixture is not UTF-8 and the formatter accepts UTF-8 text only
~~~
# SOURCE
~~~sysml
package MassRollup2 {
	private import NumericalFunctions::*;

	part def MassedThing {
		attribute simpleMass :> ISQ::mass;
		attribute totalMass :> ISQ::mass default sLmpleMass;
	}

	part composicomackagteT€ing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete slizes ClMass default
			simleMass + sum(subcomponents.totalMass);
	}

	part filter  ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
~~~
# FORMAT
~~~sysml
package MassRollup2 {
    private import NumericalFunctions::*;

    part def MassedThing {
        attribute simpleMass :> ISQ::mass;
        attribute totalMass :> ISQ::mass default = sLmpleMass;
    }

    €ing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete slizes ClMass default
			simleMass + sum(subcomponents.totalMass);
	}

    ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
    }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'MassRollup2'
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (part_def 'MassedThing'
        (attribute_usage composite 'simpleMass' :> 'ISQ::mass'[unresolved])
        (attribute_usage composite 'totalMass' :> 'ISQ::mass'[unresolved]
          (feature_value (default =))))
      (not_implemented 'malformed')
      (not_implemented 'malformed'))))
~~~
