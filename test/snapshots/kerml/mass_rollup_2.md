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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/mass_rollup_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 2) (end 6 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 2) (end 9 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 9 2) (end 10 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b83d6990feb616a01e6085d8f17bc56ce84bd86e75e3f0081eba8ad6690c9b75") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (path (named (kind package) (name "MassRollup_2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/mass_rollup_2.md") (range (start 12 23) (end 12 34)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::massedThings"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/mass_rollup_2.md") (qualified-name "MassRollup_2::MassedThing")))))
  )
)
~~~
