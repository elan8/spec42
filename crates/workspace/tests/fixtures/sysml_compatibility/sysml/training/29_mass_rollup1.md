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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup1'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'simpleMass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass'))
    (part_usage 'simpleThing' : 'MassedThing'
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'compositeThing' : 'MassedThing'
      (part_usage 'subcomponents' : 'MassedThing' multiplicity)
      (attribute_usage :>> 'totalMass' value))))
~~~
# FORMAT
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
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MassRollup1"))) (name "MassRollup1") (declared-name "MassRollup1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MassRollup1::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (name "MassedThing") (declared-name "MassedThing") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (name "simpleMass") (declared-name "simpleMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup1::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup1::MassedThing")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (name "compositeThing") (declared-name "compositeThing") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (name "subcomponents") (declared-name "subcomponents") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup1::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "simpleMass")) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "memberAccess") (reference "totalMass") (children (expression (kind "featureReference") (reference "subcomponents"))))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (role feature-value))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (name "simpleThing") (declared-name "simpleThing") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "simpleMass")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (role feature-value))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (to (node (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (to (node (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (to (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (to (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (to (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::MassedThing"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::MassedThing::simpleMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::MassedThing::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::compositeThing"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::compositeThing::subcomponents"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::compositeThing::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::simpleThing"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup1::simpleThing::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/29_mass_rollup1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
    )
  )
)
~~~
