# META
~~~ini
description=Interconnection connectors are grouped into port families: the most specific port definition every end specializes, with related connector types forming one family
type=generate
libraries=standard
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=Families::connections
~~~
# SOURCE
## model.sysml
~~~sysml
package Families {
    private import StandardViewDefinitions::*;

    port def BusPort;
    port def ControllerPort :> BusPort;
    port def TargetPort :> BusPort;
    port def PowerPort;
    port def SensorPort;
    port def ActuatorPort;
    port def SmartSensorPort :> SensorPort, ActuatorPort;
    port def SmartActuatorPort :> SensorPort, ActuatorPort;

    part def Board {
        port bus : ControllerPort;
        port loose;
    }
    part def Controller {
        port bus : ControllerPort;
        port power : PowerPort;
        port smart : SmartSensorPort;
    }
    part def Peripheral {
        port bus : TargetPort;
        port power : ~PowerPort;
        port sense : SensorPort;
        port smart : SmartActuatorPort;
    }
    part def Mount;
    part def System {
        part board : Board;
        part mcu : Controller;
        part dev1 : Peripheral;
        part dev2 : Peripheral;
        part chassis : Mount;
        part cover : Mount;
        // Same type: ControllerPort, which joins the BusPort family.
        connect board.bus to mcu.bus;
        // Sibling subtypes: BusPort. Both connectors on mcu.bus share that family.
        connect mcu.bus to dev1.bus;
        connect mcu.bus to dev2.bus;
        // Conjugation is ignored: PowerPort.
        connect mcu.power to dev1.power;
        // Two most specific common types: SensorPort and ActuatorPort.
        connect mcu.smart to dev1.smart;
        // No family: unrelated ports, parts, and an untyped port.
        connect dev2.sense to mcu.power;
        connect chassis to cover;
        connect board.loose to dev2.sense;
    }
    view connections : InterconnectionView {
        expose System;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d558f07ec0a4739c1316cd8d88c4b469f6fc85acc650397638bfc489548c60db") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "ActuatorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~ActuatorPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControllerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "BusPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~BusPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControllerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SmartSensorPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BusPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "ControllerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~ControllerPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TargetPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SensorPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SmartActuatorPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~PowerPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "SensorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~SensorPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SensorPort")) (specialization (reference "ActuatorPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "SmartActuatorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~SmartActuatorPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SensorPort")) (specialization (reference "ActuatorPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "SmartSensorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~SmartSensorPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "board::bus")) (memberAccessOperand (reference "mcu::bus")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::bus")) (memberAccessOperand (reference "dev1::bus")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::bus")) (memberAccessOperand (reference "dev2::bus")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::power")) (memberAccessOperand (reference "dev1::power")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::smart")) (memberAccessOperand (reference "dev1::smart")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "dev2::sense")) (memberAccessOperand (reference "mcu::power")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "chassis")) (connectorEnd (reference "cover")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "board::loose")) (memberAccessOperand (reference "dev2::sense")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Board")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mount")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mount")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Peripheral")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Peripheral")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BusPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "TargetPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~TargetPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "System")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControllerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControllerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (kind featureTyping) (ordinal 0))
      (authored-target "SmartSensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (kind specialization) (ordinal 0))
      (authored-target "BusPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (kind featureTyping) (ordinal 0))
      (authored-target "TargetPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (kind featureTyping) (ordinal 0))
      (authored-target "SensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (kind featureTyping) (ordinal 0))
      (authored-target "SmartActuatorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind specialization) (ordinal 0))
      (authored-target "SensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind specialization) (ordinal 1))
      (authored-target "ActuatorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind specialization) (ordinal 0))
      (authored-target "SensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind specialization) (ordinal 1))
      (authored-target "ActuatorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connectorEnd) (ordinal 0))
      (authored-target "chassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connectorEnd) (ordinal 1))
      (authored-target "cover")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "board::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::smart")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "dev2::sense")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "board::loose")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "mcu::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "dev1::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "dev2::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "dev1::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "dev1::smart")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "mcu::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "dev2::sense")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (kind featureTyping) (ordinal 0))
      (authored-target "Board")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mount")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mount")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Peripheral")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Peripheral")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (kind specialization) (ordinal 0))
      (authored-target "BusPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind specialization) (ordinal 1)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "ActuatorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "BusPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "ControllerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "SensorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "SmartActuatorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "SmartSensorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "TargetPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind port-def) (name "PowerPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections")))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) "board::bus")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) "mcu::bus")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) "mcu::bus")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) "dev1::bus")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) "mcu::bus")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) "dev2::bus")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) "mcu::power")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) "dev1::power")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) "mcu::smart")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) "dev1::smart")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) "dev2::sense")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) "mcu::power")))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connection) (end bare (feature (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))))) (end bare (feature (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))))))
  (connector (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind connection) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose"))) "board::loose")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2")) (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) "dev2::sense")))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 19) (end 13 33)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus"))) (kind featureTyping) (ordinal 0) (authored-target "ControllerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 19) (end 17 33)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus"))) (kind featureTyping) (ordinal 0) (authored-target "ControllerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 21) (end 18 30)) (probe (position 18 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 21) (end 19 36)) (probe (position 19 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart"))) (kind featureTyping) (ordinal 0) (authored-target "SmartSensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 31) (end 4 38)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ControllerPort"))) (kind specialization) (ordinal 0) (authored-target "BusPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 19) (end 22 29)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus"))) (kind featureTyping) (ordinal 0) (authored-target "TargetPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 22) (end 23 31)) (probe (position 23 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::PowerPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 21) (end 24 31)) (probe (position 24 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense"))) (kind featureTyping) (ordinal 0) (authored-target "SensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 25 21) (end 25 38)) (probe (position 25 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart"))) (kind featureTyping) (ordinal 0) (authored-target "SmartActuatorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 34) (end 10 44)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind specialization) (ordinal 0) (authored-target "SensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 46) (end 10 58)) (probe (position 10 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartActuatorPort"))) (kind specialization) (ordinal 1) (authored-target "ActuatorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 32) (end 9 42)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind specialization) (ordinal 0) (authored-target "SensorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SensorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 44) (end 9 56)) (probe (position 9 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::SmartSensorPort"))) (kind specialization) (ordinal 1) (authored-target "ActuatorPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::ActuatorPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 46 16) (end 46 23)) (probe (position 46 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connectorEnd) (ordinal 0) (authored-target "chassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 46 27) (end 46 32)) (probe (position 46 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 6))))) (kind connectorEnd) (ordinal 1) (authored-target "cover")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 36 16) (end 36 25)) (probe (position 36 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "board::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 38 16) (end 38 23)) (probe (position 38 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 39 16) (end 39 23)) (probe (position 39 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 41 16) (end 41 25)) (probe (position 41 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 43 16) (end 43 25)) (probe (position 43 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::smart")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::smart")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 45 16) (end 45 26)) (probe (position 45 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0) (authored-target "dev2::sense")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 47 16) (end 47 27)) (probe (position 47 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind memberAccessOperand) (ordinal 0) (authored-target "board::loose")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board::loose")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 36 29) (end 36 36)) (probe (position 36 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "mcu::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 38 27) (end 38 35)) (probe (position 38 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "dev1::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 39 27) (end 39 35)) (probe (position 39 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1) (authored-target "dev2::bus")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::bus")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 41 29) (end 41 39)) (probe (position 41 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1) (authored-target "dev1::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 43 29) (end 43 39)) (probe (position 43 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 1) (authored-target "dev1::smart")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::smart")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 45 30) (end 45 39)) (probe (position 45 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 5))))) (kind memberAccessOperand) (ordinal 1) (authored-target "mcu::power")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller::power")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 47 31) (end 47 41)) (probe (position 47 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind part-def) (name "System")) (anonymous (kind bare-connect) (ordinal 7))))) (kind memberAccessOperand) (ordinal 1) (authored-target "dev2::sense")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral::sense")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 29 21) (end 29 26)) (probe (position 29 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::board"))) (kind featureTyping) (ordinal 0) (authored-target "Board")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Board")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 33 23) (end 33 28)) (probe (position 33 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::chassis"))) (kind featureTyping) (ordinal 0) (authored-target "Mount")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 34 21) (end 34 26)) (probe (position 34 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::cover"))) (kind featureTyping) (ordinal 0) (authored-target "Mount")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Mount")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 31 20) (end 31 30)) (probe (position 31 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev1"))) (kind featureTyping) (ordinal 0) (authored-target "Peripheral")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 32 20) (end 32 30)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::dev2"))) (kind featureTyping) (ordinal 0) (authored-target "Peripheral")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Peripheral")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 30 19) (end 30 29)) (probe (position 30 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System::mcu"))) (kind featureTyping) (ordinal 0) (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::Controller")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 27) (end 5 34)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::TargetPort"))) (kind specialization) (ordinal 0) (authored-target "BusPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::BusPort")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 49 23) (end 49 42)) (probe (position 49 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Families::connections"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 50 15) (end 50 21)) (probe (position 50 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Families")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Families::System")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:d64e1dabac271d577052b333960d15723f56ca6c1f56fe30ed45e0ed5e916b59",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        13,
        13,
        13,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        19,
        13,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        13,
        14,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        13,
        17,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        19,
        17,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        13,
        18,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        21,
        18,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        13,
        19,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        21,
        19,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        13,
        22,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        19,
        22,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        13,
        23,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        22,
        23,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        13,
        24,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        21,
        24,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        13,
        25,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        21,
        25,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        28,
        13,
        28,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        13,
        29,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        21,
        29,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        13,
        30,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        19,
        30,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        13,
        31,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        20,
        31,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        13,
        32,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        20,
        32,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        13,
        33,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        23,
        33,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        13,
        34,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        21,
        34,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        8,
        36,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        16,
        36,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        29,
        36,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        8,
        38,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        16,
        38,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        27,
        38,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        8,
        39,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        16,
        39,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        27,
        39,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        8,
        41,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        16,
        41,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        29,
        41,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        43,
        8,
        43,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        43,
        16,
        43,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        43,
        29,
        43,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        45,
        8,
        45,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        45,
        16,
        45,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        45,
        30,
        45,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        46,
        8,
        46,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        46,
        16,
        46,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        46,
        27,
        46,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        47,
        8,
        47,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        47,
        16,
        47,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        47,
        31,
        47,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        49,
        9,
        49,
        20
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::ActuatorPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Board"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Board::bus"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Board::loose"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::BusPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Controller"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Controller::bus"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Controller::power"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Controller::smart"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::ControllerPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Mount"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Peripheral"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Peripheral::bus"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Peripheral::power"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Peripheral::sense"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::Peripheral::smart"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::PowerPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::SensorPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::SmartActuatorPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::SmartSensorPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::board"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::chassis"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::cover"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::dev1"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::dev2"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::System::mcu"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::TargetPort"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Families::connections"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 30,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 36,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 39,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 42,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 45,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 48,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "Families::System",
      "source": 51,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "connection",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "connection",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "connection",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "connection",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "connection",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "connection",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "typing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "connection",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 110,
      "relationshipKind": "connectorEnd",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 111,
      "relationshipKind": "connectorEnd",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 86,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 87,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 92,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 93,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 98,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 99,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 104,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 105,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 116,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 117,
      "relationshipKind": "memberAccessOperand",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "noncomposite-membership",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "noncomposite-membership",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typing",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "connection",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 122,
      "relationshipKind": "typing",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "typing",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "noncomposite-membership",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "noncomposite-membership",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "noncomposite-membership",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "noncomposite-membership",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "noncomposite-membership",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "noncomposite-membership",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "noncomposite-membership",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "noncomposite-membership",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "typing",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 27
    }
  ],
  "selectedView": {
    "reference": 29,
    "kind": "interconnection-view",
    "name": "connections",
    "source": 54
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 9,
        "provenance": "authored",
        "reference": 57,
        "source": 0,
        "target": 9
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 3,
        "origin": 10,
        "provenance": "implied",
        "reference": 103,
        "source": 9,
        "target": 10
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 5,
        "origin": 11,
        "provenance": "implied",
        "reference": 104,
        "source": 9,
        "target": 11
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 7,
        "origin": 12,
        "provenance": "implied",
        "reference": 105,
        "source": 9,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 13,
        "provenance": "authored",
        "reference": 58,
        "source": 0,
        "target": 13
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 9,
        "origin": 14,
        "provenance": "implied",
        "reference": 93,
        "source": 13,
        "target": 14
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 11,
        "origin": 15,
        "provenance": "implied",
        "reference": 94,
        "source": 13,
        "target": 15
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 13,
        "origin": 16,
        "provenance": "implied",
        "reference": 95,
        "source": 13,
        "target": 16
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 15,
        "origin": 17,
        "provenance": "implied",
        "reference": 96,
        "source": 13,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 18,
        "provenance": "authored",
        "reference": 59,
        "source": 0,
        "target": 18
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 9,
        "origin": 19,
        "provenance": "implied",
        "reference": 98,
        "source": 18,
        "target": 19
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 11,
        "origin": 20,
        "provenance": "implied",
        "reference": 99,
        "source": 18,
        "target": 20
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 13,
        "origin": 21,
        "provenance": "implied",
        "reference": 100,
        "source": 18,
        "target": 21
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 15,
        "origin": 22,
        "provenance": "implied",
        "reference": 101,
        "source": 18,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 18,
        "origin": 23,
        "provenance": "authored",
        "reference": 60,
        "source": 0,
        "target": 23
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 24,
        "provenance": "implied",
        "reference": 87,
        "source": 23,
        "target": 24
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 25,
        "provenance": "implied",
        "reference": 88,
        "source": 23,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 28,
        "origin": 26,
        "provenance": "authored",
        "reference": 61,
        "source": 0,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 30,
        "origin": 1,
        "provenance": "authored",
        "reference": 62,
        "source": 0,
        "target": 1
      },
      {
        "kind": "connector",
        "navigation": 31,
        "origin": 1,
        "provenance": "authored",
        "reference": 38,
        "source": 24,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 2,
        "provenance": "authored",
        "reference": 63,
        "source": 0,
        "target": 2
      },
      {
        "kind": "connector",
        "navigation": 34,
        "origin": 2,
        "provenance": "authored",
        "reference": 41,
        "source": 10,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 36,
        "origin": 3,
        "provenance": "authored",
        "reference": 64,
        "source": 0,
        "target": 3
      },
      {
        "kind": "connector",
        "navigation": 37,
        "origin": 3,
        "provenance": "authored",
        "reference": 42,
        "source": 10,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 39,
        "origin": 4,
        "provenance": "authored",
        "reference": 65,
        "source": 0,
        "target": 4
      },
      {
        "kind": "connector",
        "navigation": 40,
        "origin": 4,
        "provenance": "authored",
        "reference": 44,
        "source": 11,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 42,
        "origin": 5,
        "provenance": "authored",
        "reference": 66,
        "source": 0,
        "target": 5
      },
      {
        "kind": "connector",
        "navigation": 43,
        "origin": 5,
        "provenance": "authored",
        "reference": 46,
        "source": 12,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 45,
        "origin": 6,
        "provenance": "authored",
        "reference": 67,
        "source": 0,
        "target": 6
      },
      {
        "kind": "connector",
        "navigation": 46,
        "origin": 6,
        "provenance": "authored",
        "reference": 52,
        "source": 21,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 48,
        "origin": 7,
        "provenance": "authored",
        "reference": 68,
        "source": 0,
        "target": 7
      },
      {
        "kind": "connector",
        "navigation": 49,
        "origin": 7,
        "provenance": "authored",
        "reference": 90,
        "source": 27,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 51,
        "origin": 8,
        "provenance": "authored",
        "reference": 69,
        "source": 0,
        "target": 8
      },
      {
        "kind": "connector",
        "navigation": 52,
        "origin": 8,
        "provenance": "authored",
        "reference": 40,
        "source": 25,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 26,
        "origin": 27,
        "provenance": "authored",
        "reference": 70,
        "source": 0,
        "target": 27
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8
      ],
      "parts": [
        0,
        9,
        13,
        18,
        23,
        26,
        27
      ],
      "portFamilies": [
        {
          "connectors": [
            5
          ],
          "general": [
            {
              "label": "ActuatorPort",
              "reference": 0
            },
            {
              "label": "SensorPort",
              "reference": 17
            }
          ],
          "types": [
            {
              "label": "ActuatorPort",
              "reference": 0
            },
            {
              "label": "SensorPort",
              "reference": 17
            }
          ]
        },
        {
          "connectors": [
            1,
            2,
            3
          ],
          "general": [
            {
              "label": "BusPort",
              "reference": 4
            }
          ],
          "types": [
            {
              "label": "BusPort",
              "reference": 4
            },
            {
              "label": "ControllerPort",
              "reference": 9
            }
          ]
        },
        {
          "connectors": [
            4
          ],
          "general": [
            {
              "label": "PowerPort",
              "reference": 16
            }
          ],
          "types": [
            {
              "label": "PowerPort",
              "reference": 16
            }
          ]
        }
      ],
      "ports": [
        10,
        11,
        12,
        14,
        15,
        16,
        17,
        19,
        20,
        21,
        22,
        24,
        25
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              9,
              13,
              18,
              23,
              26,
              27
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              1,
              2,
              3,
              4,
              5,
              6,
              7,
              8
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "multiplicity": null,
        "name": "System",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:5f031ace25f1eb3db146af5d0aee6261ec6071968e1cd73951427daf317b255f",
        "reference": 20,
        "source": 17,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:c0c96f9395c275db05c3e79f11c7b85dc639507c1baae2578184ae16d3650473",
        "reference": 30,
        "source": 30,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:2b5b508c009c97f20e52f438af73f27116114f9043a0582dfe1f086f7256b9fd",
        "reference": 31,
        "source": 33,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:3ab86d6c894a862c7d0affb86d0d1047349892e50fa7d8511b8e4f139a64285c",
        "reference": 32,
        "source": 36,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:41b43ab65915f2ed0c442207bf8aff9496f594e4120104a1dd56e6589e772fa1",
        "reference": 33,
        "source": 39,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:39e30c52583f5c6d9bbfeb942fd06d0d26d3df5b6bf8c39c211c09327cdfd7f2",
        "reference": 34,
        "source": 42,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:51e08af91b0ee3342112df49c6e26b7444c3993f915cae9868ff1b0211e57a3c",
        "reference": 35,
        "source": 45,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:abe66c4e2050f6e6e00b4338f3104d22ed45b286a3e0308a890f1018147cc92f",
        "reference": 36,
        "source": 48,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ConnectionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:64f0b7f69415b6691d96a0012740fd81c1a10c76f46a6d0c4d24ddd5eebab2f5",
        "reference": 37,
        "source": 51,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              10,
              11,
              12
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "mcu",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:aafff4d3f1bd2d7c2c838dc80377ebaaf28713547b92fbfae262101b5aa43ca2",
        "reference": 27,
        "source": 20,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Controller",
              "reference": 5
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "bus",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:77be8f4ca25f84ad7969dea1e2372c391487537ea58737a6178abbf6a0615a06",
        "reference": 6,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ControllerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "power",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:015f8f474973650d136996723d4938499f35859fdfa78d03c62818348b81ec72",
        "reference": 7,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "smart",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:649d4bb5e94175fbe021c0a8dd56eaedab5f31e3276f10734f70985760d4a843",
        "reference": 8,
        "source": 7,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "SmartSensorPort",
              "reference": 19
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              14,
              15,
              16,
              17
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "dev1",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:28205323c24569058c2eaebac774dfc1401eca45965f65ea1d62f1002e331912",
        "reference": 25,
        "source": 22,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Peripheral",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "bus",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:05daafb3c1b167fce5cf98394d0d4cbd77a824746e8b3c935bc2e6d76eb1b211",
        "reference": 12,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "TargetPort",
              "reference": 28
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "power",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:d91a08cd17972d223c75a8206119aa1f9bcb6e330e346b3144a3d25ddafc3330",
        "reference": 13,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "sense",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:d23d08d6d6b3e806353e32f5d814e6838ad334e4ea6376d63c64927c8e12593f",
        "reference": 14,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "SensorPort",
              "reference": 17
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "smart",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:dbbc529409ab4c63e1e79ab6a04e14499cde04700d4dcca42bdbad5d94ae0d3d",
        "reference": 15,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "SmartActuatorPort",
              "reference": 18
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              19,
              20,
              21,
              22
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "dev2",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:51e9e0e8307dc6a105defb5b41b4191c6cc48c9c6cb88ccf87d624eaf1ae2c26",
        "reference": 26,
        "source": 24,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Peripheral",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "bus",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:58793aa61810a6bc80224c0329b440515950b00b52a4c370288f0b9f0d910d1f",
        "reference": 12,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "TargetPort",
              "reference": 28
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "power",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:feeea3b61cbdf7e614d255a3c81f4889258ba9c5babe7d997539f22f0e7e3bd6",
        "reference": 13,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "PowerPort",
              "reference": 16
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "sense",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:c12791d4c6667d80d3ad5d696fe3030c6a9e10bfddf967768213347661bc7329",
        "reference": 14,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "SensorPort",
              "reference": 17
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "smart",
        "notationRole": "usage",
        "owner": 18,
        "presentationKey": "p:04f08f69433010bcfcf10ecfd4ae87168ede4e0c3a5000a336d92ae7f89bd579",
        "reference": 15,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "SmartActuatorPort",
              "reference": 18
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              24,
              25
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "board",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:2191f0e36d335a2266425f9ff01dae167ebaa6bd6ec23ef9ae3737e4939a4c2f",
        "reference": 22,
        "source": 18,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Board",
              "reference": 1
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "bus",
        "notationRole": "usage",
        "owner": 23,
        "presentationKey": "p:33459f65e98863c5bace10fd809707ca703a8d3d60b8eaf2e03f370fc90a54c3",
        "reference": 2,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ControllerPort",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "loose",
        "notationRole": "usage",
        "owner": 23,
        "presentationKey": "p:5519459f4023e5f23042e71477e36bacadd1de0c288df7e86e9b94235d9136bf",
        "reference": 3,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "cover",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e1ae0ea70b1aef31f8b9ce76fde20eab920b4521aa6785e14a954a8b79f9bccb",
        "reference": 24,
        "source": 28,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Mount",
              "reference": 10
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "chassis",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:1849ef6c4e3cd92670c66b30c3dc3f5f1466fc38b3c05f76bf92854acd78c10e",
        "reference": 23,
        "source": 26,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Mount",
              "reference": 10
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 21,
        "provenance": "authored",
        "reference": 106,
        "source": 9,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 43,
        "source": 10,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 45,
        "source": 11,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 47,
        "source": 12,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 23,
        "provenance": "authored",
        "reference": 97,
        "source": 13,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 48,
        "source": 14,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 50,
        "source": 15,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 53,
        "source": 16,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 55,
        "source": 17,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 25,
        "provenance": "authored",
        "reference": 102,
        "source": 18,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 49,
        "source": 19,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 51,
        "source": 20,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 14,
        "provenance": "authored",
        "reference": 54,
        "source": 21,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 16,
        "provenance": "authored",
        "reference": 56,
        "source": 22,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 19,
        "provenance": "authored",
        "reference": 89,
        "source": 23,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 39,
        "source": 24,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 29,
        "provenance": "authored",
        "reference": 92,
        "source": 26,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 31,
        "provenance": "authored",
        "reference": 73,
        "source": 1,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 32,
        "provenance": "authored",
        "reference": 74,
        "source": 1,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 34,
        "provenance": "authored",
        "reference": 75,
        "source": 2,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 35,
        "provenance": "authored",
        "reference": 76,
        "source": 2,
        "target": {
          "node": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 37,
        "provenance": "authored",
        "reference": 77,
        "source": 3,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 38,
        "provenance": "authored",
        "reference": 78,
        "source": 3,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 40,
        "provenance": "authored",
        "reference": 79,
        "source": 4,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 41,
        "provenance": "authored",
        "reference": 80,
        "source": 4,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 43,
        "provenance": "authored",
        "reference": 81,
        "source": 5,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 44,
        "provenance": "authored",
        "reference": 82,
        "source": 5,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 46,
        "provenance": "authored",
        "reference": 83,
        "source": 6,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 47,
        "provenance": "authored",
        "reference": 84,
        "source": 6,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 49,
        "provenance": "authored",
        "reference": 71,
        "source": 7,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 50,
        "provenance": "authored",
        "reference": 72,
        "source": 7,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 52,
        "provenance": "authored",
        "reference": 85,
        "source": 8,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 53,
        "provenance": "authored",
        "reference": 86,
        "source": 8,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 27,
        "provenance": "authored",
        "reference": 91,
        "source": 27,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
