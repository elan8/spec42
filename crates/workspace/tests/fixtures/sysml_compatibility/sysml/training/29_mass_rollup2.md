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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,KwDefault,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,KwDefault,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,DotQuestion,OpenCurly,KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,Ident,GtEq,Ident,CloseCurly,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup2'
    (import_decl private 'NumericalFunctions::*')
    (part_def 'MassedThing'
      (attribute_usage 'simpleMass' :> 'ISQ::mass')
      (attribute_usage 'totalMass' :> 'ISQ::mass' value))
    (part_usage 'compositeThing' : 'MassedThing'
      (part_usage 'subcomponents' : 'MassedThing' multiplicity)
      (attribute_usage :>> 'totalMass' value))
    (part_usage 'filteredMassThing' :> 'compositeThing'
      (attribute_usage 'minMass' :> 'ISQ::mass')
      (attribute_usage :>> 'totalMass' value))))
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
    (element (kind "package") (id (node (document "d0") (qualified-name "MassRollup2"))) (name "MassRollup2") (declared-name "MassRollup2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MassRollup2::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (name "MassedThing") (declared-name "MassedThing") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (name "simpleMass") (declared-name "simpleMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "featureReference") (reference "simpleMass")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup2::MassedThing")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (name "compositeThing") (declared-name "compositeThing") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (name "subcomponents") (declared-name "subcomponents") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup2::MassedThing")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "simpleMass")) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "memberAccess") (reference "totalMass") (children (expression (kind "featureReference") (reference "subcomponents"))))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "MassRollup2::MassedThing")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (name "filteredMassThing") (declared-name "filteredMassThing") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (name "minMass") (declared-name "minMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (to (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (to (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (to (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (to (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::MassedThing"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::MassedThing::simpleMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::MassedThing::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::compositeThing"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::compositeThing::subcomponents"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::compositeThing::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::filteredMassThing"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MassRollup2::filteredMassThing::minMass"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/29_mass_rollup2.md"
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
        (range (start 16 2) (end 16 108))
      )
    )
  )
)
~~~
