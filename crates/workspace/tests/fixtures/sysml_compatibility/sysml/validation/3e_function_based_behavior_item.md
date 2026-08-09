# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3e-Function-based Behavior-item
type=file
~~~
# SOURCE
~~~sysml
package '3e-Function-based Behavior-item' {
	public import Definitions::*;
	
	package Definitions {
		
		item def VehicleAssembly;
		item def AssembledVehicle :> VehicleAssembly;
		
		part def Vehicle :> AssembledVehicle;		
		part def Transmission;
		part def Engine;		
		
	}
	
	package Usages {
		
		part AssemblyLine {
		
			perform action 'assemble vehicle' {
				
				action 'assemble transmission into vehicle' {
					in item 'vehicle assy without transmission or engine' : VehicleAssembly;					
					in item transmission : Transmission {
						/* Note: A part can be treated as an item. */
					}
					
					out item 'vehicle assy without engine' : VehicleAssembly = 'vehicle assy without transmission or engine' {						
						part transmission : Transmission = 'assemble transmission into vehicle'.transmission {
							/* Note: An item can become a part of something else. */
						}
					}
				}
				
				flow 'assemble transmission into vehicle'.'vehicle assy without engine' 
				    to 'assemble engine into vehicle'.'vehicle assy without engine';
				
				action 'assemble engine into vehicle' {
					in item 'vehicle assy without engine' : VehicleAssembly {
						part transmission : Transmission;
					}
					in item engine : Engine;
					
					out item assembledVehicle : AssembledVehicle = 'vehicle assy without engine' {
						part engine : Engine = 'assemble engine into vehicle'.engine;
					}
				}
			}
			
			bind 'assemble vehicle'.'assemble engine into vehicle'.assembledVehicle = vehicle;
			
			part vehicle : Vehicle {
				/*
				 * Note: An in item one context can become a part in an other.
				 */
			
				part transmission: Transmission;
				part engine: Engine;
				
				perform action providePower;
			}
			
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,KwAction,UnrestrictedName,OpenCurly,
KwAction,UnrestrictedName,OpenCurly,
KwIn,KwItem,UnrestrictedName,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwOut,KwItem,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,OpenCurly,
KwPart,Ident,Colon,Ident,Eq,UnrestrictedName,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
KwFlow,UnrestrictedName,Dot,UnrestrictedName,
KwTo,UnrestrictedName,Dot,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,OpenCurly,
KwIn,KwItem,UnrestrictedName,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Eq,UnrestrictedName,OpenCurly,
KwPart,Ident,Colon,Ident,Eq,UnrestrictedName,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwBind,UnrestrictedName,Dot,UnrestrictedName,Dot,Ident,Eq,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3e-Function-based Behavior-item''
    (import_decl public 'Definitions::*')
    (package_def 'Definitions'
      (item_def 'VehicleAssembly')
      (item_def 'AssembledVehicle' :> 'VehicleAssembly')
      (part_def 'Vehicle' :> 'AssembledVehicle')
      (part_def 'Transmission')
      (part_def 'Engine'))
    (package_def 'Usages'
      (part_usage 'AssemblyLine'
        (perform_action ''assemble vehicle''
          (action_usage ''assemble transmission into vehicle''
            (item_usage in ''vehicle assy without transmission or engine'' : 'VehicleAssembly')
            (item_usage in 'transmission' : 'Transmission'
              (comment))
            (item_usage out ''vehicle assy without engine'' : 'VehicleAssembly' value
              (part_usage 'transmission' : 'Transmission' value
                (comment))))
          (flow_usage ''assemble transmission into vehicle'')
          (action_usage ''assemble engine into vehicle''
            (item_usage in ''vehicle assy without engine'' : 'VehicleAssembly'
              (part_usage 'transmission' : 'Transmission'))
            (item_usage in 'engine' : 'Engine')
            (item_usage out 'assembledVehicle' : 'AssembledVehicle' value
              (part_usage 'engine' : 'Engine' value))))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (part_usage 'vehicle' : 'Vehicle'
          (comment)
          (part_usage 'transmission' : 'Transmission')
          (part_usage 'engine' : 'Engine')
          (perform_action 'providePower'))))))
~~~
# FORMAT
~~~sysml
package '3e-Function-based Behavior-item' {
    public import Definitions::*;

    package Definitions {

        item def VehicleAssembly;
        item def AssembledVehicle :> VehicleAssembly;

        part def Vehicle :> AssembledVehicle;
        part def Transmission;
        part def Engine;

    }

    package Usages {

        part AssemblyLine {

            perform action 'assemble vehicle' {

                action 'assemble transmission into vehicle' {
                    in item 'vehicle assy without transmission or engine' : VehicleAssembly;
                    in item transmission : Transmission {
                        /* Note: A part can be treated as an item. */
                    }

                    out item 'vehicle assy without engine' : VehicleAssembly = 'vehicle assy without transmission or engine' {
                        part transmission : Transmission = 'assemble transmission into vehicle'.transmission {
                            /* Note: An item can become a part of something else. */
                        }
                    }
                }

                flow 'assemble transmission into vehicle'.'vehicle assy without engine'
                to 'assemble engine into vehicle'.'vehicle assy without engine';

                action 'assemble engine into vehicle' {
                    in item 'vehicle assy without engine' : VehicleAssembly {
                        part transmission : Transmission;
                    }
                    in item engine : Engine;

                    out item assembledVehicle : AssembledVehicle = 'vehicle assy without engine' {
                        part engine : Engine = 'assemble engine into vehicle'.engine;
                    }
                }
            }

            bind 'assemble vehicle'.'assemble engine into vehicle'.assembledVehicle = vehicle;

            part vehicle : Vehicle {
                /*
				 * Note: An in item one context can become a part in an other.
				 */

                part transmission: Transmission;
                part engine: Engine;

                perform action providePower;
            }

        }
    }
}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'assemble transmission into vehicle'
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'assemble transmission into vehicle'
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))) (name "3e-Function-based Behavior-item") (declared-name "3e-Function-based Behavior-item")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (name "AssembledVehicle") (declared-name "AssembledVehicle"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (name "Engine") (declared-name "Engine") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
            (element (kind "item def") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (name "VehicleAssembly") (declared-name "VehicleAssembly"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (name "AssemblyLine") (declared-name "AssemblyLine") (declared (properties (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (name "assemble vehicle") (declared-name "assemble vehicle"))
                (element (kind "part") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))) (name "providePower") (declared-name "providePower") (effective (featuring-type (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
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
    (perform (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (to (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "assemble vehicle::assemble engine into vehicle::assembledVehicle") (target-expression "vehicle") (container-prefix "3e-Function-based Behavior-item::Usages::AssemblyLine"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/3e_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 48 8) (end 48 74))
      )
    )
  )
)
~~~
