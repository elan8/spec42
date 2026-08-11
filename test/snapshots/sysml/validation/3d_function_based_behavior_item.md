# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3d-Function-based Behavior-item
type=file
~~~
# SOURCE
~~~sysml
package '3d-Function-based Behavior-item' {
	private import ScalarValues::Real;
	public import Definitions::*;
	public import Usages::*;
	
	package Definitions {
		
		item def Fuel;
		
		port def FuelPort {
			out item fuel: Fuel;
		}
				
		part def Pump {
			port fuelInPort : ~FuelPort;
			port fuelOutPort : FuelPort;
		}
		
		part def StorageTank {
			port fuelOutPort : FuelPort;
		}
		
		part def FuelTank {
			port fuelInPort : ~FuelPort;
		}
		
		part def Vehicle {
			port fuelInPort : ~FuelPort;
		}
		
		action def PumpFuel {
			in fuelIn : Fuel;
			out fuelOut : Fuel;
		}
		
	}
	
	package Usages {
		
		part context {
			
			/* Storage Element */
			part storageTank : StorageTank;
			
			flow of  fuel : Fuel
				from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel {
				/*
				 * Note: Explicitly notating that the flow is "of fuel : Fuel" is optional.
				 */					
			}
			
			part pump : Pump {
				perform action pumpFuel : PumpFuel {
					in fuelIn = fuelInPort.fuel;
					out fuelOut = fuelOutPort.fuel;
				}
			}
			
			flow of fuel : Fuel
				from pump.fuelOutPort.fuel to vehicle.fuelInPort.fuel;
			
			part vehicle : Vehicle {
				flow fuelInPort.fuel to fuelTank.fuel {
					/* 
					 * Note: The semantics of flowing to a "stored item" is tentative.
					 */					
				}
				
				/* Storage Element */
				part fuelTank : FuelTank {
					attribute volumeMax : Real;
					attribute fuelLevel : Real = fuel.volume / volumeMax;
					
					 /* Stored Item */
					item fuel : Fuel {
						attribute volume : Real;
						/* isConserved = true */
					}
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3d_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 9) (end 45 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 41) (end 45 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 9) (end 59 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 34) (end 59 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 9) (end 62 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 28) (end 62 41))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
RegularComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,KwOf,Ident,Colon,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,KwOf,Ident,Colon,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Slash,Ident,Semicolon,
RegularComment,
KwItem,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3d-Function-based Behavior-item''
    (import_decl private 'ScalarValues::Real')
    (import_decl public 'Definitions::*')
    (import_decl public 'Usages::*')
    (package_def 'Definitions'
      (item_def 'Fuel')
      (port_def 'FuelPort'
        (item_usage out 'fuel' : 'Fuel'))
      (part_def 'Pump'
        (port_usage 'fuelInPort' : ~'FuelPort')
        (port_usage 'fuelOutPort' : 'FuelPort'))
      (part_def 'StorageTank'
        (port_usage 'fuelOutPort' : 'FuelPort'))
      (part_def 'FuelTank'
        (port_usage 'fuelInPort' : ~'FuelPort'))
      (part_def 'Vehicle'
        (port_usage 'fuelInPort' : ~'FuelPort'))
      (action_def 'PumpFuel'
        (default_ref_usage in 'fuelIn' : 'Fuel')
        (default_ref_usage out 'fuelOut' : 'Fuel')))
    (package_def 'Usages'
      (part_usage 'context'
        (comment)
        (part_usage 'storageTank' : 'StorageTank')
        (flow_usage 'of'
          (comment))
        (part_usage 'pump' : 'Pump'
          (perform_action 'pumpFuel' : 'PumpFuel'
            (default_ref_usage in 'fuelIn' value)
            (default_ref_usage out 'fuelOut' value)))
        (flow_usage 'of')
        (part_usage 'vehicle' : 'Vehicle'
          (flow_usage 'fuelInPort'
            (comment))
          (comment)
          (part_usage 'fuelTank' : 'FuelTank'
            (attribute_usage 'volumeMax' : 'Real')
            (attribute_usage 'fuelLevel' : 'Real' value)
            (comment)
            (item_usage 'fuel' : 'Fuel'
              (attribute_usage 'volume' : 'Real')
              (comment))))))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# FORMAT
~~~sysml
package '3d-Function-based Behavior-item' {
    private import ScalarValues::Real;
    public import Definitions::*;
    public import Usages::*;

    package Definitions {

        item def Fuel;

        port def FuelPort {
            out item fuel: Fuel;
        }

        part def Pump {
            port fuelInPort : ~FuelPort;
            port fuelOutPort : FuelPort;
        }

        part def StorageTank {
            port fuelOutPort : FuelPort;
        }

        part def FuelTank {
            port fuelInPort : ~FuelPort;
        }

        part def Vehicle {
            port fuelInPort : ~FuelPort;
        }

        action def PumpFuel {
            in fuelIn : Fuel;
            out fuelOut : Fuel;
        }

    }

    package Usages {

        part context {

            /* Storage Element */
            part storageTank : StorageTank;

            flow of  fuel : Fuel
            from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel {
                /*
				 * Note: Explicitly notating that the flow is "of fuel : Fuel" is optional.
				 */					
            }

            part pump : Pump {
                perform action pumpFuel : PumpFuel {
                    in fuelIn = fuelInPort.fuel;
                    out fuelOut = fuelOutPort.fuel;
                }
            }

            flow of fuel : Fuel
            from pump.fuelOutPort.fuel to vehicle.fuelInPort.fuel;

            part vehicle : Vehicle {
                flow fuelInPort.fuel to fuelTank.fuel {
                    /* 
					 * Note: The semantics of flowing to a "stored item" is tentative.
					 */					
                }

                /* Storage Element */
                part fuelTank : FuelTank {
                    attribute volumeMax : Real;
                    attribute fuelLevel : Real = fuel.volume / volumeMax;

                    /* Stored Item */
                    item fuel : Fuel {
                        attribute volume : Real;
                        /* isConserved = true */
                    }
                }
            }
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "46929156d2791030e4abadc26f3d2825a1185fff093ee5927a77e9b85fd6fd6d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (kind "package") (name "3d-Function-based Behavior-item") (declared-name "3d-Function-based Behavior-item") (range (start (line 0) (character 0)) (end (line 0) (character 1601))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 15)) (end (line 2) (character 26))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 25))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 15)) (end (line 3) (character 21))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 5) (character 1)) (end (line 5) (character 452))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (kind "item def") (name "Fuel") (declared-name "Fuel") (range (start (line 7) (character 2)) (end (line 7) (character 16))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (kind "port def") (name "FuelPort") (declared-name "FuelPort") (range (start (line 9) (character 2)) (end (line 9) (character 49))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind "item") (name "fuel") (declared-name "fuel") (range (start (line 10) (character 3)) (end (line 10) (character 23))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (kind "conjugated port definition") (name "~FuelPort") (declared-name "~FuelPort") (range (start (line 9) (character 2)) (end (line 9) (character 49))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (range (start (line 22) (character 2)) (end (line 22) (character 57))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 23) (character 3)) (end (line 23) (character 31))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (kind "part def") (name "Pump") (declared-name "Pump") (range (start (line 13) (character 2)) (end (line 13) (character 85))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 14) (character 3)) (end (line 14) (character 31))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind "port") (name "fuelOutPort") (declared-name "fuelOutPort") (range (start (line 15) (character 3)) (end (line 15) (character 31))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (kind "action def") (name "PumpFuel") (declared-name "PumpFuel") (range (start (line 30) (character 2)) (end (line 30) (character 71))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind "in out parameter") (name "fuelIn") (declared-name "fuelIn") (range (start (line 31) (character 3)) (end (line 31) (character 20))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (authored (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind "in out parameter") (name "fuelOut") (declared-name "fuelOut") (range (start (line 32) (character 3)) (end (line 32) (character 22))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (authored (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (kind "part def") (name "StorageTank") (declared-name "StorageTank") (range (start (line 18) (character 2)) (end (line 18) (character 60))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind "port") (name "fuelOutPort") (declared-name "fuelOutPort") (range (start (line 19) (character 3)) (end (line 19) (character 31))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 26) (character 2)) (end (line 26) (character 56))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 27) (character 3)) (end (line 27) (character 31))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 37) (character 1)) (end (line 37) (character 1005))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind "part") (name "context") (declared-name "context") (range (start (line 39) (character 2)) (end (line 39) (character 981))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind "part") (name "pump") (declared-name "pump") (range (start (line 51) (character 3)) (end (line 51) (character 144))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Pump") (range (start (line 51) (character 15)) (end (line 51) (character 19)))) (perform (reference "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (kind "action") (name "pumpFuel") (declared-name "pumpFuel") (range (start (line 52) (character 4)) (end (line 52) (character 117))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (authored (relationships (typing (reference "PumpFuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind "part") (name "storageTank") (declared-name "storageTank") (range (start (line 42) (character 3)) (end (line 42) (character 34))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "StorageTank") (range (start (line 42) (character 22)) (end (line 42) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 61) (character 3)) (end (line 61) (character 460))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 61) (character 18)) (end (line 61) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind "part") (name "fuelTank") (declared-name "fuelTank") (range (start (line 69) (character 4)) (end (line 69) (character 251))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTank") (range (start (line 69) (character 20)) (end (line 69) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind "attribute") (name "fuelLevel") (declared-name "fuelLevel") (range (start (line 71) (character 5)) (end (line 71) (character 58))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 71) (character 27)) (end (line 71) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind "attribute") (name "volumeMax") (declared-name "volumeMax") (range (start (line 70) (character 5)) (end (line 70) (character 32))) (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 70) (character 27)) (end (line 70) (character 31)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 2) (character 15)) (end (line 2) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (range (start (line 3) (character 15)) (end (line 3) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowSource) (ordinal 0)) (authored-target "storageTank::fuelOutPort::fuel") (range (start (line 45) (character 9)) (end (line 45) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowSource) (ordinal 1)) (authored-target "pump::fuelOutPort::fuel") (range (start (line 59) (character 9)) (end (line 59) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowTarget) (ordinal 0)) (authored-target "pump::fuelInPort::fuel") (range (start (line 45) (character 41)) (end (line 45) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowTarget) (ordinal 1)) (authored-target "vehicle::fuelInPort::fuel") (range (start (line 59) (character 34)) (end (line 59) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0)) (authored-target "Pump") (range (start (line 51) (character 15)) (end (line 51) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind performSource) (ordinal 0)) (authored-target "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (kind featureTyping) (ordinal 0)) (authored-target "PumpFuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageTank") (range (start (line 42) (character 22)) (end (line 42) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 61) (character 18)) (end (line 61) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "fuelInPort::fuel") (range (start (line 62) (character 9)) (end (line 62) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind flowTarget) (ordinal 0)) (authored-target "fuelTank::fuel") (range (start (line 62) (character 28)) (end (line 62) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTank") (range (start (line 69) (character 20)) (end (line 69) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 71) (character 27)) (end (line 71) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 70) (character 27)) (end (line 70) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
