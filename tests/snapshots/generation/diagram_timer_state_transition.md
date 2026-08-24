# META
~~~ini
description=Timer repository example State Transition View through the packaged diagram WASM guest
type=generate
libraries=standard
repositorySources=examples/timer/KitchenTimer.sysml,examples/timer/KitchenTimerBehavior.sysml,examples/timer/KitchenTimerPorts.sysml,examples/timer/KitchenTimerRequirements.sysml,examples/timer/KitchenTimerStructure.sysml,examples/timer/Views.sysml
plugin=repository:diagram
viewKind=state-transition-view
viewDocument=examples/timer/Views.sysml
viewQualifiedName=Views::timerStateMachine
~~~
# SOURCE
Repository sources are loaded byte-for-byte from the paths declared in META.
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
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
  (document "memory://snapshot/examples/timer/Views.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:fa1595078c56e9e6dd8ba7f575a417b826d1d6663928bfaf44c990e0200c5b3c") (contract-version "lossless-publication-completeness-v3") (admitted (standard-library 94)))
  (declarations
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
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "decrement_idle")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Expired")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Idle")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "idle")) (transitionTrigger (reference "IncrementPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "increment_idle")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Paused")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Running")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "expired")) (transitionTrigger (reference "CountdownComplete")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_expired")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "expired")) (transitionTarget (reference "idle")) (transitionTrigger (reference "ResetPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_expired")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "paused")) (transitionTarget (reference "idle")) (transitionTrigger (reference "ResetPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_paused")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "paused")) (transitionTrigger (reference "StopPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_paused")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")) (transitionTrigger (reference "StartPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "paused")) (transitionTarget (reference "running")) (transitionTrigger (reference "StartPressed")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running_resume")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
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
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimer") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "KitchenTimerBehavior") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "KitchenTimer::timerInstance")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind view) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * Structure only: parts and ports on timerInstance.\n\t\t * Filters follow SysML 7.26 (filter @SysML::Kind); Spec42 evaluates them on expose.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")) (filterMetadataTest (reference "SysML::PartUsage")) (filterMetadataTest (reference "SysML::PartDefinition")) (filterMetadataTest (reference "SysML::PortUsage")) (filterMetadataTest (reference "SysML::PortDefinition")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "KitchenTimer::timerInstance")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (kind view) (membership (kind feature) (visibility default)) (documentation (doc (text " Timer modes: Idle, Running, Paused, Expired and transitions. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "KitchenTimerBehavior::TimerStateMachine")))))
  )
  (references
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
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KitchenTimerBehavior")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "KitchenTimer::timerInstance")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 1))
      (authored-target "SysML::PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 2))
      (authored-target "SysML::PortUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 3))
      (authored-target "SysML::PortDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "KitchenTimer::timerInstance")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "KitchenTimerBehavior::TimerStateMachine")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))))
  )
  (relationships
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
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 1)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 2)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 3)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::CountdownComplete"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::DecrementPressed"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::IncrementPressed"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StopPressed"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "decrement_idle")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "decrement_idle")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "increment_idle")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "increment_idle")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_expired")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_expired")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_expired")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_expired")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_paused")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_paused")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_paused")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_paused")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running_resume")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running_resume")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::power"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::decrementPressed"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::incrementPressed"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::resetPressed"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::startPressed"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort::stopPressed"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort::buzzerOn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort::displayValue"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort::comSegDrive"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "AccuracyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "AccuracyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "AccuracyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::capacity"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::loadCurrent"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate::runtime"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BatteryRuntimeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BatteryRuntimeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BatteryRuntimeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "ButtonResponsivenessReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "ButtonResponsivenessReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "ButtonResponsivenessReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BuzzerAudibilityReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BuzzerAudibilityReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BuzzerAudibilityReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "DisplayFormatReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "DisplayFormatReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "DisplayFormatReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::objective"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::objective"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::objective"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::objective"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::objective"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::objective"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::objective"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::objective"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::objective"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::objective"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "StateConsistencyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "StateConsistencyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "StateConsistencyReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::elapsedTime"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::errorBound"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint::tickRate"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "TimerRangeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "TimerRangeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "TimerRangeReq")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::nominalVoltage"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::powerOut"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buttonIn"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::displayOut"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::lcdDrive"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3))))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (target (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (provenance implied))
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
    (filter (owner (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (form view) (state unsupported) (start 11 9) (end 12 49))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 2)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 3)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 4)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 5)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (path (named (kind package) (name "KitchenTimer")) (anonymous (kind satisfy) (ordinal 6)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::CountdownComplete")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::DecrementPressed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::IncrementPressed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::ResetPressed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StartPressed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::StopPressed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::timerMode")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "decrement_idle")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::decrement_idle")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::expired")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Expired")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::idle")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Idle")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "increment_idle")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::increment_idle")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::paused")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Paused")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::running")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::Running")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_expired")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_expired")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_expired")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_idle_from_paused")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_paused")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_paused")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (path (named (kind package) (name "KitchenTimerBehavior")) (named (kind state-def) (name "TimerStateMachine")) (named (kind transition) (name "to_running_resume")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine::to_running_resume")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::electricPower")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_mechanics.md") (qualified-name "ISQMechanics::PowerValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::scalarQuantities")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::tensorQuantities")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::vectorQuantities")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::voltage")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
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
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "AccuracyReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::AccuracyReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeEstimate")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
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
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BatteryRuntimeReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "ButtonResponsivenessReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::ButtonResponsivenessReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "BuzzerAudibilityReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "DisplayFormatReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::objective")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "StateConsistencyReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::StateConsistencyReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerAccuracyConstraint")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
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
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (path (named (kind package) (name "KitchenTimerRequirements")) (named (kind requirement-def) (name "TimerRangeReq")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::TimerRangeReq")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::User")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::user")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::user")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BatteryRuntimeReq::battery")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::capacity")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_electromagnetism.md") (qualified-name "ISQElectromagnetism::ElectricChargeValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery::runtimeEstimate")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::output")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::ButtonInputPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "ButtonInterface")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::BuzzerAudibilityReq::buzzer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::duration")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Buzzer")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::buzzerPwrOut")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::ctrlIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "BuzzerDriver")) (named (kind port) (name "pwrIn")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver::pwrIn")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::DisplayFormatReq::display")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::cmd")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::DisplayCommandPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::format")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::lcdIn")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::LcdSegmentDrivePort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Display")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::HearAlarm::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::PauseResume::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::QuickStart::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::Reset::timer")) (scopes any))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerRequirements.sysml") (qualified-name "KitchenTimerRequirements::SetAndStartTimer::timer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "KitchenTimer")) (anonymous (kind bare-connect) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::battery")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Battery")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::buzzer")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Buzzer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::buzzerOut")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BuzzerCommandPort")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::clockFrequency")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_space_time.md") (qualified-name "ISQSpaceTime::FrequencyValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::flashSize")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "Microcontroller")) (named (kind port) (name "pwr")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::pwr")))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerPorts.sysml") (qualified-name "KitchenTimerPorts::BatteryOutlet::maxCurrent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller::ramSize")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
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
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::KitchenTimer::pcb")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (path (named (kind package) (name "KitchenTimerStructure")) (named (kind part-def) (name "TimerPCB")) (anonymous (kind bare-connect) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buttons")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::ButtonInterface")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::buzzerDriver")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::BuzzerDriver")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::display")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Display")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB::mcu")))
      (featured-by (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::TimerPCB")))
      (type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (provenance authored))
      (effective-type (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (source direct))
      (supertype (node (document "memory://snapshot/examples/timer/KitchenTimerStructure.sysml") (qualified-name "KitchenTimerStructure::Microcontroller")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure")))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
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
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 2 8) (end 2 23)) (probe (position 2 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimer")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 3 8) (end 3 31)) (probe (position 3 8))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "KitchenTimerBehavior")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 15 20) (end 15 39)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::connections"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 16 9) (end 16 36)) (probe (position 16 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "connections")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "KitchenTimer::timerInstance")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 5 18) (end 5 29)) (probe (position 5 18))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 11 10) (end 11 26)) (probe (position 11 10))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 11 31) (end 11 52)) (probe (position 11 31))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 1) (authored-target "SysML::PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 12 7) (end 12 23)) (probe (position 12 7))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 2) (authored-target "SysML::PortUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 12 28) (end 12 49)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::structure"))) (kind filterMetadataTest) (ordinal 3) (authored-target "SysML::PortDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 10 9) (end 10 36)) (probe (position 10 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "structure")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "KitchenTimer::timerInstance")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimer.sysml") (qualified-name "KitchenTimer::timerInstance")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 19 26) (end 19 45)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (qualified-name "Views::timerStateMachine"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/examples/timer/Views.sysml") (range (start 21 9) (end 21 48)) (probe (position 21 9))
    (reference (id (source (node (document "memory://snapshot/examples/timer/Views.sysml") (path (named (kind package) (name "Views")) (named (kind view) (name "timerStateMachine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "KitchenTimerBehavior::TimerStateMachine")
      (outcome (status resolved) (target (node (document "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml") (qualified-name "KitchenTimerBehavior::TimerStateMachine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:86df9a9d7ad8b167d0e4a4b4841f36bcf7d64b11ce1acea9b979702cc10dfcbb",
  "documents": [
    {
      "uri": "memory://snapshot/examples/timer/KitchenTimerBehavior.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/examples/timer/Views.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/actions.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/states.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        26,
        11,
        26,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        2,
        29,
        12
      ]
    },
    {
      "document": 0,
      "range": [
        29,
        7,
        29,
        11
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        8,
        30,
        12
      ]
    },
    {
      "document": 0,
      "range": [
        30,
        15,
        30,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        8,
        31,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        31,
        18,
        31,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        8,
        32,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        32,
        17,
        32,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        8,
        33,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        33,
        18,
        33,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        2,
        34,
        68
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        13,
        34,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        30,
        34,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        42,
        34,
        54
      ]
    },
    {
      "document": 0,
      "range": [
        34,
        60,
        34,
        67
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        2,
        35,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        13,
        35,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        34,
        35,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        46,
        35,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        35,
        68,
        35,
        72
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        2,
        36,
        73
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        13,
        36,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        34,
        36,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        46,
        36,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        36,
        68,
        36,
        72
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        2,
        37,
        68
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        13,
        37,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        29,
        37,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        44,
        37,
        55
      ]
    },
    {
      "document": 0,
      "range": [
        37,
        61,
        37,
        67
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        2,
        38,
        76
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        13,
        38,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        30,
        38,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        45,
        38,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        38,
        68,
        38,
        75
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        2,
        39,
        77
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        13,
        39,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        37,
        39,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        51,
        39,
        63
      ]
    },
    {
      "document": 0,
      "range": [
        39,
        69,
        39,
        76
      ]
    },
    {
      "document": 0,
      "range": [
        40,
        2,
        40,
        76
      ]
    },
    {
      "document": 0,
      "range": [
        40,
        13,
        40,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        40,
        39,
        40,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        40,
        53,
        40,
        65
      ]
    },
    {
      "document": 0,
      "range": [
        40,
        71,
        40,
        75
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        2,
        41,
        78
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        13,
        41,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        40,
        41,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        55,
        41,
        67
      ]
    },
    {
      "document": 0,
      "range": [
        41,
        73,
        41,
        77
      ]
    },
    {
      "document": 1,
      "range": [
        19,
        6,
        19,
        23
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::CountdownComplete"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::DecrementPressed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Expired"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::IncrementPressed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Paused"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::ResetPressed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::Running"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::StartPressed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::StopPressed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::decrement_idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::decrement_idle::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::expired"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::increment_idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::increment_idle::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::paused"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::running"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_expired"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_expired::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_paused"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_paused::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running_resume"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running_resume::"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Views::timerStateMachine"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Actions::TransitionAction::accepter"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Actions::transitionActions"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::stateActions"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine",
      "source": 1,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::decrement_idle",
      "source": 21,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::increment_idle",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_expired",
      "source": 31,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_expired",
      "source": 46,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_idle_from_paused",
      "source": 41,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_paused",
      "source": 26,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "KitchenTimerBehavior::TimerStateMachine::to_running_resume",
      "source": 36,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "initialState",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "initialState",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "specializes",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "transitionSource",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "transitionTarget",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "transitionTrigger",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "specializes",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "typeFeaturing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "specializes",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "transition",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "specializes",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "transition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "transition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "transition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typeFeaturing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "specializes",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "transitionSource",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "transitionTarget",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "transitionTrigger",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "typeFeaturing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "specializes",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "typeFeaturing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "specializes",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "transition",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "transition",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typeFeaturing",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typing",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "specializes",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "transition",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "transition",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typeFeaturing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typing",
      "source": 19
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "specializes",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "transitionSource",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "transitionTarget",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "transitionTrigger",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "typeFeaturing",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "specializes",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typeFeaturing",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "specializes",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "transitionSource",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "transitionTarget",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "transitionTrigger",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "typeFeaturing",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "specializes",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "typeFeaturing",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "specializes",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "transitionSource",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "transitionTarget",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "transitionTrigger",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "typeFeaturing",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "specializes",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typeFeaturing",
      "source": 25
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "specializes",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "transitionSource",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "transitionTarget",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "transitionTrigger",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typeFeaturing",
      "source": 26
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "specializes",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typeFeaturing",
      "source": 27
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "specializes",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "transitionSource",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "transitionTarget",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "transitionTrigger",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "typeFeaturing",
      "source": 28
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "specializes",
      "source": 29
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "typeFeaturing",
      "source": 29
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "specializes",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "transitionSource",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "transitionTarget",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "transitionTrigger",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "typeFeaturing",
      "source": 30
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "specializes",
      "source": 31
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "typeFeaturing",
      "source": 31
    }
  ],
  "selectedView": {
    "reference": 32,
    "kind": "state-transition-view",
    "name": "timerStateMachine",
    "source": 51
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "authored",
        "reference": 46,
        "source": 0,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 7,
        "provenance": "authored",
        "reference": 47,
        "source": 0,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 1,
        "provenance": "authored",
        "reference": 48,
        "source": 0,
        "target": 17
      },
      {
        "kind": "initial-state",
        "navigation": 2,
        "provenance": "authored",
        "reference": 60,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 9,
        "provenance": "authored",
        "reference": 49,
        "source": 0,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 5,
        "provenance": "authored",
        "reference": 50,
        "source": 0,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 27,
        "provenance": "authored",
        "reference": 51,
        "source": 0,
        "target": 15
      },
      {
        "kind": "transition",
        "navigation": 28,
        "provenance": "implied",
        "reference": 95,
        "source": 21,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 26,
        "provenance": "authored",
        "reference": 123,
        "source": 15,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 32,
        "provenance": "authored",
        "reference": 52,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 33,
        "provenance": "implied",
        "reference": 96,
        "source": 21,
        "target": 20
      },
      {
        "kind": "containment",
        "navigation": 31,
        "provenance": "authored",
        "reference": 99,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 12,
        "provenance": "authored",
        "reference": 53,
        "source": 0,
        "target": 3
      },
      {
        "kind": "transition",
        "navigation": 13,
        "provenance": "implied",
        "reference": 76,
        "source": 18,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 11,
        "provenance": "authored",
        "reference": 131,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 22,
        "provenance": "authored",
        "reference": 54,
        "source": 0,
        "target": 5
      },
      {
        "kind": "transition",
        "navigation": 23,
        "provenance": "implied",
        "reference": 77,
        "source": 18,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 21,
        "provenance": "authored",
        "reference": 63,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 17,
        "provenance": "authored",
        "reference": 55,
        "source": 0,
        "target": 7
      },
      {
        "kind": "transition",
        "navigation": 18,
        "provenance": "implied",
        "reference": 78,
        "source": 18,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 16,
        "provenance": "authored",
        "reference": 81,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 37,
        "provenance": "authored",
        "reference": 56,
        "source": 0,
        "target": 9
      },
      {
        "kind": "transition",
        "navigation": 38,
        "provenance": "implied",
        "reference": 90,
        "source": 19,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 36,
        "provenance": "authored",
        "reference": 139,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 42,
        "provenance": "authored",
        "reference": 57,
        "source": 0,
        "target": 11
      },
      {
        "kind": "transition",
        "navigation": 43,
        "provenance": "implied",
        "reference": 91,
        "source": 19,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 41,
        "provenance": "authored",
        "reference": 115,
        "source": 11,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 47,
        "provenance": "authored",
        "reference": 58,
        "source": 0,
        "target": 13
      },
      {
        "kind": "transition",
        "navigation": 48,
        "provenance": "implied",
        "reference": 72,
        "source": 20,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 46,
        "provenance": "authored",
        "reference": 107,
        "source": 13,
        "target": 14
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [],
      "initialNodes": [
        17
      ],
      "states": [
        0,
        18,
        19,
        20,
        21
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "states",
            "members": [
              18,
              19,
              20,
              21
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "StateDefinition",
        "name": "TimerStateMachine",
        "notationRole": "definition",
        "owner": null,
        "reference": 10,
        "source": 0,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              2
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "to_expired",
        "notationRole": "usage",
        "owner": 0,
        "reference": 20,
        "source": 32,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "reference": 40,
        "source": 31,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              4
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "to_running",
        "notationRole": "usage",
        "owner": 0,
        "reference": 28,
        "source": 12,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 3,
        "reference": 44,
        "source": 11,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              6
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "decrement_idle",
        "notationRole": "usage",
        "owner": 0,
        "reference": 12,
        "source": 22,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 5,
        "reference": 38,
        "source": 21,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              8
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "increment_idle",
        "notationRole": "usage",
        "owner": 0,
        "reference": 16,
        "source": 17,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 7,
        "reference": 39,
        "source": 16,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              10
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "to_running_resume",
        "notationRole": "usage",
        "owner": 0,
        "reference": 30,
        "source": 37,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 9,
        "reference": 45,
        "source": 36,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              12
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "to_idle_from_paused",
        "notationRole": "usage",
        "owner": 0,
        "reference": 24,
        "source": 42,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 11,
        "reference": 42,
        "source": 41,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              14
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "to_idle_from_expired",
        "notationRole": "usage",
        "owner": 0,
        "reference": 22,
        "source": 47,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 13,
        "reference": 41,
        "source": 46,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              16
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "to_paused",
        "notationRole": "usage",
        "owner": 0,
        "reference": 26,
        "source": 27,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 15,
        "reference": 43,
        "source": 26,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 37,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "idle",
        "notationRole": "usage",
        "owner": 0,
        "reference": 15,
        "source": 3,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Idle",
              "reference": 3
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "paused",
        "notationRole": "usage",
        "owner": 0,
        "reference": 18,
        "source": 7,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Paused",
              "reference": 5
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "expired",
        "notationRole": "usage",
        "owner": 0,
        "reference": 14,
        "source": 9,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Expired",
              "reference": 2
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "running",
        "notationRole": "usage",
        "owner": 0,
        "reference": 19,
        "source": 5,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Running",
              "reference": 7
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 0,
        "target": {
          "reference": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 80,
        "source": 18,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 75,
        "source": 18,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 79,
        "source": 18,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 93,
        "source": 19,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 89,
        "source": 19,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 92,
        "source": 19,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 2,
        "provenance": "authored",
        "reference": 61,
        "source": 17,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 17,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 74,
        "source": 20,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 71,
        "source": 20,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 73,
        "source": 20,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 98,
        "source": 21,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 94,
        "source": 21,
        "target": {
          "reference": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 97,
        "source": 21,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 124,
        "source": 15,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 28,
        "provenance": "authored",
        "reference": 125,
        "source": 15,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 30,
        "provenance": "authored",
        "reference": 126,
        "source": 15,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 29,
        "provenance": "authored",
        "reference": 127,
        "source": 15,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 128,
        "source": 15,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 129,
        "source": 16,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 130,
        "source": 16,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 100,
        "source": 1,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 33,
        "provenance": "authored",
        "reference": 101,
        "source": 1,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 35,
        "provenance": "authored",
        "reference": 102,
        "source": 1,
        "target": {
          "node": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 34,
        "provenance": "authored",
        "reference": 103,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 104,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 105,
        "source": 2,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 106,
        "source": 2,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 132,
        "source": 3,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 13,
        "provenance": "authored",
        "reference": 133,
        "source": 3,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 15,
        "provenance": "authored",
        "reference": 134,
        "source": 3,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 14,
        "provenance": "authored",
        "reference": 135,
        "source": 3,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 136,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 137,
        "source": 4,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 138,
        "source": 4,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 64,
        "source": 5,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 23,
        "provenance": "authored",
        "reference": 65,
        "source": 5,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 25,
        "provenance": "authored",
        "reference": 66,
        "source": 5,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 24,
        "provenance": "authored",
        "reference": 67,
        "source": 5,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 68,
        "source": 5,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 69,
        "source": 6,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 6,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 82,
        "source": 7,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 18,
        "provenance": "authored",
        "reference": 83,
        "source": 7,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 20,
        "provenance": "authored",
        "reference": 84,
        "source": 7,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 19,
        "provenance": "authored",
        "reference": 85,
        "source": 7,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 86,
        "source": 7,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 87,
        "source": 8,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 88,
        "source": 8,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 140,
        "source": 9,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 38,
        "provenance": "authored",
        "reference": 141,
        "source": 9,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 40,
        "provenance": "authored",
        "reference": 142,
        "source": 9,
        "target": {
          "node": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 39,
        "provenance": "authored",
        "reference": 143,
        "source": 9,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 144,
        "source": 9,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 145,
        "source": 10,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 146,
        "source": 10,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 116,
        "source": 11,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 43,
        "provenance": "authored",
        "reference": 117,
        "source": 11,
        "target": {
          "node": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 45,
        "provenance": "authored",
        "reference": 118,
        "source": 11,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 44,
        "provenance": "authored",
        "reference": 119,
        "source": 11,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 120,
        "source": 11,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 121,
        "source": 12,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 122,
        "source": 12,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 108,
        "source": 13,
        "target": {
          "reference": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 48,
        "provenance": "authored",
        "reference": 109,
        "source": 13,
        "target": {
          "node": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 50,
        "provenance": "authored",
        "reference": 110,
        "source": 13,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 49,
        "provenance": "authored",
        "reference": 111,
        "source": 13,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 112,
        "source": 13,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 113,
        "source": 14,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 114,
        "source": 14,
        "target": {
          "node": 13,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "frame": {
        "id": "state-machine",
        "label": "TimerStateMachine",
        "navigation": 0
      },
      "kind": "state-transition",
      "transitions": [
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-0",
          "label": null,
          "navigation": 2,
          "provenance": "authored",
          "source": 0,
          "target": 1,
          "trigger": {
            "status": "absent"
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-1",
          "label": "to_paused",
          "navigation": 28,
          "provenance": "implied",
          "source": 4,
          "target": 2,
          "trigger": {
            "label": "StopPressed",
            "navigation": 29,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn11:StopPressed1:0",
              "label": "StopPressed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-2",
          "label": "to_expired",
          "navigation": 33,
          "provenance": "implied",
          "source": 4,
          "target": 3,
          "trigger": {
            "label": "CountdownComplete",
            "navigation": 34,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn17:CountdownComplete1:0",
              "label": "CountdownComplete"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-3",
          "label": "to_running",
          "navigation": 13,
          "provenance": "implied",
          "source": 1,
          "target": 4,
          "trigger": {
            "label": "StartPressed",
            "navigation": 14,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn12:StartPressed1:0",
              "label": "StartPressed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-4",
          "label": "decrement_idle",
          "navigation": 23,
          "provenance": "implied",
          "source": 1,
          "target": 1,
          "trigger": {
            "label": "DecrementPressed",
            "navigation": 24,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn16:DecrementPressed1:0",
              "label": "DecrementPressed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-5",
          "label": "increment_idle",
          "navigation": 18,
          "provenance": "implied",
          "source": 1,
          "target": 1,
          "trigger": {
            "label": "IncrementPressed",
            "navigation": 19,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn16:IncrementPressed1:0",
              "label": "IncrementPressed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-6",
          "label": "to_running_resume",
          "navigation": 38,
          "provenance": "implied",
          "source": 2,
          "target": 4,
          "trigger": {
            "label": "StartPressed",
            "navigation": 39,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn12:StartPressed1:0",
              "label": "StartPressed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-7",
          "label": "to_idle_from_paused",
          "navigation": 43,
          "provenance": "implied",
          "source": 2,
          "target": 1,
          "trigger": {
            "label": "ResetPressed",
            "navigation": 44,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn12:ResetPressed1:0",
              "label": "ResetPressed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-8",
          "label": "to_idle_from_expired",
          "navigation": 48,
          "provenance": "implied",
          "source": 3,
          "target": 1,
          "trigger": {
            "label": "ResetPressed",
            "navigation": 49,
            "status": "accept",
            "target": {
              "id": "element/v159:memory://snapshot/examples/timer/KitchenTimerBehavior.sysml7:packagen20:KitchenTimerBehavior1:08:item-defn12:ResetPressed1:0",
              "label": "ResetPressed"
            }
          }
        }
      ],
      "vertices": [
        {
          "id": "state-0",
          "kind": "initial",
          "label": "",
          "navigation": 1
        },
        {
          "id": "state-1",
          "kind": "state",
          "label": "idle",
          "navigation": 3
        },
        {
          "id": "state-2",
          "kind": "state",
          "label": "paused",
          "navigation": 7
        },
        {
          "id": "state-3",
          "kind": "state",
          "label": "expired",
          "navigation": 9
        },
        {
          "id": "state-4",
          "kind": "state",
          "label": "running",
          "navigation": 5
        }
      ]
    }
  }
}

~~~
