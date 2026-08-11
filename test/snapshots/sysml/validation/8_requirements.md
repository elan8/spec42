# META
~~~ini
description=SysML Validation (08-Requirements): 8-Requirements
type=file
~~~
# SOURCE
~~~sysml
package '8-Requirements' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import SI::*;
	public import 'Vehicle Usages'::*;
	public import 'Vehicle Requirements'::*;
	
	package 'Vehicle Definitions' {
		part def Vehicle {
			attribute mass: MassValue;
			attribute fuelLevel: Real;
			attribute fuelTankCapacity: Real;
		}
		
		part def Engine {
			port drivePwrPort: DrivePwrPort;
			perform action 'generate torque': 'Generate Torque';
		}
		
		part def Transmission {
			port clutchPort: ClutchPort;
		}
		
		port def DrivePwrPort;
		port def ClutchPort;
		
		interface def EngineToTransmissionInterface {
			end drivePwrPort: DrivePwrPort;
			end clutchPort: ClutchPort;
		}
	
		action def 'Generate Torque';
	}
	
	package 'Vehicle Usages' {
		public import 'Vehicle Definitions'::*;
		
		action 'provide power' {
			action 'generate torque' { /* ... */ }
			//...
		}
		
		part vehicle1_c1: Vehicle {
			attribute :>> mass = 2000 [kg];
			perform 'provide power';
				
			part engine_v1: Engine {
				port :>> drivePwrPort;
				perform 'provide power'.'generate torque' :>> 'generate torque';
			}
			
			part transmission: Transmission {
				port :>> clutchPort;
			}
			
			interface engineToTransmission: EngineToTransmissionInterface
				connect engine_v1.drivePwrPort to transmission.clutchPort;
		}
		
		part vehicle1_c2: Vehicle {
			attribute :>> mass = 2500 [kg];
		}
	}
	
	package 'Vehicle Requirements' {	
		public import 'Vehicle Definitions'::*;
	
		requirement def <'1'> MassLimitationRequirement {
			/*
			 * The optional requirement ID  of this requirement ('1') is given after the keyword "id" (using name syntax).
			 * Every requirement is parameterized by a "subject". The "subject" of this requirement is implicitly "Anything".
			 */
		
			// The requirement text is given by the documentation in the requirement def body.
			doc /* The actual mass shall be less than or equal to the required mass. */
			
			attribute massActual: MassValue;
			attribute massReqd: MassValue;
			
			require constraint {
				/*
				 * A constraint can be used to formalize a requirement.
				 */
				 massActual <= massReqd 
			 }
		}
		
		requirement def <'2'> ReliabilityRequirement;
		
		requirement <'1.1'> vehicleMass1: MassLimitationRequirement {
			doc /* The vehicle mass shall be less than or equal to 2000 kg when the fuel tank is full. */
			
			subject vehicle : Vehicle {
				/*
				 * The subject of this requirement is redefined to be a "Vehicle".
				 */
			}
			
			attribute :>> massActual: MassValue = vehicle.mass {
				/*
				 * This redefinition binds the vehicle mass to the actual mass.
				 */
			}
			
			attribute :>> massReqd = 2000 [kg] {
				/*
				 * This redefinition sets the required mass to 2000 kg.
				 */
			}
			
			assume constraint fuelConstraint {
				/*
				 * A constraint can also be used to specify an assumption.
				 */
			
				doc /* full fuel tank */
				vehicle.fuelLevel >= vehicle.fuelTankCapacity
			}
		}
			
		requirement <'2.1'> vehicleMass2: MassLimitationRequirement {
			doc /* The vehicle mass shall be less than or equal to 2500 kg when the fuel tank is empty. */
			
			subject vehicle : Vehicle;
			
			attribute :>> massActual: MassValue = vehicle.mass;
			attribute :>> massReqd = 2500 [kg];
		
			assume constraint fuelConstraint {
				doc /* empty fuel tank */
				vehicle.fuelLevel == 0.0
			}
		}
		
		requirement <'2.2'> vehicleReliability2: ReliabilityRequirement {
			subject vehicle : Vehicle;
		}
			
		requirement <'3.1'> drivePowerInterface {
			doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
			subject drivePwrPort: DrivePwrPort;
		}
		
		requirement <'3.2'> torqueGeneration {
			doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
			subject generateTorque: 'Generate Torque';
		}
			
	}
	
	part 'vehicle1_c1 Specification Context' {
		private import 'vehicle1-c1 Specification'::*;
		private import 'engine-v1 Specification'::*;
		
		requirement 'vehicle1-c1 Specification' {
		doc
		/*
		 * This models a "requirement group" as a requirement that references other requirements.
		 */
		
			subject vehicle : Vehicle;
			requirement references vehicleMass1 {
				/*
				 * This is a reference to a requirement defined outside the group.
				 * By default, the subject of the requirement is bound to that of the group.
				 */				
			}
			// ...
		}
		
		requirement 'engine-v1 Specification' {
			subject engine : Engine;
			/* 
			 * Here the subjects of the referenced requirements are defined to be specific properties of the
			 * subject of the group.
			 */
			require torqueGeneration {
				in :>> generateTorque = engine.'generate torque';
			}
			require drivePowerInterface {
				in :>> drivePwrPort = engine.drivePwrPort; 
			}
		}
		
		satisfy 'vehicle1-c1 Specification' by vehicle1_c1 {
			/*
			 * This asserts that if the assumptions of 'vehicle1-c1 Specification' are true with 'vehicle_c1' as
			 * the subject, then the required constraints are also true.
			 */
		}
		satisfy 'engine-v1 Specification' by vehicle1_c1.engine_v1;
	}
	
	part 'vehicle1_c2 Specification Context' {
		private import 'vehicle1-c2 Specification'::*;
		
		requirement 'vehicle1-c2 Specification' {
			subject vehicle : Vehicle;
			require vehicleMass2;
			require vehicleReliability2;
		}
		
		satisfy 'vehicle1-c2 Specification' by vehicle1_c2;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "8_requirements.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 3) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 19) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 35 16) (end 35 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 20) (end 42 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 3) (end 46 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 19) (end 46 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 22) (end 51 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 20) (end 59 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 65 16) (end 65 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 3) (end 76 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 3) (end 77 33))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 89 2) (end 89 770))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 3) (end 92 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 3) (end 98 143))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 120 2) (end 120 402))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 123 3) (end 123 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 3) (end 125 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 3) (end 135 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 140 3) (end 140 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 3) (end 145 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 10) (end 184 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 190 10) (end 190 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 190 39) (end 190 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 202 10) (end 202 37))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,UnrestrictedName,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwAction,UnrestrictedName,OpenCurly,
KwAction,UnrestrictedName,OpenCurly,RegularComment,CloseCurly,
LineComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPerform,UnrestrictedName,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Semicolon,
KwPerform,UnrestrictedName,Dot,UnrestrictedName,ColonGtGt,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwInterface,Ident,Colon,Ident,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
RegularComment,
LineComment,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,
RegularComment,
Ident,LtEq,Ident,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,Semicolon,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Eq,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,OpenCurly,
RegularComment,
CloseCurly,
KwAssume,KwConstraint,Ident,OpenCurly,
RegularComment,
KwDoc,RegularComment,
Ident,Dot,Ident,GtEq,Ident,Dot,Ident,
CloseCurly,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAssume,KwConstraint,Ident,OpenCurly,
KwDoc,RegularComment,
Ident,Dot,Ident,EqEq,DecimalValue,Dot,DecimalValue,
CloseCurly,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
KwPart,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwRequirement,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,KwReferences,Ident,OpenCurly,
RegularComment,
CloseCurly,
LineComment,
CloseCurly,
KwRequirement,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
RegularComment,
KwRequire,Ident,OpenCurly,
KwIn,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,
CloseCurly,
KwRequire,Ident,OpenCurly,
KwIn,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwSatisfy,UnrestrictedName,KwBy,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwSatisfy,UnrestrictedName,KwBy,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwRequirement,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwRequire,Ident,Semicolon,
CloseCurly,
KwSatisfy,UnrestrictedName,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''8-Requirements''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl public ''Vehicle Usages'::*')
    (import_decl public ''Vehicle Requirements'::*')
    (package_def ''Vehicle Definitions''
      (part_def 'Vehicle'
        (attribute_usage 'mass' : 'MassValue')
        (attribute_usage 'fuelLevel' : 'Real')
        (attribute_usage 'fuelTankCapacity' : 'Real'))
      (part_def 'Engine'
        (port_usage 'drivePwrPort' : 'DrivePwrPort')
        (perform_action ''generate torque'' : ''Generate Torque''))
      (part_def 'Transmission'
        (port_usage 'clutchPort' : 'ClutchPort'))
      (port_def 'DrivePwrPort')
      (port_def 'ClutchPort')
      (interface_def 'EngineToTransmissionInterface'
        (interface_end end 'drivePwrPort' : 'DrivePwrPort')
        (interface_end end 'clutchPort' : 'ClutchPort'))
      (action_def ''Generate Torque''))
    (package_def ''Vehicle Usages''
      (import_decl public ''Vehicle Definitions'::*')
      (action_usage ''provide power''
        (action_usage ''generate torque''
          (comment))
        (line_comment))
      (part_usage 'vehicle1_c1' : 'Vehicle'
        (attribute_usage :>> 'mass' value)
        (perform_action :>> ''provide power'')
        (part_usage 'engine_v1' : 'Engine'
          (port_usage :>> 'drivePwrPort')
          (perform_action :>> ''provide power'.'generate torque'')
          (default_ref_usage :>> ''generate torque''))
        (part_usage 'transmission' : 'Transmission'
          (port_usage :>> 'clutchPort'))
        (interface_usage 'EngineToTransmissionInterface' 'engineToTransmission'
          (connector_end)
          (connector_end)))
      (part_usage 'vehicle1_c2' : 'Vehicle'
        (attribute_usage :>> 'mass' value)))
    (package_def ''Vehicle Requirements''
      (import_decl public ''Vehicle Definitions'::*')
      (requirement_def 'MassLimitationRequirement'
        (comment)
        (line_comment)
        (documentation)
        (attribute_usage 'massActual' : 'MassValue')
        (attribute_usage 'massReqd' : 'MassValue')
        (sysml_decl
          (comment)
          (result_expr_member)))
      (requirement_def 'ReliabilityRequirement')
      (requirement_usage 'vehicleMass1' : 'MassLimitationRequirement'
        (documentation)
        (sysml_decl 'vehicle' : 'Vehicle'
          (comment))
        (attribute_usage :>> 'massActual' : 'MassValue' value
          (comment))
        (attribute_usage :>> 'massReqd' value
          (comment))
        (sysml_decl 'fuelConstraint'
          (comment)
          (documentation)
          (result_expr_member)))
      (requirement_usage 'vehicleMass2' : 'MassLimitationRequirement'
        (documentation)
        (sysml_decl 'vehicle' : 'Vehicle')
        (attribute_usage :>> 'massActual' : 'MassValue' value)
        (attribute_usage :>> 'massReqd' value)
        (sysml_decl 'fuelConstraint'
          (documentation)
          (result_expr_member)))
      (requirement_usage 'vehicleReliability2' : 'ReliabilityRequirement'
        (sysml_decl 'vehicle' : 'Vehicle'))
      (requirement_usage 'drivePowerInterface'
        (documentation)
        (sysml_decl 'drivePwrPort' : 'DrivePwrPort'))
      (requirement_usage 'torqueGeneration'
        (documentation)
        (sysml_decl 'generateTorque' : ''Generate Torque'')))
    (part_usage ''vehicle1_c1 Specification Context''
      (import_decl private ''vehicle1-c1 Specification'::*')
      (import_decl private ''engine-v1 Specification'::*')
      (requirement_usage ''vehicle1-c1 Specification''
        (documentation)
        (sysml_decl 'vehicle' : 'Vehicle')
        (requirement_usage references 'vehicleMass1'
          (comment))
        (line_comment))
      (requirement_usage ''engine-v1 Specification''
        (sysml_decl 'engine' : 'Engine')
        (comment)
        (sysml_decl 'torqueGeneration'
          (default_ref_usage in :>> 'generateTorque' value))
        (sysml_decl 'drivePowerInterface'
          (default_ref_usage in :>> 'drivePwrPort' value)))
      (sysml_decl ''vehicle1-c1 Specification''
        (comment))
      (sysml_decl ''engine-v1 Specification''))
    (part_usage ''vehicle1_c2 Specification Context''
      (import_decl private ''vehicle1-c2 Specification'::*')
      (requirement_usage ''vehicle1-c2 Specification''
        (sysml_decl 'vehicle' : 'Vehicle')
        (sysml_decl 'vehicleMass2')
        (sysml_decl 'vehicleReliability2'))
      (sysml_decl ''vehicle1-c2 Specification''))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'vehicle1-c1 Specification'
