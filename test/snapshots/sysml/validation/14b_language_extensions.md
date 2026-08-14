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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 12) (end 31 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 40) (end 31 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 12) (end 39 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 46) (end 39 68))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:152669b1468ad742079f5087fb1f317a7e9f498a7e403168822e1ac070bdb05a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Definitions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LibraryModel") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ECU")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ECU")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicleControlUnit::busIF")) (memberAccessOperand (reference "canBus::vehicleControlIF")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "engine::engineControlUnit::busIF")) (memberAccessOperand (reference "canBus::engineControlIF")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CanBus")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BusIF")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BusIF")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BusIF")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineControlUnit")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BusIF") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleControlUnit")))))
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BusIF") (conjugated true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Definitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LibraryModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0))
      (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0))
      (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicleControlUnit::busIF")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "engine::engineControlUnit::busIF")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "canBus::vehicleControlIF")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "canBus::engineControlIF")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0))
      (authored-target "CanBus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind featureTyping) (ordinal 0))
      (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind featureTyping) (ordinal 0))
      (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind featureTyping) (ordinal 0))
      (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind featureTyping) (ordinal 0))
      (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")))))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind featureTyping) (ordinal 0))
      (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF")) (scopes any))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF")) (scopes any))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF")) (scopes any))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF")) (scopes any))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")) (scopes any))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")) (scopes any))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF")))
      (featured-by (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit")))
      (type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (source direct))
      (supertype (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 11 18) (end 11 33)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Definitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "LibraryModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 14 33) (end 14 36)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit"))) (kind specialization) (ordinal 0) (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 13 34) (end 13 37)) (probe (position 13 34))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit"))) (kind specialization) (ordinal 0) (authored-target "ECU")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::LibraryModel::ECU")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 24 18) (end 24 32)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 26 18) (end 26 25)) (probe (position 26 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 31 12) (end 31 36)) (probe (position 31 12))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicleControlUnit::busIF")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 39 12) (end 39 42)) (probe (position 39 12))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "engine::engineControlUnit::busIF")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 31 40) (end 31 63)) (probe (position 31 40))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "canBus::vehicleControlIF")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 39 46) (end 39 68)) (probe (position 39 46))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (path (named (kind package) (name "14b-Language-Extensions")) (named (kind package) (name "UserModel")) (named (kind package) (name "Usages")) (named (kind part) (name "vehicle1")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "canBus::engineControlIF")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 33 17) (end 33 23)) (probe (position 33 17))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus"))) (kind featureTyping) (ordinal 0) (authored-target "CanBus")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::CanBus")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 35 27) (end 35 32)) (probe (position 35 27))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::engineControlIF"))) (kind featureTyping) (ordinal 0) (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 36 20) (end 36 25)) (probe (position 36 20))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::sensorIF"))) (kind featureTyping) (ordinal 0) (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 34 28) (end 34 33)) (probe (position 34 28))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::canBus::vehicleControlIF"))) (kind featureTyping) (ordinal 0) (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 41 17) (end 41 23)) (probe (position 41 17))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 42 29) (end 42 46)) (probe (position 42 29))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit"))) (kind featureTyping) (ordinal 0) (authored-target "EngineControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::EngineControlUnit")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 43 19) (end 43 24)) (probe (position 43 19))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::engine::engineControlUnit::busIF"))) (kind featureTyping) (ordinal 0) (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 27 30) (end 27 48)) (probe (position 27 30))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleControlUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit")))))
    )
  )
  (query (document "memory://snapshot/14b_language_extensions.md") (range (start 28 18) (end 28 23)) (probe (position 28 18))
    (reference (id (source (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Usages::vehicle1::vehicleControlUnit::busIF"))) (kind featureTyping) (ordinal 0) (authored-target "BusIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/14b_language_extensions.md") (qualified-name "14b-Language-Extensions::UserModel::Definitions::BusIF")))))
    )
  )
)
~~~
