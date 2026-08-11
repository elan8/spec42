# META
~~~ini
description=Fuzzer crash #1: malformed SysML input
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
package MassRkllup2 {
	private import NumericalFunctions::*;

	part def MassedThing {
		attribute simpleMass :> ISQ::mass;
		attribute totalMass :> ISQ::mass default sLmpleMass;
	}

	part composicomackagteThing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete :>> totalMass default
			simleMass + sum(subcomponents.totalMass);
	}

	part filter   ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		ates A;

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "sysml")
        (range (start 22 20) (end 22 21))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassRkllup2 {
    private import NumericalFunctions::*;

    part def MassedThing {
        attribute simpleMass :> ISQ::mass;
        attribute totalMass :> ISQ::mass default sLmpleMass;
    }

    part composicomackagteThing : MassedThing {
        p@rt subcomponents: MassedThing[*]ature redefin;
        arValuete :>> totalMass default
        simleMass + sum(subcomponents.totalMass);
    }

    part filter   ssThing :> compositeThing {
        attribute minMass :> ISQ::mass;
        atribute :>> totalMass =
        ates A;

        simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1' {

            metadata def SafetyFeature;
            metadata def Securi

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a49f08491d5450050f0ba6e0ce4290b824df20c2167d1fb0273cff7f9b8dfd07") (contract-version "canonical-resolution-v1"))
  (structure
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
