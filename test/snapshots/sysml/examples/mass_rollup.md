# META
~~~ini
description=SysML Example (Mass Roll-up): MassRollup
type=file
~~~
# SOURCE
~~~sysml
package MassRollup {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute mass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass;
	}
	
	part simpleThing : MassedThing {
		attribute redefines totalMass = mass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];
		
		attribute redefines totalMass default
			mass + sum(subcomponents.totalMass); 
	}
	
	part filteredMassThing :> compositeThing {
		abstract attribute minMass :> ISQ::mass;
		
		attribute redefines totalMass =
			mass + sum(subcomponents.totalMass.?{in p :> ISQ::mass; p > minMass});
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "mass_rollup.md"
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
        (range (start 4 20) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 25) (end 5 34))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 20 2) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 20 2) (end 20 48))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "934de937e6b5e54badfc8f44e6fbe94995ab63e45c5a6bbc68fb9d2f36e102e7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup"))) (kind "package") (name "MassRollup") (declared-name "MassRollup"))
    (element (id (node (document "d0") (qualified-name "MassRollup::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (kind "part def") (name "MassedThing") (declared-name "MassedThing") (parent (node (document "d0") (qualified-name "MassRollup"))))
    (element (id (node (document "d0") (qualified-name "MassRollup::MassedThing::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (kind "part") (name "compositeThing") (declared-name "compositeThing") (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (kind "part") (name "subcomponents") (declared-name "subcomponents") (parent (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (kind "part") (name "filteredMassThing") (declared-name "filteredMassThing") (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "compositeThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (kind "part") (name "simpleThing") (declared-name "simpleThing") (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::MassedThing::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::compositeThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (target (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (target (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (target (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (target (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (target (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (target (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 20) (end 4 29)) (probe (position 4 20))
      (reference
        (source (document "d0") (qualified-name "MassRollup::MassedThing::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 4 20) (end 4 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 25) (end 5 34)) (probe (position 5 25))
      (reference
        (source (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 25) (end 5 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 22) (end 9 31)) (probe (position 9 22))
      (reference
        (source (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))
        (kind redefinition) (ordinal 0) (authored-target "totalMass")
        (range (start 9 22) (end 9 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup::simpleThing::totalMass") (range (start 9 2) (end 9 39)))
        )
      )
    )
    (query (range (start 15 22) (end 15 31)) (probe (position 15 22))
      (reference
        (source (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))
        (kind redefinition) (ordinal 0) (authored-target "totalMass")
        (range (start 15 22) (end 15 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup::compositeThing::totalMass") (range (start 15 2) (end 15 79)))
        )
      )
    )
    (query (range (start 8 20) (end 8 31)) (probe (position 8 20))
      (reference
        (source (document "d0") (qualified-name "MassRollup::simpleThing"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 8 20) (end 8 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup::MassedThing") (range (start 3 1) (end 3 94)))
        )
      )
    )
    (query (range (start 12 23) (end 12 34)) (probe (position 12 23))
      (reference
        (source (document "d0") (qualified-name "MassRollup::compositeThing"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 12 23) (end 12 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup::MassedThing") (range (start 3 1) (end 3 94)))
        )
      )
    )
    (query (range (start 13 22) (end 13 33)) (probe (position 13 22))
      (reference
        (source (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 13 22) (end 13 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup::MassedThing") (range (start 3 1) (end 3 94)))
        )
      )
    )
    (query (range (start 19 27) (end 19 41)) (probe (position 19 27))
      (reference
        (source (document "d0") (qualified-name "MassRollup::filteredMassThing"))
        (kind subsetting) (ordinal 0) (authored-target "compositeThing")
        (range (start 19 27) (end 19 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup::compositeThing") (range (start 12 1) (end 12 161)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
