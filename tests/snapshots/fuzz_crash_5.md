# META
~~~ini
description=Fuzzer crash #5: malformed SysML input
type=file
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
  (document "memory://snapshot/fuzz_crash_5.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 22 17) (end 22 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b324b2fab99f216f8741049123f7a7aaa67a6c5629bb565fcb57faa8519ee4dc") (contract-version "semantic-metadata-projection-v6"))
  (declarations
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
