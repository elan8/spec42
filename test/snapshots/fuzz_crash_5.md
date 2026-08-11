# META
~~~ini
description=Fuzzer crash #5: malformed SysML input
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
package MassRollup2 {
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

	simpleMass + sum(subcomackage eMassponents.totalMassFpackage 'Metadata Example-1 {
	
	metadata def SatyFeature;
	m@ata def Securi
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_5.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "sysml")
        (range (start 22 17) (end 22 18))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "926d3eaad5c6d2f94e620547b73d3c736e44c1557944c847d5d2307472519d1f") (contract-version "canonical-resolution-v1"))
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
