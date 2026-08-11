# META
~~~ini
description=Fuzzer crash: malformed package with unclosed braces
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
  (document "fuzz_crash_unclosed_package.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5dad8dcaad926520628130dbf5ac38a8ed405998f1fee2e88a890190ec3ad722") (contract-version "canonical-resolution-v1"))
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
