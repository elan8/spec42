# META
~~~ini
description=Geometry View over the realistic timer repository example with explicit geometry incompleteness
type=generate
libraries=standard
repositorySources=examples/timer/KitchenTimer.sysml,examples/timer/KitchenTimerBehavior.sysml,examples/timer/KitchenTimerPorts.sysml,examples/timer/KitchenTimerRequirements.sysml,examples/timer/KitchenTimerStructure.sysml
plugin=repository:diagram
viewKind=geometry-view
viewDocument=diagram_timer_geometry.md
viewQualifiedName=TimerGeometry::selected
~~~
# SOURCE
~~~sysml
package TimerGeometry {
    private import StandardViewDefinitions::*;
    import KitchenTimer::*;
    view selected : GeometryView { expose KitchenTimer::timerInstance; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_timer_geometry.md"
    (diagnostics
    )
  )
  (document "memory://snapshot/examples/timer/KitchenTimer.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:dc8b6a3b95fc4626c5d1b82eddd731a7c746a34558632131d60a1da3b708b1e6") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimer") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeometryView")))))
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "KitchenTimer::timerInstance")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " Root package for the kitchen timer teaching example. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerPorts") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerStructure") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerBehavior") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerRequirements") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "TimerRangeReq")) (memberAccessOperand (reference "timerInstance::pcb::mcu")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "DisplayFormatReq")) (memberAccessOperand (reference "timerInstance::pcb::display")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "AccuracyReq")) (memberAccessOperand (reference "timerInstance::pcb::mcu")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "BuzzerAudibilityReq")) (memberAccessOperand (reference "timerInstance::buzzer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "ButtonResponsivenessReq")) (memberAccessOperand (reference "timerInstance::pcb::mcu")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "StateConsistencyReq")) (memberAccessOperand (reference "timerInstance::pcb::mcu")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "BatteryRuntimeReq")) (memberAccessOperand (reference "timerInstance::battery")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KitchenTimer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::CountdownComplete"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::DecrementPressed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired"))) (kind state-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Countdown reached zero; buzzer on. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle"))) (kind state-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Timer not running; user can set time via +/-. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::IncrementPressed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused"))) (kind state-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Countdown suspended; can resume or reset. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running"))) (kind state-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Countdown in progress. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StopPressed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (kind state-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Top-level: Idle, Running, Paused, Expired. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "idle")) (transitionTrigger (reference "DecrementPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Expired")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Idle")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "idle")) (transitionTrigger (reference "IncrementPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Paused")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Running")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "expired")) (transitionTrigger (reference "CountdownComplete")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "expired")) (transitionTarget (reference "idle")) (transitionTrigger (reference "ResetPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "paused")) (transitionTarget (reference "idle")) (transitionTrigger (reference "ResetPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "paused")) (transitionTrigger (reference "StopPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")) (transitionTrigger (reference "StartPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "paused")) (transitionTarget (reference "running")) (transitionTrigger (reference "StartPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::volt") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::ampere") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQElectromagnetism::electricPower") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (kind port-def) (membership (kind owning) (visibility default)) (documentation (doc (text " DC power source; e.g. 3xAAA = 4.5 V, max 0.5 A. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "electricPower") (direction out)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction out)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String") (direction out)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort"))) (kind port-def) (membership (kind owning) (visibility default)) (documentation (doc (text " COM/SEG lines from MCU LCD controller to segment LCD glass; multiplexed drive. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String") (direction out)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerStructure") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQBase::DurationValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQBase::ElectricCurrentValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQElectromagnetism::ElectricChargeValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The controller shall keep elapsed-time error within +/-1 second per minute. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "AccuracyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " Tick handling and countdown accumulation stay within the stated timing tolerance. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Microcontroller")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind constraint-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Illustrative parametric support for BatteryRuntimeReq; charge / current gives duration. "))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "runtime")) (expressionOperand (reference "capacity")) (expressionOperand (reference "loadCurrent")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricChargeValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricCurrentValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The battery shall support a runtime estimate of at least 100 hours for typical use. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BatteryRuntimeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " The modeled runtime estimate meets or exceeds the target usage duration. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Battery")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The controller shall acknowledge button events within 100 ms. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "ButtonResponsivenessReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " Input handling keeps button-to-response latency at or below 100 ms. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Microcontroller")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The buzzer shall provide an audible alarm for a typical kitchen environment. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BuzzerAudibilityReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " The buzzer output is intended to be noticeable when the timer expires. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Buzzer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The display shall present the countdown value in MM:SS format. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "DisplayFormatReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " The visible timer presentation uses a two-field minutes-and-seconds display. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Display")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm"))) (kind use-case-def) (membership (kind owning) (visibility default)) (documentation (doc (text " When countdown expires, buzzer sounds; user hears alarm. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Audible alarm on expiration. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KitchenTimer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume"))) (kind use-case-def) (membership (kind owning) (visibility default)) (documentation (doc (text " User pauses via Stop, resumes via Start. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Pause and resume countdown. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KitchenTimer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart"))) (kind use-case-def) (membership (kind owning) (visibility default)) (documentation (doc (text " User presses Start with preset (e.g., 5 min) without changing value. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Start with preset duration. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KitchenTimer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset"))) (kind use-case-def) (membership (kind owning) (visibility default)) (documentation (doc (text " User presses Reset to return to Idle and clear. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Return to idle state and clear set value. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KitchenTimer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer"))) (kind use-case-def) (membership (kind owning) (visibility default)) (documentation (doc (text " User sets time via +/-, then presses Start; countdown begins. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Start countdown with user-selected duration. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KitchenTimer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "User")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The controller state machine shall remain in one mutually exclusive timer mode at a time. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "StateConsistencyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " Idle, Running, Paused, and Expired remain mutually exclusive controller states. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Microcontroller")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (kind constraint-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Illustrative parametric support for AccuracyReq. "))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "errorBound")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The controller shall support a configurable countdown range from 0:01 to 99:59. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "TimerRangeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " The configured countdown value stays within the supported timer range. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Microcontroller")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerPorts") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerBehavior") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQBase::DurationValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQElectromagnetism::ElectricChargeValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQElectromagnetism::ElectricPotentialDifferenceValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::FrequencyValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Power supply for all subsystems; e.g. 3xAAA. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricChargeValue")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ElectricPotentialDifferenceValue")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Captures user input; debounces buttons. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ButtonInputPort")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "maxCurrent")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Piezo or similar; has only +/- terminals; buzzes when power is applied. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "maxCurrent")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Transistor or FET; switches battery power to buzzer when MCU GPIO is high. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BuzzerCommandPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "maxCurrent")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Segment LCD glass; COM/SEG from MCU LCD controller; shows MM:SS. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DisplayCommandPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LcdSegmentDrivePort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "maxCurrent")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Battery-powered kitchen timer; PCB with MCU, display, buttons, buzzer driver; buzzer off-board. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "battery::powerOut")) (memberAccessOperand (reference "pcb::mcu::pwr")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "battery::powerOut")) (memberAccessOperand (reference "pcb::display::pwr")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "battery::powerOut")) (memberAccessOperand (reference "pcb::buttons::pwr")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "battery::powerOut")) (memberAccessOperand (reference "pcb::buzzerDriver::pwrIn")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "pcb::buzzerDriver::buzzerPwrOut")) (memberAccessOperand (reference "buzzer::pwr")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Battery")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Buzzer")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimerPCB")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " MCU on PCB with built-in LCD controller; runs timer firmware; COM/SEG to display glass, GPIO to buttons. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ButtonInputPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BuzzerCommandPort")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FrequencyValue")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DisplayCommandPort")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LcdSegmentDrivePort")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutlet") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "maxCurrent")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimerStateMachine")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " PCB assembly; display and buttons mounted on board; MCU and buzzer driver. "))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "buttons::output")) (memberAccessOperand (reference "mcu::buttonIn")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::displayOut")) (memberAccessOperand (reference "display::cmd")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::lcdDrive")) (memberAccessOperand (reference "display::lcdIn")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "mcu::buzzerOut")) (memberAccessOperand (reference "buzzerDriver::ctrlIn")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ButtonInterface")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BuzzerDriver")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Display")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Microcontroller")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeometryView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "KitchenTimer::timerInstance")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerPorts")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerStructure")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerBehavior")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "TimerRangeReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0))
      (authored-target "DisplayFormatReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0))
      (authored-target "AccuracyReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfySource) (ordinal 0))
      (authored-target "BuzzerAudibilityReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind satisfySource) (ordinal 0))
      (authored-target "ButtonResponsivenessReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind satisfySource) (ordinal 0))
      (authored-target "StateConsistencyReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind satisfySource) (ordinal 0))
      (authored-target "BatteryRuntimeReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::pcb::display")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::buzzer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timerInstance::battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (kind featureTyping) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "DecrementPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::DecrementPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (kind featureTyping) (ordinal 0))
      (authored-target "Expired")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "IncrementPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::IncrementPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (kind featureTyping) (ordinal 0))
      (authored-target "Paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (kind featureTyping) (ordinal 0))
      (authored-target "Running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionSource) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionTarget) (ordinal 0))
      (authored-target "expired")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "CountdownComplete")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::CountdownComplete")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionSource) (ordinal 0))
      (authored-target "expired")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "ResetPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionSource) (ordinal 0))
      (authored-target "paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "ResetPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionSource) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionTarget) (ordinal 0))
      (authored-target "paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "StopPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StopPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "StartPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionSource) (ordinal 0))
      (authored-target "paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "StartPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::volt")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::volt")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::ampere")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQElectromagnetism::electricPower")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "electricPower")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerStructure")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQBase::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQBase::ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQElectromagnetism::ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 0))
      (authored-target "runtime")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 1))
      (authored-target "capacity")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 2))
      (authored-target "loadCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery"))) (kind featureTyping) (ordinal 0))
      (authored-target "Battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Buzzer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display"))) (kind featureTyping) (ordinal 0))
      (authored-target "Display")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer"))) (kind featureTyping) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer"))) (kind featureTyping) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer"))) (kind featureTyping) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer"))) (kind featureTyping) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer"))) (kind featureTyping) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (kind featureTyping) (ordinal 0))
      (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "errorBound")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerPorts")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerBehavior")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQBase::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQElectromagnetism::ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQElectromagnetism::ElectricPotentialDifferenceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::FrequencyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricPotentialDifferenceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "ButtonInputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (kind featureTyping) (ordinal 0))
      (authored-target "BuzzerCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "DisplayCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (kind featureTyping) (ordinal 0))
      (authored-target "LcdSegmentDrivePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "pcb::buzzerDriver::buzzerPwrOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "pcb::mcu::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "pcb::display::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "pcb::buttons::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "pcb::buzzerDriver::pwrIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "buzzer::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (kind featureTyping) (ordinal 0))
      (authored-target "Battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Buzzer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimerPCB")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (kind featureTyping) (ordinal 0))
      (authored-target "ButtonInputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (kind featureTyping) (ordinal 0))
      (authored-target "BuzzerCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (kind featureTyping) (ordinal 0))
      (authored-target "FrequencyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (kind featureTyping) (ordinal 0))
      (authored-target "DisplayCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (kind featureTyping) (ordinal 0))
      (authored-target "LcdSegmentDrivePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimerStateMachine")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "buttons::output")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::displayOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::lcdDrive")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "mcu::buzzerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "mcu::buttonIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "display::cmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "display::lcdIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "buzzerDriver::ctrlIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (kind featureTyping) (ordinal 0))
      (authored-target "ButtonInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (kind featureTyping) (ordinal 0))
      (authored-target "BuzzerDriver")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (kind featureTyping) (ordinal 0))
      (authored-target "Display")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (kind featureTyping) (ordinal 0))
      (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::DecrementPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::IncrementPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::CountdownComplete"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StopPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (target (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.5))) (unit "A")))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage"))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 4.5))) (unit "V")))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.001))) (unit "A")))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.05))) (unit "A")))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.001))) (unit "A")))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.01))) (unit "A")))
    (evaluated (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.02))) (unit "A")))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (ordinal 0) (authored "A") (start 10 30) (end 10 31) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage"))) (ordinal 0) (authored "V") (start 9 27) (end 9 28) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::volt"))) (dimension (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit")))))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (ordinal 0) (authored "A") (start 41 54) (end 41 55) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (ordinal 0) (authored "A") (start 47 53) (end 47 54) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (ordinal 0) (authored "A") (start 26 56) (end 26 57) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (ordinal 0) (authored "A") (start 35 53) (end 35 54) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (unit (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (ordinal 0) (authored "A") (start 19 53) (end 19 54) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit")))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_mechanics.md") (qualified-name "ISQMechanics::PowerUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyUnit"))))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))
      (type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_mechanics.md") (qualified-name "ISQMechanics::PowerValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "AccuracyReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate")))
      (type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BatteryRuntimeReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "ButtonResponsivenessReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BuzzerAudibilityReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "DisplayFormatReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "StateConsistencyReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "TimerRangeReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_timer_geometry.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_timer_geometry.md") (range (start 2 11) (end 2 26)) (probe (position 2 11))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/diagram_timer_geometry.md") (range (start 3 20) (end 3 32)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (qualified-name "TimerGeometry::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeometryView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView")))))
    )
  )
  (query (document "memory://snapshot/diagram_timer_geometry.md") (range (start 3 42) (end 3 69)) (probe (position 3 42))
    (reference (id (source (node (document "memory://snapshot/diagram_timer_geometry.md") (path (named (kind package) (name "TimerGeometry")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "KitchenTimer::timerInstance")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 6 8) (end 6 28)) (probe (position 6 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerPorts")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 7 8) (end 7 32)) (probe (position 7 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerStructure")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 8 8) (end 8 31)) (probe (position 8 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerBehavior")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 9 8) (end 9 35)) (probe (position 9 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 13 9) (end 13 22)) (probe (position 13 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "TimerRangeReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 14 9) (end 14 25)) (probe (position 14 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0) (authored-target "DisplayFormatReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 15 9) (end 15 20)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind satisfySource) (ordinal 0) (authored-target "AccuracyReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 16 9) (end 16 28)) (probe (position 16 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind satisfySource) (ordinal 0) (authored-target "BuzzerAudibilityReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 17 9) (end 17 32)) (probe (position 17 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind satisfySource) (ordinal 0) (authored-target "ButtonResponsivenessReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 18 9) (end 18 28)) (probe (position 18 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind satisfySource) (ordinal 0) (authored-target "StateConsistencyReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 19 9) (end 19 26)) (probe (position 19 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind satisfySource) (ordinal 0) (authored-target "BatteryRuntimeReq")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 13 26) (end 13 47)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 14 29) (end 14 54)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::pcb::display")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 15 24) (end 15 45)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 16 32) (end 16 52)) (probe (position 16 32))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::buzzer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 17 36) (end 17 57)) (probe (position 17 36))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 18 32) (end 18 53)) (probe (position 18 32))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::pcb::mcu")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 19 30) (end 19 51)) (probe (position 19 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (kind memberAccessOperand) (ordinal 0) (authored-target "timerInstance::battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (range (start 11 22) (end 11 34)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (kind featureTyping) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 29 7) (end 29 11)) (probe (position 29 7))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 36 34) (end 36 38)) (probe (position 36 34))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 36 68) (end 36 72)) (probe (position 36 68))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 36 46) (end 36 62)) (probe (position 36 46))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (kind transitionTrigger) (ordinal 0) (authored-target "DecrementPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::DecrementPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 33 18) (end 33 25)) (probe (position 33 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (kind featureTyping) (ordinal 0) (authored-target "Expired")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 30 15) (end 30 19)) (probe (position 30 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (kind featureTyping) (ordinal 0) (authored-target "Idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 35 34) (end 35 38)) (probe (position 35 34))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 35 68) (end 35 72)) (probe (position 35 68))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 35 46) (end 35 62)) (probe (position 35 46))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transitionTrigger) (ordinal 0) (authored-target "IncrementPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::IncrementPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 32 17) (end 32 23)) (probe (position 32 17))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (kind featureTyping) (ordinal 0) (authored-target "Paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 31 18) (end 31 25)) (probe (position 31 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (kind featureTyping) (ordinal 0) (authored-target "Running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 38 30) (end 38 37)) (probe (position 38 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionSource) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 38 68) (end 38 75)) (probe (position 38 68))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionTarget) (ordinal 0) (authored-target "expired")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 38 45) (end 38 62)) (probe (position 38 45))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transitionTrigger) (ordinal 0) (authored-target "CountdownComplete")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::CountdownComplete")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 41 40) (end 41 47)) (probe (position 41 40))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionSource) (ordinal 0) (authored-target "expired")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 41 73) (end 41 77)) (probe (position 41 73))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 41 55) (end 41 67)) (probe (position 41 55))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transitionTrigger) (ordinal 0) (authored-target "ResetPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 40 39) (end 40 45)) (probe (position 40 39))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionSource) (ordinal 0) (authored-target "paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 40 71) (end 40 75)) (probe (position 40 71))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 40 53) (end 40 65)) (probe (position 40 53))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transitionTrigger) (ordinal 0) (authored-target "ResetPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 37 29) (end 37 36)) (probe (position 37 29))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionSource) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 37 61) (end 37 67)) (probe (position 37 61))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionTarget) (ordinal 0) (authored-target "paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 37 44) (end 37 55)) (probe (position 37 44))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transitionTrigger) (ordinal 0) (authored-target "StopPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StopPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 34 30) (end 34 34)) (probe (position 34 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 34 60) (end 34 67)) (probe (position 34 60))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 34 42) (end 34 54)) (probe (position 34 42))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transitionTrigger) (ordinal 0) (authored-target "StartPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 39 37) (end 39 43)) (probe (position 39 37))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionSource) (ordinal 0) (authored-target "paused")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 39 69) (end 39 76)) (probe (position 39 69))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (range (start 39 51) (end 39 63)) (probe (position 39 51))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transitionTrigger) (ordinal 0) (authored-target "StartPressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 2 16) (end 2 24)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SI::volt")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::volt")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 3 16) (end 3 26)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "SI::ampere")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::ampere")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 4 16) (end 4 50)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (path (named (kind package) (name "KitchenTimerPorts")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ISQElectromagnetism::electricPower")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 8 14) (end 8 27)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (kind featureTyping) (ordinal 0) (authored-target "electricPower")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 18 24) (end 18 31)) (probe (position 18 24))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 17 24) (end 17 31)) (probe (position 17 24))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 16 20) (end 16 27)) (probe (position 16 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 14 20) (end 14 27)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 15 19) (end 15 26)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 31 17) (end 31 24)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 22 21) (end 22 27)) (probe (position 22 21))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (range (start 27 20) (end 27 26)) (probe (position 27 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 1 8) (end 1 32)) (probe (position 1 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerStructure")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 4 16) (end 4 38)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ISQBase::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 5 16) (end 5 45)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ISQBase::ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 6 16) (end 6 56)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ISQElectromagnetism::ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 21 16) (end 21 31)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu"))) (kind featureTyping) (ordinal 0) (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 100 2) (end 100 9)) (probe (position 100 2))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 0) (authored-target "runtime")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 100 13) (end 100 21)) (probe (position 100 13))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 1) (authored-target "capacity")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 100 24) (end 100 35)) (probe (position 100 24))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (kind expressionOperand) (ordinal 2) (authored-target "loadCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 97 16) (end 97 35)) (probe (position 97 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 98 19) (end 98 39)) (probe (position 98 19))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricCurrentValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::ElectricCurrentValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 99 15) (end 99 28)) (probe (position 99 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 45 20) (end 45 27)) (probe (position 45 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery"))) (kind featureTyping) (ordinal 0) (authored-target "Battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 33 16) (end 33 31)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu"))) (kind featureTyping) (ordinal 0) (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 27 19) (end 27 25)) (probe (position 27 19))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer"))) (kind featureTyping) (ordinal 0) (authored-target "Buzzer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 15 20) (end 15 27)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display"))) (kind featureTyping) (ordinal 0) (authored-target "Display")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 74 18) (end 74 30)) (probe (position 74 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer"))) (kind featureTyping) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 75 15) (end 75 19)) (probe (position 75 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 60 18) (end 60 30)) (probe (position 60 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer"))) (kind featureTyping) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 61 15) (end 61 19)) (probe (position 61 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 81 18) (end 81 30)) (probe (position 81 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer"))) (kind featureTyping) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 82 15) (end 82 19)) (probe (position 82 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 67 18) (end 67 30)) (probe (position 67 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer"))) (kind featureTyping) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 68 15) (end 68 19)) (probe (position 68 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 53 18) (end 53 30)) (probe (position 53 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer"))) (kind featureTyping) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 54 15) (end 54 19)) (probe (position 54 15))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (kind featureTyping) (ordinal 0) (authored-target "User")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 39 16) (end 39 31)) (probe (position 39 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu"))) (kind featureTyping) (ordinal 0) (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 92 2) (end 92 12)) (probe (position 92 2))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (kind expressionOperand) (ordinal 0) (authored-target "errorBound")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 90 19) (end 90 32)) (probe (position 90 19))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 91 18) (end 91 22)) (probe (position 91 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 89 16) (end 89 29)) (probe (position 89 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (range (start 9 16) (end 9 31)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu"))) (kind featureTyping) (ordinal 0) (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 1 8) (end 1 28)) (probe (position 1 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerPorts")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 2 8) (end 2 31)) (probe (position 2 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerBehavior")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 4 16) (end 4 31)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 5 16) (end 5 38)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ISQBase::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 6 16) (end 6 56)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ISQElectromagnetism::ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 7 16) (end 7 69)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ISQElectromagnetism::ElectricPotentialDifferenceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 8 16) (end 8 44)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::FrequencyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 64 23) (end 64 42)) (probe (position 64 23))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricChargeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 65 29) (end 65 61)) (probe (position 65 29))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricPotentialDifferenceValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 67 18) (end 67 31)) (probe (position 67 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 66 30) (end 66 43)) (probe (position 66 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 40 16) (end 40 31)) (probe (position 40 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (kind featureTyping) (ordinal 0) (authored-target "ButtonInputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 41 14) (end 41 27)) (probe (position 41 14))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 41 34) (end 41 44)) (probe (position 41 34))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 46 23) (end 46 36)) (probe (position 46 23))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 47 14) (end 47 27)) (probe (position 47 14))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 47 34) (end 47 44)) (probe (position 47 34))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 27 22) (end 27 35)) (probe (position 27 22))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 25 17) (end 25 34)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (kind featureTyping) (ordinal 0) (authored-target "BuzzerCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 26 16) (end 26 29)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 26 36) (end 26 46)) (probe (position 26 36))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 33 14) (end 33 32)) (probe (position 33 14))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (kind featureTyping) (ordinal 0) (authored-target "DisplayCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 32 21) (end 32 27)) (probe (position 32 21))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 34 16) (end 34 35)) (probe (position 34 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (kind featureTyping) (ordinal 0) (authored-target "LcdSegmentDrivePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 35 14) (end 35 27)) (probe (position 35 14))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 35 34) (end 35 44)) (probe (position 35 34))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 76 10) (end 76 26)) (probe (position 76 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 77 10) (end 77 26)) (probe (position 77 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 78 10) (end 78 26)) (probe (position 78 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 79 10) (end 79 26)) (probe (position 79 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "battery::powerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 80 10) (end 80 39)) (probe (position 80 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0) (authored-target "pcb::buzzerDriver::buzzerPwrOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 76 30) (end 76 41)) (probe (position 76 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "pcb::mcu::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 77 30) (end 77 45)) (probe (position 77 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "pcb::display::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 78 30) (end 78 45)) (probe (position 78 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1) (authored-target "pcb::buttons::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 79 30) (end 79 52)) (probe (position 79 30))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1) (authored-target "pcb::buzzerDriver::pwrIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 80 43) (end 80 53)) (probe (position 80 43))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (kind memberAccessOperand) (ordinal 1) (authored-target "buzzer::pwr")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 73 17) (end 73 24)) (probe (position 73 17))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (kind featureTyping) (ordinal 0) (authored-target "Battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 74 16) (end 74 22)) (probe (position 74 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (kind featureTyping) (ordinal 0) (authored-target "Buzzer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 72 13) (end 72 21)) (probe (position 72 13))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (kind featureTyping) (ordinal 0) (authored-target "TimerPCB")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 15 19) (end 15 34)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (kind featureTyping) (ordinal 0) (authored-target "ButtonInputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 18 19) (end 18 36)) (probe (position 18 19))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (kind featureTyping) (ordinal 0) (authored-target "BuzzerCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 12 29) (end 12 43)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (kind featureTyping) (ordinal 0) (authored-target "FrequencyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 16 20) (end 16 38)) (probe (position 16 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (kind featureTyping) (ordinal 0) (authored-target "DisplayCommandPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 13 24) (end 13 28)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 17 18) (end 17 37)) (probe (position 17 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (kind featureTyping) (ordinal 0) (authored-target "LcdSegmentDrivePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 19 14) (end 19 27)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 19 34) (end 19 44)) (probe (position 19 34))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "maxCurrent")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 14 22) (end 14 26)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 20 28) (end 20 45)) (probe (position 20 28))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (kind featureTyping) (ordinal 0) (authored-target "TimerStateMachine")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 56 10) (end 56 24)) (probe (position 56 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "buttons::output")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 57 10) (end 57 24)) (probe (position 57 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::displayOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 58 10) (end 58 22)) (probe (position 58 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::lcdDrive")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 59 10) (end 59 23)) (probe (position 59 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "mcu::buzzerOut")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 56 28) (end 56 40)) (probe (position 56 28))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "mcu::buttonIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 57 28) (end 57 39)) (probe (position 57 28))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "display::cmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 58 26) (end 58 39)) (probe (position 58 26))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (kind memberAccessOperand) (ordinal 1) (authored-target "display::lcdIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 59 27) (end 59 46)) (probe (position 59 27))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1) (authored-target "buzzerDriver::ctrlIn")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 54 17) (end 54 32)) (probe (position 54 17))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (kind featureTyping) (ordinal 0) (authored-target "ButtonInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 55 22) (end 55 34)) (probe (position 55 22))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (kind featureTyping) (ordinal 0) (authored-target "BuzzerDriver")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 53 17) (end 53 24)) (probe (position 53 17))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (kind featureTyping) (ordinal 0) (authored-target "Display")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (range (start 52 13) (end 52 28)) (probe (position 52 13))
    (reference (id (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (kind featureTyping) (ordinal 0) (authored-target "Microcontroller")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:053e0d48cf420d56d4e4fa3d30c992fe6f093f648f6d3dea5d74091fba48200d",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_timer_geometry.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/examples/timer/KitchenTimer.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/examples/timer/KitchenTimerPorts.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/examples/timer/KitchenTimerStructure.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/isq_base.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/isq_electromagnetism.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/isq_space_time.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/scalar_values.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        9,
        3,
        17
      ]
    },
    {
      "document": 1,
      "range": [
        11,
        6,
        11,
        19
      ]
    },
    {
      "document": 1,
      "range": [
        11,
        22,
        11,
        34
      ]
    },
    {
      "document": 2,
      "range": [
        29,
        2,
        29,
        12
      ]
    },
    {
      "document": 2,
      "range": [
        29,
        7,
        29,
        11
      ]
    },
    {
      "document": 2,
      "range": [
        30,
        8,
        30,
        12
      ]
    },
    {
      "document": 2,
      "range": [
        30,
        15,
        30,
        19
      ]
    },
    {
      "document": 2,
      "range": [
        31,
        8,
        31,
        15
      ]
    },
    {
      "document": 2,
      "range": [
        31,
        18,
        31,
        25
      ]
    },
    {
      "document": 2,
      "range": [
        32,
        8,
        32,
        14
      ]
    },
    {
      "document": 2,
      "range": [
        32,
        17,
        32,
        23
      ]
    },
    {
      "document": 2,
      "range": [
        33,
        8,
        33,
        15
      ]
    },
    {
      "document": 2,
      "range": [
        33,
        18,
        33,
        25
      ]
    },
    {
      "document": 2,
      "range": [
        34,
        13,
        34,
        23
      ]
    },
    {
      "document": 2,
      "range": [
        34,
        30,
        34,
        34
      ]
    },
    {
      "document": 2,
      "range": [
        34,
        42,
        34,
        54
      ]
    },
    {
      "document": 2,
      "range": [
        34,
        60,
        34,
        67
      ]
    },
    {
      "document": 2,
      "range": [
        35,
        13,
        35,
        27
      ]
    },
    {
      "document": 2,
      "range": [
        35,
        34,
        35,
        38
      ]
    },
    {
      "document": 2,
      "range": [
        35,
        46,
        35,
        62
      ]
    },
    {
      "document": 2,
      "range": [
        35,
        68,
        35,
        72
      ]
    },
    {
      "document": 2,
      "range": [
        36,
        13,
        36,
        27
      ]
    },
    {
      "document": 2,
      "range": [
        36,
        34,
        36,
        38
      ]
    },
    {
      "document": 2,
      "range": [
        36,
        46,
        36,
        62
      ]
    },
    {
      "document": 2,
      "range": [
        36,
        68,
        36,
        72
      ]
    },
    {
      "document": 2,
      "range": [
        37,
        13,
        37,
        22
      ]
    },
    {
      "document": 2,
      "range": [
        37,
        29,
        37,
        36
      ]
    },
    {
      "document": 2,
      "range": [
        37,
        44,
        37,
        55
      ]
    },
    {
      "document": 2,
      "range": [
        37,
        61,
        37,
        67
      ]
    },
    {
      "document": 2,
      "range": [
        38,
        13,
        38,
        23
      ]
    },
    {
      "document": 2,
      "range": [
        38,
        30,
        38,
        37
      ]
    },
    {
      "document": 2,
      "range": [
        38,
        45,
        38,
        62
      ]
    },
    {
      "document": 2,
      "range": [
        38,
        68,
        38,
        75
      ]
    },
    {
      "document": 2,
      "range": [
        39,
        13,
        39,
        30
      ]
    },
    {
      "document": 2,
      "range": [
        39,
        37,
        39,
        43
      ]
    },
    {
      "document": 2,
      "range": [
        39,
        51,
        39,
        63
      ]
    },
    {
      "document": 2,
      "range": [
        39,
        69,
        39,
        76
      ]
    },
    {
      "document": 2,
      "range": [
        40,
        13,
        40,
        32
      ]
    },
    {
      "document": 2,
      "range": [
        40,
        39,
        40,
        45
      ]
    },
    {
      "document": 2,
      "range": [
        40,
        53,
        40,
        65
      ]
    },
    {
      "document": 2,
      "range": [
        40,
        71,
        40,
        75
      ]
    },
    {
      "document": 2,
      "range": [
        41,
        13,
        41,
        33
      ]
    },
    {
      "document": 2,
      "range": [
        41,
        40,
        41,
        47
      ]
    },
    {
      "document": 2,
      "range": [
        41,
        55,
        41,
        67
      ]
    },
    {
      "document": 2,
      "range": [
        41,
        73,
        41,
        77
      ]
    },
    {
      "document": 3,
      "range": [
        8,
        6,
        8,
        11
      ]
    },
    {
      "document": 3,
      "range": [
        8,
        14,
        8,
        27
      ]
    },
    {
      "document": 3,
      "range": [
        9,
        12,
        9,
        19
      ]
    },
    {
      "document": 3,
      "range": [
        10,
        12,
        10,
        22
      ]
    },
    {
      "document": 3,
      "range": [
        14,
        5,
        14,
        17
      ]
    },
    {
      "document": 3,
      "range": [
        14,
        20,
        14,
        27
      ]
    },
    {
      "document": 3,
      "range": [
        15,
        5,
        15,
        16
      ]
    },
    {
      "document": 3,
      "range": [
        15,
        19,
        15,
        26
      ]
    },
    {
      "document": 3,
      "range": [
        16,
        5,
        16,
        17
      ]
    },
    {
      "document": 3,
      "range": [
        16,
        20,
        16,
        27
      ]
    },
    {
      "document": 3,
      "range": [
        17,
        5,
        17,
        21
      ]
    },
    {
      "document": 3,
      "range": [
        17,
        24,
        17,
        31
      ]
    },
    {
      "document": 3,
      "range": [
        18,
        5,
        18,
        21
      ]
    },
    {
      "document": 3,
      "range": [
        18,
        24,
        18,
        31
      ]
    },
    {
      "document": 3,
      "range": [
        22,
        6,
        22,
        18
      ]
    },
    {
      "document": 3,
      "range": [
        22,
        21,
        22,
        27
      ]
    },
    {
      "document": 3,
      "range": [
        27,
        6,
        27,
        17
      ]
    },
    {
      "document": 3,
      "range": [
        27,
        20,
        27,
        26
      ]
    },
    {
      "document": 3,
      "range": [
        31,
        6,
        31,
        14
      ]
    },
    {
      "document": 3,
      "range": [
        31,
        17,
        31,
        24
      ]
    },
    {
      "document": 4,
      "range": [
        12,
        12,
        12,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        12,
        29,
        12,
        43
      ]
    },
    {
      "document": 4,
      "range": [
        13,
        12,
        13,
        21
      ]
    },
    {
      "document": 4,
      "range": [
        13,
        24,
        13,
        28
      ]
    },
    {
      "document": 4,
      "range": [
        14,
        12,
        14,
        19
      ]
    },
    {
      "document": 4,
      "range": [
        14,
        22,
        14,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        15,
        7,
        15,
        15
      ]
    },
    {
      "document": 4,
      "range": [
        15,
        19,
        15,
        34
      ]
    },
    {
      "document": 4,
      "range": [
        16,
        7,
        16,
        17
      ]
    },
    {
      "document": 4,
      "range": [
        16,
        20,
        16,
        38
      ]
    },
    {
      "document": 4,
      "range": [
        17,
        7,
        17,
        15
      ]
    },
    {
      "document": 4,
      "range": [
        17,
        18,
        17,
        37
      ]
    },
    {
      "document": 4,
      "range": [
        18,
        7,
        18,
        16
      ]
    },
    {
      "document": 4,
      "range": [
        18,
        19,
        18,
        36
      ]
    },
    {
      "document": 4,
      "range": [
        19,
        7,
        19,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        19,
        14,
        19,
        27
      ]
    },
    {
      "document": 4,
      "range": [
        19,
        30,
        19,
        56
      ]
    },
    {
      "document": 4,
      "range": [
        19,
        34,
        19,
        44
      ]
    },
    {
      "document": 4,
      "range": [
        20,
        16,
        20,
        25
      ]
    },
    {
      "document": 4,
      "range": [
        20,
        28,
        20,
        45
      ]
    },
    {
      "document": 4,
      "range": [
        25,
        7,
        25,
        13
      ]
    },
    {
      "document": 4,
      "range": [
        25,
        17,
        25,
        34
      ]
    },
    {
      "document": 4,
      "range": [
        26,
        7,
        26,
        12
      ]
    },
    {
      "document": 4,
      "range": [
        26,
        16,
        26,
        29
      ]
    },
    {
      "document": 4,
      "range": [
        26,
        32,
        26,
        59
      ]
    },
    {
      "document": 4,
      "range": [
        26,
        36,
        26,
        46
      ]
    },
    {
      "document": 4,
      "range": [
        27,
        7,
        27,
        19
      ]
    },
    {
      "document": 4,
      "range": [
        27,
        22,
        27,
        35
      ]
    },
    {
      "document": 4,
      "range": [
        32,
        12,
        32,
        18
      ]
    },
    {
      "document": 4,
      "range": [
        32,
        21,
        32,
        27
      ]
    },
    {
      "document": 4,
      "range": [
        33,
        7,
        33,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        33,
        14,
        33,
        32
      ]
    },
    {
      "document": 4,
      "range": [
        34,
        7,
        34,
        12
      ]
    },
    {
      "document": 4,
      "range": [
        34,
        16,
        34,
        35
      ]
    },
    {
      "document": 4,
      "range": [
        35,
        7,
        35,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        35,
        14,
        35,
        27
      ]
    },
    {
      "document": 4,
      "range": [
        35,
        30,
        35,
        56
      ]
    },
    {
      "document": 4,
      "range": [
        35,
        34,
        35,
        44
      ]
    },
    {
      "document": 4,
      "range": [
        40,
        7,
        40,
        13
      ]
    },
    {
      "document": 4,
      "range": [
        40,
        16,
        40,
        31
      ]
    },
    {
      "document": 4,
      "range": [
        41,
        7,
        41,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        41,
        14,
        41,
        27
      ]
    },
    {
      "document": 4,
      "range": [
        41,
        30,
        41,
        57
      ]
    },
    {
      "document": 4,
      "range": [
        41,
        34,
        41,
        44
      ]
    },
    {
      "document": 4,
      "range": [
        46,
        12,
        46,
        20
      ]
    },
    {
      "document": 4,
      "range": [
        46,
        23,
        46,
        36
      ]
    },
    {
      "document": 4,
      "range": [
        47,
        7,
        47,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        47,
        14,
        47,
        27
      ]
    },
    {
      "document": 4,
      "range": [
        47,
        30,
        47,
        56
      ]
    },
    {
      "document": 4,
      "range": [
        47,
        34,
        47,
        44
      ]
    },
    {
      "document": 4,
      "range": [
        52,
        7,
        52,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        52,
        13,
        52,
        28
      ]
    },
    {
      "document": 4,
      "range": [
        53,
        7,
        53,
        14
      ]
    },
    {
      "document": 4,
      "range": [
        53,
        17,
        53,
        24
      ]
    },
    {
      "document": 4,
      "range": [
        54,
        7,
        54,
        14
      ]
    },
    {
      "document": 4,
      "range": [
        54,
        17,
        54,
        32
      ]
    },
    {
      "document": 4,
      "range": [
        55,
        7,
        55,
        19
      ]
    },
    {
      "document": 4,
      "range": [
        55,
        22,
        55,
        34
      ]
    },
    {
      "document": 4,
      "range": [
        56,
        2,
        56,
        41
      ]
    },
    {
      "document": 4,
      "range": [
        56,
        10,
        56,
        24
      ]
    },
    {
      "document": 4,
      "range": [
        56,
        28,
        56,
        40
      ]
    },
    {
      "document": 4,
      "range": [
        57,
        2,
        57,
        40
      ]
    },
    {
      "document": 4,
      "range": [
        57,
        10,
        57,
        24
      ]
    },
    {
      "document": 4,
      "range": [
        57,
        28,
        57,
        39
      ]
    },
    {
      "document": 4,
      "range": [
        58,
        2,
        58,
        40
      ]
    },
    {
      "document": 4,
      "range": [
        58,
        10,
        58,
        22
      ]
    },
    {
      "document": 4,
      "range": [
        58,
        26,
        58,
        39
      ]
    },
    {
      "document": 4,
      "range": [
        59,
        2,
        59,
        47
      ]
    },
    {
      "document": 4,
      "range": [
        59,
        10,
        59,
        23
      ]
    },
    {
      "document": 4,
      "range": [
        59,
        27,
        59,
        46
      ]
    },
    {
      "document": 4,
      "range": [
        64,
        12,
        64,
        20
      ]
    },
    {
      "document": 4,
      "range": [
        64,
        23,
        64,
        42
      ]
    },
    {
      "document": 4,
      "range": [
        65,
        12,
        65,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        65,
        29,
        65,
        61
      ]
    },
    {
      "document": 4,
      "range": [
        66,
        12,
        66,
        27
      ]
    },
    {
      "document": 4,
      "range": [
        66,
        30,
        66,
        43
      ]
    },
    {
      "document": 4,
      "range": [
        67,
        7,
        67,
        15
      ]
    },
    {
      "document": 4,
      "range": [
        67,
        18,
        67,
        31
      ]
    },
    {
      "document": 4,
      "range": [
        72,
        7,
        72,
        10
      ]
    },
    {
      "document": 4,
      "range": [
        72,
        13,
        72,
        21
      ]
    },
    {
      "document": 4,
      "range": [
        73,
        7,
        73,
        14
      ]
    },
    {
      "document": 4,
      "range": [
        73,
        17,
        73,
        24
      ]
    },
    {
      "document": 4,
      "range": [
        74,
        7,
        74,
        13
      ]
    },
    {
      "document": 4,
      "range": [
        74,
        16,
        74,
        22
      ]
    },
    {
      "document": 4,
      "range": [
        76,
        2,
        76,
        42
      ]
    },
    {
      "document": 4,
      "range": [
        76,
        10,
        76,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        76,
        30,
        76,
        41
      ]
    },
    {
      "document": 4,
      "range": [
        77,
        2,
        77,
        46
      ]
    },
    {
      "document": 4,
      "range": [
        77,
        10,
        77,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        77,
        30,
        77,
        45
      ]
    },
    {
      "document": 4,
      "range": [
        78,
        2,
        78,
        46
      ]
    },
    {
      "document": 4,
      "range": [
        78,
        10,
        78,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        78,
        30,
        78,
        45
      ]
    },
    {
      "document": 4,
      "range": [
        79,
        2,
        79,
        53
      ]
    },
    {
      "document": 4,
      "range": [
        79,
        10,
        79,
        26
      ]
    },
    {
      "document": 4,
      "range": [
        79,
        30,
        79,
        52
      ]
    },
    {
      "document": 4,
      "range": [
        80,
        2,
        80,
        54
      ]
    },
    {
      "document": 4,
      "range": [
        80,
        10,
        80,
        39
      ]
    },
    {
      "document": 4,
      "range": [
        80,
        43,
        80,
        53
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "TimerGeometry::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimer::timerInstance"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::CountdownComplete"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::DecrementPressed"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Expired"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Idle"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::IncrementPressed"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Paused"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::ResetPressed"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Running"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::StartPressed"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::StopPressed"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::decrement_idle"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::expired"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::idle"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::increment_idle"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::paused"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::running"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_expired"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_paused"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running_resume"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::BatteryOutlet"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::BatteryOutlet::maxCurrent"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::BatteryOutlet::power"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::BatteryOutlet::voltage"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::ButtonInputPort"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::ButtonInputPort::decrementPressed"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::ButtonInputPort::incrementPressed"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::ButtonInputPort::resetPressed"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::ButtonInputPort::startPressed"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::ButtonInputPort::stopPressed"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::BuzzerCommandPort"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::DisplayCommandPort"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::DisplayCommandPort::displayValue"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::LcdSegmentDrivePort"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Battery"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Battery::capacity"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Battery::nominalVoltage"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Battery::powerOut"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Battery::runtimeEstimate"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::ButtonInterface"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::ButtonInterface::output"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::ButtonInterface::pwr"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::ButtonInterface::pwr::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Buzzer"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Buzzer::duration"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Buzzer::pwr"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Buzzer::pwr::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::BuzzerDriver"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::BuzzerDriver::ctrlIn"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::BuzzerDriver::pwrIn"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::BuzzerDriver::pwrIn::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Display"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Display::cmd"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Display::format"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Display::lcdIn"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Display::pwr"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Display::pwr::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::KitchenTimer"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::KitchenTimer::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::KitchenTimer::battery"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::KitchenTimer::buzzer"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::KitchenTimer::pcb"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::buttonIn"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::buzzerOut"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::clockFrequency"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::displayOut"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::flashSize"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::lcdDrive"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::pwr"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::pwr::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::ramSize"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::Microcontroller::timerMode"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::TimerPCB"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::TimerPCB::"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::TimerPCB::buttons"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::TimerPCB::buzzerDriver"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::TimerPCB::display"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerStructure::TimerPCB::mcu"
    },
    {
      "document": 5,
      "kind": "qualified-name",
      "qualifiedName": "ISQBase::DurationValue"
    },
    {
      "document": 6,
      "kind": "qualified-name",
      "qualifiedName": "ISQElectromagnetism::ElectricChargeValue"
    },
    {
      "document": 6,
      "kind": "qualified-name",
      "qualifiedName": "ISQElectromagnetism::ElectricPotentialDifferenceValue"
    },
    {
      "document": 6,
      "kind": "qualified-name",
      "qualifiedName": "ISQElectromagnetism::electricPower"
    },
    {
      "document": 7,
      "kind": "qualified-name",
      "qualifiedName": "ISQSpaceTime::FrequencyValue"
    },
    {
      "document": 8,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::Boolean"
    },
    {
      "document": 8,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::Real"
    },
    {
      "document": 8,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::String"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AttributeUsage",
      "ownerQualifiedName": "KitchenTimerStructure::ButtonInterface::pwr",
      "source": 107,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AttributeUsage",
      "ownerQualifiedName": "KitchenTimerStructure::Buzzer::pwr",
      "source": 113,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AttributeUsage",
      "ownerQualifiedName": "KitchenTimerStructure::BuzzerDriver::pwrIn",
      "source": 89,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AttributeUsage",
      "ownerQualifiedName": "KitchenTimerStructure::Display::pwr",
      "source": 101,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::KitchenTimer",
      "source": 149,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::KitchenTimer",
      "source": 152,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::KitchenTimer",
      "source": 155,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::KitchenTimer",
      "source": 158,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::KitchenTimer",
      "source": 161,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AttributeUsage",
      "ownerQualifiedName": "KitchenTimerStructure::Microcontroller::pwr",
      "source": 81,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::TimerPCB",
      "source": 123,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::TimerPCB",
      "source": 126,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::TimerPCB",
      "source": 129,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConnectionUsage",
      "ownerQualifiedName": "KitchenTimerStructure::TimerPCB",
      "source": 132,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 84,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 91,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 92,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 93,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 94,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 95,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 96,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "initialState",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "initialState",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "transitionSource",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "transitionTarget",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "transitionTrigger",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "transition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "transition",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "transition",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "transition",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "transitionSource",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "transitionTarget",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "transitionTrigger",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "transition",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "transition",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "typing",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "transition",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "transition",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "transitionSource",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "transitionTarget",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "transitionTrigger",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "transitionSource",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "transitionTarget",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "transitionTrigger",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "transitionSource",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "transitionTarget",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "transitionTrigger",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "transitionSource",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "transitionTarget",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "transitionTrigger",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "transitionSource",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "transitionTarget",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "transitionTrigger",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "transitionSource",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "transitionTarget",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "transitionTrigger",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 85,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 88,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 103,
      "relationshipKind": "typing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typing",
      "source": 31
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "typing",
      "source": 31
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "typing",
      "source": 32
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "typing",
      "source": 32
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typing",
      "source": 33
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "typing",
      "source": 33
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 34
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "typing",
      "source": 34
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "typing",
      "source": 35
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "typing",
      "source": 35
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "typing",
      "source": 37
    },
    {
      "kind": "relationship",
      "ordinal": 83,
      "relationshipKind": "typing",
      "source": 37
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "typing",
      "source": 39
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "typing",
      "source": 39
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 41
    },
    {
      "kind": "relationship",
      "ordinal": 76,
      "relationshipKind": "typing",
      "source": 41
    },
    {
      "kind": "relationship",
      "ordinal": 104,
      "relationshipKind": "typing",
      "source": 43
    },
    {
      "kind": "relationship",
      "ordinal": 105,
      "relationshipKind": "typing",
      "source": 44
    },
    {
      "kind": "relationship",
      "ordinal": 98,
      "relationshipKind": "containment",
      "source": 45
    },
    {
      "kind": "relationship",
      "ordinal": 99,
      "relationshipKind": "containment",
      "source": 45
    },
    {
      "kind": "relationship",
      "ordinal": 100,
      "relationshipKind": "containment",
      "source": 45
    },
    {
      "kind": "relationship",
      "ordinal": 102,
      "relationshipKind": "typing",
      "source": 45
    },
    {
      "kind": "relationship",
      "ordinal": 106,
      "relationshipKind": "typing",
      "source": 46
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "containment",
      "source": 48
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "containment",
      "source": 48
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "containment",
      "source": 48
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "containment",
      "source": 48
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "containment",
      "source": 48
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typing",
      "source": 48
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "containment",
      "source": 49
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "containment",
      "source": 49
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "containment",
      "source": 49
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "containment",
      "source": 49
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "typing",
      "source": 49
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "redefinition",
      "source": 50
    },
    {
      "kind": "relationship",
      "ordinal": 90,
      "relationshipKind": "typing",
      "source": 52
    },
    {
      "kind": "relationship",
      "ordinal": 86,
      "relationshipKind": "containment",
      "source": 53
    },
    {
      "kind": "relationship",
      "ordinal": 87,
      "relationshipKind": "containment",
      "source": 53
    },
    {
      "kind": "relationship",
      "ordinal": 88,
      "relationshipKind": "containment",
      "source": 53
    },
    {
      "kind": "relationship",
      "ordinal": 89,
      "relationshipKind": "containment",
      "source": 53
    },
    {
      "kind": "relationship",
      "ordinal": 87,
      "relationshipKind": "typing",
      "source": 53
    },
    {
      "kind": "relationship",
      "ordinal": 89,
      "relationshipKind": "redefinition",
      "source": 54
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "containment",
      "source": 56
    },
    {
      "kind": "relationship",
      "ordinal": 82,
      "relationshipKind": "containment",
      "source": 56
    },
    {
      "kind": "relationship",
      "ordinal": 83,
      "relationshipKind": "containment",
      "source": 56
    },
    {
      "kind": "relationship",
      "ordinal": 84,
      "relationshipKind": "typing",
      "source": 56
    },
    {
      "kind": "relationship",
      "ordinal": 79,
      "relationshipKind": "containment",
      "source": 57
    },
    {
      "kind": "relationship",
      "ordinal": 82,
      "relationshipKind": "typing",
      "source": 57
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "containment",
      "source": 58
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "containment",
      "source": 58
    },
    {
      "kind": "relationship",
      "ordinal": 76,
      "relationshipKind": "containment",
      "source": 58
    },
    {
      "kind": "relationship",
      "ordinal": 77,
      "relationshipKind": "containment",
      "source": 58
    },
    {
      "kind": "relationship",
      "ordinal": 79,
      "relationshipKind": "typing",
      "source": 58
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "redefinition",
      "source": 59
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "containment",
      "source": 61
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "typing",
      "source": 61
    },
    {
      "kind": "relationship",
      "ordinal": 77,
      "relationshipKind": "typing",
      "source": 62
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "containment",
      "source": 63
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "typing",
      "source": 63
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "containment",
      "source": 64
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "containment",
      "source": 64
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "containment",
      "source": 64
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "containment",
      "source": 64
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "typing",
      "source": 64
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "redefinition",
      "source": 65
    },
    {
      "kind": "relationship",
      "ordinal": 91,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 92,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 93,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 94,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 95,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 96,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 97,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 98,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 99,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 100,
      "relationshipKind": "memberAccessOperand",
      "source": 67
    },
    {
      "kind": "relationship",
      "ordinal": 97,
      "relationshipKind": "containment",
      "source": 68
    },
    {
      "kind": "relationship",
      "ordinal": 101,
      "relationshipKind": "containment",
      "source": 68
    },
    {
      "kind": "relationship",
      "ordinal": 102,
      "relationshipKind": "containment",
      "source": 68
    },
    {
      "kind": "relationship",
      "ordinal": 103,
      "relationshipKind": "containment",
      "source": 68
    },
    {
      "kind": "relationship",
      "ordinal": 101,
      "relationshipKind": "typing",
      "source": 68
    },
    {
      "kind": "relationship",
      "ordinal": 85,
      "relationshipKind": "containment",
      "source": 69
    },
    {
      "kind": "relationship",
      "ordinal": 90,
      "relationshipKind": "containment",
      "source": 69
    },
    {
      "kind": "relationship",
      "ordinal": 86,
      "relationshipKind": "typing",
      "source": 69
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "containment",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 70
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 72
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 72
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 72
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 72
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 72
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typing",
      "source": 72
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 73
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typing",
      "source": 73
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "typing",
      "source": 74
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "containment",
      "source": 75
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "typing",
      "source": 75
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "typing",
      "source": 76
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 77
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typing",
      "source": 77
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 78
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 78
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 78
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 78
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typing",
      "source": 78
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "redefinition",
      "source": 79
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "typing",
      "source": 80
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "containment",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "typing",
      "source": 81
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "memberAccessOperand",
      "source": 83
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "containment",
      "source": 84
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "containment",
      "source": 84
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typing",
      "source": 84
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "containment",
      "source": 85
    },
    {
      "kind": "relationship",
      "ordinal": 78,
      "relationshipKind": "containment",
      "source": 85
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "containment",
      "source": 85
    },
    {
      "kind": "relationship",
      "ordinal": 78,
      "relationshipKind": "typing",
      "source": 85
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "containment",
      "source": 86
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "containment",
      "source": 86
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "containment",
      "source": 86
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "containment",
      "source": 86
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "typing",
      "source": 86
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "containment",
      "source": 87
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typing",
      "source": 87
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "geometry-view",
    "name": "selected",
    "source": 0
  },
  "completeness": {
    "status": "incomplete",
    "reasons": [
      {
        "code": "geometry-facts-unavailable"
      }
    ]
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 143,
        "provenance": "implied",
        "reference": 111,
        "source": 0,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 115,
        "provenance": "implied",
        "reference": 248,
        "source": 6,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 79,
        "provenance": "implied",
        "reference": 312,
        "source": 23,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 271,
        "source": 26,
        "target": 29
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 272,
        "source": 26,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 273,
        "source": 26,
        "target": 27
      },
      {
        "kind": "containment",
        "navigation": 81,
        "provenance": "authored",
        "reference": 274,
        "source": 26,
        "target": 30
      },
      {
        "kind": "containment",
        "navigation": 71,
        "provenance": "implied",
        "reference": 313,
        "source": 23,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 51,
        "provenance": "implied",
        "reference": 257,
        "source": 31,
        "target": 32
      },
      {
        "kind": "containment",
        "navigation": 53,
        "provenance": "implied",
        "reference": 258,
        "source": 31,
        "target": 33
      },
      {
        "kind": "containment",
        "navigation": 49,
        "provenance": "implied",
        "reference": 259,
        "source": 31,
        "target": 34
      },
      {
        "kind": "containment",
        "navigation": 57,
        "provenance": "implied",
        "reference": 260,
        "source": 31,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 55,
        "provenance": "implied",
        "reference": 261,
        "source": 31,
        "target": 36
      },
      {
        "kind": "containment",
        "navigation": 75,
        "provenance": "implied",
        "reference": 314,
        "source": 23,
        "target": 37
      },
      {
        "kind": "containment",
        "navigation": 61,
        "provenance": "implied",
        "reference": 269,
        "source": 37,
        "target": 38
      },
      {
        "kind": "containment",
        "navigation": 77,
        "provenance": "implied",
        "reference": 315,
        "source": 23,
        "target": 39
      },
      {
        "kind": "containment",
        "navigation": 63,
        "provenance": "implied",
        "reference": 263,
        "source": 39,
        "target": 40
      },
      {
        "kind": "containment",
        "navigation": 83,
        "provenance": "implied",
        "reference": 316,
        "source": 23,
        "target": 41
      },
      {
        "kind": "containment",
        "navigation": 5,
        "provenance": "implied",
        "reference": 278,
        "source": 41,
        "target": 51
      },
      {
        "kind": "containment",
        "navigation": 9,
        "provenance": "implied",
        "reference": 279,
        "source": 41,
        "target": 52
      },
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "implied",
        "reference": 280,
        "source": 41,
        "target": 50
      },
      {
        "kind": "initial-state",
        "navigation": 4,
        "provenance": "authored",
        "reference": 121,
        "source": 50,
        "target": 51
      },
      {
        "kind": "containment",
        "navigation": 11,
        "provenance": "implied",
        "reference": 281,
        "source": 41,
        "target": 53
      },
      {
        "kind": "containment",
        "navigation": 7,
        "provenance": "implied",
        "reference": 282,
        "source": 41,
        "target": 54
      },
      {
        "kind": "containment",
        "navigation": 25,
        "provenance": "implied",
        "reference": 283,
        "source": 41,
        "target": 49
      },
      {
        "kind": "transition",
        "navigation": 26,
        "provenance": "authored",
        "reference": 137,
        "source": 54,
        "target": 52
      },
      {
        "kind": "containment",
        "navigation": 29,
        "provenance": "implied",
        "reference": 284,
        "source": 41,
        "target": 42
      },
      {
        "kind": "transition",
        "navigation": 30,
        "provenance": "authored",
        "reference": 138,
        "source": 54,
        "target": 53
      },
      {
        "kind": "containment",
        "navigation": 13,
        "provenance": "implied",
        "reference": 285,
        "source": 41,
        "target": 43
      },
      {
        "kind": "transition",
        "navigation": 14,
        "provenance": "authored",
        "reference": 127,
        "source": 51,
        "target": 54
      },
      {
        "kind": "containment",
        "navigation": 21,
        "provenance": "implied",
        "reference": 286,
        "source": 41,
        "target": 44
      },
      {
        "kind": "transition",
        "navigation": 22,
        "provenance": "authored",
        "reference": 128,
        "source": 51,
        "target": 51
      },
      {
        "kind": "containment",
        "navigation": 17,
        "provenance": "implied",
        "reference": 287,
        "source": 41,
        "target": 45
      },
      {
        "kind": "transition",
        "navigation": 18,
        "provenance": "authored",
        "reference": 129,
        "source": 51,
        "target": 51
      },
      {
        "kind": "containment",
        "navigation": 33,
        "provenance": "implied",
        "reference": 288,
        "source": 41,
        "target": 46
      },
      {
        "kind": "transition",
        "navigation": 34,
        "provenance": "authored",
        "reference": 134,
        "source": 52,
        "target": 54
      },
      {
        "kind": "containment",
        "navigation": 37,
        "provenance": "implied",
        "reference": 289,
        "source": 41,
        "target": 47
      },
      {
        "kind": "transition",
        "navigation": 38,
        "provenance": "authored",
        "reference": 135,
        "source": 52,
        "target": 51
      },
      {
        "kind": "containment",
        "navigation": 41,
        "provenance": "implied",
        "reference": 290,
        "source": 41,
        "target": 48
      },
      {
        "kind": "transition",
        "navigation": 42,
        "provenance": "authored",
        "reference": 125,
        "source": 53,
        "target": 51
      },
      {
        "kind": "containment",
        "navigation": 73,
        "provenance": "implied",
        "reference": 317,
        "source": 23,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 59,
        "provenance": "implied",
        "reference": 266,
        "source": 24,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 69,
        "provenance": "implied",
        "reference": 318,
        "source": 23,
        "target": 56
      },
      {
        "kind": "containment",
        "navigation": 67,
        "provenance": "implied",
        "reference": 319,
        "source": 23,
        "target": 57
      },
      {
        "kind": "containment",
        "navigation": 65,
        "provenance": "implied",
        "reference": 320,
        "source": 23,
        "target": 55
      },
      {
        "kind": "containment",
        "navigation": 123,
        "provenance": "implied",
        "reference": 249,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 126,
        "provenance": "implied",
        "reference": 250,
        "source": 6,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 129,
        "provenance": "implied",
        "reference": 251,
        "source": 6,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 132,
        "provenance": "implied",
        "reference": 252,
        "source": 6,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 119,
        "provenance": "implied",
        "reference": 253,
        "source": 6,
        "target": 58
      },
      {
        "kind": "containment",
        "navigation": 105,
        "provenance": "implied",
        "reference": 300,
        "source": 58,
        "target": 59
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 194,
        "source": 59,
        "target": 62
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 195,
        "source": 59,
        "target": 61
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 196,
        "source": 59,
        "target": 60
      },
      {
        "kind": "containment",
        "navigation": 107,
        "provenance": "authored",
        "reference": 197,
        "source": 59,
        "target": 63
      },
      {
        "kind": "containment",
        "navigation": 103,
        "provenance": "implied",
        "reference": 301,
        "source": 58,
        "target": 64
      },
      {
        "kind": "containment",
        "navigation": 51,
        "provenance": "implied",
        "reference": 188,
        "source": 64,
        "target": 65
      },
      {
        "kind": "containment",
        "navigation": 53,
        "provenance": "implied",
        "reference": 189,
        "source": 64,
        "target": 66
      },
      {
        "kind": "containment",
        "navigation": 49,
        "provenance": "implied",
        "reference": 190,
        "source": 64,
        "target": 67
      },
      {
        "kind": "containment",
        "navigation": 57,
        "provenance": "implied",
        "reference": 191,
        "source": 64,
        "target": 68
      },
      {
        "kind": "containment",
        "navigation": 55,
        "provenance": "implied",
        "reference": 192,
        "source": 64,
        "target": 69
      },
      {
        "kind": "containment",
        "navigation": 117,
        "provenance": "implied",
        "reference": 254,
        "source": 6,
        "target": 70
      },
      {
        "kind": "containment",
        "navigation": 95,
        "provenance": "implied",
        "reference": 307,
        "source": 70,
        "target": 71
      },
      {
        "kind": "containment",
        "navigation": 59,
        "provenance": "implied",
        "reference": 219,
        "source": 71,
        "target": 72
      },
      {
        "kind": "containment",
        "navigation": 99,
        "provenance": "implied",
        "reference": 308,
        "source": 70,
        "target": 73
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 224,
        "source": 73,
        "target": 76
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 225,
        "source": 73,
        "target": 75
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 226,
        "source": 73,
        "target": 74
      },
      {
        "kind": "containment",
        "navigation": 101,
        "provenance": "authored",
        "reference": 227,
        "source": 73,
        "target": 77
      },
      {
        "kind": "containment",
        "navigation": 97,
        "provenance": "implied",
        "reference": 309,
        "source": 70,
        "target": 78
      },
      {
        "kind": "containment",
        "navigation": 61,
        "provenance": "implied",
        "reference": 222,
        "source": 78,
        "target": 79
      },
      {
        "kind": "containment",
        "navigation": 93,
        "provenance": "implied",
        "reference": 310,
        "source": 70,
        "target": 80
      },
      {
        "kind": "containment",
        "navigation": 121,
        "provenance": "implied",
        "reference": 255,
        "source": 6,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 87,
        "provenance": "implied",
        "reference": 303,
        "source": 11,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 213,
        "source": 16,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 214,
        "source": 16,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 215,
        "source": 16,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 89,
        "provenance": "authored",
        "reference": 216,
        "source": 16,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 85,
        "provenance": "implied",
        "reference": 304,
        "source": 11,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 63,
        "provenance": "implied",
        "reference": 211,
        "source": 21,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 91,
        "provenance": "implied",
        "reference": 305,
        "source": 11,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 207,
        "source": 12,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 208,
        "source": 12,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 209,
        "source": 12,
        "target": 13
      },
      {
        "kind": "containment",
        "navigation": 147,
        "provenance": "implied",
        "reference": 112,
        "source": 0,
        "target": 81
      },
      {
        "kind": "containment",
        "navigation": 111,
        "provenance": "implied",
        "reference": 245,
        "source": 81,
        "target": 82
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 201,
        "source": 82,
        "target": 85
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 202,
        "source": 82,
        "target": 84
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 203,
        "source": 82,
        "target": 83
      },
      {
        "kind": "containment",
        "navigation": 113,
        "provenance": "authored",
        "reference": 204,
        "source": 82,
        "target": 86
      },
      {
        "kind": "containment",
        "navigation": 109,
        "provenance": "implied",
        "reference": 246,
        "source": 81,
        "target": 87
      },
      {
        "kind": "containment",
        "navigation": 149,
        "provenance": "implied",
        "reference": 113,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 152,
        "provenance": "implied",
        "reference": 114,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 155,
        "provenance": "implied",
        "reference": 115,
        "source": 0,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 158,
        "provenance": "implied",
        "reference": 116,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 161,
        "provenance": "implied",
        "reference": 117,
        "source": 0,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 145,
        "provenance": "implied",
        "reference": 118,
        "source": 0,
        "target": 88
      },
      {
        "kind": "containment",
        "navigation": 141,
        "provenance": "implied",
        "reference": 240,
        "source": 88,
        "target": 89
      },
      {
        "kind": "containment",
        "navigation": 45,
        "provenance": "implied",
        "reference": 183,
        "source": 89,
        "target": 92
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "implied",
        "reference": 184,
        "source": 89,
        "target": 91
      },
      {
        "kind": "containment",
        "navigation": 48,
        "provenance": "implied",
        "reference": 185,
        "source": 89,
        "target": 90
      },
      {
        "kind": "containment",
        "navigation": 135,
        "provenance": "implied",
        "reference": 241,
        "source": 88,
        "target": 95
      },
      {
        "kind": "containment",
        "navigation": 137,
        "provenance": "implied",
        "reference": 242,
        "source": 88,
        "target": 93
      },
      {
        "kind": "containment",
        "navigation": 139,
        "provenance": "implied",
        "reference": 243,
        "source": 88,
        "target": 94
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "geometry-view",
    "metadata": {
      "elements": [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60,
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
        72,
        73,
        74,
        75,
        76,
        77,
        78,
        79,
        80,
        81,
        82,
        83,
        84,
        85,
        86,
        87,
        88,
        89,
        90,
        91,
        92,
        93,
        94,
        95
      ],
      "primitives": []
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              6,
              81,
              88
            ],
            "provenance": "inherited"
          },
          {
            "kind": "connections",
            "members": [
              1,
              2,
              3,
              4,
              5
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "timerInstance",
        "notationRole": "usage",
        "owner": null,
        "reference": 1,
        "source": 1,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "KitchenTimer",
              "reference": 66
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 101,
        "source": 149,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 102,
        "source": 152,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 103,
        "source": 155,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 104,
        "source": 158,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 105,
        "source": 161,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              11,
              23,
              58,
              70
            ],
            "provenance": "inherited"
          },
          {
            "kind": "connections",
            "members": [
              7,
              8,
              9,
              10
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "pcb",
        "notationRole": "usage",
        "owner": 0,
        "reference": 70,
        "source": 143,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "TimerPCB",
              "reference": 82
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 6,
        "reference": 107,
        "source": 123,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 6,
        "reference": 108,
        "source": 126,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 6,
        "reference": 109,
        "source": 129,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ConnectionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 6,
        "reference": 110,
        "source": 132,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              12,
              16,
              21
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "buzzerDriver",
        "notationRole": "usage",
        "owner": 6,
        "reference": 85,
        "source": 121,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BuzzerDriver",
              "reference": 55
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              13,
              14,
              15
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "buzzerPwrOut",
        "notationRole": "usage",
        "owner": 11,
        "reference": 56,
        "source": 91,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 12,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 12,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 12,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              20
            ],
            "provenance": "direct"
          },
          {
            "kind": "attributes",
            "members": [
              17,
              18,
              19
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "pwrIn",
        "notationRole": "usage",
        "owner": 11,
        "reference": 58,
        "source": 87,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 16,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 16,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 16,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 16,
        "reference": 99,
        "source": 89,
        "typing": {
          "status": "partial",
          "types": []
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              22
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "ctrlIn",
        "notationRole": "usage",
        "owner": 11,
        "reference": 57,
        "source": 85,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BuzzerCommandPort",
              "reference": 36
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "buzzerOn",
        "notationRole": "reference-usage",
        "owner": 21,
        "reference": 37,
        "source": 63,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              55,
              56,
              57
            ],
            "provenance": "inherited"
          },
          {
            "kind": "ports",
            "members": [
              24,
              26,
              31,
              37,
              39
            ],
            "provenance": "inherited"
          },
          {
            "kind": "states",
            "members": [
              41
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "mcu",
        "notationRole": "usage",
        "owner": 6,
        "reference": 87,
        "source": 115,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Microcontroller",
              "reference": 71
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              25
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "displayOut",
        "notationRole": "usage",
        "owner": 23,
        "reference": 75,
        "source": 73,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DisplayCommandPort",
              "reference": 38
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "displayValue",
        "notationRole": "reference-usage",
        "owner": 24,
        "reference": 39,
        "source": 59,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "String",
              "reference": 95
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              30
            ],
            "provenance": "direct"
          },
          {
            "kind": "attributes",
            "members": [
              27,
              28,
              29
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "pwr",
        "notationRole": "usage",
        "owner": 23,
        "reference": 78,
        "source": 79,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 26,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 26,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 26,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 26,
        "reference": 106,
        "source": 81,
        "typing": {
          "status": "partial",
          "types": []
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              32,
              33,
              34,
              35,
              36
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "buttonIn",
        "notationRole": "usage",
        "owner": 23,
        "reference": 72,
        "source": 71,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ButtonInputPort",
              "reference": 30
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "stopPressed",
        "notationRole": "reference-usage",
        "owner": 31,
        "reference": 35,
        "source": 51,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "resetPressed",
        "notationRole": "reference-usage",
        "owner": 31,
        "reference": 33,
        "source": 53,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "startPressed",
        "notationRole": "reference-usage",
        "owner": 31,
        "reference": 34,
        "source": 49,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "decrementPressed",
        "notationRole": "reference-usage",
        "owner": 31,
        "reference": 31,
        "source": 57,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "incrementPressed",
        "notationRole": "reference-usage",
        "owner": 31,
        "reference": 32,
        "source": 55,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              38
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "lcdDrive",
        "notationRole": "usage",
        "owner": 23,
        "reference": 77,
        "source": 75,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "LcdSegmentDrivePort",
              "reference": 40
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "comSegDrive",
        "notationRole": "reference-usage",
        "owner": 37,
        "reference": 41,
        "source": 61,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "String",
              "reference": 95
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              40
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "buzzerOut",
        "notationRole": "usage",
        "owner": 23,
        "reference": 73,
        "source": 77,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BuzzerCommandPort",
              "reference": 36
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "buzzerOn",
        "notationRole": "reference-usage",
        "owner": 39,
        "reference": 37,
        "source": 63,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "states",
            "members": [
              51,
              52,
              53,
              54
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "StateUsage",
        "name": "timerMode",
        "notationRole": "usage",
        "owner": 23,
        "reference": 81,
        "source": 83,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "TimerStateMachine",
              "reference": 12
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "to_expired",
        "notationRole": "usage",
        "owner": 41,
        "reference": 20,
        "source": 29,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "to_running",
        "notationRole": "usage",
        "owner": 41,
        "reference": 24,
        "source": 13,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "decrement_idle",
        "notationRole": "usage",
        "owner": 41,
        "reference": 14,
        "source": 21,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "increment_idle",
        "notationRole": "usage",
        "owner": 41,
        "reference": 17,
        "source": 17,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "to_running_resume",
        "notationRole": "usage",
        "owner": 41,
        "reference": 25,
        "source": 33,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "to_idle_from_paused",
        "notationRole": "usage",
        "owner": 41,
        "reference": 22,
        "source": 37,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "to_idle_from_expired",
        "notationRole": "usage",
        "owner": 41,
        "reference": 21,
        "source": 41,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "TransitionUsage",
        "name": "to_paused",
        "notationRole": "usage",
        "owner": 41,
        "reference": 23,
        "source": 25,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 41,
        "reference": 96,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "idle",
        "notationRole": "usage",
        "owner": 41,
        "reference": 16,
        "source": 5,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Idle",
              "reference": 5
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "paused",
        "notationRole": "usage",
        "owner": 41,
        "reference": 18,
        "source": 9,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Paused",
              "reference": 7
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "expired",
        "notationRole": "usage",
        "owner": 41,
        "reference": 15,
        "source": 11,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Expired",
              "reference": 4
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "running",
        "notationRole": "usage",
        "owner": 41,
        "reference": 19,
        "source": 7,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Running",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "clockFrequency",
        "notationRole": "usage",
        "owner": 23,
        "reference": 74,
        "source": 65,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "FrequencyValue",
              "reference": 92
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "ramSize",
        "notationRole": "usage",
        "owner": 23,
        "reference": 80,
        "source": 69,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Real",
              "reference": 94
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "flashSize",
        "notationRole": "usage",
        "owner": 23,
        "reference": 76,
        "source": 67,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Real",
              "reference": 94
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              59,
              64
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "buttons",
        "notationRole": "usage",
        "owner": 6,
        "reference": 84,
        "source": 119,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ButtonInterface",
              "reference": 47
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              63
            ],
            "provenance": "direct"
          },
          {
            "kind": "attributes",
            "members": [
              60,
              61,
              62
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "pwr",
        "notationRole": "usage",
        "owner": 58,
        "reference": 49,
        "source": 105,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 59,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 59,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 59,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 59,
        "reference": 97,
        "source": 107,
        "typing": {
          "status": "partial",
          "types": []
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              65,
              66,
              67,
              68,
              69
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "output",
        "notationRole": "usage",
        "owner": 58,
        "reference": 48,
        "source": 103,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ButtonInputPort",
              "reference": 30
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "stopPressed",
        "notationRole": "reference-usage",
        "owner": 64,
        "reference": 35,
        "source": 51,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "resetPressed",
        "notationRole": "reference-usage",
        "owner": 64,
        "reference": 33,
        "source": 53,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "startPressed",
        "notationRole": "reference-usage",
        "owner": 64,
        "reference": 34,
        "source": 49,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "decrementPressed",
        "notationRole": "reference-usage",
        "owner": 64,
        "reference": 31,
        "source": 57,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "incrementPressed",
        "notationRole": "reference-usage",
        "owner": 64,
        "reference": 32,
        "source": 55,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 93
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              80
            ],
            "provenance": "inherited"
          },
          {
            "kind": "ports",
            "members": [
              71,
              73,
              78
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "display",
        "notationRole": "usage",
        "owner": 6,
        "reference": 86,
        "source": 117,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Display",
              "reference": 60
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              72
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "cmd",
        "notationRole": "usage",
        "owner": 70,
        "reference": 61,
        "source": 95,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DisplayCommandPort",
              "reference": 38
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "displayValue",
        "notationRole": "reference-usage",
        "owner": 71,
        "reference": 39,
        "source": 59,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "String",
              "reference": 95
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              77
            ],
            "provenance": "direct"
          },
          {
            "kind": "attributes",
            "members": [
              74,
              75,
              76
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "pwr",
        "notationRole": "usage",
        "owner": 70,
        "reference": 64,
        "source": 99,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 73,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 73,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 73,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 73,
        "reference": 100,
        "source": 101,
        "typing": {
          "status": "partial",
          "types": []
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              79
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "lcdIn",
        "notationRole": "usage",
        "owner": 70,
        "reference": 63,
        "source": 97,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "LcdSegmentDrivePort",
              "reference": 40
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "comSegDrive",
        "notationRole": "reference-usage",
        "owner": 78,
        "reference": 41,
        "source": 61,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "String",
              "reference": 95
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "format",
        "notationRole": "usage",
        "owner": 70,
        "reference": 62,
        "source": 93,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "String",
              "reference": 95
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              87
            ],
            "provenance": "inherited"
          },
          {
            "kind": "ports",
            "members": [
              82
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "buzzer",
        "notationRole": "usage",
        "owner": 0,
        "reference": 69,
        "source": 147,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Buzzer",
              "reference": 51
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              86
            ],
            "provenance": "direct"
          },
          {
            "kind": "attributes",
            "members": [
              83,
              84,
              85
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "pwr",
        "notationRole": "usage",
        "owner": 81,
        "reference": 53,
        "source": 111,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 82,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 82,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 82,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 82,
        "reference": 98,
        "source": 113,
        "typing": {
          "status": "partial",
          "types": []
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "duration",
        "notationRole": "usage",
        "owner": 81,
        "reference": 52,
        "source": 109,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DurationValue",
              "reference": 88
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              93,
              94,
              95
            ],
            "provenance": "inherited"
          },
          {
            "kind": "ports",
            "members": [
              89
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartUsage",
        "name": "battery",
        "notationRole": "usage",
        "owner": 0,
        "reference": 68,
        "source": 145,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Battery",
              "reference": 42
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              90,
              91,
              92
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PortUsage",
        "name": "powerOut",
        "notationRole": "usage",
        "owner": 88,
        "reference": 45,
        "source": 141,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "BatteryOutlet",
              "reference": 26
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "maxCurrent",
        "notationRole": "usage",
        "owner": 89,
        "reference": 27,
        "source": 48,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "voltage",
        "notationRole": "usage",
        "owner": 89,
        "reference": 29,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "ReferenceUsage",
        "name": "power",
        "notationRole": "reference-usage",
        "owner": 89,
        "reference": 28,
        "source": 45,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "electricPower",
              "reference": 91
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "nominalVoltage",
        "notationRole": "usage",
        "owner": 88,
        "reference": 44,
        "source": 137,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ElectricPotentialDifferenceValue",
              "reference": 90
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "runtimeEstimate",
        "notationRole": "usage",
        "owner": 88,
        "reference": 46,
        "source": 139,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DurationValue",
              "reference": 88
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "AttributeUsage",
        "name": "capacity",
        "notationRole": "usage",
        "owner": 88,
        "reference": 43,
        "source": 135,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "ElectricChargeValue",
              "reference": 89
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 119,
        "source": 0,
        "target": {
          "reference": 66,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 144,
        "provenance": "authored",
        "reference": 256,
        "source": 6,
        "target": {
          "reference": 82,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 116,
        "provenance": "authored",
        "reference": 321,
        "source": 23,
        "target": {
          "reference": 71,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 80,
        "provenance": "authored",
        "reference": 275,
        "source": 26,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 158,
        "source": 29,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 82,
        "provenance": "authored",
        "reference": 276,
        "source": 30,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 72,
        "provenance": "authored",
        "reference": 262,
        "source": 31,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 52,
        "provenance": "authored",
        "reference": 173,
        "source": 32,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 54,
        "provenance": "authored",
        "reference": 169,
        "source": 33,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 50,
        "provenance": "authored",
        "reference": 171,
        "source": 34,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 58,
        "provenance": "authored",
        "reference": 165,
        "source": 35,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 56,
        "provenance": "authored",
        "reference": 167,
        "source": 36,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 76,
        "provenance": "authored",
        "reference": 270,
        "source": 37,
        "target": {
          "reference": 40,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 62,
        "provenance": "authored",
        "reference": 179,
        "source": 38,
        "target": {
          "reference": 95,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 78,
        "provenance": "authored",
        "reference": 264,
        "source": 39,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 64,
        "provenance": "authored",
        "reference": 175,
        "source": 40,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 84,
        "provenance": "authored",
        "reference": 291,
        "source": 41,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 130,
        "source": 51,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 136,
        "source": 52,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 4,
        "provenance": "authored",
        "reference": 120,
        "source": 50,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 126,
        "source": 53,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 139,
        "source": 54,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 26,
        "provenance": "authored",
        "reference": 149,
        "source": 49,
        "target": {
          "node": 54,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 28,
        "provenance": "authored",
        "reference": 150,
        "source": 49,
        "target": {
          "node": 52,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 27,
        "provenance": "authored",
        "reference": 151,
        "source": 49,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 30,
        "provenance": "authored",
        "reference": 140,
        "source": 42,
        "target": {
          "node": 54,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 32,
        "provenance": "authored",
        "reference": 141,
        "source": 42,
        "target": {
          "node": 53,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 31,
        "provenance": "authored",
        "reference": 142,
        "source": 42,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 14,
        "provenance": "authored",
        "reference": 152,
        "source": 43,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 16,
        "provenance": "authored",
        "reference": 153,
        "source": 43,
        "target": {
          "node": 54,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 15,
        "provenance": "authored",
        "reference": 154,
        "source": 43,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 22,
        "provenance": "authored",
        "reference": 122,
        "source": 44,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 24,
        "provenance": "authored",
        "reference": 123,
        "source": 44,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 23,
        "provenance": "authored",
        "reference": 124,
        "source": 44,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 18,
        "provenance": "authored",
        "reference": 131,
        "source": 45,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 20,
        "provenance": "authored",
        "reference": 132,
        "source": 45,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 19,
        "provenance": "authored",
        "reference": 133,
        "source": 45,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 34,
        "provenance": "authored",
        "reference": 155,
        "source": 46,
        "target": {
          "node": 52,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 36,
        "provenance": "authored",
        "reference": 156,
        "source": 46,
        "target": {
          "node": 54,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 35,
        "provenance": "authored",
        "reference": 157,
        "source": 46,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 38,
        "provenance": "authored",
        "reference": 146,
        "source": 47,
        "target": {
          "node": 52,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 40,
        "provenance": "authored",
        "reference": 147,
        "source": 47,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 39,
        "provenance": "authored",
        "reference": 148,
        "source": 47,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 42,
        "provenance": "authored",
        "reference": 143,
        "source": 48,
        "target": {
          "node": 53,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 44,
        "provenance": "authored",
        "reference": 144,
        "source": 48,
        "target": {
          "node": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 43,
        "provenance": "authored",
        "reference": 145,
        "source": 48,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 74,
        "provenance": "authored",
        "reference": 267,
        "source": 24,
        "target": {
          "reference": 38,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 60,
        "provenance": "authored",
        "reference": 177,
        "source": 25,
        "target": {
          "reference": 95,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 70,
        "provenance": "authored",
        "reference": 277,
        "source": 56,
        "target": {
          "reference": 94,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 68,
        "provenance": "authored",
        "reference": 268,
        "source": 57,
        "target": {
          "reference": 94,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 66,
        "provenance": "authored",
        "reference": 265,
        "source": 55,
        "target": {
          "reference": 92,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 124,
        "provenance": "authored",
        "reference": 292,
        "source": 7,
        "target": {
          "node": 64,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 125,
        "provenance": "authored",
        "reference": 293,
        "source": 7,
        "target": {
          "node": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 127,
        "provenance": "authored",
        "reference": 294,
        "source": 8,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 128,
        "provenance": "authored",
        "reference": 295,
        "source": 8,
        "target": {
          "node": 71,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 130,
        "provenance": "authored",
        "reference": 296,
        "source": 9,
        "target": {
          "node": 37,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 131,
        "provenance": "authored",
        "reference": 297,
        "source": 9,
        "target": {
          "node": 78,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 133,
        "provenance": "authored",
        "reference": 298,
        "source": 10,
        "target": {
          "node": 39,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 134,
        "provenance": "authored",
        "reference": 299,
        "source": 10,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 120,
        "provenance": "authored",
        "reference": 302,
        "source": 58,
        "target": {
          "reference": 47,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 106,
        "provenance": "authored",
        "reference": 198,
        "source": 59,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 159,
        "source": 62,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 108,
        "provenance": "authored",
        "reference": 199,
        "source": 63,
        "target": {
          "node": 60,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 104,
        "provenance": "authored",
        "reference": 193,
        "source": 64,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 52,
        "provenance": "authored",
        "reference": 174,
        "source": 65,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 54,
        "provenance": "authored",
        "reference": 170,
        "source": 66,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 50,
        "provenance": "authored",
        "reference": 172,
        "source": 67,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 58,
        "provenance": "authored",
        "reference": 166,
        "source": 68,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 56,
        "provenance": "authored",
        "reference": 168,
        "source": 69,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 118,
        "provenance": "authored",
        "reference": 311,
        "source": 70,
        "target": {
          "reference": 60,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 96,
        "provenance": "authored",
        "reference": 220,
        "source": 71,
        "target": {
          "reference": 38,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 60,
        "provenance": "authored",
        "reference": 178,
        "source": 72,
        "target": {
          "reference": 95,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 100,
        "provenance": "authored",
        "reference": 228,
        "source": 73,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 160,
        "source": 76,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 102,
        "provenance": "authored",
        "reference": 229,
        "source": 77,
        "target": {
          "node": 74,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 98,
        "provenance": "authored",
        "reference": 223,
        "source": 78,
        "target": {
          "reference": 40,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 62,
        "provenance": "authored",
        "reference": 180,
        "source": 79,
        "target": {
          "reference": 95,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 94,
        "provenance": "authored",
        "reference": 221,
        "source": 80,
        "target": {
          "reference": 95,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 122,
        "provenance": "authored",
        "reference": 306,
        "source": 11,
        "target": {
          "reference": 55,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 88,
        "provenance": "authored",
        "reference": 217,
        "source": 16,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 161,
        "source": 19,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 90,
        "provenance": "authored",
        "reference": 218,
        "source": 20,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 86,
        "provenance": "authored",
        "reference": 212,
        "source": 21,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 64,
        "provenance": "authored",
        "reference": 176,
        "source": 22,
        "target": {
          "reference": 93,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 92,
        "provenance": "authored",
        "reference": 210,
        "source": 12,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 162,
        "source": 15,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 148,
        "provenance": "authored",
        "reference": 247,
        "source": 81,
        "target": {
          "reference": 51,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 112,
        "provenance": "authored",
        "reference": 205,
        "source": 82,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 163,
        "source": 85,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 114,
        "provenance": "authored",
        "reference": 206,
        "source": 86,
        "target": {
          "node": 83,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 110,
        "provenance": "authored",
        "reference": 200,
        "source": 87,
        "target": {
          "reference": 88,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 150,
        "provenance": "authored",
        "reference": 230,
        "source": 1,
        "target": {
          "node": 89,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 151,
        "provenance": "authored",
        "reference": 231,
        "source": 1,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 153,
        "provenance": "authored",
        "reference": 232,
        "source": 2,
        "target": {
          "node": 89,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 154,
        "provenance": "authored",
        "reference": 233,
        "source": 2,
        "target": {
          "node": 73,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 156,
        "provenance": "authored",
        "reference": 234,
        "source": 3,
        "target": {
          "node": 89,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 157,
        "provenance": "authored",
        "reference": 235,
        "source": 3,
        "target": {
          "node": 59,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 159,
        "provenance": "authored",
        "reference": 236,
        "source": 4,
        "target": {
          "node": 89,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 160,
        "provenance": "authored",
        "reference": 237,
        "source": 4,
        "target": {
          "node": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 162,
        "provenance": "authored",
        "reference": 238,
        "source": 5,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 163,
        "provenance": "authored",
        "reference": 239,
        "source": 5,
        "target": {
          "node": 82,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 146,
        "provenance": "authored",
        "reference": 244,
        "source": 88,
        "target": {
          "reference": 42,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 142,
        "provenance": "authored",
        "reference": 186,
        "source": 89,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 46,
        "provenance": "authored",
        "reference": 164,
        "source": 92,
        "target": {
          "reference": 91,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 136,
        "provenance": "authored",
        "reference": 181,
        "source": 95,
        "target": {
          "reference": 89,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 138,
        "provenance": "authored",
        "reference": 182,
        "source": 93,
        "target": {
          "reference": 90,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 140,
        "provenance": "authored",
        "reference": 187,
        "source": 94,
        "target": {
          "reference": 88,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "geometry"
    }
  }
}

~~~
