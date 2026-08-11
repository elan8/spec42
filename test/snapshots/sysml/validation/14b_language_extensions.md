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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14b_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 18) (end 11 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 34) (end 13 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 14 33) (end 14 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 18) (end 24 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 18) (end 26 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 30) (end 27 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 5) (end 28 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 17) (end 33 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 5) (end 34 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 5) (end 35 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 5) (end 36 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 17) (end 41 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 29) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 6) (end 43 25))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6d2bd28e50ba345c0ecd652db51d3d0033827d2b94ece67422c829f376eb3b1d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions"))) (kind "package") (name "14b-Language-Extensions") (declared-name "14b-Language-Extensions") (range (start (line 0) (character 0)) (end (line 0) (character 939))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel"))) (kind "package") (name "LibraryModel") (declared-name "LibraryModel") (range (start (line 2) (character 1)) (end (line 2) (character 48))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (kind "part def") (name "ECU") (declared-name "ECU") (range (start (line 4) (character 2)) (end (line 4) (character 15))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))) (kind "package") (name "UserModel") (declared-name "UserModel") (range (start (line 8) (character 1)) (end (line 8) (character 848))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 10) (character 2)) (end (line 10) (character 231))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 3)) (end (line 11) (character 34))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "LibraryModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 18)) (end (line 11) (character 30))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (kind "port def") (name "BusIF") (declared-name "BusIF") (range (start (line 20) (character 3)) (end (line 20) (character 18))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF::~BusIF"))) (kind "conjugated port definition") (name "~BusIF") (declared-name "~BusIF") (range (start (line 20) (character 3)) (end (line 20) (character 18))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (kind "part def") (name "CanBus") (declared-name "CanBus") (range (start (line 18) (character 3)) (end (line 18) (character 19))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 17) (character 3)) (end (line 17) (character 19))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind "part def") (name "EngineControlUnit") (declared-name "EngineControlUnit") (range (start (line 14) (character 3)) (end (line 14) (character 37))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ECU") (range (start (line 14) (character 33)) (end (line 14) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 16) (character 3)) (end (line 16) (character 20))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind "part def") (name "VehicleControlUnit") (declared-name "VehicleControlUnit") (range (start (line 13) (character 3)) (end (line 13) (character 38))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ECU") (range (start (line 13) (character 34)) (end (line 13) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 23) (character 2)) (end (line 23) (character 581))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 24) (character 3)) (end (line 24) (character 33))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 24) (character 18)) (end (line 24) (character 29))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (range (start (line 26) (character 3)) (end (line 26) (character 520))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 26) (character 18)) (end (line 26) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind "part") (name "canBus") (declared-name "canBus") (range (start (line 33) (character 4)) (end (line 33) (character 132))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "CanBus") (range (start (line 33) (character 17)) (end (line 33) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind "port") (name "engineControlIF") (declared-name "engineControlIF") (range (start (line 35) (character 5)) (end (line 35) (character 33))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (authored (membership (kind Feature)) (relationships (typing (reference "BusIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind "port") (name "sensorIF") (declared-name "sensorIF") (range (start (line 36) (character 5)) (end (line 36) (character 26))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (authored (membership (kind Feature)) (relationships (typing (reference "BusIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind "port") (name "vehicleControlIF") (declared-name "vehicleControlIF") (range (start (line 34) (character 5)) (end (line 34) (character 34))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (authored (membership (kind Feature)) (relationships (typing (reference "BusIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 41) (character 4)) (end (line 41) (character 117))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 41) (character 17)) (end (line 41) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind "part") (name "engineControlUnit") (declared-name "engineControlUnit") (range (start (line 42) (character 5)) (end (line 42) (character 81))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineControlUnit") (range (start (line 42) (character 29)) (end (line 42) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind "port") (name "busIF") (declared-name "busIF") (range (start (line 43) (character 6)) (end (line 43) (character 25))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "~BusIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind "part") (name "vehicleControlUnit") (declared-name "vehicleControlUnit") (range (start (line 27) (character 4)) (end (line 27) (character 81))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleControlUnit") (range (start (line 27) (character 30)) (end (line 27) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind "port") (name "busIF") (declared-name "busIF") (range (start (line 28) (character 5)) (end (line 28) (character 24))) (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "~BusIF") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LibraryModel::*") (range (start (line 11) (character 18)) (end (line 11) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0)) (authored-target "ECU") (range (start (line 14) (character 33)) (end (line 14) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0)) (authored-target "ECU") (range (start (line 13) (character 34)) (end (line 13) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 24) (character 18)) (end (line 24) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 26) (character 18)) (end (line 26) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 0)) (authored-target "vehicleControlUnit::busIF") (range (start (line 31) (character 12)) (end (line 31) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 1)) (authored-target "engine::engineControlUnit::busIF") (range (start (line 39) (character 12)) (end (line 39) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionTarget) (ordinal 0)) (authored-target "canBus::vehicleControlIF") (range (start (line 31) (character 40)) (end (line 31) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionTarget) (ordinal 1)) (authored-target "canBus::engineControlIF") (range (start (line 39) (character 46)) (end (line 39) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0)) (authored-target "CanBus") (range (start (line 33) (character 17)) (end (line 33) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind featureTyping) (ordinal 0)) (authored-target "BusIF") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind featureTyping) (ordinal 0)) (authored-target "BusIF") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind featureTyping) (ordinal 0)) (authored-target "BusIF") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 41) (character 17)) (end (line 41) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineControlUnit") (range (start (line 42) (character 29)) (end (line 42) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind featureTyping) (ordinal 0)) (authored-target "~BusIF") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleControlUnit") (range (start (line 27) (character 30)) (end (line 27) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind featureTyping) (ordinal 0)) (authored-target "~BusIF") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "engine::engineControlUnit::busIF") (target "canBus::engineControlIF") (source-range (start (line 39) (character 12)) (end (line 39) (character 42))) (target-range (start (line 39) (character 46)) (end (line 39) (character 68)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "vehicleControlUnit::busIF") (target "canBus::vehicleControlIF") (source-range (start (line 31) (character 12)) (end (line 31) (character 36))) (target-range (start (line 31) (character 40)) (end (line 31) (character 63)))))
  )
  (evaluation
  )
)
~~~
