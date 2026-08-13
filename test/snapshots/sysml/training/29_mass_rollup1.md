# META
~~~ini
description=SysML Training 29 (Expressions): MassRollup1
type=file
~~~
# SOURCE
~~~sysml
package MassRollup1 {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute simpleMass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass;
	}
	
	part simpleThing : MassedThing {
		attribute :>> totalMass = simpleMass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];		
		attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass); 
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/29_mass_rollup1.md"
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
        (range (start 9 28) (end 9 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2aabd2b4b6764ffef81178db1f001b5b209afb433fc32e220d809d26c21988d8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "totalMass"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "totalMass")) (expressionOperand (reference "simpleMass"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "simpleMass")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 4 26) (end 4 35)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 5 25) (end 5 34)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 12 23) (end 12 34)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 14 16) (end 14 25)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 13 22) (end 13 33)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 8 20) (end 8 31)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing")))))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 9 16) (end 9 25)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_mass_rollup1.md") (qualified-name "MassRollup1::MassedThing::totalMass")))))
  )
  (query (document "memory://snapshot/29_mass_rollup1.md") (range (start 9 28) (end 9 38)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/29_mass_rollup1.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "simpleMass")
      (outcome (status unresolved)))
  )
)
~~~
