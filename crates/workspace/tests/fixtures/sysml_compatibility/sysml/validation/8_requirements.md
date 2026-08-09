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
            attribute mass : MassValue;
            attribute fuelLevel : Real;
            attribute fuelTankCapacity : Real;
        }

        part def Engine {
            port drivePwrPort : DrivePwrPort;
            perform action 'generate torque' : 'Generate Torque';
        }

        part def Transmission {
            port clutchPort : ClutchPort;
        }

        port def DrivePwrPort;
        port def ClutchPort;

        interface def EngineToTransmissionInterface {
            end drivePwrPort : DrivePwrPort;
            end clutchPort : ClutchPort;
        }

        action def 'Generate Torque';
    }

    package 'Vehicle Usages' {
        public import 'Vehicle Definitions'::*;

        action 'provide power' {
            action 'generate torque' {
                /* ... */
            }
            //...
        }

        part vehicle1_c1 : Vehicle {
            attribute :>> mass = 2000 [kg];
            perform :>> 'provide power';

            part engine_v1 : Engine {
                port :>> drivePwrPort;
                perform :>> 'provide power'.'generate torque';
                :>> 'generate torque';
            }

            part transmission : Transmission {
                port :>> clutchPort;
            }

            interface engineToTransmission : EngineToTransmissionInterface connect engine_v1.drivePwrPort to transmission.clutchPort;
        }

        part vehicle1_c2 : Vehicle {
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

            attribute massActual : MassValue;
            attribute massReqd : MassValue;

            require constraint {
                /*
				 * A constraint can be used to formalize a requirement.
				 */
                = massActual <= massReqd;
            }
        }

        requirement def <'2'> ReliabilityRequirement;

        requirement <'1.1'> vehicleMass1 : MassLimitationRequirement {
            doc /* The vehicle mass shall be less than or equal to 2000 kg when the fuel tank is full. */

            subject vehicle : Vehicle {
                /*
				 * The subject of this requirement is redefined to be a "Vehicle".
				 */
            }

            attribute :>> massActual : MassValue = vehicle.mass {
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
                = vehicle.fuelLevel >= vehicle.fuelTankCapacity;
            }
        }

        requirement <'2.1'> vehicleMass2 : MassLimitationRequirement {
            doc /* The vehicle mass shall be less than or equal to 2500 kg when the fuel tank is empty. */

            subject vehicle : Vehicle;

            attribute :>> massActual : MassValue = vehicle.mass;
            attribute :>> massReqd = 2500 [kg];

            assume constraint fuelConstraint {
                doc /* empty fuel tank */
                = vehicle.fuelLevel == 0.0;
            }
        }

        requirement <'2.2'> vehicleReliability2 : ReliabilityRequirement {
            subject vehicle : Vehicle;
        }

        requirement <'3.1'> drivePowerInterface {
            doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
            subject drivePwrPort : DrivePwrPort;
        }

        requirement <'3.2'> torqueGeneration {
            doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
            subject generateTorque : 'Generate Torque';
        }
    }

    part 'vehicle1_c1 Specification Context' {
        private import 'vehicle1-c1 Specification'::*;
        private import 'engine-v1 Specification'::*;

        requirement 'vehicle1-c1 Specification' {
            doc /*
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
            require constraint torqueGeneration {
                in :>> generateTorque = engine.'generate torque';
            }
            require constraint drivePowerInterface {
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
            require constraint vehicleMass2;
            require constraint vehicleReliability2;
        }

        satisfy 'vehicle1-c2 Specification' by vehicle1_c2;
    }
}
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
# SMG
~~~
(model
  (namespace
    (package '8-Requirements'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import public -> '8-Requirements::Vehicle Usages'[package])
      (namespace_import public -> '8-Requirements::Vehicle Requirements'[package])
      (package 'Vehicle Definitions'
        (part_def 'Vehicle'
          (attribute_usage composite 'mass' : 'MassValue'[unresolved])
          (attribute_usage composite 'fuelLevel' : 'Real'[unresolved])
          (attribute_usage composite 'fuelTankCapacity' : 'Real'[unresolved]))
        (part_def 'Engine'
          (port_usage composite 'drivePwrPort' : '8-Requirements::Vehicle Definitions::DrivePwrPort'[port_def])
          (perform_action_usage 'generate torque' : '8-Requirements::Vehicle Definitions::Generate Torque'[action_def]))
        (part_def 'Transmission'
          (port_usage composite 'clutchPort' : '8-Requirements::Vehicle Definitions::ClutchPort'[port_def]))
        (port_def 'DrivePwrPort')
        (port_def 'ClutchPort')
        (interface_def 'EngineToTransmissionInterface'
          (port_usage end 'drivePwrPort' : '8-Requirements::Vehicle Definitions::DrivePwrPort'[port_def])
          (port_usage end 'clutchPort' : '8-Requirements::Vehicle Definitions::ClutchPort'[port_def]))
        (action_def 'Generate Torque'))
      (package 'Vehicle Usages'
        (namespace_import public -> '8-Requirements::Vehicle Definitions'[package])
        (action_usage 'provide power'
          (action_usage composite 'generate torque'))
        (part_usage 'vehicle1_c1' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def]
          (attribute_usage composite :>> '8-Requirements::Vehicle Definitions::Vehicle::mass'[attribute_usage]
            (feature_value (=)))
          (perform_action_usage :>> '8-Requirements::Vehicle Usages::provide power'[action_usage])
          (part_usage composite 'engine_v1' : '8-Requirements::Vehicle Definitions::Engine'[part_def]
            (port_usage composite :>> '8-Requirements::Vehicle Definitions::Engine::drivePwrPort'[port_usage])
            (perform_action_usage :>> '8-Requirements::Vehicle Usages::provide power::generate torque'[action_usage])
            (reference_usage reference :>> '8-Requirements::Vehicle Definitions::Engine::generate torque'[perform_action_usage]))
          (part_usage composite 'transmission' : '8-Requirements::Vehicle Definitions::Transmission'[part_def]
            (port_usage composite :>> '8-Requirements::Vehicle Definitions::Transmission::clutchPort'[port_usage]))
          (interface_usage composite 'engineToTransmission' : '8-Requirements::Vehicle Definitions::EngineToTransmissionInterface'[interface_def]
            (connector_end 'engine_v1.drivePwrPort')
            (connector_end 'transmission.clutchPort')))
        (part_usage 'vehicle1_c2' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def]
          (attribute_usage composite :>> '8-Requirements::Vehicle Definitions::Vehicle::mass'[attribute_usage]
            (feature_value (=)))))
      (package 'Vehicle Requirements'
        (namespace_import public -> '8-Requirements::Vehicle Definitions'[package])
        (requirement_def 'MassLimitationRequirement'
          (documentation)
          (attribute_usage composite 'massActual' : 'MassValue'[unresolved])
          (attribute_usage composite 'massReqd' : 'MassValue'[unresolved])
          (require_constraint_usage composite
            (result_expr_membership)))
        (requirement_def 'ReliabilityRequirement')
        (requirement_usage 'vehicleMass1' : '8-Requirements::Vehicle Requirements::MassLimitationRequirement'[requirement_def]
          (documentation)
          (subject_membership in 'vehicle' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def])
          (attribute_usage composite :>> '8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual'[attribute_usage] : 'MassValue'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> '8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd'[attribute_usage]
            (feature_value (=)))
          (assume_constraint_usage composite 'fuelConstraint'
            (documentation)
            (result_expr_membership)))
        (requirement_usage 'vehicleMass2' : '8-Requirements::Vehicle Requirements::MassLimitationRequirement'[requirement_def]
          (documentation)
          (subject_membership in 'vehicle' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def])
          (attribute_usage composite :>> '8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual'[attribute_usage] : 'MassValue'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> '8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd'[attribute_usage]
            (feature_value (=)))
          (assume_constraint_usage composite 'fuelConstraint'
            (documentation)
            (result_expr_membership)))
        (requirement_usage 'vehicleReliability2' : '8-Requirements::Vehicle Requirements::ReliabilityRequirement'[requirement_def]
          (subject_membership in 'vehicle' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def]))
        (requirement_usage 'drivePowerInterface'
          (documentation)
          (subject_membership in 'drivePwrPort' : '8-Requirements::Vehicle Definitions::DrivePwrPort'[port_def]))
        (requirement_usage 'torqueGeneration'
          (documentation)
          (subject_membership in 'generateTorque' : '8-Requirements::Vehicle Definitions::Generate Torque'[action_def])))
      (part_usage 'vehicle1_c1 Specification Context'
        (namespace_import private -> '8-Requirements::vehicle1_c1 Specification Context::vehicle1-c1 Specification'[satisfy_requirement_usage])
        (namespace_import private -> '8-Requirements::vehicle1_c1 Specification Context::engine-v1 Specification'[satisfy_requirement_usage])
        (requirement_usage composite 'vehicle1-c1 Specification'
          (documentation)
          (subject_membership in 'vehicle' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def])
          (requirement_usage composite :> '8-Requirements::Vehicle Requirements::vehicleMass1'[requirement_usage]))
        (requirement_usage composite 'engine-v1 Specification'
          (subject_membership in 'engine' : '8-Requirements::Vehicle Definitions::Engine'[part_def])
          (require_constraint_usage composite 'torqueGeneration'
            (reference_usage in reference :>> 'generateTorque'[unresolved]
              (feature_value (=))))
          (require_constraint_usage composite 'drivePowerInterface'
            (reference_usage in reference :>> 'drivePwrPort'[unresolved]
              (feature_value (=)))))
        (satisfy_requirement_usage 'vehicle1-c1 Specification' by '8-Requirements::Vehicle Usages::vehicle1_c1'[part_usage])
        (satisfy_requirement_usage 'engine-v1 Specification' by '8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1'[part_usage]))
      (part_usage 'vehicle1_c2 Specification Context'
        (namespace_import private -> '8-Requirements::vehicle1_c2 Specification Context::vehicle1-c2 Specification'[satisfy_requirement_usage])
        (requirement_usage composite 'vehicle1-c2 Specification'
          (subject_membership in 'vehicle' : '8-Requirements::Vehicle Definitions::Vehicle'[part_def])
          (require_constraint_usage composite 'vehicleMass2')
          (require_constraint_usage composite 'vehicleReliability2'))
        (satisfy_requirement_usage 'vehicle1-c2 Specification' by '8-Requirements::Vehicle Usages::vehicle1_c2'[part_usage])))))
~~~
