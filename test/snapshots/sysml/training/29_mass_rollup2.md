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
  (document "29_mass_rollup2.md"
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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 5 2) (end 5 54))
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
        (range (start 15 23) (end 15 32))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 16 2) (end 16 108))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "4f32c3f12e81c112068df36f8721363d5b49be465fadc2a009408c5381ed3a54") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup2"))) (kind "package") (name "MassRollup2") (declared-name "MassRollup2"))
    (element (id (node (document "d0") (qualified-name "MassRollup2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (kind "part def") (name "MassedThing") (declared-name "MassedThing") (parent (node (document "d0") (qualified-name "MassRollup2"))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (parent (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (kind "part") (name "compositeThing") (declared-name "compositeThing") (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind "part") (name "subcomponents") (declared-name "subcomponents") (parent (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (parent (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (kind "part") (name "filteredMassThing") (declared-name "filteredMassThing") (parent (node (document "d0") (qualified-name "MassRollup2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "compositeThing")))))
    (element (id (node (document "d0") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (kind "attribute") (name "minMass") (declared-name "minMass") (parent (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup2::compositeThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (target (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (target (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (target (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 26) (end 4 35)) (probe (position 4 26))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 4 26) (end 4 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 25) (end 5 34)) (probe (position 5 25))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 25) (end 5 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 25)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))
        (kind redefinition) (ordinal 0) (authored-target "totalMass")
        (range (start 10 16) (end 10 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass") (range (start 10 2) (end 10 79)))
        )
      )
    )
    (query (range (start 15 23) (end 15 32)) (probe (position 15 23))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::filteredMassThing::minMass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 15 23) (end 15 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 23) (end 8 34)) (probe (position 8 23))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::compositeThing"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 8 23) (end 8 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup2::MassedThing") (range (start 3 1) (end 3 119)))
        )
      )
    )
    (query (range (start 9 22) (end 9 33)) (probe (position 9 22))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))
        (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
        (range (start 9 22) (end 9 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup2::MassedThing") (range (start 3 1) (end 3 119)))
        )
      )
    )
    (query (range (start 14 27) (end 14 41)) (probe (position 14 27))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::filteredMassThing"))
        (kind subsetting) (ordinal 0) (authored-target "compositeThing")
        (range (start 14 27) (end 14 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MassRollup2::compositeThing") (range (start 8 1) (end 8 160)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MassRollup2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
