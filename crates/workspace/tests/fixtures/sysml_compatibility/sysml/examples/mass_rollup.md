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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MassRollup"))) (name "MassRollup") (declared-name "MassRollup")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MassRollup::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (name "MassedThing") (declared-name "MassedThing") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup::MassedThing::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (name "compositeThing") (declared-name "compositeThing") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (name "subcomponents") (declared-name "subcomponents") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "mass")) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "memberAccess") (reference "totalMass") (children (expression (kind "featureReference") (reference "subcomponents"))))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup::MassedThing")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (name "filteredMassThing") (declared-name "filteredMassThing") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (name "simpleThing") (declared-name "simpleThing") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "mass")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup::MassedThing"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (role feature-value))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassRollup::compositeThing::totalMass"))) (to (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassRollup::simpleThing::totalMass"))) (to (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "MassRollup::MassedThing::totalMass"))) (to (node (document "d0") (qualified-name "MassRollup::MassedThing::mass"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "MassRollup::filteredMassThing"))) (to (node (document "d0") (qualified-name "MassRollup::compositeThing"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup::compositeThing"))) (to (node (document "d0") (qualified-name "MassRollup::MassedThing"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup::compositeThing::subcomponents"))) (to (node (document "d0") (qualified-name "MassRollup::MassedThing"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup::simpleThing"))) (to (node (document "d0") (qualified-name "MassRollup::MassedThing"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/mass_rollup.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
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
