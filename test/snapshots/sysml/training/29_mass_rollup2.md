# META
~~~ini
description=SysML Training 29 (Expressions): MassRollup2
type=file
~~~
# SOURCE
~~~sysml
package MassRollup2 {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute simpleMass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass default simpleMass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];		
		attribute :>> totalMass default
			simpleMass + sum(subcomponents.totalMass); 
	}
	
	part filteredMassThing :> compositeThing {
		attribute minMass :> ISQ::mass;		
		attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass.?{in p:>ISQ::mass; p >= minMass});
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/29_mass_rollup2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 26) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 25) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 16) (end 10 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 23) (end 15 32))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 16 2) (end 18 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d9915002c26e8fd14d3175fdbfde968ca299f94841b7af4d308edbdd651a0e50") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "totalMass"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "compositeThing"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing"))) (kind subsetting) (ordinal 0))
      (authored-target "compositeThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing"))) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing"))) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 4 26) (end 4 35)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 5 25) (end 5 34)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 8 23) (end 8 34)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing")))))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 10 16) (end 10 25)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "totalMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 9 22) (end 9 33)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::MassedThing")))))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 14 27) (end 14 41)) (probe (position 14 27))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing"))) (kind subsetting) (ordinal 0) (authored-target "compositeThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::compositeThing")))))
  )
  (query (document "memory://snapshot/29_mass_rollup2.md") (range (start 15 23) (end 15 32)) (probe (position 15 23))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup2.md") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
)
~~~
