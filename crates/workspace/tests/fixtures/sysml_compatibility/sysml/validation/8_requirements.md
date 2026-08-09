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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "8-Requirements"))) (name "8-Requirements") (declared-name "8-Requirements")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "package") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))) (name "Vehicle Definitions") (declared-name "Vehicle Definitions")
          (contains
            (element (kind "port def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))) (name "ClutchPort") (declared-name "ClutchPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort::~ClutchPort"))) (name "~ClutchPort") (declared-name "~ClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))) (name "DrivePwrPort") (declared-name "DrivePwrPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort::~DrivePwrPort"))) (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (name "Engine") (declared-name "Engine") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (name "generate torque") (declared-name "generate torque") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine")))))
              )
            )
            (element (kind "interface def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))) (name "Generate Torque") (declared-name "Generate Torque"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (name "fuelLevel") (declared-name "fuelLevel") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (name "fuelTankCapacity") (declared-name "fuelTankCapacity") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (name "Vehicle Requirements") (declared-name "Vehicle Requirements")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::*"))) (name "*") (declared-name "*"))
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
              )
            )
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement"))) (name "ReliabilityRequirement") (declared-name "ReliabilityRequirement"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (name "drivePowerInterface") (declared-name "drivePowerInterface")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::_documentation"))) (name ""))
                (element (kind "subject") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort"))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (name "torqueGeneration") (declared-name "torqueGeneration")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::_documentation"))) (name ""))
                (element (kind "subject") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (name "generateTorque") (declared-name "generateTorque"))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (name "vehicleMass1") (declared-name "vehicleMass1")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (name "vehicleMass2") (declared-name "vehicleMass2")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (name "vehicleReliability2") (declared-name "vehicleReliability2")
              (contains
                (element (kind "subject") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (name "Vehicle Usages") (declared-name "Vehicle Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::*"))) (name "*") (declared-name "*"))
            (element (kind "action") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (name "provide power") (declared-name "provide power") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque"))) (name "generate torque") (declared-name "generate torque") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (name "engine_v1") (declared-name "engine_v1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power.generate torque"))) (name "provide power.generate torque") (declared-name "provide power.generate torque") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 2000)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (role feature-value))))
                (element (kind "action") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power"))) (name "provide power") (declared-name "provide power") (effective (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (name "vehicle1_c2") (declared-name "vehicle1_c2") (declared (properties (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 2500)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (role feature-value))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (name "vehicle1_c1 Specification Context") (declared-name "vehicle1_c1 Specification Context") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (name "vehicle1_c2 Specification Context") (declared-name "vehicle1_c2 Specification Context") (declared (properties (ordered false))))
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context::unresolved_satisfy_source#diagnostic"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0::_documentation"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (connect (source-expression "engine_v1::drivePwrPort") (target-expression "transmission::clutchPort") (container-prefix "8-Requirements::Vehicle Usages::vehicle1_c1") (interface-usage true) (interface-type "EngineToTransmissionInterface")))
    (perform (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort::~ClutchPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort::~DrivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (to (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1") (target-qualified "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power::generate torque"))
  )
  (pending-expression-relationships
    (satisfy (status pending-expression) (document "d0") (source-expression "engine-v1 Specification") (target-expression "vehicle1_c1::engine_v1") (container-prefix "8-Requirements::vehicle1_c1 Specification Context"))
    (satisfy (status pending-expression) (document "d0") (source-expression "vehicle1-c1 Specification") (target-expression "vehicle1_c1") (container-prefix "8-Requirements::vehicle1_c1 Specification Context"))
    (satisfy (status pending-expression) (document "d0") (source-expression "vehicle1-c2 Specification") (target-expression "vehicle1_c2") (container-prefix "8-Requirements::vehicle1_c2 Specification Context"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/8_requirements.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 22))
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
        (range (start 10 3) (end 10 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 3) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 35 2) (end 35 41))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 43 3) (end 43 34))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 60 3) (end 60 34))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 3) (end 98 143))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 104 3) (end 104 119))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 3) (end 125 54))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 126 3) (end 126 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 184 10) (end 184 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 190 10) (end 190 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 202 10) (end 202 37))
      )
    )
  )
)
~~~
