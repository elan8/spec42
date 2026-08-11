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
  (document "29_mass_rollup1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
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
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7d381d309eb7892f93d95488a821af1d501689ed3efdd006a17fa9e92cd6b976") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup1"))) (kind "package") (name "MassRollup1") (declared-name "MassRollup1"))
    (element (id (node (document "d0") (qualified-name "MassRollup1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassRollup1"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (kind "part def") (name "MassedThing") (declared-name "MassedThing") (parent (node (document "d0") (qualified-name "MassRollup1"))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (parent (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (kind "part") (name "compositeThing") (declared-name "compositeThing") (parent (node (document "d0") (qualified-name "MassRollup1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind "part") (name "subcomponents") (declared-name "subcomponents") (parent (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (kind "part") (name "simpleThing") (declared-name "simpleThing") (parent (node (document "d0") (qualified-name "MassRollup1"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup1::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (target (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (target (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (target (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (target (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (target (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 26) (end 4 35)) (probe (position 4 26))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::MassedThing::simpleMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 4 26) (end 4 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 25) (end 5 34)) (probe (position 5 25))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 25) (end 5 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 25)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))
        (kind redefinition) (ordinal 0) (authored-target "totalMass")
        (range (start 9 16) (end 9 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass") (range (start 9 2) (end 9 39)))
        )
      )
    )
    (query (range (start 14 16) (end 14 25)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))
        (kind redefinition) (ordinal 0) (authored-target "totalMass")
        (range (start 14 16) (end 14 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass") (range (start 14 2) (end 14 73)))
        )
      )
    )
    (query (range (start 8 20) (end 8 31)) (probe (position 8 20))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::simpleThing"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 8 20) (end 8 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup1::MassedThing") (range (start 3 1) (end 3 100)))
        )
      )
    )
    (query (range (start 12 23) (end 12 34)) (probe (position 12 23))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::compositeThing"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 12 23) (end 12 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup1::MassedThing") (range (start 3 1) (end 3 100)))
        )
      )
    )
    (query (range (start 13 22) (end 13 33)) (probe (position 13 22))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 13 22) (end 13 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup1::MassedThing") (range (start 3 1) (end 3 100)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
