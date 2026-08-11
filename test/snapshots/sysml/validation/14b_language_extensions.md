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
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions"))) (kind "package") (name "14b-Language-Extensions") (declared-name "14b-Language-Extensions"))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel"))) (kind "package") (name "LibraryModel") (declared-name "LibraryModel") (parent (node (document "d0") (qualified-name "14b-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (kind "part def") (name "ECU") (declared-name "ECU") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::LibraryModel"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))) (kind "package") (name "UserModel") (declared-name "UserModel") (parent (node (document "d0") (qualified-name "14b-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "LibraryModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (kind "port def") (name "BusIF") (declared-name "BusIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF::~BusIF"))) (kind "conjugated port definition") (name "~BusIF") (declared-name "~BusIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (kind "part def") (name "CanBus") (declared-name "CanBus") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind "part def") (name "EngineControlUnit") (declared-name "EngineControlUnit") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ECU")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind "part def") (name "VehicleControlUnit") (declared-name "VehicleControlUnit") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ECU")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel"))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind "part") (name "canBus") (declared-name "canBus") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "CanBus")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind "port") (name "engineControlIF") (declared-name "engineControlIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (authored (membership (kind Feature)) (relationships (typing (reference "BusIF")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind "port") (name "sensorIF") (declared-name "sensorIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (authored (membership (kind Feature)) (relationships (typing (reference "BusIF")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind "port") (name "vehicleControlIF") (declared-name "vehicleControlIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (authored (membership (kind Feature)) (relationships (typing (reference "BusIF")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind "part") (name "engineControlUnit") (declared-name "engineControlUnit") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineControlUnit")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind "port") (name "busIF") (declared-name "busIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "~BusIF")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind "part") (name "vehicleControlUnit") (declared-name "vehicleControlUnit") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleControlUnit")))))
    (element (id (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind "port") (name "busIF") (declared-name "busIF") (parent (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "~BusIF")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LibraryModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0)) (authored-target "ECU") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0)) (authored-target "ECU") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 0)) (authored-target "vehicleControlUnit::busIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 1)) (authored-target "engine::engineControlUnit::busIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionTarget) (ordinal 0)) (authored-target "canBus::vehicleControlIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionTarget) (ordinal 1)) (authored-target "canBus::engineControlIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF")))))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0)) (authored-target "CanBus") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind featureTyping) (ordinal 0)) (authored-target "BusIF") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind featureTyping) (ordinal 0)) (authored-target "BusIF") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind featureTyping) (ordinal 0)) (authored-target "BusIF") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineControlUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind featureTyping) (ordinal 0)) (authored-target "~BusIF") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleControlUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind featureTyping) (ordinal 0)) (authored-target "~BusIF") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "engine::engineControlUnit::busIF") (target "canBus::engineControlIF")))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (target (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "vehicleControlUnit::busIF") (target "canBus::vehicleControlIF")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 13 34) (end 13 37)) (probe (position 13 34))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))
        (kind specialization) (ordinal 0) (authored-target "ECU")
        (range (start 13 34) (end 13 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 33) (end 14 36)) (probe (position 14 33))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))
        (kind specialization) (ordinal 0) (authored-target "ECU")
        (range (start 14 33) (end 14 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 17) (end 33 23)) (probe (position 33 17))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))
        (kind featureTyping) (ordinal 0) (authored-target "CanBus")
        (range (start 33 17) (end 33 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 41 17) (end 41 23)) (probe (position 41 17))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 41 17) (end 41 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 18) (end 26 25)) (probe (position 26 18))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 26 18) (end 26 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 18) (end 24 29)) (probe (position 24 18))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 24 18) (end 24 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 18) (end 11 30)) (probe (position 11 18))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Definitions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "LibraryModel::*")
        (range (start 11 18) (end 11 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 29) (end 42 46)) (probe (position 42 29))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))
        (kind featureTyping) (ordinal 0) (authored-target "EngineControlUnit")
        (range (start 42 29) (end 42 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 30) (end 27 48)) (probe (position 27 30))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleControlUnit")
        (range (start 27 30) (end 27 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 46) (end 39 68)) (probe (position 39 46))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))
        (kind connectionTarget) (ordinal 1) (authored-target "canBus::engineControlIF")
        (range (start 39 46) (end 39 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF") (range (start 35 5) (end 35 33)))
        )
      )
    )
    (query (range (start 31 40) (end 31 63)) (probe (position 31 40))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))
        (kind connectionTarget) (ordinal 0) (authored-target "canBus::vehicleControlIF")
        (range (start 31 40) (end 31 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF") (range (start 34 5) (end 34 34)))
        )
      )
    )
    (query (range (start 31 12) (end 31 36)) (probe (position 31 12))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))
        (kind connectionSource) (ordinal 0) (authored-target "vehicleControlUnit::busIF")
        (range (start 31 12) (end 31 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF") (range (start 28 5) (end 28 24)))
        )
      )
    )
    (query (range (start 39 12) (end 39 42)) (probe (position 39 12))
      (reference
        (source (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))
        (kind connectionSource) (ordinal 1) (authored-target "engine::engineControlUnit::busIF")
        (range (start 39 12) (end 39 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF") (range (start 43 6) (end 43 25)))
        )
      )
    )
  )
)
~~~
