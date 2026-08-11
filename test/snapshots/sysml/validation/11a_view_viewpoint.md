# META
~~~ini
description=SysML Validation (11-View and Viewpoint): 11a-View-Viewpoint
type=file
~~~
# SOURCE
~~~sysml
package '11a-View-Viewpoint' {
	
	package SystemModel {
		private import SI::*;
		
		part def Vehicle;
		part def AxleAssembly;
		part def Axle;
		part def Wheel;
		
		part vehicle : Vehicle {
			attribute mass :> ISQ::mass = 2500[SI::kg];
			part frontAxleAssembly : AxleAssembly[1] {
				attribute mass :> ISQ::mass = 150[kg];
				part frontWheel : Wheel[2];
				part frontAxle : Axle[1] {
					attribute mass;
					attribute steeringAngle;
				}
			}
			part rearAxleAssembly : AxleAssembly[1] {
				attribute mass :> ISQ::mass = 250[kg];
				part rearWheel : Wheel[2];
				part rearAxle : Axle[1] {
					attribute mass;
				}
			}
		}
		
	}
	
	package ViewModel {
		private import Views::*;
	
		part 'systems engineer';
		
		concern 'system breakdown' {
			subject;
			stakeholder :>> 'systems engineer';
		}
		
		viewpoint 'system structure perspective' {		
			frame 'system breakdown';
		}
		
		view 'system structure generation' {
			satisfy 'system structure perspective';
			expose SystemModel::vehicle::**[@SysML::PartUsage];
			render asElementTable {
				view :>> columnView[1] {
					render asTextualNotation;
				}
			}
		}
	
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,UnrestrictedName,Semicolon,
KwConcern,UnrestrictedName,OpenCurly,
KwSubject,Semicolon,
KwStakeholder,ColonGtGt,UnrestrictedName,Semicolon,
CloseCurly,
KwViewpoint,UnrestrictedName,OpenCurly,
KwFrame,UnrestrictedName,Semicolon,
CloseCurly,
KwView,UnrestrictedName,OpenCurly,
KwSatisfy,UnrestrictedName,Semicolon,
KwExpose,Ident,ColonColon,Ident,ColonColon,StarStar,OpenSquare,At,Ident,ColonColon,Ident,CloseSquare,Semicolon,
KwRender,Ident,OpenCurly,
KwView,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRender,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''11a-View-Viewpoint''
    (package_def 'SystemModel'
      (import_decl private 'SI::*')
      (part_def 'Vehicle')
      (part_def 'AxleAssembly')
      (part_def 'Axle')
      (part_def 'Wheel')
      (part_usage 'vehicle' : 'Vehicle'
        (attribute_usage 'mass' :> 'ISQ::mass' value)
        (part_usage 'frontAxleAssembly' : 'AxleAssembly' multiplicity
          (attribute_usage 'mass' :> 'ISQ::mass' value)
          (part_usage 'frontWheel' : 'Wheel' multiplicity)
          (part_usage 'frontAxle' : 'Axle' multiplicity
            (attribute_usage 'mass')
            (attribute_usage 'steeringAngle')))
        (part_usage 'rearAxleAssembly' : 'AxleAssembly' multiplicity
          (attribute_usage 'mass' :> 'ISQ::mass' value)
          (part_usage 'rearWheel' : 'Wheel' multiplicity)
          (part_usage 'rearAxle' : 'Axle' multiplicity
            (attribute_usage 'mass')))))
    (package_def 'ViewModel'
      (import_decl private 'Views::*')
      (part_usage ''systems engineer'')
      (sysml_decl ''system breakdown''
        (sysml_decl)
        (sysml_decl :>> ''systems engineer''))
      (sysml_decl ''system structure perspective''
        (sysml_decl ''system breakdown''))
      (sysml_decl ''system structure generation''
        (sysml_decl ''system structure perspective'')
        (expose_member)
        (view_rendering)))))
