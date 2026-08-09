# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Usages' {
	private import SI::*;
	private import 'Requirement Definitions'::*;
	
	requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 2000[kg];
		
		assume constraint {
			doc /* Full tank is full. */
			vehicle.fuelMass == vehicle.fuelFullMass
		}
	}
	
	requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 1500[kg];
		
		assume constraint {
			doc /* Full tank is empty. */
			vehicle.fuelMass == 0[kg]
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,
KwDoc,RegularComment,
Ident,Dot,Ident,EqEq,Ident,Dot,Ident,
CloseCurly,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,OpenCurly,
KwDoc,RegularComment,
Ident,Dot,Ident,EqEq,DecimalValue,OpenSquare,Ident,CloseSquare,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Requirement Usages''
    (import_decl private 'SI::*')
    (import_decl private ''Requirement Definitions'::*')
    (requirement_usage 'fullVehicleMassLimit' : 'VehicleMassLimitationRequirement'
      (sysml_decl 'vehicle' : 'Vehicle')
      (attribute_usage :>> 'massReqd' value)
      (sysml_decl
        (documentation)
        (result_expr_member)))
    (requirement_usage 'emptyVehicleMassLimit' : 'VehicleMassLimitationRequirement'
      (sysml_decl 'vehicle' : 'Vehicle')
      (attribute_usage :>> 'massReqd' value)
      (sysml_decl
        (documentation)
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package 'Requirement Usages' {
    private import SI::*;
    private import 'Requirement Definitions'::*;

    requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        attribute :>> massReqd = 2000[kg];

        assume constraint {
            doc /* Full tank is full. */
            vehicle.fuelMass == vehicle.fuelFullMass
        }
    }

    requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        attribute :>> massReqd = 1500[kg];

        assume constraint {
            doc /* Full tank is empty. */
            vehicle.fuelMass == 0[kg]
        }
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
semantic.unresolved_name 'VehicleMassLimitationRequirement'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'massReqd'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Requirement Usages"))) (name "Requirement Usages") (declared-name "Requirement Usages")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Usages::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Usages::*#import"))) (name "*") (declared-name "*"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (name "emptyVehicleMassLimit") (declared-name "emptyVehicleMassLimit")
          (contains
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0::_documentation"))) (name ""))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (name "fullVehicleMassLimit") (declared-name "fullVehicleMassLimit")
          (contains
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0::_documentation"))) (name ""))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (to (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (to (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))))
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
  (document "sysml/training/32_requirement_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 45))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 4 1) (end 4 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 1) (end 4 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 6 2) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 6 2) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 14 1) (end 14 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 1) (end 14 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 28))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 16 2) (end 16 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 16 2) (end 16 36))
      )
    )
  )
)
~~~
