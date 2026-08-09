# META
~~~ini
description=SysML Validation (14-Language Extensions): 14b-Language Extensions
type=file
~~~
# SOURCE
~~~sysml
package '14b-Language-Extensions' {
	
	package LibraryModel {
		
		part def ECU;
		
	}
	
	package UserModel {
		
		package Definitions {
			private import LibraryModel::*;
			
			part def VehicleControlUnit :> ECU;
			part def EngineControlUnit :> ECU;
			
			part def Vehicle;
			part def Engine;
			part def CanBus;
			
			port def BusIF;
		}
		
		package Usages {
			private import Definitions::*;
			
			part vehicle1: Vehicle {
				part vehicleControlUnit : VehicleControlUnit {
					port busIF: ~BusIF;
				}
				
				connect vehicleControlUnit.busIF to canBus.vehicleControlIF;
				
				part canBus: CanBus {
					port vehicleControlIF: BusIF;
					port engineControlIF: BusIF;
					port sensorIF: BusIF;					
				}
				
				connect engine.engineControlUnit.busIF to canBus.engineControlIF;
				
				part engine: Engine {
					part engineControlUnit: EngineControlUnit {
						port busIF: ~BusIF;
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
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
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
  (package_def ''14b-Language-Extensions''
    (package_def 'LibraryModel'
      (part_def 'ECU'))
    (package_def 'UserModel'
      (package_def 'Definitions'
        (import_decl private 'LibraryModel::*')
        (part_def 'VehicleControlUnit' :> 'ECU')
        (part_def 'EngineControlUnit' :> 'ECU')
        (part_def 'Vehicle')
        (part_def 'Engine')
        (part_def 'CanBus')
        (port_def 'BusIF'))
      (package_def 'Usages'
        (import_decl private 'Definitions::*')
        (part_usage 'vehicle1' : 'Vehicle'
          (part_usage 'vehicleControlUnit' : 'VehicleControlUnit'
            (port_usage 'busIF' : ~'BusIF'))
          (connection_usage
            (connector_end)
            (connector_end))
          (part_usage 'canBus' : 'CanBus'
            (port_usage 'vehicleControlIF' : 'BusIF')
            (port_usage 'engineControlIF' : 'BusIF')
            (port_usage 'sensorIF' : 'BusIF'))
          (connection_usage
            (connector_end)
            (connector_end))
          (part_usage 'engine' : 'Engine'
            (part_usage 'engineControlUnit' : 'EngineControlUnit'
              (port_usage 'busIF' : ~'BusIF'))))))))
~~~
# FORMAT
~~~sysml
package '14b-Language-Extensions' {

    package LibraryModel {

        part def ECU;

    }

    package UserModel {

        package Definitions {
            private import LibraryModel::*;

            part def VehicleControlUnit :> ECU;
            part def EngineControlUnit :> ECU;

            part def Vehicle;
            part def Engine;
            part def CanBus;

            port def BusIF;
        }

        package Usages {
            private import Definitions::*;

            part vehicle1: Vehicle {
                part vehicleControlUnit : VehicleControlUnit {
                    port busIF: ~BusIF;
                }

                connect vehicleControlUnit.busIF to canBus.vehicleControlIF;

                part canBus: CanBus {
                    port vehicleControlIF: BusIF;
                    port engineControlIF: BusIF;
                    port sensorIF: BusIF;
                }

                connect engine.engineControlUnit.busIF to canBus.engineControlIF;

                part engine: Engine {
                    part engineControlUnit: EngineControlUnit {
                        port busIF: ~BusIF;
                    }
                }
            }
        }

    }
}

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "14b-Language-Extensions"))) (name "14b-Language-Extensions") (declared-name "14b-Language-Extensions")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel"))) (name "LibraryModel") (declared-name "LibraryModel")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (name "ECU") (declared-name "ECU") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))) (name "UserModel") (declared-name "UserModel")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (name "Definitions") (declared-name "Definitions")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::*"))) (name "*") (declared-name "*"))
                (element (kind "port def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (name "BusIF") (declared-name "BusIF")
                  (contains
                    (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF::~BusIF"))) (name "~BusIF") (declared-name "~BusIF") (effective (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
                  )
                )
                (element (kind "part def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (name "CanBus") (declared-name "CanBus") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (name "Engine") (declared-name "Engine") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (name "EngineControlUnit") (declared-name "EngineControlUnit") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
                (element (kind "part def") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (name "VehicleControlUnit") (declared-name "VehicleControlUnit") (declared))
              )
            )
            (element (kind "package") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (name "Usages") (declared-name "Usages")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::*"))) (name "*") (declared-name "*"))
                (element (kind "part") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (name "vehicle1") (declared-name "vehicle1") (declared (properties (ordered false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (name "canBus") (declared-name "canBus") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (name "engineControlIF") (declared-name "engineControlIF") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (name "sensorIF") (declared-name "sensorIF") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (name "vehicleControlIF") (declared-name "vehicleControlIF") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (name "engineControlUnit") (declared-name "engineControlUnit") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))))
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (name "busIF") (declared-name "busIF") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (name "vehicleControlUnit") (declared-name "vehicleControlUnit") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (name "busIF") (declared-name "busIF") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")))))
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
    (connection (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (connect (source-expression "engine::engineControlUnit::busIF") (target-expression "canBus::engineControlIF") (container-prefix "14b-Language-Extensions::UserModel::Usages::vehicle1")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (connect (source-expression "vehicleControlUnit::busIF") (target-expression "canBus::vehicleControlIF") (container-prefix "14b-Language-Extensions::UserModel::Usages::vehicle1")))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF::~BusIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF::~BusIF"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (to (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF::~BusIF"))))
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
  (document "sysml/validation/14b_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 18) (end 11 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 18) (end 24 29))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 36 5) (end 36 26))
      )
    )
  )
)
~~~