~~~
# FORMAT
~~~sysml
package '11a-View-Viewpoint' {

    package SystemModel {
        private import SI::*;

        part def Vehicle;
        part def AxleAssembly;
        part def Axle;
        part def Wheel;

        part vehicle : Vehicle {
            attribute mass :> ISQ::mass = 2500[SI::kg];
            part frontAxleAssembly : AxleAssembly[1] {
                attribute mass :> ISQ::mass = 150[kg];
                part frontWheel : Wheel[2];
                part frontAxle : Axle[1] {
                    attribute mass;
                    attribute steeringAngle;
                }
            }
            part rearAxleAssembly : AxleAssembly[1] {
                attribute mass :> ISQ::mass = 250[kg];
                part rearWheel : Wheel[2];
                part rearAxle : Axle[1] {
                    attribute mass;
                }
            }
        }

    }

    package ViewModel {
        private import Views::*;

        part 'systems engineer';

        concern 'system breakdown' {
            subject;
            stakeholder :>> 'systems engineer';
        }

        viewpoint 'system structure perspective' {
            frame 'system breakdown';
        }

        view 'system structure generation' {
            satisfy 'system structure perspective';
            expose SystemModel::vehicle::**[@SysML::PartUsage];
            render asElementTable {
                view :>> columnView[1] {
                    render asTextualNotation;
                }
            }
        }

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
    (element (kind "package") (id (node (document "d0") (qualified-name "11a-View-Viewpoint"))) (name "11a-View-Viewpoint") (declared-name "11a-View-Viewpoint")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel"))) (name "SystemModel") (declared-name "SystemModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::*"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (name "Axle") (declared-name "Axle") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
            (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::steeringAngle"))) (name "steeringAngle") (declared-name "steeringAngle") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (name "frontWheel") (declared-name "frontWheel") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 150))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 2500))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "SI::kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
                (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 250))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (name "rearWheel") (declared-name "rearWheel") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel"))) (name "ViewModel") (declared-name "ViewModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::*"))) (name "*") (declared-name "*"))
            (element (kind "concern") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))) (name "system breakdown") (declared-name "system breakdown")
              (contains
                (element (kind "stakeholder") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown::_stakeholder_systems engineer"))) (name "systems engineer") (declared-name "systems engineer"))
              )
            )
            (element (kind "view") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (name "system structure generation") (declared-name "system structure generation")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::**"))) (name "**") (declared-name "**"))
                (element (kind "view rendering") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable"))) (name "asElementTable") (declared-name "asElementTable")
                  (contains
                    (element (kind "view column") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable::columnView[1]"))) (name "columnView[1]") (declared-name "columnView[1]"))
                  )
                )
              )
            )
            (element (kind "viewpoint") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))) (name "system structure perspective") (declared-name "system structure perspective")
              (contains
                (element (kind "frame") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective::system breakdown"))) (name "system breakdown") (declared-name "system breakdown"))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::systems engineer"))) (name "systems engineer") (declared-name "systems engineer") (declared (properties (ordered false))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (to (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Axle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::AxleAssembly"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::Wheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontAxle::steeringAngle"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::frontWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::frontAxleAssembly::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearAxle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::SystemModel::vehicle::rearAxleAssembly::rearWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown"))) (status missing-prerequisite) (target "Requirements::concernChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system breakdown::_stakeholder_systems engineer"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation"))) (status missing-prerequisite) (target "Views::views"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure generation::asElementTable"))) (status missing-prerequisite) (target "Views::renderings"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::system structure perspective"))) (status missing-prerequisite) (target "Views::viewpoints"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11a-View-Viewpoint::ViewModel::systems engineer"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/11a_view_viewpoint.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 17) (end 3 19))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 11 3) (end 11 46))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 13 4) (end 13 42))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 21 4) (end 21 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 32 17) (end 32 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 49 4) (end 49 65))
      )
    )
  )
)
~~~
