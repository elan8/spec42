# META
~~~ini
description=Fuzzer crash #0: malformed SysML input
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
		wtes A;

	simpleMass + sum(subcomackage eMassponents import Numeric.totalMassFpackage 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def Securi
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_0.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "3db1a8e7c5813d2628ba8c96cc38adf8a0cde552f8d3bb7ebf9327869fe9afe9") (contract-version "canonical-resolution-v1"))
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