semantic.duplicate_name 'engine-v1 Specification'
semantic.duplicate_name 'vehicle1-c2 Specification'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'generateTorque'
semantic.unresolved_name 'drivePwrPort'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'vehicle1-c1 Specification'
semantic.duplicate_name 'engine-v1 Specification'
semantic.duplicate_name 'vehicle1-c2 Specification'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'generateTorque'
semantic.unresolved_name 'drivePwrPort'
~~~
# FORMAT
~~~sysml
package '8-Requirements' {
    private import ScalarValues::Real;
    private import ISQ::*;
    private import SI::*;
    public import 'Vehicle Usages'::*;
    public import 'Vehicle Requirements'::*;

    package 'Vehicle Definitions' {
        part def Vehicle {
            attribute mass: MassValue;
            attribute fuelLevel: Real;
            attribute fuelTankCapacity: Real;
        }

        part def Engine {
            port drivePwrPort: DrivePwrPort;
            perform action 'generate torque': 'Generate Torque';
        }

        part def Transmission {
            port clutchPort: ClutchPort;
        }

        port def DrivePwrPort;
        port def ClutchPort;

        interface def EngineToTransmissionInterface {
            end drivePwrPort: DrivePwrPort;
            end clutchPort: ClutchPort;
        }

        action def 'Generate Torque';
    }

    package 'Vehicle Usages' {
        public import 'Vehicle Definitions'::*;

        action 'provide power' {
            action 'generate torque' { /* ... */ }
            //...
        }

        part vehicle1_c1: Vehicle {
            attribute :>> mass = 2000 [kg];
            perform 'provide power';

            part engine_v1: Engine {
                port :>> drivePwrPort;
                perform 'provide power'.'generate torque' :>> 'generate torque';
            }

            part transmission: Transmission {
                port :>> clutchPort;
            }

            interface engineToTransmission: EngineToTransmissionInterface
            connect engine_v1.drivePwrPort to transmission.clutchPort;
        }

        part vehicle1_c2: Vehicle {
            attribute :>> mass = 2500 [kg];
        }
    }

    package 'Vehicle Requirements' {
        public import 'Vehicle Definitions'::*;

        requirement def <'1'> MassLimitationRequirement {
            /*
			 * The optional requirement ID  of this requirement ('1') is given after the keyword "id" (using name syntax).
			 * Every requirement is parameterized by a "subject". The "subject" of this requirement is implicitly "Anything".
			 */

            // The requirement text is given by the documentation in the requirement def body.
            doc /* The actual mass shall be less than or equal to the required mass. */

            attribute massActual: MassValue;
            attribute massReqd: MassValue;

            require constraint {
                /*
				 * A constraint can be used to formalize a requirement.
				 */
                massActual <= massReqd
            }
        }

        requirement def <'2'> ReliabilityRequirement;

        requirement <'1.1'> vehicleMass1: MassLimitationRequirement {
            doc /* The vehicle mass shall be less than or equal to 2000 kg when the fuel tank is full. */

            subject vehicle : Vehicle {
                /*
				 * The subject of this requirement is redefined to be a "Vehicle".
				 */
            }

            attribute :>> massActual: MassValue = vehicle.mass {
                /*
				 * This redefinition binds the vehicle mass to the actual mass.
				 */
            }

            attribute :>> massReqd = 2000 [kg] {
                /*
				 * This redefinition sets the required mass to 2000 kg.
				 */
            }

            assume constraint fuelConstraint {
                /*
				 * A constraint can also be used to specify an assumption.
				 */

                doc /* full fuel tank */
                vehicle.fuelLevel >= vehicle.fuelTankCapacity
            }
        }

        requirement <'2.1'> vehicleMass2: MassLimitationRequirement {
            doc /* The vehicle mass shall be less than or equal to 2500 kg when the fuel tank is empty. */

            subject vehicle : Vehicle;

            attribute :>> massActual: MassValue = vehicle.mass;
            attribute :>> massReqd = 2500 [kg];

            assume constraint fuelConstraint {
                doc /* empty fuel tank */
                vehicle.fuelLevel == 0.0
            }
        }

        requirement <'2.2'> vehicleReliability2: ReliabilityRequirement {
            subject vehicle : Vehicle;
        }

        requirement <'3.1'> drivePowerInterface {
            doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
            subject drivePwrPort: DrivePwrPort;
        }

        requirement <'3.2'> torqueGeneration {
            doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
            subject generateTorque: 'Generate Torque';
        }

    }

    part 'vehicle1_c1 Specification Context' {
        private import 'vehicle1-c1 Specification'::*;
        private import 'engine-v1 Specification'::*;

        requirement 'vehicle1-c1 Specification' {
            doc
            /*
		 * This models a "requirement group" as a requirement that references other requirements.
		 */

            subject vehicle : Vehicle;
            requirement references vehicleMass1 {
                /*
				 * This is a reference to a requirement defined outside the group.
				 * By default, the subject of the requirement is bound to that of the group.
				 */				
            }
            // ...
        }

        requirement 'engine-v1 Specification' {
            subject engine : Engine;
            /* 
			 * Here the subjects of the referenced requirements are defined to be specific properties of the
			 * subject of the group.
			 */
            require torqueGeneration {
                in :>> generateTorque = engine.'generate torque';
            }
            require drivePowerInterface {
                in :>> drivePwrPort = engine.drivePwrPort;
            }
        }

        satisfy 'vehicle1-c1 Specification' by vehicle1_c1 {
            /*
			 * This asserts that if the assumptions of 'vehicle1-c1 Specification' are true with 'vehicle_c1' as
			 * the subject, then the required constraints are also true.
			 */
        }
        satisfy 'engine-v1 Specification' by vehicle1_c1.engine_v1;
    }

    part 'vehicle1_c2 Specification Context' {
        private import 'vehicle1-c2 Specification'::*;

        requirement 'vehicle1-c2 Specification' {
            subject vehicle : Vehicle;
            require vehicleMass2;
            require vehicleReliability2;
        }

        satisfy 'vehicle1-c2 Specification' by vehicle1_c2;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "29bd4e8db76250c28caa803858441e744569a4b15157d93b086eb52e108067b9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "8-Requirements"))) (kind "package") (name "8-Requirements") (declared-name "8-Requirements") (range (start (line 0) (character 0)) (end (line 0) (character 5449))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 22))) (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 18))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 35))) (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 15)) (end (line 4) (character 31))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 1)) (end (line 5) (character 41))) (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Requirements::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 15)) (end (line 5) (character 37))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))) (kind "package") (name "Vehicle Definitions") (declared-name "Vehicle Definitions") (range (start (line 7) (character 1)) (end (line 7) (character 547))) (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (range (start (line 24) (character 2)) (end (line 24) (character 22))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (range (start (line 24) (character 2)) (end (line 24) (character 22))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))) (kind "port def") (name "DrivePwrPort") (declared-name "DrivePwrPort") (range (start (line 23) (character 2)) (end (line 23) (character 24))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort::~DrivePwrPort"))) (kind "conjugated port definition") (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (range (start (line 23) (character 2)) (end (line 23) (character 24))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 14) (character 2)) (end (line 14) (character 115))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "8-Requirements::Vehicle Definitions::Engine::generate torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 15) (character 3)) (end (line 15) (character 35))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (range (start (line 16) (character 3)) (end (line 16) (character 55))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (authored (relationships (typing (reference "Generate Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (kind "interface def") (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface") (range (start (line 26) (character 2)) (end (line 26) (character 117))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (kind "interface end") (name "clutchPort") (declared-name "clutchPort") (range (start (line 28) (character 3)) (end (line 28) (character 30))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind "interface end") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 27) (character 3)) (end (line 27) (character 34))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (range (start (line 31) (character 2)) (end (line 31) (character 31))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 19) (character 2)) (end (line 19) (character 61))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 20) (character 3)) (end (line 20) (character 31))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 8) (character 2)) (end (line 8) (character 121))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind "attribute") (name "fuelLevel") (declared-name "fuelLevel") (range (start (line 10) (character 3)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 10) (character 24)) (end (line 10) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind "attribute") (name "fuelTankCapacity") (declared-name "fuelTankCapacity") (range (start (line 11) (character 3)) (end (line 11) (character 36))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 11) (character 31)) (end (line 11) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 9) (character 3)) (end (line 9) (character 29))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 9) (character 19)) (end (line 9) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (kind "package") (name "Vehicle Requirements") (declared-name "Vehicle Requirements") (range (start (line 64) (character 1)) (end (line 64) (character 2481))) (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 65) (character 2)) (end (line 65) (character 41))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 65) (character 16)) (end (line 65) (character 37))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (kind "requirement def") (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement") (range (start (line 67) (character 2)) (end (line 67) (character 679))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 67) (character 2)) (end (line 67) (character 679))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 79) (character 3)) (end (line 79) (character 133))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 76) (character 3)) (end (line 76) (character 35))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 77) (character 3)) (end (line 77) (character 33))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement"))) (kind "requirement def") (name "ReliabilityRequirement") (declared-name "ReliabilityRequirement") (range (start (line 87) (character 2)) (end (line 87) (character 47))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (kind "requirement") (name "drivePowerInterface") (declared-name "drivePowerInterface") (range (start (line 138) (character 2)) (end (line 138) (character 192))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (subject (reference "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::_documentation"))) (kind "documentation") (name "") (range (start (line 138) (character 2)) (end (line 138) (character 192))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (kind "subject") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 140) (character 3)) (end (line 140) (character 38))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (authored (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (kind "requirement") (name "torqueGeneration") (declared-name "torqueGeneration") (range (start (line 143) (character 2)) (end (line 143) (character 178))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (subject (reference "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::_documentation"))) (kind "documentation") (name "") (range (start (line 143) (character 2)) (end (line 143) (character 178))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (kind "subject") (name "generateTorque") (declared-name "generateTorque") (range (start (line 145) (character 3)) (end (line 145) (character 45))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (authored (relationships (typing (reference "Generate Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind "requirement") (name "vehicleMass1") (declared-name "vehicleMass1") (range (start (line 89) (character 2)) (end (line 89) (character 770))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassLimitationRequirement") (range none)) (subject (reference "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_documentation"))) (kind "documentation") (name "") (range (start (line 89) (character 2)) (end (line 89) (character 770))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 110) (character 3)) (end (line 110) (character 203))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (range (start (line 110) (character 3)) (end (line 110) (character 203))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 98) (character 3)) (end (line 98) (character 143))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (authored (relationships (typing (reference "MassValue") (range none)) (redefinition (reference "massActual") (range (start (line 98) (character 17)) (end (line 98) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 104) (character 3)) (end (line 104) (character 119))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (authored (relationships (redefinition (reference "massReqd") (range (start (line 104) (character 17)) (end (line 104) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 92) (character 3)) (end (line 92) (character 121))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind "requirement") (name "vehicleMass2") (declared-name "vehicleMass2") (range (start (line 120) (character 2)) (end (line 120) (character 402))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassLimitationRequirement") (range none)) (subject (reference "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_documentation"))) (kind "documentation") (name "") (range (start (line 120) (character 2)) (end (line 120) (character 402))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 128) (character 3)) (end (line 128) (character 101))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (range (start (line 128) (character 3)) (end (line 128) (character 101))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 125) (character 3)) (end (line 125) (character 54))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (authored (relationships (typing (reference "MassValue") (range none)) (redefinition (reference "massActual") (range (start (line 125) (character 17)) (end (line 125) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 126) (character 3)) (end (line 126) (character 38))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (authored (relationships (redefinition (reference "massReqd") (range (start (line 126) (character 17)) (end (line 126) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 123) (character 3)) (end (line 123) (character 29))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind "requirement") (name "vehicleReliability2") (declared-name "vehicleReliability2") (range (start (line 134) (character 2)) (end (line 134) (character 101))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "ReliabilityRequirement") (range none)) (subject (reference "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 135) (character 3)) (end (line 135) (character 29))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (kind "package") (name "Vehicle Usages") (declared-name "Vehicle Usages") (range (start (line 34) (character 1)) (end (line 34) (character 666))) (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 35) (character 2)) (end (line 35) (character 41))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 35) (character 16)) (end (line 35) (character 37))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 37) (character 2)) (end (line 37) (character 81))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Feature)) (relationships (perform (reference "8-Requirements::Vehicle Usages::provide power::generate torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (range (start (line 38) (character 3)) (end (line 38) (character 41))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 42) (character 2)) (end (line 42) (character 433))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 42) (character 20)) (end (line 42) (character 27)))) (perform (reference "8-Requirements::Vehicle Usages::vehicle1_c1::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind "part") (name "engine_v1") (declared-name "engine_v1") (range (start (line 46) (character 3)) (end (line 46) (character 128))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 46) (character 19)) (end (line 46) (character 25)))) (perform (reference "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power::generate torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 47) (character 4)) (end (line 47) (character 26))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "drivePwrPort") (range (start (line 47) (character 13)) (end (line 47) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power.generate torque"))) (kind "action") (name "provide power.generate torque") (declared-name "provide power.generate torque") (range (start (line 48) (character 4)) (end (line 48) (character 68))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 43) (character 3)) (end (line 43) (character 34))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 43) (character 17)) (end (line 43) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 44) (character 3)) (end (line 44) (character 27))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 51) (character 3)) (end (line 51) (character 66))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 51) (character 22)) (end (line 51) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 52) (character 4)) (end (line 52) (character 24))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "clutchPort") (range (start (line 52) (character 13)) (end (line 52) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind "part") (name "vehicle1_c2") (declared-name "vehicle1_c2") (range (start (line 59) (character 2)) (end (line 59) (character 68))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 59) (character 20)) (end (line 59) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 60) (character 3)) (end (line 60) (character 34))) (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 60) (character 17)) (end (line 60) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind "part") (name "vehicle1_c1 Specification Context") (declared-name "vehicle1_c1 Specification Context") (range (start (line 150) (character 1)) (end (line 150) (character 1261))) (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind "part") (name "vehicle1_c2 Specification Context") (declared-name "vehicle1_c2 Specification Context") (range (start (line 193) (character 1)) (end (line 193) (character 290))) (parent (node (document "d0") (qualified-name "8-Requirements"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 3) (character 16)) (end (line 3) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Usages::*") (range (start (line 4) (character 15)) (end (line 4) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Requirements::*") (range (start (line 5) (character 15)) (end (line 5) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Definitions::Engine::generate torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 10) (character 24)) (end (line 10) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 11) (character 31)) (end (line 11) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 9) (character 19)) (end (line 9) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Definitions::*") (range (start (line 65) (character 16)) (end (line 65) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind featureTyping) (ordinal 0)) (authored-target "MassLimitationRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (range (start (line 98) (character 17)) (end (line 98) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (range (start (line 104) (character 17)) (end (line 104) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind featureTyping) (ordinal 0)) (authored-target "MassLimitationRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (range (start (line 125) (character 17)) (end (line 125) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (range (start (line 126) (character 17)) (end (line 126) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind featureTyping) (ordinal 0)) (authored-target "ReliabilityRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Definitions::*") (range (start (line 35) (character 16)) (end (line 35) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Usages::provide power::generate torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 42) (character 20)) (end (line 42) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 0)) (authored-target "engine_v1::drivePwrPort") (range (start (line 56) (character 12)) (end (line 56) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind connectionTarget) (ordinal 0)) (authored-target "transmission::clutchPort") (range (start (line 56) (character 38)) (end (line 56) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Usages::vehicle1_c1::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 46) (character 19)) (end (line 46) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power::generate torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (kind redefinition) (ordinal 0)) (authored-target "drivePwrPort") (range (start (line 47) (character 13)) (end (line 47) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 43) (character 17)) (end (line 43) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 51) (character 22)) (end (line 51) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (range (start (line 52) (character 13)) (end (line 52) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 59) (character 20)) (end (line 59) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 60) (character 17)) (end (line 60) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicle1-c1 Specification") (range (start (line 184) (character 10)) (end (line 184) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfySource) (ordinal 1)) (authored-target "engine-v1 Specification") (range (start (line 190) (character 10)) (end (line 190) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle1_c1") (range (start (line 184) (character 41)) (end (line 184) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfyTarget) (ordinal 1)) (authored-target "vehicle1_c1::engine_v1") (range (start (line 190) (character 39)) (end (line 190) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicle1-c2 Specification") (range (start (line 202) (character 10)) (end (line 202) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle1_c2") (range (start (line 202) (character 41)) (end (line 202) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (target (node (document "d0") (qualified-name "8-Requirements::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (target (node (document "d0") (qualified-name "8-Requirements::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (target (node (document "d0") (qualified-name "8-Requirements::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (target (node (document "d0") (qualified-name "8-Requirements::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "engine_v1::drivePwrPort") (target "transmission::clutchPort") (source-range (start (line 56) (character 12)) (end (line 56) (character 34))) (target-range (start (line 56) (character 38)) (end (line 56) (character 61)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
