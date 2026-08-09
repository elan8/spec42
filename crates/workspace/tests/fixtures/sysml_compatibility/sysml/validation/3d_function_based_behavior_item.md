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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (name "3d-Function-based Behavior-item") (declared-name "3d-Function-based Behavior-item")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (name "Fuel") (declared-name "Fuel"))
            (element (kind "port def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (name "FuelPort") (declared-name "FuelPort")
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (name "fuel") (declared-name "fuel") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (name "~FuelPort") (declared-name "~FuelPort") (effective (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (name "FuelTank") (declared-name "FuelTank") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (name "Pump") (declared-name "Pump") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (name "fuelOutPort") (declared-name "fuelOutPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (name "PumpFuel") (declared-name "PumpFuel")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (name "fuelIn") (declared-name "fuelIn") (effective (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (name "fuelOut") (declared-name "fuelOut") (effective (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (name "StorageTank") (declared-name "StorageTank") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (name "fuelOutPort") (declared-name "fuelOutPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "package") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (name "context") (declared-name "context") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (name "pump") (declared-name "pump") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (name "pumpFuel") (declared-name "pumpFuel") (effective (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (name "storageTank") (declared-name "storageTank") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (name "fuelTank") (declared-name "fuelTank") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (name "fuelLevel") (declared-name "fuelLevel") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "memberAccess") (reference "volume") (children (expression (kind "featureReference") (reference "fuel")))) (expression (kind "featureReference") (reference "volumeMax")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (role feature-value))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (name "volumeMax") (declared-name "volumeMax") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank")))))
                      )
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (to (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/3d_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 5) (end 70 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 5) (end 71 58))
      )
    )
  )
)
~~~
