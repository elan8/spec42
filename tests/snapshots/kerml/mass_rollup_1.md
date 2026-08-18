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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/mass_rollup_1.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 17) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 22) (end 7 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 10) (end 8 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:050cb6d5a2677fd04625c53665d71621fc0745133638e729e678c91c4095fcb8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (path (named (kind package) (name "MassRollup_1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")) (expressionOperand (reference "mass")) (memberAccessOperand (reference "subcomponents::totalMass")) (invocationCallee (reference "sum")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (path (named (kind package) (name "MassRollup_1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind expressionOperand) (ordinal 0))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass")))))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "subcomponents::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents"))) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind memberAccessOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")))
      (subtype (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass")))
      (featured-by (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents")))
      (featured-by (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")))
      (type (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass")))
      (featured-by (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (path (named (kind package) (name "MassRollup_1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 4 17) (end 4 35)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 5 27) (end 5 38)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::subcomponents"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing")))))
    )
  )
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 7 22) (end 7 40)) (probe (position 7 22))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 8 3) (end 8 7)) (probe (position 8 3))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind expressionOperand) (ordinal 0) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::mass")))))
    )
  )
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 8 14) (end 8 37)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind memberAccessOperand) (ordinal 0) (authored-target "subcomponents::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass")))))
    )
  )
  (query (document "memory://snapshot/mass_rollup_1.md") (range (start 8 10) (end 8 13)) (probe (position 8 10))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_1.md") (qualified-name "MassRollup_1::MassedThing::totalMass"))) (kind invocationCallee) (ordinal 0) (authored-target "sum")
      (outcome (status unresolved)))
    )
  )
)
~~~
