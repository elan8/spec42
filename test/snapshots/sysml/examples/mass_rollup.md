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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwAttribute,KwRedefines,Ident,KwDefault,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAbstract,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,DotQuestion,OpenCurly,KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,Ident,CloseAngle,Ident,CloseCurly,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'mass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass'))
    (part_usage 'simpleThing' : 'MassedThing'
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'compositeThing' : 'MassedThing'
      (part_usage 'subcomponents' : 'MassedThing' multiplicity)
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'filteredMassThing' :> 'compositeThing'
      (attribute_usage abstract 'minMass' :> 'ISQ::mass')
      (attribute_usage :>> 'totalMass' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "f5895c3e1a62e3f3dc36ea264be90a9987b6db5e573a1c832acd66a54a2d0dbf") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MassRollup"))) (kind "package") (name "MassRollup") (declared-name "MassRollup") (range (start (line 0) (character 0)) (end (line 0) (character 605))))
    (element (id (node (document "d0") (qualified-name "MassRollup::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (kind "part def") (name "MassedThing") (declared-name "MassedThing") (range (start (line 3) (character 1)) (end (line 3) (character 94))) (parent (node (document "d0") (qualified-name "MassRollup"))))
    (element (id (node (document "d0") (qualified-name "MassRollup::MassedThing::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 4) (character 2)) (end (line 4) (character 30))) (parent (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 4) (character 20)) (end (line 4) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (range (start (line 5) (character 2)) (end (line 5) (character 35))) (parent (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 5) (character 25)) (end (line 5) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (kind "part") (name "compositeThing") (declared-name "compositeThing") (range (start (line 12) (character 1)) (end (line 12) (character 161))) (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing") (range (start (line 12) (character 23)) (end (line 12) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (kind "part") (name "subcomponents") (declared-name "subcomponents") (range (start (line 13) (character 2)) (end (line 13) (character 37))) (parent (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing") (range (start (line 13) (character 22)) (end (line 13) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (range (start (line 15) (character 2)) (end (line 15) (character 79))) (parent (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass") (range (start (line 15) (character 22)) (end (line 15) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (kind "part") (name "filteredMassThing") (declared-name "filteredMassThing") (range (start (line 19) (character 1)) (end (line 19) (character 200))) (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "compositeThing") (range (start (line 19) (character 27)) (end (line 19) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (kind "part") (name "simpleThing") (declared-name "simpleThing") (range (start (line 8) (character 1)) (end (line 8) (character 76))) (parent (node (document "d0") (qualified-name "MassRollup"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassedThing") (range (start (line 8) (character 20)) (end (line 8) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (kind "attribute") (name "totalMass") (declared-name "totalMass") (range (start (line 9) (character 2)) (end (line 9) (character 39))) (parent (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "totalMass") (range (start (line 9) (character 22)) (end (line 9) (character 31)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::MassedThing::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 4) (character 20)) (end (line 4) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 5) (character 25)) (end (line 5) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (range (start (line 12) (character 23)) (end (line 12) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (range (start (line 13) (character 22)) (end (line 13) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (range (start (line 15) (character 22)) (end (line 15) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (range (start (line 19) (character 27)) (end (line 19) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::compositeThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (kind featureTyping) (ordinal 0)) (authored-target "MassedThing") (range (start (line 8) (character 20)) (end (line 8) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
    (reference (id (source (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (kind redefinition) (ordinal 0)) (authored-target "totalMass") (range (start (line 9) (character 22)) (end (line 9) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass")))))
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
