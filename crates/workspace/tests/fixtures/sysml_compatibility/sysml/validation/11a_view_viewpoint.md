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
            part frontAxleAssembly : AxleAssembly [1] {
                attribute mass :> ISQ::mass = 150[kg];
                part frontWheel : Wheel [2];
                part frontAxle : Axle [1] {
                    attribute mass;
                    attribute steeringAngle;
                }
            }
            part rearAxleAssembly : AxleAssembly [1] {
                attribute mass :> ISQ::mass = 250[kg];
                part rearWheel : Wheel [2];
                part rearAxle : Axle [1] {
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
            expose SystemModel::vehicle::**;
            render asElementTable {
                view :>> columnView [1] {
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
(model
  (namespace
    (package '11a-View-Viewpoint'
      (package 'SystemModel'
        (namespace_import private -> 'SI'[unresolved])
        (part_def 'Vehicle')
        (part_def 'AxleAssembly')
        (part_def 'Axle')
        (part_def 'Wheel')
        (part_usage 'vehicle' : '11a-View-Viewpoint::SystemModel::Vehicle'[part_def]
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
            (feature_value (=)))
          (part_usage composite 'frontAxleAssembly' : '11a-View-Viewpoint::SystemModel::AxleAssembly'[part_def]
            (multiplicity_range [1])
            (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
              (feature_value (=)))
            (part_usage composite 'frontWheel' : '11a-View-Viewpoint::SystemModel::Wheel'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'frontAxle' : '11a-View-Viewpoint::SystemModel::Axle'[part_def]
              (multiplicity_range [1])
              (attribute_usage composite 'mass')
              (attribute_usage composite 'steeringAngle')))
          (part_usage composite 'rearAxleAssembly' : '11a-View-Viewpoint::SystemModel::AxleAssembly'[part_def]
            (multiplicity_range [1])
            (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]
              (feature_value (=)))
            (part_usage composite 'rearWheel' : '11a-View-Viewpoint::SystemModel::Wheel'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'rearAxle' : '11a-View-Viewpoint::SystemModel::Axle'[part_def]
              (multiplicity_range [1])
              (attribute_usage composite 'mass')))))
      (package 'ViewModel'
        (namespace_import private -> 'Views'[unresolved])
        (part_usage 'systems engineer')
        (concern_usage 'system breakdown'
          (subject_membership in)
          (stakeholder_membership in :>> '11a-View-Viewpoint::ViewModel::systems engineer'[part_usage]))
        (viewpoint_usage 'system structure perspective'
          (framed_concern_membership 'system breakdown'))
        (view_usage 'system structure generation'
          (satisfy_requirement_usage 'system structure perspective')
          (namespace_expose all recursive -> '11a-View-Viewpoint::SystemModel::vehicle'[part_usage])
          (view_rendering_membership -> 'asElementTable'[unresolved]))))))
~~~
