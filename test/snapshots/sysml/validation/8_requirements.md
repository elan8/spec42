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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0da62ed91c3515f5d2acc3e4ddedb01d0ab5844ea90d3788aedc28445cd23a1a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "8-Requirements"))) (kind "package") (name "8-Requirements") (declared-name "8-Requirements"))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Requirements::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "8-Requirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))) (kind "package") (name "Vehicle Definitions") (declared-name "Vehicle Definitions") (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))) (kind "port def") (name "DrivePwrPort") (declared-name "DrivePwrPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort::~DrivePwrPort"))) (kind "conjugated port definition") (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "8-Requirements::Vehicle Definitions::Engine::generate torque")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (authored (relationships (typing (reference "Generate Torque")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (kind "interface def") (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (kind "interface end") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind "interface end") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind "attribute") (name "fuelLevel") (declared-name "fuelLevel") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind "attribute") (name "fuelTankCapacity") (declared-name "fuelTankCapacity") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (kind "package") (name "Vehicle Requirements") (declared-name "Vehicle Requirements") (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (kind "requirement def") (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement"))) (kind "requirement def") (name "ReliabilityRequirement") (declared-name "ReliabilityRequirement") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (kind "requirement") (name "drivePowerInterface") (declared-name "drivePowerInterface") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (subject (reference "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (kind "subject") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (authored (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (kind "requirement") (name "torqueGeneration") (declared-name "torqueGeneration") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (subject (reference "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (kind "subject") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (authored (relationships (typing (reference "Generate Torque")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind "requirement") (name "vehicleMass1") (declared-name "vehicleMass1") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassLimitationRequirement")) (subject (reference "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (authored (relationships (typing (reference "MassValue")) (redefinition (reference "massActual")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (authored (relationships (redefinition (reference "massReqd")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind "requirement") (name "vehicleMass2") (declared-name "vehicleMass2") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassLimitationRequirement")) (subject (reference "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (authored (relationships (typing (reference "MassValue")) (redefinition (reference "massActual")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (authored (relationships (redefinition (reference "massReqd")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind "requirement") (name "vehicleReliability2") (declared-name "vehicleReliability2") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "ReliabilityRequirement")) (subject (reference "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (kind "package") (name "Vehicle Usages") (declared-name "Vehicle Usages") (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Import) (visibility "public") (import (reference "Vehicle Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Feature)) (relationships (perform (reference "8-Requirements::Vehicle Usages::provide power::generate torque")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")) (perform (reference "8-Requirements::Vehicle Usages::vehicle1_c1::provide power")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind "part") (name "engine_v1") (declared-name "engine_v1") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")) (perform (reference "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power::generate torque")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "drivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power.generate torque"))) (kind "action") (name "provide power.generate torque") (declared-name "provide power.generate torque") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "clutchPort")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind "part") (name "vehicle1_c2") (declared-name "vehicle1_c2") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind "part") (name "vehicle1_c1 Specification Context") (declared-name "vehicle1_c1 Specification Context") (parent (node (document "d0") (qualified-name "8-Requirements"))))
    (element (id (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind "part") (name "vehicle1_c2 Specification Context") (declared-name "vehicle1_c2 Specification Context") (parent (node (document "d0") (qualified-name "8-Requirements"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Requirements::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Definitions::Engine::generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Engine::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::drivePowerInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::torqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind featureTyping) (ordinal 0)) (authored-target "MassLimitationRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind featureTyping) (ordinal 0)) (authored-target "MassLimitationRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::MassLimitationRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind featureTyping) (ordinal 0)) (authored-target "ReliabilityRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::ReliabilityRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleReliability2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Vehicle Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Usages::provide power::generate torque") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 0)) (authored-target "engine_v1::drivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind connectionTarget) (ordinal 0)) (authored-target "transmission::clutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Usages::vehicle1_c1::provide power") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind performSource) (ordinal 0)) (authored-target "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::provide power::generate torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (kind redefinition) (ordinal 0)) (authored-target "drivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicle1-c1 Specification") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfySource) (ordinal 1)) (authored-target "engine-v1 Specification") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle1_c1") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1")))))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind satisfyTarget) (ordinal 1)) (authored-target "vehicle1_c1::engine_v1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicle1-c2 Specification") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle1_c2") (outcome (status resolved) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2")))))
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
    (relationship (kind connection) (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))) (target (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "engine_v1::drivePwrPort") (target "transmission::clutchPort")))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 18)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 3 16) (end 3 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 24) (end 10 28)) (probe (position 10 24))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 10 24) (end 10 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 11 31) (end 11 35)) (probe (position 11 31))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 11 31) (end 11 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 43 17) (end 43 21)) (probe (position 43 17))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 43 17) (end 43 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::mass") (range (start 43 3) (end 43 34)))
        )
      )
    )
    (query (range (start 60 17) (end 60 21)) (probe (position 60 17))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 60 17) (end 60 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2::mass") (range (start 60 3) (end 60 34)))
        )
      )
    )
    (query (range (start 46 19) (end 46 25)) (probe (position 46 19))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 46 19) (end 46 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 20) (end 42 27)) (probe (position 42 20))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 42 20) (end 42 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 59 20) (end 59 27)) (probe (position 59 20))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 59 20) (end 59 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 104 17) (end 104 25)) (probe (position 104 17))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd"))
        (kind redefinition) (ordinal 0) (authored-target "massReqd")
        (range (start 104 17) (end 104 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massReqd") (range (start 104 3) (end 104 119)))
        )
      )
    )
    (query (range (start 126 17) (end 126 25)) (probe (position 126 17))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd"))
        (kind redefinition) (ordinal 0) (authored-target "massReqd")
        (range (start 126 17) (end 126 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massReqd") (range (start 126 3) (end 126 38)))
        )
      )
    )
    (query (range (start 9 19) (end 9 28)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 9 19) (end 9 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 13) (end 52 23)) (probe (position 52 13))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort"))
        (kind redefinition) (ordinal 0) (authored-target "clutchPort")
        (range (start 52 13) (end 52 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort") (range (start 52 4) (end 52 24)))
        )
      )
    )
    (query (range (start 98 17) (end 98 27)) (probe (position 98 17))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual"))
        (kind redefinition) (ordinal 0) (authored-target "massActual")
        (range (start 98 17) (end 98 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass1::massActual") (range (start 98 3) (end 98 143)))
        )
      )
    )
    (query (range (start 125 17) (end 125 27)) (probe (position 125 17))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual"))
        (kind redefinition) (ordinal 0) (authored-target "massActual")
        (range (start 125 17) (end 125 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::vehicleMass2::massActual") (range (start 125 3) (end 125 54)))
        )
      )
    )
    (query (range (start 184 41) (end 184 52)) (probe (position 184 41))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))
        (kind satisfyTarget) (ordinal 0) (authored-target "vehicle1_c1")
        (range (start 184 41) (end 184 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1") (range (start 42 2) (end 42 433)))
        )
      )
    )
    (query (range (start 202 41) (end 202 52)) (probe (position 202 41))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))
        (kind satisfyTarget) (ordinal 0) (authored-target "vehicle1_c2")
        (range (start 202 41) (end 202 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2") (range (start 59 2) (end 59 68)))
        )
      )
    )
    (query (range (start 47 13) (end 47 25)) (probe (position 47 13))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort"))
        (kind redefinition) (ordinal 0) (authored-target "drivePwrPort")
        (range (start 47 13) (end 47 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort") (range (start 47 4) (end 47 26)))
        )
      )
    )
    (query (range (start 51 22) (end 51 34)) (probe (position 51 22))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 51 22) (end 51 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 15) (end 4 31)) (probe (position 4 15))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Usages::*")
        (range (start 4 15) (end 4 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages") (range (start 34 1) (end 34 666)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 35 16) (end 35 37)) (probe (position 35 16))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Definitions::*")
        (range (start 35 16) (end 35 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 65 16) (end 65 37)) (probe (position 65 16))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Requirements::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Definitions::*")
        (range (start 65 16) (end 65 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 190 39) (end 190 60)) (probe (position 190 39))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))
        (kind satisfyTarget) (ordinal 1) (authored-target "vehicle1_c1::engine_v1")
        (range (start 190 39) (end 190 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 15) (end 5 37)) (probe (position 5 15))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Requirements::*")
        (range (start 5 15) (end 5 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Requirements") (range (start 64 1) (end 64 2481)))
        )
      )
    )
    (query (range (start 56 12) (end 56 34)) (probe (position 56 12))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))
        (kind connectionSource) (ordinal 0) (authored-target "engine_v1::drivePwrPort")
        (range (start 56 12) (end 56 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1::drivePwrPort") (range (start 47 4) (end 47 26)))
        )
      )
    )
    (query (range (start 56 38) (end 56 61)) (probe (position 56 38))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))
        (kind connectionTarget) (ordinal 0) (authored-target "transmission::clutchPort")
        (range (start 56 38) (end 56 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission::clutchPort") (range (start 52 4) (end 52 24)))
        )
      )
    )
    (query (range (start 190 10) (end 190 35)) (probe (position 190 10))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))
        (kind satisfySource) (ordinal 1) (authored-target "engine-v1 Specification")
        (range (start 190 10) (end 190 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 184 10) (end 184 37)) (probe (position 184 10))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))
        (kind satisfySource) (ordinal 0) (authored-target "vehicle1-c1 Specification")
        (range (start 184 10) (end 184 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 202 10) (end 202 37)) (probe (position 202 10))
      (reference
        (source (document "d0") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))
        (kind satisfySource) (ordinal 0) (authored-target "vehicle1-c2 Specification")
        (range (start 202 10) (end 202 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
