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
  (document "memory://snapshot/14b_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 3) (end 20 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 28 5) (end 28 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 31 4) (end 31 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 34 5) (end 34 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 35 5) (end 35 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 36 5) (end 36 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 39 4) (end 39 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 43 6) (end 43 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:152669b1468ad742079f5087fb1f317a7e9f498a7e403168822e1ac070bdb05a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LibraryModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ECU"))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ECU"))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CanBus"))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineControlUnit"))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleControlUnit"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LibraryModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0))
      (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0))
      (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0))
      (authored-target "CanBus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 11 18) (end 11 33)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "LibraryModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 14 33) (end 14 36)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0) (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 13 34) (end 13 37)) (probe (position 13 34))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0) (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 24 18) (end 24 32)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 26 18) (end 26 25)) (probe (position 26 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 33 17) (end 33 23)) (probe (position 33 17))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0) (authored-target "CanBus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 41 17) (end 41 23)) (probe (position 41 17))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 42 29) (end 42 46)) (probe (position 42 29))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0) (authored-target "EngineControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")))))
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 27 30) (end 27 48)) (probe (position 27 30))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")))))
  )
)
~~~
